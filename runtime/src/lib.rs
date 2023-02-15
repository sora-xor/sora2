// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

extern crate alloc;
use alloc::string::String;

mod bags_thresholds;
/// Constant values used within the runtime.
pub mod constants;
mod extensions;
mod impls;
pub mod migrations;

#[cfg(test)]
pub mod mock;

#[cfg(test)]
pub mod tests;

use crate::impls::{BridgeAssetRegistryImpl, PreimageWeightInfo, SubstrateBridgeCallFilter};
use bridge_types::types::{AdditionalEVMInboundData, LeafExtraData, ParachainMessage};
use common::prelude::constants::{BIG_FEE, SMALL_FEE};
use common::prelude::QuoteAmount;
use common::{AssetId32, Description, PredefinedAssetId, XOR};
use constants::currency::deposit;
use constants::time::*;
use frame_support::instances::{Instance1, Instance2};
use frame_support::weights::ConstantMultiplier;

// Make the WASM binary available.
#[cfg(all(feature = "std", feature = "build-wasm-binary"))]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub use beefy_primitives::crypto::AuthorityId as BeefyId;
use beefy_primitives::mmr::MmrLeafVersion;
use core::time::Duration;
use currencies::BasicCurrencyAdapter;
use extensions::ChargeTransactionPayment;
use frame_election_provider_support::{generate_solution_type, onchain, SequentialPhragmen};
use frame_support::traits::{ConstU128, ConstU32, Currency, EitherOfDiverse};
use frame_system::offchain::{Account, SigningTypes};
use frame_system::EnsureRoot;
use hex_literal::hex;
use pallet_grandpa::{
    fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
use pallet_session::historical as pallet_session_historical;
use pallet_staking::sora::ValBurnedNotifier;
#[cfg(feature = "std")]
use serde::{Serialize, Serializer};
use sp_api::impl_runtime_apis;
use sp_core::crypto::KeyTypeId;
use sp_core::{Encode, OpaqueMetadata, H160, U256};
use sp_mmr_primitives as mmr;
use sp_runtime::traits::{
    BlakeTwo256, Block as BlockT, Convert, IdentifyAccount, IdentityLookup, NumberFor, OpaqueKeys,
    SaturatedConversion, Verify,
};
use sp_runtime::transaction_validity::{
    TransactionLongevity, TransactionPriority, TransactionSource, TransactionValidity,
};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys, ApplyExtrinsicResult, DispatchError,
    FixedPointNumber, MultiSignature, Perbill, Percent, Perquintill,
};
use sp_std::cmp::Ordering;
use sp_std::prelude::*;
use sp_std::vec::Vec;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
use static_assertions::assert_eq_size;
use traits::parameter_type_with_key;

// A few exports that help ease life for downstream crates.
pub use common::prelude::{
    Balance, BalanceWrapper, PresetWeightInfo, SwapAmount, SwapOutcome, SwapVariant,
};
pub use common::weights::{BlockLength, BlockWeights, TransactionByteFee};
pub use common::{
    balance, fixed, fixed_from_basis_points, AssetName, AssetSymbol, BalancePrecision, BasisPoints,
    ContentSource, FilterMode, Fixed, FromGenericPair, LiquiditySource, LiquiditySourceFilter,
    LiquiditySourceId, LiquiditySourceType, OnPswapBurned, OnValBurned,
};
use constants::rewards::{PSWAP_BURN_PERCENT, VAL_BURN_PERCENT};
pub use ethereum_light_client::EthereumHeader;
pub use frame_support::dispatch::DispatchClass;
pub use frame_support::traits::schedule::Named as ScheduleNamed;
pub use frame_support::traits::{
    KeyOwnerProofSystem, LockIdentifier, OnUnbalanced, Randomness, U128CurrencyToVote,
};
pub use frame_support::weights::constants::{
    BlockExecutionWeight, RocksDbWeight, WEIGHT_PER_SECOND,
};
pub use frame_support::weights::Weight;
pub use frame_support::{construct_runtime, debug, parameter_types, StorageValue};
pub use pallet_balances::Call as BalancesCall;
pub use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
pub use pallet_staking::StakerStatus;
pub use pallet_timestamp::Call as TimestampCall;
pub use pallet_transaction_payment::{Multiplier, MultiplierUpdate};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use vested_rewards::CrowdloanReward;

use eth_bridge::offchain::SignatureParams;
use eth_bridge::requests::{AssetKind, OffchainRequest, OutgoingRequestEncoded, RequestStatus};
use impls::{
    CollectiveWeightInfo, DemocracyWeightInfo, DispatchableSubstrateBridgeCall,
    NegativeImbalanceOf, OnUnbalancedDemocracySlash,
};

use frame_support::traits::{
    Contains, Everything, ExistenceRequirement, Get, PrivilegeCmp, WithdrawReasons,
};
use sp_runtime::traits::Keccak256;
pub use {assets, eth_bridge, frame_system, multicollateral_bonding_curve_pool, xst};

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// This assert is needed for `technical` pallet in order to create
// `AccountId` from the hash type.
assert_eq_size!(AccountId, sp_core::H256);

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem;

/// Identification of DEX.
pub type DEXId = u32;

pub type Moment = u64;

pub type PeriodicSessions = pallet_session::PeriodicSessions<SessionPeriod, SessionOffset>;

type CouncilCollective = pallet_collective::Instance1;
type TechnicalCollective = pallet_collective::Instance2;

type MoreThanHalfCouncil = EitherOfDiverse<
    EnsureRoot<AccountId>,
    pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
>;
type AtLeastHalfCouncil = EitherOfDiverse<
    pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>,
    EnsureRoot<AccountId>,
>;
type AtLeastTwoThirdsCouncil = EitherOfDiverse<
    pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>,
    EnsureRoot<AccountId>,
>;

type SlashCancelOrigin = EitherOfDiverse<
    EnsureRoot<AccountId>,
    pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>,
>;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core datastructures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub babe: Babe,
            pub grandpa: Grandpa,
            pub im_online: ImOnline,
            pub beefy: Beefy,
        }
    }
}

/// Types used by oracle related pallets
pub mod oracle_types {
    use common::SymbolName;

    pub type Symbol = SymbolName;

    pub type ResolveTime = u64;
}
pub use oracle_types::*;

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("sora-substrate"),
    impl_name: create_runtime_str!("sora-substrate"),
    authoring_version: 1,
    spec_version: 44,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 44,
    state_version: 0,
};

/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

pub const FARMING_PSWAP_PER_DAY: Balance = balance!(2500000);
pub const FARMING_REFRESH_FREQUENCY: BlockNumber = 2 * HOURS;
// Defined in the article
pub const FARMING_VESTING_COEFF: u32 = 3;
pub const FARMING_VESTING_FREQUENCY: BlockNumber = 6 * HOURS;

parameter_types! {
    pub const BlockHashCount: BlockNumber = 250;
    pub const Version: RuntimeVersion = VERSION;
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
    pub const EpochDuration: u64 = EPOCH_DURATION_IN_BLOCKS as u64;
    pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
    pub const UncleGenerations: BlockNumber = 0;
    pub const SessionsPerEra: sp_staking::SessionIndex = 6; // 6 hours
    pub const BondingDuration: sp_staking::EraIndex = 28; // 28 eras for unbonding (7 days).
    pub const ReportLongevity: u64 =
        BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
    pub const SlashDeferDuration: sp_staking::EraIndex = 27; // 27 eras in which slashes can be cancelled (slightly less than 7 days).
    pub const MaxNominatorRewardedPerValidator: u32 = 256;
    pub const ElectionLookahead: BlockNumber = EPOCH_DURATION_IN_BLOCKS / 4;
    pub const MaxIterations: u32 = 10;
    // 0.05%. The higher the value, the more strict solution acceptance becomes.
    pub MinSolutionScoreBump: Perbill = Perbill::from_rational(5u32, 10_000);
    pub const ValRewardCurve: pallet_staking::sora::ValRewardCurve = pallet_staking::sora::ValRewardCurve {
        duration_to_reward_flatline: Duration::from_secs(5 * 365 * 24 * 60 * 60),
        min_val_burned_percentage_reward: Percent::from_percent(35),
        max_val_burned_percentage_reward: Percent::from_percent(90),
    };
    pub const SessionPeriod: BlockNumber = 150;
    pub const SessionOffset: BlockNumber = 0;
    pub const SS58Prefix: u8 = 69;
    /// A limit for off-chain phragmen unsigned solution submission.
    ///
    /// We want to keep it as high as possible, but can't risk having it reject,
    /// so we always subtract the base block execution weight.
    pub OffchainSolutionWeightLimit: Weight = BlockWeights::get()
    .get(DispatchClass::Normal)
    .max_extrinsic
    .expect("Normal extrinsics have weight limit configured by default; qed")
    .saturating_sub(BlockExecutionWeight::get());
    /// A limit for off-chain phragmen unsigned solution length.
    ///
    /// We allow up to 90% of the block's size to be consumed by the solution.
    pub OffchainSolutionLengthLimit: u32 = Perbill::from_rational(90_u32, 100) *
        *BlockLength::get()
        .max
        .get(DispatchClass::Normal);
    pub const DemocracyEnactmentPeriod: BlockNumber = 30 * DAYS;
    pub const DemocracyLaunchPeriod: BlockNumber = 28 * DAYS;
    pub const DemocracyVotingPeriod: BlockNumber = 14 * DAYS;
    pub const DemocracyMinimumDeposit: Balance = balance!(1);
    pub const DemocracyFastTrackVotingPeriod: BlockNumber = 3 * HOURS;
    pub const DemocracyInstantAllowed: bool = true;
    pub const DemocracyCooloffPeriod: BlockNumber = 28 * DAYS;
    pub const DemocracyPreimageByteDeposit: Balance = balance!(0.000002); // 2 * 10^-6, 5 MiB -> 10.48576 XOR
    pub const DemocracyMaxVotes: u32 = 100;
    pub const DemocracyMaxProposals: u32 = 100;
    pub const DemocracyMaxDeposits: u32 = 100; // todo
    pub const DemocracyMaxBlacklisted: u32 = 100; // todo
    pub const CouncilCollectiveMotionDuration: BlockNumber = 5 * DAYS;
    pub const CouncilCollectiveMaxProposals: u32 = 100;
    pub const CouncilCollectiveMaxMembers: u32 = 100;
    pub const TechnicalCollectiveMotionDuration: BlockNumber = 5 * DAYS;
    pub const TechnicalCollectiveMaxProposals: u32 = 100;
    pub const TechnicalCollectiveMaxMembers: u32 = 100;
    pub const SchedulerMaxWeight: Weight = Weight::from_ref_time(1024);
    pub OffencesWeightSoftLimit: Weight = Perbill::from_percent(60) * BlockWeights::get().max_block;
    pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
    pub const SessionDuration: BlockNumber = EPOCH_DURATION_IN_BLOCKS;
    pub const ElectionsCandidacyBond: Balance = balance!(1);
    // 1 storage item created, key size is 32 bytes, value size is 16+16.
    pub const ElectionsVotingBondBase: Balance = balance!(0.000001);
    // additional data per vote is 32 bytes (account id).
    pub const ElectionsVotingBondFactor: Balance = balance!(0.000001);
    pub const ElectionsTermDuration: BlockNumber = 7 * DAYS;
    /// 13 members initially, to be increased to 23 eventually.
    pub const ElectionsDesiredMembers: u32 = 13;
    pub const ElectionsDesiredRunnersUp: u32 = 20;
    pub const ElectionsModuleId: LockIdentifier = *b"phrelect";
    pub FarmingRewardDoublingAssets: Vec<AssetId> = vec![GetPswapAssetId::get(), GetValAssetId::get(), GetDaiAssetId::get(), GetEthAssetId::get(), GetXstAssetId::get()];
    pub const MaxAuthorities: u32 = 100_000;
    pub const NoPreimagePostponement: Option<u32> = Some(10);
}

impl frame_system::Config for Runtime {
    type BaseCallFilter = Everything;
    type BlockWeights = BlockWeights;
    /// Maximum size of all encoded transactions (in bytes) that are allowed in one block.
    type BlockLength = BlockLength;
    /// The ubiquitous origin type.
    type RuntimeOrigin = RuntimeOrigin;
    /// The aggregated dispatch type that is available for extrinsics.
    type RuntimeCall = RuntimeCall;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = IdentityLookup<AccountId>;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// Runtime version.
    type Version = Version;
    type PalletInfo = PalletInfo;
    /// Converts a module to an index of this module in the runtime.
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<65536>;
}

impl pallet_babe::Config for Runtime {
    type EpochDuration = EpochDuration;
    type ExpectedBlockTime = ExpectedBlockTime;
    type EpochChangeTrigger = pallet_babe::ExternalTrigger;
    type DisabledValidators = Session;
    type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        pallet_babe::AuthorityId,
    )>>::Proof;
    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        pallet_babe::AuthorityId,
    )>>::IdentificationTuple;
    type KeyOwnerProofSystem = Historical;
    type HandleEquivocation =
        pallet_babe::EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;
    type WeightInfo = ();
    type MaxAuthorities = MaxAuthorities;
}

impl pallet_collective::Config<CouncilCollective> for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type Proposal = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type MotionDuration = CouncilCollectiveMotionDuration;
    type MaxProposals = CouncilCollectiveMaxProposals;
    type MaxMembers = CouncilCollectiveMaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
    type WeightInfo = CollectiveWeightInfo<Self>;
}

impl pallet_collective::Config<TechnicalCollective> for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type Proposal = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type MotionDuration = TechnicalCollectiveMotionDuration;
    type MaxProposals = TechnicalCollectiveMaxProposals;
    type MaxMembers = TechnicalCollectiveMaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
    type WeightInfo = CollectiveWeightInfo<Self>;
}

impl pallet_democracy::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type EnactmentPeriod = DemocracyEnactmentPeriod;
    type LaunchPeriod = DemocracyLaunchPeriod;
    type VotingPeriod = DemocracyVotingPeriod;
    type MinimumDeposit = DemocracyMinimumDeposit;
    /// `external_propose` call condition
    type ExternalOrigin = AtLeastHalfCouncil;
    /// A super-majority can have the next scheduled referendum be a straight majority-carries vote.
    /// `external_propose_majority` call condition
    type ExternalMajorityOrigin = AtLeastHalfCouncil;
    /// `external_propose_default` call condition
    type ExternalDefaultOrigin = AtLeastHalfCouncil;
    /// Two thirds of the technical committee can have an ExternalMajority/ExternalDefault vote
    /// be tabled immediately and with a shorter voting/enactment period.
    type FastTrackOrigin = EitherOfDiverse<
        pallet_collective::EnsureProportionMoreThan<AccountId, TechnicalCollective, 1, 2>,
        EnsureRoot<AccountId>,
    >;
    type InstantOrigin = EitherOfDiverse<
        pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 2, 3>,
        EnsureRoot<AccountId>,
    >;
    type InstantAllowed = DemocracyInstantAllowed;
    type FastTrackVotingPeriod = DemocracyFastTrackVotingPeriod;
    /// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
    /// `emergency_cancel` call condition.
    type CancellationOrigin = AtLeastTwoThirdsCouncil;
    type CancelProposalOrigin = AtLeastTwoThirdsCouncil;
    type BlacklistOrigin = EnsureRoot<AccountId>;
    /// `veto_external` - vetoes and blacklists the external proposal hash
    type VetoOrigin = pallet_collective::EnsureMember<AccountId, TechnicalCollective>;
    type CooloffPeriod = DemocracyCooloffPeriod;
    type Slash = OnUnbalancedDemocracySlash<Self>;
    type Scheduler = Scheduler;
    type PalletsOrigin = OriginCaller;
    type MaxVotes = DemocracyMaxVotes;
    type WeightInfo = DemocracyWeightInfo;
    type MaxProposals = DemocracyMaxProposals;
    type VoteLockingPeriod = DemocracyEnactmentPeriod;
    type Preimages = Preimage;
    type MaxDeposits = DemocracyMaxDeposits;
    type MaxBlacklisted = DemocracyMaxBlacklisted;
}

