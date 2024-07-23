use crate::migrations::VotingOption;
use crate::{self as hermes_governance_platform};
use codec::{Decode, Encode};
use common::mock::{ExistentialDeposits, GetTradingPairRestrictedFlag};
use common::prelude::Balance;
use common::{
    balance, fixed, mock_assets_config, mock_common_config, mock_currencies_config,
    mock_frame_system_config, mock_pallet_balances_config, mock_pallet_timestamp_config,
    mock_permissions_config, mock_technical_config, mock_tokens_config, AssetId32, AssetName,
    AssetSymbol, BalancePrecision, ContentSource, Description, Fixed, HERMES_ASSET_ID, PSWAP, TBCD,
};
use currencies::BasicCurrencyAdapter;
use frame_support::traits::{Everything, Hooks};
use frame_support::weights::Weight;
use frame_support::{construct_runtime, parameter_types};
use frame_system;
use frame_system::pallet_prelude::BlockNumberFor;
use hex_literal::hex;
use sp_core::crypto::AccountId32;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup, Zero};
use sp_runtime::{BuildStorage, Perbill, Percent};

pub type TechAccountId = common::TechAccountId<AccountId, TechAssetId, DEXId>;
type TechAssetId = common::TechAssetId<common::PredefinedAssetId>;
type DEXId = common::DEXId;
type Block = frame_system::mocking::MockBlock<Runtime>;
type Moment = u64;

construct_runtime! {
    pub enum Runtime {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Assets: assets::{Pallet, Call, Config<T>, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Tokens: tokens::{Pallet, Call, Config<T>, Storage, Event<T>},
        Currencies: currencies::{Pallet, Call, Storage},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
        DexManager: dex_manager::{Pallet, Call, Config<T>, Storage},
        Permissions: permissions::{Pallet, Call, Config<T>, Storage, Event<T>},
        Technical: technical::{Pallet, Call, Config<T>, Storage, Event<T>},
        PoolXYK: pool_xyk::{Pallet, Call, Storage, Event<T>},
        PswapDistribution: pswap_distribution::{Pallet, Call, Config<T>, Storage, Event<T>},
        CeresLiquidityLocker: ceres_liquidity_locker::{Pallet, Call, Storage, Event<T>},
        CeresGovernancePlatform: ceres_governance_platform::{Pallet, Call, Storage, Event<T>},
        DemeterFarmingPlatform: demeter_farming_platform::{Pallet, Call, Storage, Event<T>},
        HermesGovernancePlatform: hermes_governance_platform::{Pallet, Call, Storage, Event<T>},
    }
}

pub type AccountId = AccountId32;
pub type BlockNumber = u64;
pub type Amount = i128;
pub type AssetId = AssetId32<common::PredefinedAssetId>;

pub const ALICE: AccountId = AccountId::new(hex!(
    "0000000000000000000000000000000000000000000000000000000000000001"
));
pub const BOB: AccountId = AccountId::new(hex!(
    "0000000000000000000000000000000000000000000000000000000000000002"
));
pub const CHARLES: AccountId = AccountId::new(hex!(
    "0000000000000000000000000000000000000000000000000000000000000003"
));

#[derive(Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
pub struct OldHermesVotingInfo {
    /// Voting option
    pub voting_option: VotingOption,
    /// Number of hermes
    pub number_of_hermes: Balance,
    /// Hermes withdrawn
    pub hermes_withdrawn: bool,
}

mock_technical_config!(Runtime, pool_xyk::PolySwapAction<DEXId, AssetId, AccountId, TechAccountId>);
mock_currencies_config!(Runtime);
mock_pallet_balances_config!(Runtime);
mock_frame_system_config!(Runtime);
mock_common_config!(Runtime);
mock_tokens_config!(Runtime);
mock_assets_config!(Runtime);
mock_permissions_config!(Runtime);
mock_pallet_timestamp_config!(Runtime);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = Weight::from_parts(1024, 0);
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    pub GetXykFee: Fixed = fixed!(0.003);
    pub GetIncentiveAssetId: AssetId = PSWAP.into();
    pub const GetDefaultSubscriptionFrequency: BlockNumber = 10;
    pub const GetBurnUpdateFrequency: BlockNumber = 14400;
    pub GetParliamentAccountId: AccountId = AccountId32::from([100; 32]);
    pub GetPswapDistributionAccountId: AccountId = AccountId32::from([101; 32]);
    pub GetXykIrreducibleReservePercent: Percent = Percent::from_percent(1);
}

parameter_types! {
    pub const HermesAssetId: AssetId = HERMES_ASSET_ID;
    pub const StringLimit: u32 = 64;
    pub const OptionsLimit: u32 = 5;
    pub const TitleLimit: u32 = 128;
    pub const DescriptionLimit: u32 = 4096;
}

impl crate::Config for Runtime {
    const MIN_DURATION_OF_POLL: Self::Moment = 14_400_000;
    const MAX_DURATION_OF_POLL: Self::Moment = 604_800_000;
    type StringLimit = StringLimit;
    type OptionsLimit = OptionsLimit;
    type TitleLimit = TitleLimit;
    type DescriptionLimit = DescriptionLimit;
    type RuntimeEvent = RuntimeEvent;
    type HermesAssetId = HermesAssetId;
    type WeightInfo = ();
    type AssetInfoProvider = assets::Pallet<Runtime>;
}

