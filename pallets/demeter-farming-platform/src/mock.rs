use common::mock::{ExistentialDeposits, GetTradingPairRestrictedFlag};
use common::prelude::Balance;
use common::{
    balance, fixed, hash, mock_assets_config, mock_common_config, mock_currencies_config,
    mock_frame_system_config, mock_pallet_balances_config, mock_pallet_timestamp_config,
    mock_permissions_config, mock_technical_config, mock_tokens_config, mock_vested_rewards_config,
    DEXId, DEXInfo, Fixed, CERES_ASSET_ID, DEMETER_ASSET_ID, PSWAP, VXOR, XOR, XST, XSTUSD,
};
use currencies::BasicCurrencyAdapter;
use frame_support::traits::{Everything, Hooks};
use frame_support::weights::Weight;
use frame_support::{construct_runtime, parameter_types};
use frame_system;
use frame_system::pallet_prelude::BlockNumberFor;
use permissions::{Scope, MANAGE_DEX};
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};
use sp_runtime::AccountId32;
use sp_runtime::{BuildStorage, Perbill, Percent};

pub type BlockNumber = u64;
pub type AccountId = AccountId32;
pub type Amount = i128;
pub type AssetId = common::AssetId32<common::PredefinedAssetId>;
pub type TechAssetId = common::TechAssetId<common::PredefinedAssetId>;
pub type TechAccountId = common::TechAccountId<AccountId, TechAssetId, DEXId>;
type Block = frame_system::mocking::MockBlock<Runtime>;
type Moment = u64;

construct_runtime! {
    pub enum Runtime {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Assets: assets::{Pallet, Call, Config<T>, Storage, Event<T>},
        Tokens: tokens::{Pallet, Call, Config<T>, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Currencies: currencies::{Pallet, Call, Storage},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
        DexManager: dex_manager::{Pallet, Call, Config<T>, Storage},
        TradingPair: trading_pair::{Pallet, Call, Config<T>, Storage, Event<T>},
        Permissions: permissions::{Pallet, Call, Config<T>, Storage, Event<T>},
        Technical: technical::{Pallet, Call, Config<T>, Storage, Event<T>},
        PoolXYK: pool_xyk::{Pallet, Call, Storage, Event<T>},
        PswapDistribution: pswap_distribution::{Pallet, Call, Config<T>, Storage, Event<T>},
        MBCPool: multicollateral_bonding_curve_pool::{Pallet, Call, Config<T>, Storage, Event<T>},
        VestedRewards: vested_rewards::{Pallet, Call, Storage, Event<T>},
        CeresLiquidityLocker: ceres_liquidity_locker::{Pallet, Call, Storage, Event<T>},
        DemeterFarmingPlatform: demeter_farming_platform::{Pallet, Call, Storage, Event<T>}
    }
}

pub const ALICE: AccountId = AccountId32::new([1u8; 32]);
pub const BOB: AccountId = AccountId32::new([2u8; 32]);
pub const CHARLES: AccountId = AccountId32::new([3u8; 32]);
pub const DEX_A_ID: DEXId = DEXId::Polkaswap;
pub const DEX_B_ID: DEXId = DEXId::PolkaswapXSTUSD;

mock_technical_config!(Runtime, pool_xyk::PolySwapAction<DEXId, AssetId, AccountId, TechAccountId>);
mock_currencies_config!(Runtime);
mock_pallet_balances_config!(Runtime);
mock_frame_system_config!(Runtime);
mock_common_config!(Runtime);
mock_tokens_config!(Runtime);
mock_assets_config!(Runtime);
mock_pallet_timestamp_config!(Runtime);
mock_permissions_config!(Runtime);
mock_vested_rewards_config!(Runtime);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = Weight::from_parts(1024, 0);
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    pub GetXykFee: Fixed = fixed!(0.003);
    pub GetXykMaxIssuanceRatio: Fixed = fixed!(1.5);
    pub GetIncentiveAssetId: AssetId = PSWAP.into();
    pub const GetDefaultSubscriptionFrequency: BlockNumber = 10;
    pub const GetBurnUpdateFrequency: BlockNumber = 14400;
    pub GetParliamentAccountId: AccountId = AccountId32::new([100u8; 32]);
    pub GetPswapDistributionAccountId: AccountId = AccountId32::new([101u8; 32]);
    pub GetMarketMakerRewardsAccountId: AccountId = AccountId32::new([102u8; 32]);
    pub GetBondingCurveRewardsAccountId: AccountId = AccountId32::new([103u8; 32]);
    pub GetFarmingRewardsAccountId: AccountId = AccountId32::new([104u8; 32]);
    pub GetCrowdloanRewardsAccountId: AccountId = AccountId32::new([105u8; 32]);
}

parameter_types! {
    pub const DemeterAssetId: AssetId = DEMETER_ASSET_ID;
}

impl demeter_farming_platform::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type DemeterAssetId = DemeterAssetId;
    const BLOCKS_PER_HOUR_AND_A_HALF: BlockNumberFor<Self> = 900;
    type WeightInfo = ();
    type AssetInfoProvider = assets::Pallet<Runtime>;
}

parameter_types! {
    pub const GetBaseAssetId: AssetId = XOR;
    pub const GetBuyBackAssetId: AssetId = VXOR;
    pub GetTBCBuyBackTBCDPercent: Fixed = fixed!(0.025);
    pub GetXykIrreducibleReservePercent: Percent = Percent::from_percent(1);
    pub GetTbcIrreducibleReservePercent: Percent = Percent::from_percent(1);
}