impl pallet_elections_phragmen::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type PalletId = ElectionsModuleId;
    type Currency = Balances;
    type ChangeMembers = Council;
    type InitializeMembers = Council;
    type CurrencyToVote = frame_support::traits::U128CurrencyToVote;
    type CandidacyBond = ElectionsCandidacyBond;
    type VotingBondBase = ElectionsVotingBondBase;
    type VotingBondFactor = ElectionsVotingBondFactor;
    type LoserCandidate = OnUnbalancedDemocracySlash<Self>;
    type KickedMember = OnUnbalancedDemocracySlash<Self>;
    type DesiredMembers = ElectionsDesiredMembers;
    type DesiredRunnersUp = ElectionsDesiredRunnersUp;
    type TermDuration = ElectionsTermDuration;
    type MaxVoters = ();
    type MaxCandidates = ();
    type WeightInfo = ();
}

impl pallet_membership::Config<pallet_membership::Instance1> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type AddOrigin = MoreThanHalfCouncil;
    type RemoveOrigin = MoreThanHalfCouncil;
    type SwapOrigin = MoreThanHalfCouncil;
    type ResetOrigin = MoreThanHalfCouncil;
    type PrimeOrigin = MoreThanHalfCouncil;
    type MembershipInitialized = TechnicalCommittee;
    type MembershipChanged = TechnicalCommittee;
    type MaxMembers = ();
    type WeightInfo = ();
}

impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;

    type KeyOwnerProofSystem = Historical;

    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;

    type HandleEquivocation = pallet_grandpa::EquivocationHandler<
        Self::KeyOwnerIdentification,
        Offences,
        ReportLongevity,
    >;
    type WeightInfo = ();
    type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = Moment;
    type OnTimestampSet = Babe;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl pallet_session::Config for Runtime {
    type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, XorFee>;
    type Keys = opaque::SessionKeys;
    type ShouldEndSession = Babe;
    type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = AccountId;
    type ValidatorIdOf = pallet_staking::StashOf<Self>;
    type NextSessionRotation = Babe;
    type WeightInfo = ();
}

impl pallet_session::historical::Config for Runtime {
    type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
    type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}

impl pallet_authorship::Config for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = (Staking, ImOnline);
}

/// A reasonable benchmarking config for staking pallet.
pub struct StakingBenchmarkingConfig;
impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
    type MaxValidators = ConstU32<1000>;
    type MaxNominators = ConstU32<1000>;
}

parameter_types! {
    pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17);
    pub const MaxNominations: u32 = <NposCompactSolution24 as frame_election_provider_support::NposSolution>::LIMIT as u32;
}

impl pallet_staking::Config for Runtime {
    type Currency = Balances;
    type MultiCurrency = Tokens;
    type CurrencyBalance = Balance;
    type ValTokenId = GetValAssetId;
    type ValRewardCurve = ValRewardCurve;
    type UnixTime = Timestamp;
    type CurrencyToVote = U128CurrencyToVote;
    type RuntimeEvent = RuntimeEvent;
    type Slash = ();
    type SessionsPerEra = SessionsPerEra;
    type BondingDuration = BondingDuration;
    type SlashDeferDuration = SlashDeferDuration;
    type SlashCancelOrigin = SlashCancelOrigin;
    type SessionInterface = Self;
    type NextNewSession = Session;
    type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
    type VoterList = BagsList;
    type ElectionProvider = ElectionProviderMultiPhase;
    type BenchmarkingConfig = StakingBenchmarkingConfig;
    type MaxUnlockingChunks = ConstU32<32>;
    type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
    type MaxNominations = MaxNominations;
    type GenesisElectionProvider = onchain::UnboundedExecution<OnChainSeqPhragmen>;
    type OnStakerSlash = ();
    type WeightInfo = ();
}

/// The numbers configured here could always be more than the the maximum limits of staking pallet
/// to ensure election snapshot will not run out of memory. For now, we set them to smaller values
/// since the staking is bounded and the weight pipeline takes hours for this single pallet.
pub struct ElectionBenchmarkConfig;
impl pallet_election_provider_multi_phase::BenchmarkingConfig for ElectionBenchmarkConfig {
    const VOTERS: [u32; 2] = [1000, 2000];
    const TARGETS: [u32; 2] = [500, 1000];
    const ACTIVE_VOTERS: [u32; 2] = [500, 800];
    const DESIRED_TARGETS: [u32; 2] = [200, 400];
    const SNAPSHOT_MAXIMUM_VOTERS: u32 = 1000;
    const MINER_MAXIMUM_VOTERS: u32 = 1000;
    const MAXIMUM_TARGETS: u32 = 300;
}

parameter_types! {
    // phase durations. 1/4 of the last session for each.
    // in testing: 1min or half of the session for each
    pub SignedPhase: u32 = EPOCH_DURATION_IN_BLOCKS / 4;
    pub UnsignedPhase: u32 = EPOCH_DURATION_IN_BLOCKS / 4;

    // signed config
    pub const SignedMaxSubmissions: u32 = 16;
    pub const SignedMaxRefunds: u32 = 16 / 4;
    pub const SignedDepositBase: Balance = deposit(2, 0);
    pub const SignedDepositByte: Balance = deposit(0, 10) / 1024;
    pub SignedRewardBase: Balance =  constants::currency::UNITS / 10;
    pub SolutionImprovementThreshold: Perbill = Perbill::from_rational(5u32, 10_000);
    pub BetterUnsignedThreshold: Perbill = Perbill::from_rational(5u32, 10_000);

    // 1 hour session, 15 minutes unsigned phase, 8 offchain executions.
    pub OffchainRepeat: BlockNumber = UnsignedPhase::get() / 8;

    /// We take the top 12500 nominators as electing voters..
    pub const MaxElectingVoters: u32 = 12_500;
    /// ... and all of the validators as electable targets. Whilst this is the case, we cannot and
    /// shall not increase the size of the validator intentions.
    pub const MaxElectableTargets: u16 = u16::MAX;
    pub NposSolutionPriority: TransactionPriority =
        Perbill::from_percent(90) * TransactionPriority::max_value();
}

generate_solution_type!(
    #[compact]
    pub struct NposCompactSolution24::<
        VoterIndex = u32,
        TargetIndex = u16,
        Accuracy = sp_runtime::PerU16,
        MaxVoters = MaxElectingVoters,
    >(24)
);

/// The accuracy type used for genesis election provider;
pub type OnChainAccuracy = sp_runtime::Perbill;

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
    type System = Runtime;
    type Solver = SequentialPhragmen<AccountId, OnChainAccuracy>;
    type DataProvider = Staking;
    type WeightInfo = ();
}

impl pallet_election_provider_multi_phase::MinerConfig for Runtime {
    type AccountId = AccountId;
    type MaxLength = OffchainSolutionLengthLimit;
    type MaxWeight = OffchainSolutionWeightLimit;
    type Solution = NposCompactSolution24;
    type MaxVotesPerVoter = <
		<Self as pallet_election_provider_multi_phase::Config>::DataProvider
		as
		frame_election_provider_support::ElectionDataProvider
	>::MaxVotesPerVoter;

    // The unsigned submissions have to respect the weight of the submit_unsigned call, thus their
    // weight estimate function is wired to this call's weight.
    fn solution_weight(v: u32, t: u32, a: u32, d: u32) -> Weight {
        <
			<Self as pallet_election_provider_multi_phase::Config>::WeightInfo
			as
			pallet_election_provider_multi_phase::WeightInfo
		>::submit_unsigned(v, t, a, d)
    }
}

impl pallet_election_provider_multi_phase::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type EstimateCallFee = TransactionPayment;
    type UnsignedPhase = UnsignedPhase;
    type SignedMaxSubmissions = SignedMaxSubmissions;
    type SignedMaxRefunds = SignedMaxRefunds;
    type SignedRewardBase = SignedRewardBase;
    type SignedDepositBase = SignedDepositBase;
    type SignedDepositByte = SignedDepositByte;
    type SignedDepositWeight = ();
    type SignedMaxWeight =
        <Self::MinerConfig as pallet_election_provider_multi_phase::MinerConfig>::MaxWeight;
    type MinerConfig = Self;
    type SlashHandler = (); // burn slashes
    type RewardHandler = (); // nothing to do upon rewards
    type SignedPhase = SignedPhase;
    type BetterUnsignedThreshold = BetterUnsignedThreshold;
    type BetterSignedThreshold = ();
    type OffchainRepeat = OffchainRepeat;
    type MinerTxPriority = NposSolutionPriority;
    type DataProvider = Staking;
    type Fallback = pallet_election_provider_multi_phase::NoFallback<Self>;
    type GovernanceFallback = onchain::UnboundedExecution<OnChainSeqPhragmen>;
    type Solver = SequentialPhragmen<
        AccountId,
        pallet_election_provider_multi_phase::SolutionAccuracyOf<Self>,
        (),
    >;
    type BenchmarkingConfig = ElectionBenchmarkConfig;
    type ForceOrigin = EitherOfDiverse<
        EnsureRoot<AccountId>,
        pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>,
    >;
    type WeightInfo = ();
    type MaxElectingVoters = MaxElectingVoters;
    type MaxElectableTargets = MaxElectableTargets;
}

parameter_types! {
    pub const BagThresholds: &'static [u64] = &bags_thresholds::THRESHOLDS;
}

impl pallet_bags_list::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ScoreProvider = Staking;
    type WeightInfo = ();
    type BagThresholds = BagThresholds;
    type Score = sp_npos_elections::VoteWeight;
}

/// Used the compare the privilege of an origin inside the scheduler.
pub struct OriginPrivilegeCmp;

impl PrivilegeCmp<OriginCaller> for OriginPrivilegeCmp {
    fn cmp_privilege(left: &OriginCaller, right: &OriginCaller) -> Option<Ordering> {
        if left == right {
            return Some(Ordering::Equal);
        }

        match (left, right) {
            // Root is greater than anything.
            (OriginCaller::system(frame_system::RawOrigin::Root), _) => Some(Ordering::Greater),
            // Check which one has more yes votes.
            (
                OriginCaller::Council(pallet_collective::RawOrigin::Members(l_yes_votes, l_count)),
                OriginCaller::Council(pallet_collective::RawOrigin::Members(r_yes_votes, r_count)),
            ) => Some((l_yes_votes * r_count).cmp(&(r_yes_votes * l_count))),
            // For every other origin we don't care, as they are not used for `ScheduleOrigin`.
            _ => None,
        }
    }
}

impl pallet_scheduler::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type MaximumWeight = SchedulerMaxWeight;
    type ScheduleOrigin = frame_system::EnsureRoot<AccountId>;
    type MaxScheduledPerBlock = ();
    type WeightInfo = ();
    type OriginPrivilegeCmp = OriginPrivilegeCmp;
    type Preimages = Preimage;
}

parameter_types! {
    pub PreimageBaseDeposit: Balance = deposit(2, 64);
    pub PreimageByteDeposit: Balance = deposit(0, 1);
}

impl pallet_preimage::Config for Runtime {
    type WeightInfo = PreimageWeightInfo;
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ManagerOrigin = EnsureRoot<AccountId>;
    type BaseDeposit = PreimageBaseDeposit;
    type ByteDeposit = PreimageByteDeposit;
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 0;
    pub const TransferFee: u128 = 0;
    pub const CreationFee: u128 = 0;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    /// The type for recording an account's balance.
    type Balance = Balance;
    type DustRemoval = ();
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
    type ReserveIdentifier = ();
}

pub type Amount = i128;

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
        0
    };
}

impl tokens::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = AssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
    type OnSlash = ();
    type OnDeposit = ();
    type OnTransfer = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
    type OnNewTokenAccount = ();
    type OnKilledTokenAccount = ();
    type DustRemovalWhitelist = Everything;
}

parameter_types! {
    // This is common::PredefinedAssetId with 0 index, 2 is size, 0 and 0 is code.
    pub const GetXorAssetId: AssetId = common::AssetId32::from_bytes(hex!("0200000000000000000000000000000000000000000000000000000000000000"));
    pub const GetDotAssetId: AssetId = common::AssetId32::from_bytes(hex!("0200010000000000000000000000000000000000000000000000000000000000"));
    pub const GetKsmAssetId: AssetId = common::AssetId32::from_bytes(hex!("0200020000000000000000000000000000000000000000000000000000000000"));
    pub const GetUsdAssetId: AssetId = common::AssetId32::from_bytes(hex!("0200030000000000000000000000000000000000000000000000000000000000"));
    pub const GetValAssetId: AssetId = common::AssetId32::from_bytes(hex!("0200040000000000000000000000000000000000000000000000000000000000"));
    pub const GetPswapAssetId: AssetId = common::AssetId32::from_bytes(hex!("0200050000000000000000000000000000000000000000000000000000000000"));
    pub const GetDaiAssetId: AssetId = common::AssetId32::from_bytes(hex!("0200060000000000000000000000000000000000000000000000000000000000"));
    pub const GetEthAssetId: AssetId = common::AssetId32::from_bytes(hex!("0200070000000000000000000000000000000000000000000000000000000000"));
    pub const GetXstAssetId: AssetId = common::AssetId32::from_bytes(hex!("0200090000000000000000000000000000000000000000000000000000000000"));

    pub const GetBaseAssetId: AssetId = GetXorAssetId::get();
    pub const GetBuyBackAssetId: AssetId = GetXstAssetId::get();
    pub GetBuyBackSupplyAssets: Vec<AssetId> = vec![GetValAssetId::get(), GetPswapAssetId::get()];
    pub const GetBuyBackPercentage: u8 = 10;
    pub const GetBuyBackAccountId: AccountId = AccountId::new(hex!("feb92c0acb61f75309730290db5cbe8ac9b46db7ad6f3bbb26a550a73586ea71"));
    pub const GetBuyBackDexId: DEXId = 0;
    pub const GetSyntheticBaseAssetId: AssetId = GetXstAssetId::get();
}