parameter_types! {
    pub const GetBaseAssetId: AssetId = HERMES_ASSET_ID;
    pub const GetBuyBackAssetId: AssetId = TBCD;
}

impl dex_manager::Config for Runtime {}

impl demeter_farming_platform::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type DemeterAssetId = ();
    const BLOCKS_PER_HOUR_AND_A_HALF: BlockNumberFor<Self> = 900;
    type WeightInfo = ();
    type AssetInfoProvider = assets::Pallet<Runtime>;
}

impl ceres_governance_platform::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type StringLimit = StringLimit;
    type OptionsLimit = OptionsLimit;
    type TitleLimit = TitleLimit;
    type DescriptionLimit = DescriptionLimit;
}

impl pool_xyk::Config for Runtime {
    const MIN_XOR: Balance = balance!(0.0007);
    type RuntimeEvent = RuntimeEvent;
    type PairSwapAction = pool_xyk::PairSwapAction<DEXId, AssetId, AccountId, TechAccountId>;
    type DepositLiquidityAction =
        pool_xyk::DepositLiquidityAction<AssetId, AccountId, TechAccountId>;
    type WithdrawLiquidityAction =
        pool_xyk::WithdrawLiquidityAction<AssetId, AccountId, TechAccountId>;
    type PolySwapAction = pool_xyk::PolySwapAction<DEXId, AssetId, AccountId, TechAccountId>;
    type EnsureDEXManager = dex_manager::Pallet<Runtime>;
    type TradingPairSourceManager = ();
    type DexInfoProvider = dex_manager::Pallet<Runtime>;
    type EnsureTradingPairExists = ();
    type EnabledSourcesManager = ();
    type GetFee = GetXykFee;
    type OnPoolCreated = PswapDistribution;
    type OnPoolReservesChanged = ();
    type XSTMarketInfo = ();
    type GetTradingPairRestrictedFlag = GetTradingPairRestrictedFlag;
    type GetChameleonPool = common::mock::GetChameleonPool;
    type GetChameleonPoolBaseAssetId = common::mock::GetChameleonPoolBaseAssetId;
    type AssetInfoProvider = assets::Pallet<Runtime>;
    type AssetRegulator = ();
    type IrreducibleReserve = GetXykIrreducibleReservePercent;
    type WeightInfo = ();
}

impl ceres_liquidity_locker::Config for Runtime {
    const BLOCKS_PER_ONE_DAY: BlockNumberFor<Self> = 14_440;
    type RuntimeEvent = RuntimeEvent;
    type XYKPool = PoolXYK;
    type DemeterFarmingPlatform = DemeterFarmingPlatform;
    type CeresAssetId = ();
    type WeightInfo = ();
}

impl pswap_distribution::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    const PSWAP_BURN_PERCENT: Percent = Percent::from_percent(3);
    type GetIncentiveAssetId = GetIncentiveAssetId;
    type GetTBCDAssetId = GetBuyBackAssetId;
    type LiquidityProxy = ();
    type CompatBalance = Balance;
    type GetDefaultSubscriptionFrequency = GetDefaultSubscriptionFrequency;
    type GetBurnUpdateFrequency = GetBurnUpdateFrequency;
    type GetTechnicalAccountId = GetPswapDistributionAccountId;
    type EnsureDEXManager = ();
    type OnPswapBurnedAggregator = ();
    type WeightInfo = ();
    type GetParliamentAccountId = GetParliamentAccountId;
    type PoolXykPallet = PoolXYK;
    type BuyBackHandler = ();
    type DexInfoProvider = dex_manager::Pallet<Runtime>;
    type GetChameleonPoolBaseAssetId = common::mock::GetChameleonPoolBaseAssetId;
    type AssetInfoProvider = assets::Pallet<Runtime>;
}
pub struct ExtBuilder {
    endowed_assets: Vec<(
        AssetId,
        AccountId,
        AssetSymbol,
        AssetName,
        BalancePrecision,
        Balance,
        bool,
        Option<ContentSource>,
        Option<Description>,
    )>,
    endowed_accounts: Vec<(AccountId, AssetId, Balance)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            endowed_assets: vec![(
                HERMES_ASSET_ID,
                ALICE,
                AssetSymbol(b"HMX".to_vec()),
                AssetName(b"Hermes".to_vec()),
                18,
                Balance::zero(),
                true,
                None,
                None,
            )],
            endowed_accounts: vec![
                (ALICE, HERMES_ASSET_ID, balance!(300000)),
                (BOB, HERMES_ASSET_ID, balance!(500)),
                (CHARLES, HERMES_ASSET_ID, balance!(300000)),
            ],
        }
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = SystemConfig::default().build_storage().unwrap();

        pallet_balances::GenesisConfig::<Runtime> {
            balances: self
                .endowed_accounts
                .iter()
                .map(|(acc, _, balance)| (acc.clone(), *balance))
                .collect(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        PermissionsConfig {
            initial_permission_owners: vec![],
            initial_permissions: vec![],
        }
        .assimilate_storage(&mut t)
        .unwrap();

        assets::GenesisConfig::<Runtime> {
            endowed_assets: self.endowed_assets,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        TokensConfig {
            balances: self.endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
    }
}