impl dex_manager::Config for Runtime {}

impl trading_pair::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type EnsureDEXManager = dex_manager::Pallet<Runtime>;
    type DexInfoProvider = dex_manager::Pallet<Runtime>;
    type WeightInfo = ();
    type AssetInfoProvider = assets::Pallet<Runtime>;
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
    type TradingPairSourceManager = trading_pair::Pallet<Runtime>;
    type DexInfoProvider = dex_manager::Pallet<Runtime>;
    type EnsureTradingPairExists = trading_pair::Pallet<Runtime>;
    type EnabledSourcesManager = trading_pair::Pallet<Runtime>;
    type GetFee = GetXykFee;
    type GetMaxIssuanceRatio = GetXykMaxIssuanceRatio;
    type OnPoolCreated = PswapDistribution;
    type OnPoolReservesChanged = ();
    type XSTMarketInfo = ();
    type GetTradingPairRestrictedFlag = GetTradingPairRestrictedFlag;
    type GetChameleonPools = common::mock::GetChameleonPools;
    type AssetInfoProvider = assets::Pallet<Runtime>;
    type AssetRegulator = ();
    type IrreducibleReserve = GetXykIrreducibleReservePercent;
    type PoolAdjustPeriod = sp_runtime::traits::ConstU64<1>;
    type WeightInfo = ();
}
parameter_types! {
    pub const CeresAssetId: AssetId = CERES_ASSET_ID;
}

parameter_types! {
    pub const TransferFee: u128 = 0;
    pub const CreationFee: u128 = 0;
    pub const TransactionByteFee: u128 = 1;
}

impl ceres_liquidity_locker::Config for Runtime {
    const BLOCKS_PER_ONE_DAY: BlockNumberFor<Self> = 14400;
    type RuntimeEvent = RuntimeEvent;
    type XYKPool = PoolXYK;
    type DemeterFarmingPlatform = DemeterFarmingPlatform;
    type CeresAssetId = CeresAssetId;
    type WeightInfo = ();
}

impl multicollateral_bonding_curve_pool::Config for Runtime {
    const RETRY_DISTRIBUTION_FREQUENCY: BlockNumber = 1000;
    type RuntimeEvent = RuntimeEvent;
    type LiquidityProxy = ();
    type EnsureDEXManager = dex_manager::Pallet<Runtime>;
    type EnsureTradingPairExists = trading_pair::Pallet<Runtime>;
    type PriceToolsPallet = ();
    type VestedRewardsPallet = VestedRewards;
    type TradingPairSourceManager = trading_pair::Pallet<Runtime>;
    type BuyBackHandler = ();
    type BuyBackTBCDPercent = GetTBCBuyBackTBCDPercent;
    type AssetInfoProvider = assets::Pallet<Runtime>;
    type IrreducibleReserve = GetTbcIrreducibleReservePercent;
    type WeightInfo = ();
}

impl pswap_distribution::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    const PSWAP_BURN_PERCENT: Percent = Percent::from_percent(3);
    type GetIncentiveAssetId = GetIncentiveAssetId;
    type GetBuyBackAssetId = GetBuyBackAssetId;
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
    type GetChameleonPools = common::mock::GetChameleonPools;
    type AssetInfoProvider = assets::Pallet<Runtime>;
}

pub struct ExtBuilder {
    initial_dex_list: Vec<(DEXId, DEXInfo<AssetId>)>,
    endowed_accounts: Vec<(AccountId, AssetId, Balance)>,
    initial_permission_owners: Vec<(u32, Scope, Vec<AccountId>)>,
    initial_permissions: Vec<(AccountId, Scope, Vec<u32>)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            initial_dex_list: vec![
                (
                    DEX_A_ID,
                    DEXInfo {
                        base_asset_id: XOR.into(),
                        synthetic_base_asset_id: XST.into(),
                        is_public: true,
                    },
                ),
                (
                    DEX_B_ID,
                    DEXInfo {
                        base_asset_id: XSTUSD.into(),
                        synthetic_base_asset_id: XST.into(),
                        is_public: true,
                    },
                ),
            ],
            endowed_accounts: vec![
                (ALICE, CERES_ASSET_ID.into(), balance!(1000)),
                (BOB, CERES_ASSET_ID.into(), balance!(500)),
            ],
            initial_permission_owners: vec![
                (MANAGE_DEX, Scope::Limited(hash(&DEX_A_ID)), vec![BOB]),
                (MANAGE_DEX, Scope::Limited(hash(&DEX_B_ID)), vec![BOB]),
            ],
            initial_permissions: vec![
                (ALICE, Scope::Limited(hash(&DEX_A_ID)), vec![MANAGE_DEX]),
                (ALICE, Scope::Limited(hash(&DEX_B_ID)), vec![MANAGE_DEX]),
            ],
        }
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = SystemConfig::default().build_storage().unwrap();

        dex_manager::GenesisConfig::<Runtime> {
            dex_list: self.initial_dex_list,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        TokensConfig {
            balances: self.endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        permissions::GenesisConfig::<Runtime> {
            initial_permission_owners: self.initial_permission_owners,
            initial_permissions: self.initial_permissions,
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
        DemeterFarmingPlatform::on_initialize(System::block_number());
    }
}