impl currencies::Config for Runtime {
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = <Runtime as assets::Config>::GetBaseAssetId;
    type WeightInfo = ();
}

impl common::Config for Runtime {
    type DEXId = DEXId;
    type LstId = common::LiquiditySourceType;
}

pub struct GetTotalBalance;

impl assets::GetTotalBalance<Runtime> for GetTotalBalance {
    fn total_balance(asset_id: &AssetId, who: &AccountId) -> Result<Balance, DispatchError> {
        if asset_id == &GetXorAssetId::get() {
            Ok(Referrals::referrer_balance(who).unwrap_or(0))
        } else {
            Ok(0)
        }
    }
}

impl assets::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ExtraAccountId = [u8; 32];
    type ExtraAssetRecordArg =
        common::AssetIdExtraAssetRecordArg<DEXId, common::LiquiditySourceType, [u8; 32]>;
    type AssetId = AssetId;
    type GetBaseAssetId = GetBaseAssetId;
    type GetBuyBackAssetId = GetBuyBackAssetId;
    type GetBuyBackSupplyAssets = GetBuyBackSupplyAssets;
    type GetBuyBackPercentage = GetBuyBackPercentage;
    type GetBuyBackAccountId = GetBuyBackAccountId;
    type GetBuyBackDexId = GetBuyBackDexId;
    type BuyBackLiquidityProxy = liquidity_proxy::Pallet<Runtime>;
    type Currency = currencies::Pallet<Runtime>;
    type GetTotalBalance = GetTotalBalance;
    type WeightInfo = assets::weights::WeightInfo<Runtime>;
}

impl trading_pair::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type EnsureDEXManager = dex_manager::Pallet<Runtime>;
    type WeightInfo = ();
}

impl dex_manager::Config for Runtime {}

pub type TechAccountId = common::TechAccountId<AccountId, TechAssetId, DEXId>;
pub type TechAssetId = common::TechAssetId<common::PredefinedAssetId>;
pub type AssetId = common::AssetId32<common::PredefinedAssetId>;

impl technical::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type TechAssetId = TechAssetId;
    type TechAccountId = TechAccountId;
    type Trigger = ();
    type Condition = ();
    type SwapAction = pool_xyk::PolySwapAction<AssetId, AccountId, TechAccountId>;
}

parameter_types! {
    pub GetFee: Fixed = fixed!(0.003);
}

impl pool_xyk::Config for Runtime {
    const MIN_XOR: Balance = balance!(0.0007);
    type RuntimeEvent = RuntimeEvent;
    type PairSwapAction = pool_xyk::PairSwapAction<AssetId, AccountId, TechAccountId>;
    type DepositLiquidityAction =
        pool_xyk::DepositLiquidityAction<AssetId, AccountId, TechAccountId>;
    type WithdrawLiquidityAction =
        pool_xyk::WithdrawLiquidityAction<AssetId, AccountId, TechAccountId>;
    type PolySwapAction = pool_xyk::PolySwapAction<AssetId, AccountId, TechAccountId>;
    type EnsureDEXManager = dex_manager::Pallet<Runtime>;
    type GetFee = GetFee;
    type OnPoolCreated = (PswapDistribution, Farming);
    type OnPoolReservesChanged = PriceTools;
    type WeightInfo = pool_xyk::weights::WeightInfo<Runtime>;
}

parameter_types! {
    pub GetLiquidityProxyTechAccountId: TechAccountId = {
        // TODO(Harrm): why pswap_distribution?
        let tech_account_id = TechAccountId::from_generic_pair(
            pswap_distribution::TECH_ACCOUNT_PREFIX.to_vec(),
            pswap_distribution::TECH_ACCOUNT_MAIN.to_vec(),
        );
        tech_account_id
    };
    pub GetLiquidityProxyAccountId: AccountId = {
        let tech_account_id = GetLiquidityProxyTechAccountId::get();
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub const GetNumSamples: usize = 5;
    pub const BasicDeposit: Balance = balance!(0.01);
    pub const FieldDeposit: Balance = balance!(0.01);
    pub const SubAccountDeposit: Balance = balance!(0.01);
    pub const MaxSubAccounts: u32 = 100;
    pub const MaxAdditionalFields: u32 = 100;
    pub const MaxRegistrars: u32 = 20;
    pub ReferralsReservesAcc: AccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            b"referrals".to_vec(),
            b"main".to_vec(),
        );
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
}

impl liquidity_proxy::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type LiquidityRegistry = dex_api::Pallet<Runtime>;
    type GetNumSamples = GetNumSamples;
    type GetTechnicalAccountId = GetLiquidityProxyAccountId;
    type PrimaryMarketTBC = multicollateral_bonding_curve_pool::Pallet<Runtime>;
    type PrimaryMarketXST = xst::Pallet<Runtime>;
    type SecondaryMarket = pool_xyk::Pallet<Runtime>;
    type WeightInfo = liquidity_proxy::weights::WeightInfo<Runtime>;
    type VestedRewardsPallet = VestedRewards;
}

impl mock_liquidity_source::Config<mock_liquidity_source::Instance1> for Runtime {
    type GetFee = GetFee;
    type EnsureDEXManager = dex_manager::Pallet<Runtime>;
    type EnsureTradingPairExists = trading_pair::Pallet<Runtime>;
}

impl mock_liquidity_source::Config<mock_liquidity_source::Instance2> for Runtime {
    type GetFee = GetFee;
    type EnsureDEXManager = dex_manager::Pallet<Runtime>;
    type EnsureTradingPairExists = trading_pair::Pallet<Runtime>;
}

impl mock_liquidity_source::Config<mock_liquidity_source::Instance3> for Runtime {
    type GetFee = GetFee;
    type EnsureDEXManager = dex_manager::Pallet<Runtime>;
    type EnsureTradingPairExists = trading_pair::Pallet<Runtime>;
}

impl mock_liquidity_source::Config<mock_liquidity_source::Instance4> for Runtime {
    type GetFee = GetFee;
    type EnsureDEXManager = dex_manager::Pallet<Runtime>;
    type EnsureTradingPairExists = trading_pair::Pallet<Runtime>;
}

impl dex_api::Config for Runtime {
    type MockLiquiditySource =
        mock_liquidity_source::Pallet<Runtime, mock_liquidity_source::Instance1>;
    type MockLiquiditySource2 =
        mock_liquidity_source::Pallet<Runtime, mock_liquidity_source::Instance2>;
    type MockLiquiditySource3 =
        mock_liquidity_source::Pallet<Runtime, mock_liquidity_source::Instance3>;
    type MockLiquiditySource4 =
        mock_liquidity_source::Pallet<Runtime, mock_liquidity_source::Instance4>;
    type MulticollateralBondingCurvePool = multicollateral_bonding_curve_pool::Pallet<Runtime>;
    type XYKPool = pool_xyk::Pallet<Runtime>;
    type XSTPool = xst::Pallet<Runtime>;
}

impl pallet_multisig::Config for Runtime {
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type DepositBase = DepositBase;
    type DepositFactor = DepositFactor;
    type MaxSignatories = MaxSignatories;
    type WeightInfo = ();
}

impl iroha_migration::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = iroha_migration::weights::WeightInfo<Runtime>;
}

impl pallet_identity::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BasicDeposit = BasicDeposit;
    type FieldDeposit = FieldDeposit;
    type SubAccountDeposit = SubAccountDeposit;
    type MaxSubAccounts = MaxSubAccounts;
    type MaxAdditionalFields = MaxAdditionalFields;
    type MaxRegistrars = MaxRegistrars;
    type Slashed = ();
    type ForceOrigin = MoreThanHalfCouncil;
    type RegistrarOrigin = MoreThanHalfCouncil;
    type WeightInfo = ();
}

impl<T: SigningTypes> frame_system::offchain::SignMessage<T> for Runtime {
    type SignatureData = ();

    fn sign_message(&self, _message: &[u8]) -> Self::SignatureData {
        unimplemented!()
    }

    fn sign<TPayload, F>(&self, _f: F) -> Self::SignatureData
    where
        F: Fn(&Account<T>) -> TPayload,
        TPayload: frame_system::offchain::SignedPayload<T>,
    {
        unimplemented!()
    }
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
    RuntimeCall: From<LocalCall>,
{
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: RuntimeCall,
        public: <Signature as sp_runtime::traits::Verify>::Signer,
        account: AccountId,
        index: Index,
    ) -> Option<(
        RuntimeCall,
        <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
    )> {
        let period = BlockHashCount::get() as u64;
        let current_block = System::block_number()
            .saturated_into::<u64>()
            .saturating_sub(1);
        let extra: SignedExtra = (
            frame_system::CheckSpecVersion::<Runtime>::new(),
            frame_system::CheckTxVersion::<Runtime>::new(),
            frame_system::CheckGenesis::<Runtime>::new(),
            frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
            frame_system::CheckNonce::<Runtime>::from(index),
            frame_system::CheckWeight::<Runtime>::new(),
            ChargeTransactionPayment::<Runtime>::new(),
        );
        #[cfg_attr(not(feature = "std"), allow(unused_variables))]
        let raw_payload = SignedPayload::new(call, extra)
            .map_err(|e| {
                frame_support::log::warn!("SignedPayload error: {:?}", e);
            })
            .ok()?;

        let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;

        let address = account;
        let (call, extra, _) = raw_payload.deconstruct();
        Some((call, (address, signature, extra)))
    }
}

impl frame_system::offchain::SigningTypes for Runtime {
    type Public = <Signature as sp_runtime::traits::Verify>::Signer;
    type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
    RuntimeCall: From<C>,
{
    type OverarchingCall = RuntimeCall;
    type Extrinsic = UncheckedExtrinsic;
}

impl referrals::Config for Runtime {
    type ReservesAcc = ReferralsReservesAcc;
    type WeightInfo = referrals::weights::WeightInfo<Runtime>;
}

impl rewards::Config for Runtime {
    const BLOCKS_PER_DAY: BlockNumber = 1 * DAYS;
    const UPDATE_FREQUENCY: BlockNumber = 10 * MINUTES;
    const MAX_CHUNK_SIZE: usize = 100;
    const MAX_VESTING_RATIO: Percent = Percent::from_percent(55);
    const TIME_TO_SATURATION: BlockNumber = 5 * 365 * DAYS; // 5 years
    const VAL_BURN_PERCENT: Percent = VAL_BURN_PERCENT;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = rewards::weights::WeightInfo<Runtime>;
}

// Multiplied flat fees implementation for the selected extrinsics.
// Returns a value (* multiplier) if the extrinsic is subject to manual fee
// adjustment and `None` otherwise
impl<T> xor_fee::ApplyCustomFees<RuntimeCall> for xor_fee::Pallet<T> {
    fn compute_fee(call: &RuntimeCall) -> Option<Balance> {
        let result = match call {
            RuntimeCall::Assets(assets::Call::register { .. })
            | RuntimeCall::EthBridge(eth_bridge::Call::transfer_to_sidechain { .. })
            | RuntimeCall::PoolXYK(pool_xyk::Call::withdraw_liquidity { .. })
            | RuntimeCall::Rewards(rewards::Call::claim { .. })
            | RuntimeCall::VestedRewards(vested_rewards::Call::claim_rewards { .. }) => {
                Some(BIG_FEE)
            }
            RuntimeCall::Assets(..)
            | RuntimeCall::EthBridge(..)
            | RuntimeCall::LiquidityProxy(..)
            | RuntimeCall::MulticollateralBondingCurvePool(..)
            | RuntimeCall::PoolXYK(..)
            | RuntimeCall::Rewards(..)
            | RuntimeCall::Staking(pallet_staking::Call::payout_stakers { .. })
            | RuntimeCall::TradingPair(..)
            | RuntimeCall::Referrals(..) => Some(SMALL_FEE),
            _ => None,
        };
        result.map(|fee| XorFee::multiplier().saturating_mul_int(fee))
    }
}

impl xor_fee::ExtractProxySwap for RuntimeCall {
    type AccountId = AccountId;
    type DexId = DEXId;
    type AssetId = AssetId;
    type Amount = SwapAmount<u128>;
    fn extract(
        &self,
    ) -> Option<xor_fee::SwapInfo<Self::AccountId, Self::DexId, Self::AssetId, Self::Amount>> {
        match self {
            RuntimeCall::LiquidityProxy(liquidity_proxy::Call::swap {
                dex_id,
                input_asset_id,
                output_asset_id,
                swap_amount,
                selected_source_types,
                filter_mode,
            }) => Some(xor_fee::SwapInfo {
                fee_source: None,
                dex_id: *dex_id,
                input_asset_id: *input_asset_id,
                output_asset_id: *output_asset_id,
                amount: *swap_amount,
                selected_source_types: selected_source_types.to_vec(),
                filter_mode: filter_mode.clone(),
            }),
            RuntimeCall::LiquidityProxy(liquidity_proxy::Call::swap_transfer {
                receiver,
                dex_id,
                input_asset_id,
                output_asset_id,
                swap_amount,
                selected_source_types,
                filter_mode,
                ..
            }) => Some(xor_fee::SwapInfo {
                fee_source: Some(receiver.clone()),
                dex_id: *dex_id,
                input_asset_id: *input_asset_id,
                output_asset_id: *output_asset_id,
                amount: *swap_amount,
                selected_source_types: selected_source_types.to_vec(),
                filter_mode: filter_mode.clone(),
            }),
            _ => None,
        }
    }
}

impl xor_fee::IsCalledByBridgePeer<AccountId> for RuntimeCall {
    fn is_called_by_bridge_peer(&self, who: &AccountId) -> bool {
        match self {
            RuntimeCall::BridgeMultisig(call) => match call {
                bridge_multisig::Call::as_multi {
                    id: multisig_id, ..
                }
                | bridge_multisig::Call::as_multi_threshold_1 {
                    id: multisig_id, ..
                } => bridge_multisig::Accounts::<Runtime>::get(multisig_id)
                    .map(|acc| acc.is_signatory(&who)),
                _ => None,
            },
            RuntimeCall::EthBridge(call) => match call {
                eth_bridge::Call::approve_request { network_id, .. } => {
                    Some(eth_bridge::Pallet::<Runtime>::is_peer(who, *network_id))
                }
                eth_bridge::Call::register_incoming_request { incoming_request } => {
                    let net_id = incoming_request.network_id();
                    eth_bridge::BridgeAccount::<Runtime>::get(net_id).map(|acc| acc == *who)
                }
                eth_bridge::Call::import_incoming_request {
                    load_incoming_request,
                    ..
                } => {
                    let net_id = load_incoming_request.network_id();
                    eth_bridge::BridgeAccount::<Runtime>::get(net_id).map(|acc| acc == *who)
                }
                eth_bridge::Call::finalize_incoming_request { network_id, .. }
                | eth_bridge::Call::abort_request { network_id, .. } => {
                    eth_bridge::BridgeAccount::<Runtime>::get(network_id).map(|acc| acc == *who)
                }
                _ => None,
            },
            _ => None,
        }
        .unwrap_or(false)
    }
}

pub struct ValBurnedAggregator<T>(sp_std::marker::PhantomData<T>);

impl<T> OnValBurned for ValBurnedAggregator<T>
where
    T: ValBurnedNotifier<Balance>,
{
    fn on_val_burned(amount: Balance) {
        Rewards::on_val_burned(amount);
        T::notify_val_burned(amount);
    }
}

pub struct WithdrawFee;

impl xor_fee::WithdrawFee<Runtime> for WithdrawFee {
    fn withdraw_fee(
        who: &AccountId,
        call: &RuntimeCall,
        fee: Balance,
    ) -> Result<(AccountId, Option<NegativeImbalanceOf<Runtime>>), DispatchError> {
        match call {
            RuntimeCall::Referrals(referrals::Call::set_referrer { referrer })
                if Referrals::can_set_referrer(who) =>
            {
                Referrals::withdraw_fee(referrer, fee)?;
                Ok((
                    referrer.clone(),
                    Some(Balances::withdraw(
                        &ReferralsReservesAcc::get(),
                        fee,
                        WithdrawReasons::TRANSACTION_PAYMENT,
                        ExistenceRequirement::KeepAlive,
                    )?),
                ))
            }
            _ => Ok((
                who.clone(),
                Some(Balances::withdraw(
                    who,
                    fee,
                    WithdrawReasons::TRANSACTION_PAYMENT,
                    ExistenceRequirement::KeepAlive,
                )?),
            )),
        }
    }
}

parameter_types! {
    pub const DEXIdValue: DEXId = 0;
}

impl xor_fee::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    // Pass native currency.
    type XorCurrency = Balances;
    type ReferrerWeight = ReferrerWeight;
    type XorBurnedWeight = XorBurnedWeight;
    type XorIntoValBurnedWeight = XorIntoValBurnedWeight;
    type SoraParliamentShare = SoraParliamentShare;
    type XorId = GetXorAssetId;
    type ValId = GetValAssetId;
    type DEXIdValue = DEXIdValue;
    type LiquidityProxy = LiquidityProxy;
    type OnValBurned = ValBurnedAggregator<Staking>;
    type CustomFees = XorFee;
    type GetTechnicalAccountId = GetXorFeeAccountId;
    type GetParliamentAccountId = GetParliamentAccountId;
    type SessionManager = Staking;
    type WeightInfo = xor_fee::weights::WeightInfo<Runtime>;
    type WithdrawFee = WithdrawFee;
}

