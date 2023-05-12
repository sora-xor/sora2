// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.15;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./interfaces/IRewardSource.sol";
import "./interfaces/ISimplifiedMMRProof.sol";
import "./libraries/ScaleCodec.sol";
import "./BeefyLightClient.sol";

/**
 * @dev The contract was analyzed using Slither static analysis framework. All recommendations have been taken
 * into account and some detectors have been disabled at developers' discretion using `slither-disable-next-line`.
 */
contract InboundChannel is AccessControl, ISimplifiedMMRProof, ReentrancyGuard {
    using ScaleCodec for uint256;
    using ScaleCodec for uint64;
    using ScaleCodec for uint32;
    using ScaleCodec for uint16;
    // the last submitted batch nonce
    uint64 public batch_nonce;
    // storage for light client verification of logs with gas used
    mapping(uint64 => bytes32) public gas_proofs;

    struct Message {
        address target;
        uint256 fee;
        uint256 max_gas;
        bytes payload;
    }

    struct Batch {
        uint256 nonce;
        // Must be equal to sum of `max_gas` in `messages`
        uint256 total_max_gas;
        Message[] messages;
    }

    struct LeafBytes {
        bytes digestPrefix;
        bytes digestSuffix;
        bytes leafPrefix;
    }

    uint256 public constant GAS_BUFFER = 60000;

    // Governance contracts will administer using this role.
    bytes32 public constant CONFIG_UPDATE_ROLE =
        keccak256("CONFIG_UPDATE_ROLE");

    IRewardSource private rewardSource;

    BeefyLightClient public beefyLightClient;

    // Batch of messages was dispatched by relayer
    // - result - message results bitmap
    // - results_length - number of messages were dispatched
    // - gas_spent - gas spent for batch submission. Since event emitted before tx committed, actual gas is greater
    // (at least 10500 gas should be added).
    // - base fee - current block base fee.
    event BatchDispatched(
        uint64 batch_nonce,
        address relayer,
        uint256 results,
        uint256 results_length,
        uint256 gas_spent,
        uint256 base_fee
    );

    constructor(address _beefyLightClient) {
        _setupRole(DEFAULT_ADMIN_ROLE, msg.sender);
        beefyLightClient = BeefyLightClient(_beefyLightClient);
    }

    // Once-off post-construction call to set initial configuration.
    function initialize(address initialRewardSource)
        external
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        // Set initial configuration
        rewardSource = IRewardSource(initialRewardSource);

        // drop admin privileges
        renounceRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function submit(
        Batch calldata batch,
        LeafBytes calldata leafBytes,
        SimplifiedMMRProof calldata proof
    ) external nonReentrant {
        require(batch.messages.length <= 256, "must be <= 256 messages in the batch");

        batch_nonce = batch_nonce + 1;
        // Check batch nonce is correct for replay protection
        require(batch.nonce == batch_nonce, "invalid batch nonce");

        // Since we verify that the batch was generated by substrate, we can safely
        // assume that all fields are correct (for example, `fee`, `max_gas` are
        // appropriate, and `total_max_gas` = `sum(max_gas)`)
        verifyMerkleLeaf(batch, leafBytes, proof);

        // Require there is enough gas to execute all messages
        require(
            gasleft() >= batch.total_max_gas + GAS_BUFFER,
            "insufficient gas for delivery of all messages"
        );

        uint256 results = processMessages(payable(msg.sender), batch.messages);

        uint256 gas_used = block.gaslimit - gasleft();
        gas_proofs[batch_nonce] = keccak256(abi.encodePacked(gas_used, block.basefee));
        emit BatchDispatched(
            batch_nonce,
            msg.sender,
            results,
            batch.messages.length,
            gas_used,
            block.basefee
        );
    }

    function verifyMerkleLeaf(
        Batch calldata batch,
        LeafBytes calldata leafBytes,
        SimplifiedMMRProof calldata proof
    ) internal view {
        bytes32 commitment = keccak256(abi.encode(batch));
        bytes32 digestHash = keccak256(
            bytes.concat(
                leafBytes.digestPrefix,
                block.chainid.encode256(),
                commitment,
                leafBytes.digestSuffix
            )
        );
        bytes32 leafHash = keccak256(
            bytes.concat(leafBytes.leafPrefix, digestHash)
        );

        require(
            beefyLightClient.verifyBeefyMerkleLeaf(leafHash, proof),
            "Invalid proof"
        );
    }

    // - result - message results bitmap, up to 256 messages
    function processMessages(
        address payable relayer,
        Message[] calldata messages
    ) internal returns (uint256 results) {
        uint256 rewardAmount;
        for (uint256 i = 0; i < messages.length; i++) {
            // Deliver the message to the target
            // Delivery will have fixed maximum gas allowed for the target app
            // slither-disable-next-line low-level-calls
            (bool success, ) = messages[i].target.call{
                value: 0,
                gas: messages[i].max_gas
            }(messages[i].payload);
            results |= uint256(success ? 1 : 0) << i;
            rewardAmount = rewardAmount + messages[i].fee;
        }
        // reward the relayer
        rewardSource.reward(relayer, rewardAmount);
        return results;
    }
}
