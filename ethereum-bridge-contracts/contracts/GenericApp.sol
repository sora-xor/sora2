// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.15;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./interfaces/IOutboundChannel.sol";

contract GenericApp is AccessControl, ReentrancyGuard {
    IOutboundChannel public outbound;
    address public inbound;

    bytes32 public constant INBOUND_CHANNEL_ROLE =
        keccak256("INBOUND_CHANNEL_ROLE");

    constructor(address inboundChannel, address outboundChannel) {
        require(inboundChannel != address(0), "Invalid inbound channel address");
        require(outboundChannel != address(0), "Invalid outbound channel address");
        _setupRole(DEFAULT_ADMIN_ROLE, inboundChannel);
        _setupRole(INBOUND_CHANNEL_ROLE, inboundChannel);
        outbound = IOutboundChannel(outboundChannel);
        inbound = inboundChannel;
    }
}