pub struct ConstantFeeMultiplier;

impl MultiplierUpdate for ConstantFeeMultiplier {
    fn min() -> Multiplier {
        Default::default()
    }
    fn max() -> Multiplier {
        Default::default()
    }
    fn target() -> Perquintill {
        Default::default()
    }
    fn variability() -> Multiplier {
        Default::default()
    }
}
impl Convert<Multiplier, Multiplier> for ConstantFeeMultiplier {
    fn convert(previous: Multiplier) -> Multiplier {
        previous
    }
}

parameter_types! {
    pub const OperationalFeeMultiplier: u8 = 5;
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = XorFee;
    type WeightToFee = XorFee;
    type FeeMultiplierUpdate = ConstantFeeMultiplier;
    type OperationalFeeMultiplier = OperationalFeeMultiplier;
    type LengthToFee = ConstantMultiplier<Balance, ConstU128<0>>;
}

#[cfg(feature = "private-net")]
impl pallet_sudo::Config for Runtime {
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
}

impl permissions::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_utility::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = ();
    type PalletsOrigin = OriginCaller;
}

parameter_types! {
    pub const DepositBase: u64 = 1;
    pub const DepositFactor: u64 = 1;
    pub const MaxSignatories: u16 = 100;
}

impl bridge_multisig::Config for Runtime {
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type DepositBase = DepositBase;
    type DepositFactor = DepositFactor;
    type MaxSignatories = MaxSignatories;
    type WeightInfo = ();
}

parameter_types! {
    pub const GetEthNetworkId: u32 = 0;
}

pub struct RemoveTemporaryPeerAccountIds;

#[cfg(feature = "private-net")]
impl Get<Vec<(AccountId, H160)>> for RemoveTemporaryPeerAccountIds {
    fn get() -> Vec<(AccountId, H160)> {
        vec![
            // Dev
            (
                AccountId::new(hex!(
                    "aa79aa80b94b1cfba69c4a7d60eeb7b469e6411d1f686cc61de8adc8b1b76a69"
                )),
                H160(hex!("f858c8366f3a2553516a47f3e0503a85ef93bbba")),
            ),
            (
                AccountId::new(hex!(
                    "60dc5adadc262770cbe904e3f65a26a89d46b70447640cd7968b49ddf5a459bc"
                )),
                H160(hex!("ccd7fe44d58640dc79c55b98f8c3474646e5ea2b")),
            ),
            (
                AccountId::new(hex!(
                    "70d61e980602e09ac8b5fb50658ebd345774e73b8248d3b61862ba1a9a035082"
                )),
                H160(hex!("13d26a91f791e884fe6faa7391c4ef401638baa4")),
            ),
            (
                AccountId::new(hex!(
                    "05918034f4a7f7c5d99cd0382aa6574ec2aba148aa3d769e50e0ac7663e36d58"
                )),
                H160(hex!("aa19829ae887212206be8e97ea47d8fed2120d4e")),
            ),
            // Test
            (
                AccountId::new(hex!(
                    "07f5670d08b8f3bd493ff829482a489d94494fd50dd506957e44e9fdc2e98684"
                )),
                H160(hex!("457d710255184dbf63c019ab50f65743c6cb072f")),
            ),
            (
                AccountId::new(hex!(
                    "211bb96e9f746183c05a1d583bccf513f9d8f679d6f36ecbd06609615a55b1cc"
                )),
                H160(hex!("6d04423c97e8ce36d04c9b614926ce0d029d04df")),
            ),
            (
                AccountId::new(hex!(
                    "ef3139b81d14977d5bf6b4a3994872337dfc1d2af2069a058bc26123a3ed1a5c"
                )),
                H160(hex!("e34022904b1ab539729cc7b5bfa5c8a74b165e80")),
            ),
            (
                AccountId::new(hex!(
                    "71124b336fbf3777d743d4390acce6be1cf5e0781e40c51d4cf2e5b5fd8e41e1"
                )),
                H160(hex!("ee74a5b5346915012d103cf1ccee288f25bcbc81")),
            ),
            // Stage
            (
                AccountId::new(hex!(
                    "07f5670d08b8f3bd493ff829482a489d94494fd50dd506957e44e9fdc2e98684"
                )),
                H160(hex!("457d710255184dbf63c019ab50f65743c6cb072f")),
            ),
            (
                AccountId::new(hex!(
                    "211bb96e9f746183c05a1d583bccf513f9d8f679d6f36ecbd06609615a55b1cc"
                )),
                H160(hex!("6d04423c97e8ce36d04c9b614926ce0d029d04df")),
            ),
        ]
    }
}

#[cfg(not(feature = "private-net"))]
impl Get<Vec<(AccountId, H160)>> for RemoveTemporaryPeerAccountIds {
    fn get() -> Vec<(AccountId, H160)> {
        vec![] // the peer is already removed on main-net.
    }
}

#[cfg(not(feature = "private-net"))]
parameter_types! {
    pub const RemovePendingOutgoingRequestsAfter: BlockNumber = 1 * DAYS;
    pub const TrackPendingIncomingRequestsAfter: (BlockNumber, u64) = (1 * DAYS, 12697214);
}

#[cfg(feature = "private-net")]
parameter_types! {
    pub const RemovePendingOutgoingRequestsAfter: BlockNumber = 30 * MINUTES;
    pub const TrackPendingIncomingRequestsAfter: (BlockNumber, u64) = (30 * MINUTES, 0);
}

pub type NetworkId = u32;

impl eth_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type PeerId = eth_bridge::offchain::crypto::TestAuthId;
    type NetworkId = NetworkId;
    type GetEthNetworkId = GetEthNetworkId;
    type WeightInfo = eth_bridge::weights::WeightInfo<Runtime>;
    type RemovePendingOutgoingRequestsAfter = RemovePendingOutgoingRequestsAfter;
    type TrackPendingIncomingRequestsAfter = TrackPendingIncomingRequestsAfter;
    type RemovePeerAccountIds = RemoveTemporaryPeerAccountIds;
    type SchedulerOriginCaller = OriginCaller;
    type Scheduler = Scheduler;
    type WeightToFee = XorFee;
}

#[cfg(feature = "private-net")]
impl faucet::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = faucet::weights::WeightInfo<Runtime>;
}

parameter_types! {
    pub GetPswapDistributionTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            pswap_distribution::TECH_ACCOUNT_PREFIX.to_vec(),
            pswap_distribution::TECH_ACCOUNT_MAIN.to_vec(),
        );
        tech_account_id
    };
    pub GetPswapDistributionAccountId: AccountId = {
        let tech_account_id = GetPswapDistributionTechAccountId::get();
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub GetParliamentAccountId: AccountId = hex!("881b87c9f83664b95bd13e2bb40675bfa186287da93becc0b22683334d411e4e").into();
    pub GetXorFeeTechAccountId: TechAccountId = {
        TechAccountId::from_generic_pair(
            xor_fee::TECH_ACCOUNT_PREFIX.to_vec(),
            xor_fee::TECH_ACCOUNT_MAIN.to_vec(),
        )
    };
    pub GetXorFeeAccountId: AccountId = {
        let tech_account_id = GetXorFeeTechAccountId::get();
        technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
            .expect("Failed to get ordinary account id for technical account id.")
    };
    pub GetXSTPoolPermissionedTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            xst::TECH_ACCOUNT_PREFIX.to_vec(),
            xst::TECH_ACCOUNT_PERMISSIONED.to_vec(),
        );
        tech_account_id
    };
    pub GetXSTPoolPermissionedAccountId: AccountId = {
        let tech_account_id = GetXSTPoolPermissionedTechAccountId::get();
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub GetTrustlessBridgeTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            bridge_types::types::TECH_ACCOUNT_PREFIX.to_vec(),
            bridge_types::types::TECH_ACCOUNT_MAIN.to_vec(),
        );
        tech_account_id
    };
    pub GetTrustlessBridgeAccountId: AccountId = {
        let tech_account_id = GetTrustlessBridgeTechAccountId::get();
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub GetTrustlessBridgeFeesTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            bridge_types::types::TECH_ACCOUNT_PREFIX.to_vec(),
            bridge_types::types::TECH_ACCOUNT_FEES.to_vec(),
        );
        tech_account_id
    };
    pub GetTrustlessBridgeFeesAccountId: AccountId = {
        let tech_account_id = GetTrustlessBridgeFeesTechAccountId::get();
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub GetTreasuryTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            bridge_types::types::TECH_ACCOUNT_TREASURY_PREFIX.to_vec(),
            bridge_types::types::TECH_ACCOUNT_MAIN.to_vec(),
        );
        tech_account_id
    };
    pub GetTreasuryAccountId: AccountId = {
        let tech_account_id = GetTreasuryTechAccountId::get();
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
}

#[cfg(feature = "reduced-pswap-reward-periods")]
parameter_types! {
    pub const GetDefaultSubscriptionFrequency: BlockNumber = 150;
    pub const GetBurnUpdateFrequency: BlockNumber = 150;
}

#[cfg(not(feature = "reduced-pswap-reward-periods"))]
parameter_types! {
    pub const GetDefaultSubscriptionFrequency: BlockNumber = 14400;
    pub const GetBurnUpdateFrequency: BlockNumber = 14400;
}

pub struct RuntimeOnPswapBurnedAggregator;

impl OnPswapBurned for RuntimeOnPswapBurnedAggregator {
    fn on_pswap_burned(distribution: common::PswapRemintInfo) {
        VestedRewards::on_pswap_burned(distribution);
    }
}

impl farming::Config for Runtime {
    const PSWAP_PER_DAY: Balance = FARMING_PSWAP_PER_DAY;
    const REFRESH_FREQUENCY: BlockNumber = FARMING_REFRESH_FREQUENCY;
    const VESTING_COEFF: u32 = FARMING_VESTING_COEFF;
    const VESTING_FREQUENCY: BlockNumber = FARMING_VESTING_FREQUENCY;
    const BLOCKS_PER_DAY: BlockNumber = 1 * DAYS;
    type RuntimeCall = RuntimeCall;
    type SchedulerOriginCaller = OriginCaller;
    type Scheduler = Scheduler;
    type RewardDoublingAssets = FarmingRewardDoublingAssets;
    type WeightInfo = ();
}

impl pswap_distribution::Config for Runtime {
    const PSWAP_BURN_PERCENT: Percent = PSWAP_BURN_PERCENT;
    type RuntimeEvent = RuntimeEvent;
    type GetIncentiveAssetId = GetPswapAssetId;
    type LiquidityProxy = LiquidityProxy;
    type CompatBalance = Balance;
    type GetDefaultSubscriptionFrequency = GetDefaultSubscriptionFrequency;
    type GetBurnUpdateFrequency = GetBurnUpdateFrequency;
    type GetTechnicalAccountId = GetPswapDistributionAccountId;
    type EnsureDEXManager = DEXManager;
    type OnPswapBurnedAggregator = RuntimeOnPswapBurnedAggregator;
    type WeightInfo = pswap_distribution::weights::WeightInfo<Runtime>;
    type GetParliamentAccountId = GetParliamentAccountId;
    type PoolXykPallet = PoolXYK;
}

parameter_types! {
    pub GetMbcReservesTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            multicollateral_bonding_curve_pool::TECH_ACCOUNT_PREFIX.to_vec(),
            multicollateral_bonding_curve_pool::TECH_ACCOUNT_RESERVES.to_vec(),
        );
        tech_account_id
    };
    pub GetMbcReservesAccountId: AccountId = {
        let tech_account_id = GetMbcReservesTechAccountId::get();
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub GetMbcPoolRewardsTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            multicollateral_bonding_curve_pool::TECH_ACCOUNT_PREFIX.to_vec(),
            multicollateral_bonding_curve_pool::TECH_ACCOUNT_REWARDS.to_vec(),
        );
        tech_account_id
    };
    pub GetMbcPoolRewardsAccountId: AccountId = {
        let tech_account_id = GetMbcPoolRewardsTechAccountId::get();
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub GetMbcPoolFreeReservesTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            multicollateral_bonding_curve_pool::TECH_ACCOUNT_PREFIX.to_vec(),
            multicollateral_bonding_curve_pool::TECH_ACCOUNT_FREE_RESERVES.to_vec(),
        );
        tech_account_id
    };
    pub GetMbcPoolFreeReservesAccountId: AccountId = {
        let tech_account_id = GetMbcPoolFreeReservesTechAccountId::get();
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub GetMarketMakerRewardsTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            vested_rewards::TECH_ACCOUNT_PREFIX.to_vec(),
            vested_rewards::TECH_ACCOUNT_MARKET_MAKERS.to_vec(),
        );
        tech_account_id
    };
    pub GetMarketMakerRewardsAccountId: AccountId = {
        let tech_account_id = GetMarketMakerRewardsTechAccountId::get();
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub GetCrowdloanRewardsTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            vested_rewards::TECH_ACCOUNT_PREFIX.to_vec(),
            vested_rewards::TECH_ACCOUNT_CROWDLOAN.to_vec(),
        );
        tech_account_id
    };
    pub GetCrowdloanRewardsAccountId: AccountId = {
        let tech_account_id = GetCrowdloanRewardsTechAccountId::get();
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub GetFarmingRewardsTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            vested_rewards::TECH_ACCOUNT_PREFIX.to_vec(),
            vested_rewards::TECH_ACCOUNT_FARMING.to_vec(),
        );
        tech_account_id
    };
    pub GetFarmingRewardsAccountId: AccountId = {
        let tech_account_id = GetFarmingRewardsTechAccountId::get();
        let account_id =
            technical::Pallet::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
}

impl multicollateral_bonding_curve_pool::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type LiquidityProxy = LiquidityProxy;
    type EnsureDEXManager = DEXManager;
    type EnsureTradingPairExists = TradingPair;
    type PriceToolsPallet = PriceTools;
    type VestedRewardsPallet = VestedRewards;
    type WeightInfo = multicollateral_bonding_curve_pool::weights::WeightInfo<Runtime>;
}

parameter_types! {
    pub const GetXstPoolConversionAssetId: AssetId = GetXstAssetId::get();
}

impl xst::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type GetSyntheticBaseAssetId = GetXstPoolConversionAssetId;
    type LiquidityProxy = LiquidityProxy;
    type EnsureDEXManager = DEXManager;
    type EnsureTradingPairExists = TradingPair;
    type PriceToolsPallet = PriceTools;
    type WeightInfo = xst::weights::WeightInfo<Runtime>;
}

parameter_types! {
    pub const MaxKeys: u32 = 10_000;
    pub const MaxPeerInHeartbeats: u32 = 10_000;
    pub const MaxPeerDataEncodingSize: u32 = 1_000;
}

impl pallet_im_online::Config for Runtime {
    type AuthorityId = ImOnlineId;
    type RuntimeEvent = RuntimeEvent;
    type ValidatorSet = Historical;
    type NextSessionRotation = Babe;
    type ReportUnresponsiveness = Offences;
    type UnsignedPriority = ImOnlineUnsignedPriority;
    type WeightInfo = ();
    type MaxKeys = MaxKeys;
    type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
    type MaxPeerDataEncodingSize = MaxPeerDataEncodingSize;
}

impl pallet_offences::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
    type OnOffenceHandler = Staking;
}

impl vested_rewards::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type GetBondingCurveRewardsAccountId = GetMbcPoolRewardsAccountId;
    type GetFarmingRewardsAccountId = GetFarmingRewardsAccountId;
    type GetMarketMakerRewardsAccountId = GetMarketMakerRewardsAccountId;
    type GetCrowdloanRewardsAccountId = GetCrowdloanRewardsAccountId;
    type WeightInfo = vested_rewards::weights::WeightInfo<Runtime>;
}

impl price_tools::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type LiquidityProxy = LiquidityProxy;
    type WeightInfo = price_tools::weights::WeightInfo<Runtime>;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

impl pallet_beefy::Config for Runtime {
    type BeefyId = BeefyId;
    type MaxAuthorities = MaxAuthorities;
    type OnNewValidatorSet = MmrLeaf;
}

impl pallet_mmr::Config for Runtime {
    const INDEXING_PREFIX: &'static [u8] = b"mmr";
    type Hashing = Keccak256;
    type Hash = <Keccak256 as sp_runtime::traits::Hash>::Output;
    type OnNewRoot = pallet_beefy_mmr::DepositBeefyDigest<Runtime>;
    type WeightInfo = ();
    type LeafData = pallet_beefy_mmr::Pallet<Runtime>;
}

impl leaf_provider::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Hashing = Keccak256;
    type Hash = <Keccak256 as sp_runtime::traits::Hash>::Output;
    type Randomness = pallet_babe::RandomnessFromTwoEpochsAgo<Self>;
}

parameter_types! {
    /// Version of the produced MMR leaf.
    ///
    /// The version consists of two parts;
    /// - `major` (3 bits)
    /// - `minor` (5 bits)
    ///
    /// `major` should be updated only if decoding the previous MMR Leaf format from the payload
    /// is not possible (i.e. backward incompatible change).
    /// `minor` should be updated if fields are added to the previous MMR Leaf, which given SCALE
    /// encoding does not prevent old leafs from being decoded.
    ///
    /// Hence we expect `major` to be changed really rarely (think never).
    /// See [`MmrLeafVersion`] type documentation for more details.
    pub LeafVersion: MmrLeafVersion = MmrLeafVersion::new(0, 0);
}

impl pallet_beefy_mmr::Config for Runtime {
    type LeafVersion = LeafVersion;
    type BeefyAuthorityToMerkleLeaf = pallet_beefy_mmr::BeefyEcdsaToEthereum;
    type LeafExtra =
        LeafExtraData<<Self as leaf_provider::Config>::Hash, <Self as frame_system::Config>::Hash>;
    type BeefyDataProvider = leaf_provider::Pallet<Runtime>;
}

parameter_types! {
    pub const CeresPerDay: Balance = balance!(6.66666666667);
    pub const CeresAssetId: AssetId = common::AssetId32::from_bytes
        (hex!("008bcfd2387d3fc453333557eecb0efe59fcba128769b2feefdd306e98e66440"));
    pub const MaximumCeresInStakingPool: Balance = balance!(14400);
}

impl ceres_launchpad::Config for Runtime {
    const MILLISECONDS_PER_DAY: Moment = 86_400_000;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ceres_launchpad::weights::WeightInfo<Runtime>;
}

impl ceres_staking::Config for Runtime {
    const BLOCKS_PER_ONE_DAY: BlockNumber = 1 * DAYS;
    type RuntimeEvent = RuntimeEvent;
    type CeresPerDay = CeresPerDay;
    type CeresAssetId = CeresAssetId;
    type MaximumCeresInStakingPool = MaximumCeresInStakingPool;
    type WeightInfo = ceres_staking::weights::WeightInfo<Runtime>;
}

impl ceres_liquidity_locker::Config for Runtime {
    const BLOCKS_PER_ONE_DAY: BlockNumber = 1 * DAYS;
    type RuntimeEvent = RuntimeEvent;
    type XYKPool = PoolXYK;
    type DemeterFarmingPlatform = DemeterFarmingPlatform;
    type CeresAssetId = CeresAssetId;
    type WeightInfo = ceres_liquidity_locker::weights::WeightInfo<Runtime>;
}

impl ceres_token_locker::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type CeresAssetId = CeresAssetId;
    type WeightInfo = ceres_token_locker::weights::WeightInfo<Runtime>;
}

impl ceres_governance_platform::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type CeresAssetId = CeresAssetId;
    type WeightInfo = ceres_governance_platform::weights::WeightInfo<Runtime>;
}

parameter_types! {
    pub const DemeterAssetId: AssetId = common::DEMETER_ASSET_ID;
}

impl demeter_farming_platform::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type DemeterAssetId = DemeterAssetId;
    const BLOCKS_PER_HOUR_AND_A_HALF: BlockNumber = 3 * HOURS / 2;
    type WeightInfo = demeter_farming_platform::weights::WeightInfo<Runtime>;
}

impl oracle_proxy::Config for Runtime {
    type Symbol = Symbol;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = oracle_proxy::weights::WeightInfo<Runtime>;
    type BandChainOracle = band::Pallet<Runtime>;
}

impl band::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Symbol = Symbol;
    type WeightInfo = band::weights::WeightInfo<Runtime>;
    type OnNewSymbolsRelayedHook = oracle_proxy::Pallet<Runtime>;
}

/// Payload data to be signed when making signed transaction from off-chain workers,
///   inside `create_transaction` function.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;

parameter_types! {
    pub const ReferrerWeight: u32 = 10;
    pub const XorBurnedWeight: u32 = 40;
    pub const XorIntoValBurnedWeight: u32 = 50;
    pub const SoraParliamentShare: Percent = Percent::from_percent(10);
}

// Ethereum bridge pallets

pub struct CallFilter;
impl Contains<RuntimeCall> for CallFilter {
    fn contains(_: &RuntimeCall) -> bool {
        true
    }
}

impl dispatch::Config<Instance1> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type NetworkId = EVMChainId;
    type Additional = AdditionalEVMInboundData;
    type OriginOutput =
        bridge_types::types::CallOriginOutput<EVMChainId, H256, AdditionalEVMInboundData>;
    type Origin = RuntimeOrigin;
    type MessageId = bridge_types::types::MessageId;
    type Hashing = Keccak256;
    type Call = RuntimeCall;
    type CallFilter = CallFilter;
}

use bridge_types::{EVMChainId, SubNetworkId, CHANNEL_INDEXING_PREFIX, H256};

parameter_types! {
    pub const BridgeMaxMessagePayloadSize: u64 = 256;
    pub const BridgeMaxMessagesPerCommit: u64 = 20;
    pub const BridgeMaxTotalGasLimit: u64 = 5_000_000;
    pub const Decimals: u32 = 12;
}

pub struct FeeConverter;
impl Convert<U256, Balance> for FeeConverter {
    fn convert(amount: U256) -> Balance {
        common::eth::unwrap_balance(amount, Decimals::get())
            .expect("Should not panic unless runtime is misconfigured")
    }
}

parameter_types! {
    pub const FeeCurrency: AssetId32<PredefinedAssetId> = XOR;
}

impl bridge_inbound_channel::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Verifier = ethereum_light_client::Pallet<Runtime>;
    type MessageDispatch = Dispatch;
    type Hashing = Keccak256;
    type MessageStatusNotifier = EvmBridgeProxy;
    type FeeConverter = FeeConverter;
    type WeightInfo = ();
    type FeeAssetId = FeeCurrency;
    type OutboundChannel = BridgeOutboundChannel;
    type FeeTechAccountId = GetTrustlessBridgeFeesTechAccountId;
    type TreasuryTechAccountId = GetTreasuryTechAccountId;
}

impl bridge_outbound_channel::Config for Runtime {
    const INDEXING_PREFIX: &'static [u8] = CHANNEL_INDEXING_PREFIX;
    type RuntimeEvent = RuntimeEvent;
    type Hashing = Keccak256;
    type MaxMessagePayloadSize = BridgeMaxMessagePayloadSize;
    type MaxMessagesPerCommit = BridgeMaxMessagesPerCommit;
    type MaxTotalGasLimit = BridgeMaxTotalGasLimit;
    type FeeCurrency = FeeCurrency;
    type FeeTechAccountId = GetTrustlessBridgeFeesTechAccountId;
    type MessageStatusNotifier = EvmBridgeProxy;
    type WeightInfo = ();
}

parameter_types! {
    pub const DescendantsUntilFinalized: u8 = 30;
    pub const VerifyPoW: bool = true;
    // Not as important as some essential transactions (e.g. im_online or similar ones)
    pub EthereumLightClientPriority: TransactionPriority = Perbill::from_percent(10) * TransactionPriority::max_value();
    // We don't want to have not relevant imports be stuck in transaction pool
    // for too long
    pub EthereumLightClientLongevity: TransactionLongevity = EPOCH_DURATION_IN_BLOCKS as u64;
}

impl ethereum_light_client::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type DescendantsUntilFinalized = DescendantsUntilFinalized;
    type VerifyPoW = VerifyPoW;
    type WeightInfo = ();
    type UnsignedPriority = EthereumLightClientPriority;
    type UnsignedLongevity = EthereumLightClientLongevity;
    type ImportSignature = Signature;
    type Submitter = <Signature as Verify>::Signer;
}

impl eth_app::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OutboundChannel = BridgeOutboundChannel;
    type CallOrigin = dispatch::EnsureAccount<
        EVMChainId,
        AdditionalEVMInboundData,
        bridge_types::types::CallOriginOutput<EVMChainId, H256, AdditionalEVMInboundData>,
    >;
    type BridgeTechAccountId = GetTrustlessBridgeTechAccountId;
    type MessageStatusNotifier = EvmBridgeProxy;
    type WeightInfo = ();
}

impl erc20_app::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OutboundChannel = BridgeOutboundChannel;
    type CallOrigin = dispatch::EnsureAccount<
        EVMChainId,
        AdditionalEVMInboundData,
        bridge_types::types::CallOriginOutput<EVMChainId, H256, AdditionalEVMInboundData>,
    >;
    type AppRegistry = BridgeInboundChannel;
    type BridgeTechAccountId = GetTrustlessBridgeTechAccountId;
    type MessageStatusNotifier = EvmBridgeProxy;
    type WeightInfo = ();
}

impl migration_app::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OutboundChannel = BridgeOutboundChannel;
    type WeightInfo = ();
}

impl evm_bridge_proxy::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ERC20App = ERC20App;
    type EthApp = EthApp;
    type WeightInfo = ();
}

impl beefy_light_client::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Message = Vec<ParachainMessage<Balance>>;
    type Randomness = pallet_babe::RandomnessFromTwoEpochsAgo<Self>;
}

impl dispatch::Config<Instance2> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type NetworkId = SubNetworkId;
    type Additional = ();
    type OriginOutput = bridge_types::types::CallOriginOutput<SubNetworkId, H256, ()>;
    type Origin = RuntimeOrigin;
    type MessageId = bridge_types::types::MessageId;
    type Hashing = Keccak256;
    type Call = DispatchableSubstrateBridgeCall;
    type CallFilter = SubstrateBridgeCallFilter;
}

impl substrate_bridge_channel::inbound::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Verifier = BeefyLightClient;
    type ProvedMessage =
        beefy_light_client::ProvedSubstrateBridgeMessage<Vec<ParachainMessage<Balance>>>;
    type MessageDispatch = SubstrateDispatch;
    type WeightInfo = ();
    type FeeAssetId = FeeCurrency;
    type FeeAccountId = GetTrustlessBridgeFeesAccountId;
    type TreasuryAccountId = GetTreasuryAccountId;
    type FeeConverter = FeeConverter;
    type Currency = Currencies;
}

impl substrate_bridge_channel::outbound::Config for Runtime {
    const INDEXING_PREFIX: &'static [u8] = CHANNEL_INDEXING_PREFIX;
    type RuntimeEvent = RuntimeEvent;
    type Hashing = Keccak256;
    type FeeCurrency = FeeCurrency;
    type FeeAccountId = GetTrustlessBridgeFeesAccountId;
    type MessageStatusNotifier = EvmBridgeProxy;
    type MaxMessagePayloadSize = BridgeMaxMessagePayloadSize;
    type MaxMessagesPerCommit = BridgeMaxMessagesPerCommit;
    type AuxiliaryDigestHandler = LeafProvider;
    type Currency = Currencies;
    type WeightInfo = ();
}

impl substrate_bridge_app::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OutboundChannel = SubstrateBridgeOutboundChannel;
    type CallOrigin = dispatch::EnsureAccount<
        SubNetworkId,
        (),
        bridge_types::types::CallOriginOutput<SubNetworkId, H256, ()>,
    >;
    type MessageStatusNotifier = EvmBridgeProxy;
    type BridgeAccountId = GetTrustlessBridgeAccountId;
    type Currency = Currencies;
    type AssetRegistry = BridgeAssetRegistryImpl;
    type WeightInfo = ();
}

#[cfg(feature = "private-net")]
construct_runtime! {
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Pallet, Call, Storage, Config, Event<T>} = 0,

        Babe: pallet_babe::{Pallet, Call, Storage, Config, ValidateUnsigned} = 14,

        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 1,
        // Balances in native currency - XOR.
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 2,
        Sudo: pallet_sudo::{Pallet, Call, Storage, Config<T>, Event<T>} = 3,
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage} = 4,
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>} = 5,
        Permissions: permissions::{Pallet, Call, Storage, Config<T>, Event<T>} = 6,
        Referrals: referrals::{Pallet, Call, Storage} = 7,
        Rewards: rewards::{Pallet, Call, Config<T>, Storage, Event<T>} = 8,
        XorFee: xor_fee::{Pallet, Call, Storage, Event<T>} = 9,
        BridgeMultisig: bridge_multisig::{Pallet, Call, Storage, Config<T>, Event<T>} = 10,
        Utility: pallet_utility::{Pallet, Call, Event} = 11,

        // Consensus and staking.
        Authorship: pallet_authorship::{Pallet, Call, Storage, Inherent} = 16,
        Staking: pallet_staking::{Pallet, Call, Config<T>, Storage, Event<T>} = 17,
        Offences: pallet_offences::{Pallet, Storage, Event} = 37,
        Historical: pallet_session_historical::{Pallet} = 13,
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>} = 12,
        Grandpa: pallet_grandpa::{Pallet, Call, Storage, Config, Event} = 15,
        ImOnline: pallet_im_online::{Pallet, Call, Storage, Event<T>, ValidateUnsigned, Config<T>} = 36,

        // Non-native tokens - everything apart of XOR.
        Tokens: tokens::{Pallet, Storage, Config<T>, Event<T>} = 18,
        // Unified interface for XOR and non-native tokens.
        Currencies: currencies::{Pallet, Call} = 19,
        TradingPair: trading_pair::{Pallet, Call, Storage, Config<T>, Event<T>} = 20,
        Assets: assets::{Pallet, Call, Storage, Config<T>, Event<T>} = 21,
        DEXManager: dex_manager::{Pallet, Storage, Config<T>} = 22,
        MulticollateralBondingCurvePool: multicollateral_bonding_curve_pool::{Pallet, Call, Storage, Config<T>, Event<T>} = 23,
        Technical: technical::{Pallet, Call, Config<T>, Event<T>, Storage} = 24,
        PoolXYK: pool_xyk::{Pallet, Call, Storage, Event<T>} = 25,
        LiquidityProxy: liquidity_proxy::{Pallet, Call, Event<T>} = 26,
        Council: pallet_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 27,
        TechnicalCommittee: pallet_collective::<Instance2>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 28,
        Democracy: pallet_democracy::{Pallet, Call, Storage, Config<T>, Event<T>} = 29,
        DEXAPI: dex_api::{Pallet, Call, Storage, Config} = 30,
        EthBridge: eth_bridge::{Pallet, Call, Storage, Config<T>, Event<T>} = 31,
        PswapDistribution: pswap_distribution::{Pallet, Call, Storage, Config<T>, Event<T>} = 32,
        Multisig: pallet_multisig::{Pallet, Call, Storage, Event<T>} = 33,
        Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 34,
        IrohaMigration: iroha_migration::{Pallet, Call, Storage, Config<T>, Event<T>} = 35,
        TechnicalMembership: pallet_membership::<Instance1>::{Pallet, Call, Storage, Event<T>, Config<T>} = 38,
        ElectionsPhragmen: pallet_elections_phragmen::{Pallet, Call, Storage, Event<T>, Config<T>} = 39,
        VestedRewards: vested_rewards::{Pallet, Call, Storage, Event<T>, Config} = 40,
        Identity: pallet_identity::{Pallet, Call, Storage, Event<T>} = 41,
        Farming: farming::{Pallet, Storage} = 42,
        XSTPool: xst::{Pallet, Call, Storage, Config<T>, Event<T>} = 43,
        PriceTools: price_tools::{Pallet, Storage, Event<T>} = 44,
        CeresStaking: ceres_staking::{Pallet, Call, Storage, Event<T>} = 45,
        CeresLiquidityLocker: ceres_liquidity_locker::{Pallet, Call, Storage, Event<T>} = 46,
        CeresTokenLocker: ceres_token_locker::{Pallet, Call, Storage, Event<T>} = 47,
        CeresGovernancePlatform: ceres_governance_platform::{Pallet, Call, Storage, Event<T>} = 48,
        CeresLaunchpad: ceres_launchpad::{Pallet, Call, Storage, Event<T>} = 49,
        DemeterFarmingPlatform: demeter_farming_platform::{Pallet, Call, Storage, Event<T>} = 50,
        // Provides a semi-sorted list of nominators for staking.
        BagsList: pallet_bags_list::{Pallet, Call, Storage, Event<T>} = 51,
        ElectionProviderMultiPhase: pallet_election_provider_multi_phase::{Pallet, Call, Storage, Event<T>, ValidateUnsigned} = 52,
        Band: band::{Pallet, Call, Storage, Event<T>} = 53,
        OracleProxy: oracle_proxy::{Pallet, Call, Storage, Event<T>} = 54,

        // Available only for test net
        Faucet: faucet::{Pallet, Call, Config<T>, Event<T>} = 80,

        // Trustless ethereum bridge
        Mmr: pallet_mmr::{Pallet, Storage} = 90,
        Beefy: pallet_beefy::{Pallet, Config<T>, Storage} = 91,
        MmrLeaf: pallet_beefy_mmr::{Pallet, Storage} = 92,
        EthereumLightClient: ethereum_light_client::{Pallet, Call, Storage, Event<T>, Config, ValidateUnsigned} = 93,
        BridgeInboundChannel: bridge_inbound_channel::{Pallet, Call, Config, Storage, Event<T>} = 96,
        BridgeOutboundChannel: bridge_outbound_channel::{Pallet, Config<T>, Storage, Event<T>} = 97,
        Dispatch: dispatch::<Instance1>::{Pallet, Storage, Event<T>, Origin<T>} = 98,
        LeafProvider: leaf_provider::{Pallet, Storage, Event<T>} = 99,
        EthApp: eth_app::{Pallet, Call, Storage, Event<T>, Config<T>} = 100,
        ERC20App: erc20_app::{Pallet, Call, Storage, Event<T>, Config<T>} = 101,
        MigrationApp: migration_app::{Pallet, Call, Storage, Event<T>, Config} = 102,
        EvmBridgeProxy: evm_bridge_proxy::{Pallet, Call, Storage, Event} = 103,

        BeefyLightClient: beefy_light_client::{Pallet, Call, Storage, Event<T>, Config} = 104,
        Preimage: pallet_preimage::{Pallet, Call, Storage, Event<T>} = 105,
        SubstrateBridgeInboundChannel: substrate_bridge_channel::inbound::{Pallet, Call, Config, Storage, Event<T>} = 106,
        SubstrateBridgeOutboundChannel: substrate_bridge_channel::outbound::{Pallet, Config<T>, Storage, Event<T>} = 107,
        SubstrateDispatch: dispatch::<Instance2>::{Pallet, Storage, Event<T>, Origin<T>} = 108,
        SubstrateBridgeApp: substrate_bridge_app::{Pallet, Config<T>, Storage, Event<T>, Call} = 109,
    }
}

#[cfg(not(feature = "private-net"))]
construct_runtime! {
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Pallet, Call, Storage, Config, Event<T>} = 0,

        Babe: pallet_babe::{Pallet, Call, Storage, Config, ValidateUnsigned} = 14,

        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 1,
        // Balances in native currency - XOR.
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 2,
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage} = 4,
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>} = 5,
        Permissions: permissions::{Pallet, Call, Storage, Config<T>, Event<T>} = 6,
        Referrals: referrals::{Pallet, Call, Storage} = 7,
        Rewards: rewards::{Pallet, Call, Config<T>, Storage, Event<T>} = 8,
        XorFee: xor_fee::{Pallet, Call, Storage, Event<T>} = 9,
        BridgeMultisig: bridge_multisig::{Pallet, Call, Storage, Config<T>, Event<T>} = 10,
        Utility: pallet_utility::{Pallet, Call, Event} = 11,

        // Consensus and staking.
        Authorship: pallet_authorship::{Pallet, Call, Storage, Inherent} = 16,
        Staking: pallet_staking::{Pallet, Call, Config<T>, Storage, Event<T>} = 17,
        Offences: pallet_offences::{Pallet, Storage, Event} = 37,
        Historical: pallet_session_historical::{Pallet} = 13,
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>} = 12,
        Grandpa: pallet_grandpa::{Pallet, Call, Storage, Config, Event} = 15,
        ImOnline: pallet_im_online::{Pallet, Call, Storage, Event<T>, ValidateUnsigned, Config<T>} = 36,

        // Non-native tokens - everything apart of XOR.
        Tokens: tokens::{Pallet, Storage, Config<T>, Event<T>} = 18,
        // Unified interface for XOR and non-native tokens.
        Currencies: currencies::{Pallet, Call} = 19,
        TradingPair: trading_pair::{Pallet, Call, Storage, Config<T>, Event<T>} = 20,
        Assets: assets::{Pallet, Call, Storage, Config<T>, Event<T>} = 21,
        DEXManager: dex_manager::{Pallet, Storage, Config<T>} = 22,
        MulticollateralBondingCurvePool: multicollateral_bonding_curve_pool::{Pallet, Call, Storage, Config<T>, Event<T>} = 23,
        Technical: technical::{Pallet, Call, Config<T>, Event<T>, Storage} = 24,
        PoolXYK: pool_xyk::{Pallet, Call, Storage, Event<T>} = 25,
        LiquidityProxy: liquidity_proxy::{Pallet, Call, Event<T>} = 26,
        Council: pallet_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 27,
        TechnicalCommittee: pallet_collective::<Instance2>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 28,
        Democracy: pallet_democracy::{Pallet, Call, Storage, Config<T>, Event<T>} = 29,
        DEXAPI: dex_api::{Pallet, Call, Storage, Config} = 30,
        EthBridge: eth_bridge::{Pallet, Call, Storage, Config<T>, Event<T>} = 31,
        PswapDistribution: pswap_distribution::{Pallet, Call, Storage, Config<T>, Event<T>} = 32,
        Multisig: pallet_multisig::{Pallet, Call, Storage, Event<T>} = 33,
        Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 34,
        IrohaMigration: iroha_migration::{Pallet, Call, Storage, Config<T>, Event<T>} = 35,
        TechnicalMembership: pallet_membership::<Instance1>::{Pallet, Call, Storage, Event<T>, Config<T>} = 38,
        ElectionsPhragmen: pallet_elections_phragmen::{Pallet, Call, Storage, Event<T>, Config<T>} = 39,
        VestedRewards: vested_rewards::{Pallet, Call, Storage, Event<T>, Config} = 40,
        Identity: pallet_identity::{Pallet, Call, Storage, Event<T>} = 41,
        Farming: farming::{Pallet, Storage} = 42,
        XSTPool: xst::{Pallet, Call, Storage, Config<T>, Event<T>} = 43,
        PriceTools: price_tools::{Pallet, Storage, Event<T>} = 44,
        CeresStaking: ceres_staking::{Pallet, Call, Storage, Event<T>} = 45,
        CeresLiquidityLocker: ceres_liquidity_locker::{Pallet, Call, Storage, Event<T>} = 46,
        CeresTokenLocker: ceres_token_locker::{Pallet, Call, Storage, Event<T>} = 47,
        CeresGovernancePlatform: ceres_governance_platform::{Pallet, Call, Storage, Event<T>} = 48,
        CeresLaunchpad: ceres_launchpad::{Pallet, Call, Storage, Event<T>} = 49,
        DemeterFarmingPlatform: demeter_farming_platform::{Pallet, Call, Storage, Event<T>} = 50,
        // Provides a semi-sorted list of nominators for staking.
        BagsList: pallet_bags_list::{Pallet, Call, Storage, Event<T>} = 51,
        ElectionProviderMultiPhase: pallet_election_provider_multi_phase::{Pallet, Call, Storage, Event<T>, ValidateUnsigned} = 52,
        Band: band::{Pallet, Call, Storage, Event<T>} = 53,
        OracleProxy: oracle_proxy::{Pallet, Call, Storage, Event<T>} = 54,


        // Trustless ethereum bridge
        Mmr: pallet_mmr::{Pallet, Storage} = 90,
        Beefy: pallet_beefy::{Pallet, Config<T>, Storage} = 91,
        MmrLeaf: pallet_beefy_mmr::{Pallet, Storage} = 92,
        EthereumLightClient: ethereum_light_client::{Pallet, Call, Storage, Event<T>, Config} = 93,
        BridgeInboundChannel: bridge_inbound_channel::{Pallet, Call, Config, Storage, Event<T>} = 96,
        BridgeOutboundChannel: bridge_outbound_channel::{Pallet, Config<T>, Storage, Event<T>} = 97,
        Dispatch: dispatch::<Instance1>::{Pallet, Storage, Event<T>, Origin<T>} = 98,
        LeafProvider: leaf_provider::{Pallet, Storage, Event<T>} = 99,
        EthApp: eth_app::{Pallet, Call, Storage, Event<T>, Config<T>} = 100,
        ERC20App: erc20_app::{Pallet, Call, Storage, Event<T>, Config<T>} = 101,
        MigrationApp: migration_app::{Pallet, Call, Storage, Event<T>, Config} = 102,
        EvmBridgeProxy: evm_bridge_proxy::{Pallet, Call, Storage, Event} = 103,

        BeefyLightClient: beefy_light_client::{Pallet, Call, Storage, Event<T>, Config} = 104,
        Preimage: pallet_preimage::{Pallet, Call, Storage, Event<T>} = 105,
        SubstrateBridgeInboundChannel: substrate_bridge_channel::inbound::{Pallet, Call, Config, Storage, Event<T>} = 106,
        SubstrateBridgeOutboundChannel: substrate_bridge_channel::outbound::{Pallet, Config<T>, Storage, Event<T>} = 107,
        SubstrateDispatch: dispatch::<Instance2>::{Pallet, Storage, Event<T>, Origin<T>} = 108,
        SubstrateBridgeApp: substrate_bridge_app::{Pallet, Config<T>, Storage, Event<T>, Call} = 109,
    }
}

// This is needed, because the compiler automatically places `Serialize` bound
// when `derive` is used, but the method is never actually used
#[cfg(feature = "std")]
impl Serialize for Runtime {
    fn serialize<S>(
        &self,
        _serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        unreachable!("we never serialize runtime; qed")
    }
}

/// The address format for describing accounts.
pub type Address = AccountId;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    (
        pallet_multisig::migrations::v1::MigrateToV1<Runtime>,
        pallet_preimage::migration::v1::Migration<Runtime>,
        pallet_democracy::migrations::v1::Migration<Runtime>,
        pallet_scheduler::migration::v3::MigrateToV4<Runtime>,
    ),
>;

pub type MmrHashing = <Runtime as pallet_mmr::Config>::Hashing;

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(
            extrinsic: <Block as BlockT>::Extrinsic,
        ) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(block: Block, data: sp_inherents::InherentData) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }

        // fn random_seed() -> <Block as BlockT>::Hash {
        //     RandomnessCollectiveFlip::random_seed()
        // }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }

        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
        Block,
        Balance,
    > for Runtime {
        fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            let maybe_dispatch_info = XorFee::query_info(&uxt, len);
            let output = match maybe_dispatch_info {
                Some(dispatch_info) => dispatch_info,
                _ => TransactionPayment::query_info(uxt, len),
            };
            output
        }

        fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> pallet_transaction_payment_rpc_runtime_api::FeeDetails<Balance> {
            let maybe_fee_details = XorFee::query_fee_details(&uxt, len);
            let output = match maybe_fee_details {
                Some(fee_details) => fee_details,
                _ => TransactionPayment::query_fee_details(uxt, len),
            };
            output
        }
    }

    impl dex_manager_runtime_api::DEXManagerAPI<Block, DEXId> for Runtime {
        fn list_dex_ids() -> Vec<DEXId> {
            DEXManager::list_dex_ids()
        }
    }

    impl dex_runtime_api::DEXAPI<
        Block,
        AssetId,
        DEXId,
        Balance,
        LiquiditySourceType,
        SwapVariant,
    > for Runtime {
        #[cfg_attr(not(feature = "private-net"), allow(unused))]
        fn quote(
            dex_id: DEXId,
            liquidity_source_type: LiquiditySourceType,
            input_asset_id: AssetId,
            output_asset_id: AssetId,
            desired_input_amount: BalanceWrapper,
            swap_variant: SwapVariant,
        ) -> Option<dex_runtime_api::SwapOutcomeInfo<Balance>> {
            #[cfg(feature = "private-net")]
            {
                DEXAPI::quote(
                    &LiquiditySourceId::new(dex_id, liquidity_source_type),
                    &input_asset_id,
                    &output_asset_id,
                    QuoteAmount::with_variant(swap_variant, desired_input_amount.into()),
                    true,
                ).ok().map(|(sa, _)| dex_runtime_api::SwapOutcomeInfo::<Balance> { amount: sa.amount, fee: sa.fee})
            }
            #[cfg(not(feature = "private-net"))]
            {
                // Mainnet should not be able to access liquidity source quote directly, to avoid arbitrage exploits.
                None
            }
        }

        fn can_exchange(
            dex_id: DEXId,
            liquidity_source_type: LiquiditySourceType,
            input_asset_id: AssetId,
            output_asset_id: AssetId,
        ) -> bool {
            DEXAPI::can_exchange(
                &LiquiditySourceId::new(dex_id, liquidity_source_type),
                &input_asset_id,
                &output_asset_id,
            )
        }

        fn list_supported_sources() -> Vec<LiquiditySourceType> {
            DEXAPI::get_supported_types()
        }
    }

    impl trading_pair_runtime_api::TradingPairAPI<Block, DEXId, common::TradingPair<AssetId>, AssetId, LiquiditySourceType> for Runtime {
        fn list_enabled_pairs(dex_id: DEXId) -> Vec<common::TradingPair<AssetId>> {
            // TODO: error passing PR fixes this crunch return
            TradingPair::list_trading_pairs(&dex_id).unwrap_or(Vec::new())
        }

        fn is_pair_enabled(dex_id: DEXId, asset_id_a: AssetId, asset_id_b: AssetId) -> bool {
            // TODO: error passing PR fixes this crunch return
            TradingPair::is_trading_pair_enabled(&dex_id, &asset_id_a, &asset_id_b).unwrap_or(false)
                || TradingPair::is_trading_pair_enabled(&dex_id, &asset_id_b, &asset_id_a).unwrap_or(false)
        }

        fn list_enabled_sources_for_pair(
            dex_id: DEXId,
            base_asset_id: AssetId,
            target_asset_id: AssetId,
        ) -> Vec<LiquiditySourceType> {
            // TODO: error passing PR fixes this crunch return
            TradingPair::list_enabled_sources_for_trading_pair(&dex_id, &base_asset_id, &target_asset_id).map(|bts| bts.into_iter().collect::<Vec<_>>()).unwrap_or(Vec::new())
        }

        fn is_source_enabled_for_pair(
            dex_id: DEXId,
            base_asset_id: AssetId,
            target_asset_id: AssetId,
            source_type: LiquiditySourceType,
        ) -> bool {
            // TODO: error passing PR fixes this crunch return
            TradingPair::is_source_enabled_for_trading_pair(&dex_id, &base_asset_id, &target_asset_id, source_type).unwrap_or(false)
        }
    }

    impl assets_runtime_api::AssetsAPI<Block, AccountId, AssetId, Balance, AssetSymbol, AssetName, BalancePrecision, ContentSource, Description> for Runtime {
        fn free_balance(account_id: AccountId, asset_id: AssetId) -> Option<assets_runtime_api::BalanceInfo<Balance>> {
            Assets::free_balance(&asset_id, &account_id).ok().map(|balance|
                assets_runtime_api::BalanceInfo::<Balance> {
                    balance: balance.clone(),
                }
            )
        }

        fn usable_balance(account_id: AccountId, asset_id: AssetId) -> Option<assets_runtime_api::BalanceInfo<Balance>> {
            let usable_balance = if asset_id == <Runtime as currencies::Config>::GetNativeCurrencyId::get() {
                Balances::usable_balance(account_id)
            } else {
                let account_data = Tokens::accounts(account_id, asset_id);
                account_data.free.saturating_sub(account_data.frozen)
            };
            Some(assets_runtime_api::BalanceInfo { balance: usable_balance })
        }

        fn total_balance(account_id: AccountId, asset_id: AssetId) -> Option<assets_runtime_api::BalanceInfo<Balance>> {
            Assets::total_balance(&asset_id, &account_id).ok().map(|balance|
                assets_runtime_api::BalanceInfo::<Balance> {
                    balance: balance.clone(),
                }
            )
        }

        fn total_supply(asset_id: AssetId) -> Option<assets_runtime_api::BalanceInfo<Balance>> {
            Assets::total_issuance(&asset_id).ok().map(|balance|
                assets_runtime_api::BalanceInfo::<Balance> {
                    balance: balance.clone(),
                }
            )
        }

        fn list_asset_ids() -> Vec<AssetId> {
            Assets::list_registered_asset_ids()
        }

        fn list_asset_infos() -> Vec<assets_runtime_api::AssetInfo<AssetId, AssetSymbol, AssetName, u8, ContentSource, Description>> {
            Assets::list_registered_asset_infos().into_iter().map(|(asset_id, symbol, name, precision, is_mintable, content_source, description)|
                assets_runtime_api::AssetInfo::<AssetId, AssetSymbol, AssetName, BalancePrecision, ContentSource, Description> {
                    asset_id,
                    symbol,
                    name,
                    precision,
                    is_mintable,
                    content_source,
                    description
                }
            ).collect()
        }

        fn get_asset_info(asset_id: AssetId) -> Option<assets_runtime_api::AssetInfo<AssetId, AssetSymbol, AssetName, BalancePrecision, ContentSource, Description>> {
            let (symbol, name, precision, is_mintable, content_source, description) = Assets::get_asset_info(&asset_id);
            Some(assets_runtime_api::AssetInfo::<AssetId, AssetSymbol, AssetName, BalancePrecision, ContentSource, Description> {
                asset_id,
                symbol,
                name,
                precision,
                is_mintable,
                content_source,
                description
            })
        }

        fn get_asset_content_src(asset_id: AssetId) -> Option<ContentSource> {
            Assets::get_asset_content_src(&asset_id)
        }
    }

    impl
        eth_bridge_runtime_api::EthBridgeRuntimeApi<
            Block,
            sp_core::H256,
            SignatureParams,
            AccountId,
            AssetKind,
            AssetId,
            sp_core::H160,
            OffchainRequest<Runtime>,
            RequestStatus,
            OutgoingRequestEncoded,
            NetworkId,
            BalancePrecision,
        > for Runtime
    {
        fn get_requests(
            hashes: Vec<sp_core::H256>,
            network_id: Option<NetworkId>,
            redirect_finished_load_requests: bool,
        ) -> Result<
            Vec<(
                OffchainRequest<Runtime>,
                RequestStatus,
            )>,
            DispatchError,
        > {
            EthBridge::get_requests(&hashes, network_id, redirect_finished_load_requests)
        }

        fn get_approved_requests(
            hashes: Vec<sp_core::H256>,
            network_id: Option<NetworkId>
        ) -> Result<
            Vec<(
                OutgoingRequestEncoded,
                Vec<SignatureParams>,
            )>,
            DispatchError,
        > {
            EthBridge::get_approved_requests(&hashes, network_id)
        }

        fn get_approvals(
            hashes: Vec<sp_core::H256>,
            network_id: Option<NetworkId>
        ) -> Result<Vec<Vec<SignatureParams>>, DispatchError> {
            EthBridge::get_approvals(&hashes, network_id)
        }

        fn get_account_requests(account_id: AccountId, status_filter: Option<RequestStatus>) -> Result<Vec<(NetworkId, sp_core::H256)>, DispatchError> {
            EthBridge::get_account_requests(&account_id, status_filter)
        }

        fn get_registered_assets(
            network_id: Option<NetworkId>
        ) -> Result<Vec<(
                AssetKind,
                (AssetId, BalancePrecision),
                Option<(sp_core::H160, BalancePrecision)
        >)>, DispatchError> {
            EthBridge::get_registered_assets(network_id)
        }
    }

    impl iroha_migration_runtime_api::IrohaMigrationAPI<Block> for Runtime {
        fn needs_migration(iroha_address: String) -> bool {
            IrohaMigration::needs_migration(&iroha_address)
        }
    }

    impl beefy_light_client_runtime_api::BeefyLightClientAPI<Block, beefy_light_client::BitField> for Runtime {
        fn get_random_bitfield(network_id: SubNetworkId, prior: beefy_light_client::BitField, num_of_validators: u32) -> beefy_light_client::BitField {
            let len = prior.len() as usize;
            BeefyLightClient::create_random_bit_field(network_id, prior, num_of_validators).unwrap_or(beefy_light_client::BitField::with_capacity(len))
        }
    }

    impl liquidity_proxy_runtime_api::LiquidityProxyAPI<
        Block,
        DEXId,
        AssetId,
        Balance,
        SwapVariant,
        LiquiditySourceType,
        FilterMode,
    > for Runtime {
        fn quote(
            dex_id: DEXId,
            input_asset_id: AssetId,
            output_asset_id: AssetId,
            amount: BalanceWrapper,
            swap_variant: SwapVariant,
            selected_source_types: Vec<LiquiditySourceType>,
            filter_mode: FilterMode,
        ) -> Option<liquidity_proxy_runtime_api::SwapOutcomeInfo<Balance, AssetId>> {
            if LiquidityProxy::is_forbidden_filter(&input_asset_id, &output_asset_id, &selected_source_types, &filter_mode) {
                return None;
            }

            LiquidityProxy::inner_quote(
                dex_id,
                &input_asset_id,
                &output_asset_id,
                QuoteAmount::with_variant(swap_variant, amount.into()),
                LiquiditySourceFilter::with_mode(dex_id, filter_mode, selected_source_types),
                false,
                true,
            ).ok().map(|(asa, rewards, _, _)| liquidity_proxy_runtime_api::SwapOutcomeInfo::<Balance, AssetId> {
                amount: asa.amount,
                fee: asa.fee,
                rewards: rewards.into_iter()
                                .map(|(amount, currency, reason)| liquidity_proxy_runtime_api::RewardsInfo::<Balance, AssetId> {
                                    amount,
                                    currency,
                                    reason
                                }).collect()
                })
        }

        fn is_path_available(
            dex_id: DEXId,
            input_asset_id: AssetId,
            output_asset_id: AssetId
        ) -> bool {
            LiquidityProxy::is_path_available(
                dex_id, input_asset_id, output_asset_id
            ).unwrap_or(false)
        }

        fn list_enabled_sources_for_path(
            dex_id: DEXId,
            input_asset_id: AssetId,
            output_asset_id: AssetId,
        ) -> Vec<LiquiditySourceType> {
            LiquidityProxy::list_enabled_sources_for_path_with_xyk_forbidden(
                dex_id, input_asset_id, output_asset_id
            ).unwrap_or(Vec::new())
        }
    }

    impl oracle_proxy_runtime_api::OracleProxyAPI<
        Block,
        Symbol,
        ResolveTime
    > for Runtime {
        fn quote(symbol: Symbol) -> Result<Option<oracle_proxy_runtime_api::RateInfo>, DispatchError>  {
            let rate_wrapped = <
                OracleProxy as common::DataFeed<Symbol, common::Rate, ResolveTime>
            >::quote(&symbol);
            match rate_wrapped {
                Ok(rate) => Ok(rate.map(|rate| oracle_proxy_runtime_api::RateInfo{
                    value: rate.value,
                    last_updated: rate.last_updated
                })),
                Err(e) => Err(e)
            }
        }

        fn list_enabled_symbols() -> Result<Vec<(Symbol, ResolveTime)>, DispatchError> {
            <
                OracleProxy as common::DataFeed<Symbol, common::Rate, ResolveTime>
            >::list_enabled_symbols()
        }
    }

    impl pswap_distribution_runtime_api::PswapDistributionAPI<
        Block,
        AccountId,
        Balance,
    > for Runtime {
        fn claimable_amount(
            account_id: AccountId,
        ) -> pswap_distribution_runtime_api::BalanceInfo<Balance> {
            let claimable = PswapDistribution::claimable_amount(&account_id).unwrap_or(0);
            pswap_distribution_runtime_api::BalanceInfo::<Balance> {
                balance: claimable
            }
        }
    }

    impl rewards_runtime_api::RewardsAPI<Block, sp_core::H160, Balance> for Runtime {
        fn claimables(eth_address: sp_core::H160) -> Vec<rewards_runtime_api::BalanceInfo<Balance>> {
            Rewards::claimables(&eth_address).into_iter().map(|balance| rewards_runtime_api::BalanceInfo::<Balance> { balance }).collect()
        }
    }

    impl sp_consensus_babe::BabeApi<Block> for Runtime {
            fn configuration() -> sp_consensus_babe::BabeConfiguration {
                    // The choice of `c` parameter (where `1 - c` represents the
                    // probability of a slot being empty), is done in accordance to the
                    // slot duration and expected target block time, for safely
                    // resisting network delays of maximum two seconds.
                    // <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
                    sp_consensus_babe::BabeConfiguration {
                            slot_duration: Babe::slot_duration(),
                            epoch_length: EpochDuration::get(),
                            c: PRIMARY_PROBABILITY,
                            authorities: Babe::authorities().to_vec(),
                            randomness: Babe::randomness(),
                            allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryVRFSlots,
                    }
            }

            fn current_epoch() -> sp_consensus_babe::Epoch {
                Babe::current_epoch()
            }

            fn current_epoch_start() -> sp_consensus_babe::Slot {
                Babe::current_epoch_start()
            }

            fn next_epoch() -> sp_consensus_babe::Epoch {
                Babe::next_epoch()
            }

            fn generate_key_ownership_proof(
                    _slot_number: sp_consensus_babe::Slot,
                    authority_id: sp_consensus_babe::AuthorityId,
            ) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
                    use codec::Encode;
                    Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id))
                            .map(|p| p.encode())
                            .map(sp_consensus_babe::OpaqueKeyOwnershipProof::new)
            }

            fn submit_report_equivocation_unsigned_extrinsic(
                    equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
                    key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
            ) -> Option<()> {
                    let key_owner_proof = key_owner_proof.decode()?;
                    Babe::submit_unsigned_equivocation_report(
                            equivocation_proof,
                            key_owner_proof,
                    )
            }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl beefy_primitives::BeefyApi<Block> for Runtime {
        fn validator_set() -> Option<beefy_primitives::ValidatorSet<BeefyId>> {
                Beefy::validator_set()
        }
    }

    impl mmr::MmrApi<Block, Hash, BlockNumber> for Runtime {
        fn generate_proof(block_number: BlockNumber)
            -> Result<(mmr::EncodableOpaqueLeaf, mmr::Proof<Hash>), mmr::Error>
        {
            Mmr::generate_batch_proof(vec![block_number])
                .and_then(|(leaves, proof)| Ok((
                    mmr::EncodableOpaqueLeaf::from_leaf(&leaves[0]),
                    mmr::BatchProof::into_single_leaf_proof(proof)?
                )))
        }

        fn verify_proof(leaf: mmr::EncodableOpaqueLeaf, proof: mmr::Proof<Hash>)
            -> Result<(), mmr::Error>
        {
            pub type MmrLeaf = <<Runtime as pallet_mmr::Config>::LeafData as mmr::LeafDataProvider>::LeafData;
            let leaf: MmrLeaf = leaf
                .into_opaque_leaf()
                .try_decode()
                .ok_or(mmr::Error::Verify)?;
            Mmr::verify_leaves(vec![leaf], mmr::Proof::into_batch_proof(proof))
        }

        fn verify_proof_stateless(
            root: Hash,
            leaf: mmr::EncodableOpaqueLeaf,
            proof: mmr::Proof<Hash>
        ) -> Result<(), mmr::Error> {
            let node = mmr::DataOrHash::Data(leaf.into_opaque_leaf());
            pallet_mmr::verify_leaves_proof::<MmrHashing, _>(root, vec![node], mmr::Proof::into_batch_proof(proof))
        }

        fn mmr_root() -> Result<Hash, mmr::Error> {
            Ok(Mmr::mmr_root())
        }

        fn generate_batch_proof(block_numbers: Vec<BlockNumber>)
            -> Result<(Vec<mmr::EncodableOpaqueLeaf>, mmr::BatchProof<Hash>), mmr::Error>
        {
            Mmr::generate_batch_proof(block_numbers)
                .map(|(leaves, proof)| (leaves.into_iter().map(|leaf| mmr::EncodableOpaqueLeaf::from_leaf(&leaf)).collect(), proof))
        }

        fn generate_historical_batch_proof(block_numbers: Vec<BlockNumber>,
            best_known_block_number: BlockNumber)
            -> Result<(Vec<mmr::EncodableOpaqueLeaf>, mmr::BatchProof<Hash>), mmr::Error>
        {
            Mmr::generate_historical_batch_proof(block_numbers, best_known_block_number)
                .map(|(leaves, proof)| (leaves.into_iter().map(|leaf| mmr::EncodableOpaqueLeaf::from_leaf(&leaf)).collect(), proof))
        }

        fn verify_batch_proof(leaves: Vec<mmr::EncodableOpaqueLeaf>, proof: mmr::BatchProof<Hash>)
            -> Result<(), mmr::Error>
        {
            pub type MmrLeaf = <<Runtime as pallet_mmr::Config>::LeafData as mmr::LeafDataProvider>::LeafData;
            let leaves = leaves.into_iter().map(|leaf|
                leaf.into_opaque_leaf()
                .try_decode()
                .ok_or(mmr::Error::Verify)).collect::<Result<Vec<MmrLeaf>, mmr::Error>>()?;
            Mmr::verify_leaves(leaves, proof)
        }

        fn verify_batch_proof_stateless(
            root: Hash,
            leaves: Vec<mmr::EncodableOpaqueLeaf>,
            proof: mmr::BatchProof<Hash>
        ) -> Result<(), mmr::Error> {
            let nodes = leaves.into_iter().map(|leaf|mmr::DataOrHash::Data(leaf.into_opaque_leaf())).collect();
            pallet_mmr::verify_leaves_proof::<MmrHashing, _>(root, nodes, proof)
        }
    }

    impl fg_primitives::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> GrandpaAuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> fg_primitives::SetId {
            Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            equivocation_proof: fg_primitives::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            let key_owner_proof = key_owner_proof.decode()?;
            Grandpa::submit_unsigned_equivocation_report(
                equivocation_proof,
                key_owner_proof,
            )
        }

        fn generate_key_ownership_proof(
            _set_id: fg_primitives::SetId,
            authority_id: GrandpaId,
        ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
            use codec::Encode;
            Historical::prove((fg_primitives::KEY_TYPE, authority_id))
                .map(|p| p.encode())
                .map(fg_primitives::OpaqueKeyOwnershipProof::new)
        }
    }

    impl leaf_provider_runtime_api::LeafProviderAPI<Block> for Runtime {
        fn latest_digest() -> Option<bridge_types::types::AuxiliaryDigest> {
                LeafProvider::latest_digest().map(|logs| bridge_types::types::AuxiliaryDigest{ logs })
        }

    }

    impl evm_bridge_proxy_runtime_api::EvmBridgeProxyAPI<Block, AssetId> for Runtime {
        fn list_apps(network_id: bridge_types::EVMChainId) -> Vec<bridge_types::types::BridgeAppInfo> {
            EvmBridgeProxy::list_apps(network_id)
        }

        fn list_supported_assets(network_id: bridge_types::EVMChainId) -> Vec<bridge_types::types::BridgeAssetInfo<AssetId>> {
            EvmBridgeProxy::list_supported_assets(network_id)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;

            use liquidity_proxy_benchmarking::Pallet as LiquidityProxyBench;
            use pool_xyk_benchmarking::Pallet as XYKPoolBench;
            use pswap_distribution_benchmarking::Pallet as PswapDistributionBench;
            use ceres_liquidity_locker_benchmarking::Pallet as CeresLiquidityLockerBench;
            use demeter_farming_platform_benchmarking::Pallet as DemeterFarmingPlatformBench;

            let mut list = Vec::<BenchmarkList>::new();

            list_benchmark!(list, extra, assets, Assets);
            #[cfg(feature = "private-net")]
            list_benchmark!(list, extra, faucet, Faucet);
            list_benchmark!(list, extra, farming, Farming);
            list_benchmark!(list, extra, iroha_migration, IrohaMigration);
            list_benchmark!(list, extra, liquidity_proxy, LiquidityProxyBench::<Runtime>);
            list_benchmark!(list, extra, multicollateral_bonding_curve_pool, MulticollateralBondingCurvePool);
            list_benchmark!(list, extra, pswap_distribution, PswapDistributionBench::<Runtime>);
            list_benchmark!(list, extra, rewards, Rewards);
            list_benchmark!(list, extra, trading_pair, TradingPair);
            list_benchmark!(list, extra, pool_xyk, XYKPoolBench::<Runtime>);
            list_benchmark!(list, extra, eth_bridge, EthBridge);
            list_benchmark!(list, extra, vested_rewards, VestedRewards);
            list_benchmark!(list, extra, price_tools, PriceTools);
            list_benchmark!(list, extra, xor_fee, XorFee);
            list_benchmark!(list, extra, ethereum_light_client, EthereumLightClient);
            list_benchmark!(list, extra, referrals, Referrals);
            list_benchmark!(list, extra, ceres_staking, CeresStaking);
            list_benchmark!(list, extra, ceres_liquidity_locker, CeresLiquidityLockerBench::<Runtime>);
            list_benchmark!(list, extra, ceres_token_locker, CeresTokenLocker);
            list_benchmark!(list, extra, ceres_governance_platform, CeresGovernancePlatform);
            list_benchmark!(list, extra, ceres_launchpad, CeresLaunchpad);
            list_benchmark!(list, extra, demeter_farming_platform, DemeterFarmingPlatformBench::<Runtime>);
            list_benchmark!(list, extra, evm_bridge_proxy, EvmBridgeProxy);
            list_benchmark!(list, extra, band, Band);
            list_benchmark!(list, extra, xst, XSTPool);
            list_benchmark!(list, extra, oracle_proxy, OracleProxy);

            // Trustless bridge
            list_benchmark!(list, extra, ethereum_light_client, EthereumLightClient);
            list_benchmark!(list, extra, bridge_inbound_channel, BridgeInboundChannel);
            list_benchmark!(list, extra, bridge_outbound_channel, BridgeOutboundChannel);
            list_benchmark!(list, extra, eth_app, EthApp);
            list_benchmark!(list, extra, erc20_app, ERC20App);
            list_benchmark!(list, extra, migration_app, MigrationApp);
            list_benchmark!(list, extra, evm_bridge_proxy, EvmBridgeProxy);

            let storage_info = AllPalletsWithSystem::storage_info();

            return (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

            use liquidity_proxy_benchmarking::Pallet as LiquidityProxyBench;
            use pool_xyk_benchmarking::Pallet as XYKPoolBench;
            use pswap_distribution_benchmarking::Pallet as PswapDistributionBench;
            use ceres_liquidity_locker_benchmarking::Pallet as CeresLiquidityLockerBench;
            use demeter_farming_platform_benchmarking::Pallet as DemeterFarmingPlatformBench;

            impl liquidity_proxy_benchmarking::Config for Runtime {}
            impl pool_xyk_benchmarking::Config for Runtime {}
            impl pswap_distribution_benchmarking::Config for Runtime {}
            impl ceres_liquidity_locker_benchmarking::Config for Runtime {}

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
                // Event Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
                // Treasury Account
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da95ecffd7b6c0f78751baa9d281e0bfa3a6d6f646c70792f74727372790000000000000000000000000000000000000000").to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmark!(params, batches, assets, Assets);
            #[cfg(feature = "private-net")]
            add_benchmark!(params, batches, faucet, Faucet);
            add_benchmark!(params, batches, farming, Farming);
            add_benchmark!(params, batches, iroha_migration, IrohaMigration);
            add_benchmark!(params, batches, liquidity_proxy, LiquidityProxyBench::<Runtime>);
            add_benchmark!(params, batches, multicollateral_bonding_curve_pool, MulticollateralBondingCurvePool);
            add_benchmark!(params, batches, pswap_distribution, PswapDistributionBench::<Runtime>);
            add_benchmark!(params, batches, rewards, Rewards);
            add_benchmark!(params, batches, trading_pair, TradingPair);
            add_benchmark!(params, batches, pool_xyk, XYKPoolBench::<Runtime>);
            add_benchmark!(params, batches, eth_bridge, EthBridge);
            add_benchmark!(params, batches, vested_rewards, VestedRewards);
            add_benchmark!(params, batches, price_tools, PriceTools);
            add_benchmark!(params, batches, ethereum_light_client, EthereumLightClient);
            add_benchmark!(params, batches, xor_fee, XorFee);
            add_benchmark!(params, batches, referrals, Referrals);
            add_benchmark!(params, batches, ceres_staking, CeresStaking);
            add_benchmark!(params, batches, ceres_liquidity_locker, CeresLiquidityLockerBench::<Runtime>);
            add_benchmark!(params, batches, ceres_token_locker, CeresTokenLocker);
            add_benchmark!(params, batches, ceres_governance_platform, CeresGovernancePlatform);
            add_benchmark!(params, batches, ceres_launchpad, CeresLaunchpad);
            add_benchmark!(params, batches, demeter_farming_platform, DemeterFarmingPlatformBench::<Runtime>);
            add_benchmark!(params, batches, evm_bridge_proxy, EvmBridgeProxy);
            add_benchmark!(params, batches, band, Band);
            add_benchmark!(params, batches, xst, XSTPool);
            add_benchmark!(params, batches, oracle_proxy, OracleProxy);

            // Trustless bridge
            add_benchmark!(params, batches, ethereum_light_client, EthereumLightClient);
            add_benchmark!(params, batches, bridge_inbound_channel, BridgeInboundChannel);
            add_benchmark!(params, batches, bridge_outbound_channel, BridgeOutboundChannel);
            add_benchmark!(params, batches, eth_app, EthApp);
            add_benchmark!(params, batches, erc20_app, ERC20App);
            add_benchmark!(params, batches, migration_app, MigrationApp);
            add_benchmark!(params, batches, evm_bridge_proxy, EvmBridgeProxy);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }

    impl vested_rewards_runtime_api::VestedRewardsApi<Block, AccountId, AssetId, Balance> for Runtime {
        fn crowdloan_claimable(account_id: AccountId, asset_id: AssetId) -> Option<vested_rewards_runtime_api::BalanceInfo<Balance>> {
            use sp_runtime::traits::UniqueSaturatedInto;

            let current_block_num = <frame_system::Pallet<Runtime>>::block_number().unique_saturated_into();
            VestedRewards::crowdloan_reward_for_asset(&account_id, &asset_id, current_block_num).ok().map(|balance|
                vested_rewards_runtime_api::BalanceInfo::<Balance> {
                    balance
                }
            )
        }

        fn crowdloan_lease() -> vested_rewards_runtime_api::CrowdloanLease {
            use vested_rewards::{LEASE_START_BLOCK, LEASE_TOTAL_DAYS, BLOCKS_PER_DAY};

            vested_rewards_runtime_api::CrowdloanLease {
                start_block: LEASE_START_BLOCK,
                total_days: LEASE_TOTAL_DAYS,
                blocks_per_day: BLOCKS_PER_DAY,
            }
        }
    }

    impl farming_runtime_api::FarmingApi<Block, AssetId> for Runtime {
        fn reward_doubling_assets() -> Vec<AssetId> {
            Farming::reward_doubling_assets()
        }
    }
}
