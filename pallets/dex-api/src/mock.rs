use crate::{Module, Trait};
use common::{
    balance, fixed, fixed_from_basis_points, hash, prelude::Balance, Amount, AssetId32, DEXInfo,
    Fixed, LiquiditySourceType, DOT, KSM, XOR,
};
use currencies::BasicCurrencyAdapter;
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use frame_system as system;
use hex_literal::hex;
use permissions::{Scope, INIT_DEX, MANAGE_DEX};
use sp_core::crypto::AccountId32;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

pub type AccountId = AccountId32;
pub type BlockNumber = u64;

type TechAssetId = common::TechAssetId<common::AssetId>;
type TechAccountId = common::TechAccountId<AccountId, TechAssetId, DEXId>;
type AssetId = AssetId32<common::AssetId>;

pub fn alice() -> AccountId {
    AccountId32::from(hex!(
        "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
    ))
}

pub fn bob() -> AccountId {
    AccountId32::from([2u8; 32])
}

pub const DEX_A_ID: DEXId = 1;
pub const DEX_B_ID: DEXId = 2;

impl_outer_origin! {
    pub enum Origin for Runtime {}
}

#[derive(Clone, Eq, PartialEq)]
pub struct Runtime;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for Runtime {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type PalletInfo = ();
}

impl Trait for Runtime {
    type Event = ();
    type MockLiquiditySource =
        mock_liquidity_source::Module<Runtime, mock_liquidity_source::Instance1>;
    type MockLiquiditySource2 =
        mock_liquidity_source::Module<Runtime, mock_liquidity_source::Instance2>;
    type MockLiquiditySource3 =
        mock_liquidity_source::Module<Runtime, mock_liquidity_source::Instance3>;
    type MockLiquiditySource4 =
        mock_liquidity_source::Module<Runtime, mock_liquidity_source::Instance4>;
    type BondingCurvePool = ();
    type MulticollateralBondingCurvePool = ();
    type XYKPool = pool_xyk::Module<Runtime>;
    type WeightInfo = ();
}

impl tokens::Trait for Runtime {
    type Event = ();
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = <Runtime as assets::Trait>::AssetId;
    type OnReceived = ();
    type WeightInfo = ();
}

impl permissions::Trait for Runtime {
    type Event = ();
}

parameter_types! {
    pub const GetBaseAssetId: AssetId = XOR;
}

impl currencies::Trait for Runtime {
    type Event = ();
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = <Runtime as assets::Trait>::GetBaseAssetId;
    type WeightInfo = ();
}

type DEXId = u32;

impl assets::Trait for Runtime {
    type Event = ();
    type ExtraDEXId = DEXId;
    type ExtraLstId = common::LiquiditySourceType;
    type ExtraAccountId = [u8; 32];
    type ExtraTupleArg = common::AssetIdExtraTupleArg<DEXId, common::LiquiditySourceType, [u8; 32]>;
    type AssetId = AssetId;
    type GetBaseAssetId = GetBaseAssetId;
    type Currency = currencies::Module<Runtime>;
    type WeightInfo = ();
}

impl common::Trait for Runtime {
    type DEXId = DEXId;
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
    pub const TransferFee: u128 = 0;
    pub const CreationFee: u128 = 0;
    pub const TransactionByteFee: u128 = 1;
}

impl pallet_balances::Trait for Runtime {
    type Balance = Balance;
    type Event = ();
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
}

parameter_types! {
    pub GetFee: Fixed = fixed_from_basis_points(30u16);
}

impl mock_liquidity_source::Trait<mock_liquidity_source::Instance1> for Runtime {
    type Event = ();
    type GetFee = GetFee;
    type EnsureDEXManager = dex_manager::Module<Runtime>;
    type EnsureTradingPairExists = trading_pair::Module<Runtime>;
}

impl mock_liquidity_source::Trait<mock_liquidity_source::Instance2> for Runtime {
    type Event = ();
    type GetFee = GetFee;
    type EnsureDEXManager = dex_manager::Module<Runtime>;
    type EnsureTradingPairExists = trading_pair::Module<Runtime>;
}

impl mock_liquidity_source::Trait<mock_liquidity_source::Instance3> for Runtime {
    type Event = ();
    type GetFee = GetFee;
    type EnsureDEXManager = dex_manager::Module<Runtime>;
    type EnsureTradingPairExists = trading_pair::Module<Runtime>;
}

impl mock_liquidity_source::Trait<mock_liquidity_source::Instance4> for Runtime {
    type Event = ();
    type GetFee = GetFee;
    type EnsureDEXManager = dex_manager::Module<Runtime>;
    type EnsureTradingPairExists = trading_pair::Module<Runtime>;
}

impl technical::Trait for Runtime {
    type Event = ();
    type TechAssetId = TechAssetId;
    type TechAccountId = TechAccountId;
    type Trigger = ();
    type Condition = ();
    type SwapAction =
        pool_xyk::PolySwapAction<AssetId, TechAssetId, Balance, AccountId, TechAccountId>;
    type WeightInfo = ();
}

impl dex_manager::Trait for Runtime {
    type Event = ();
    type WeightInfo = ();
}

impl trading_pair::Trait for Runtime {
    type Event = ();
    type EnsureDEXManager = dex_manager::Module<Runtime>;
    type WeightInfo = ();
}

impl pool_xyk::Trait for Runtime {
    type Event = ();
    type PairSwapAction = pool_xyk::PairSwapAction<AssetId, Balance, AccountId, TechAccountId>;
    type DepositLiquidityAction =
        pool_xyk::DepositLiquidityAction<AssetId, TechAssetId, Balance, AccountId, TechAccountId>;
    type WithdrawLiquidityAction =
        pool_xyk::WithdrawLiquidityAction<AssetId, TechAssetId, Balance, AccountId, TechAccountId>;
    type PolySwapAction =
        pool_xyk::PolySwapAction<AssetId, TechAssetId, Balance, AccountId, TechAccountId>;
    type EnsureDEXManager = dex_manager::Module<Runtime>;
    type WeightInfo = ();
}

parameter_types! {
    pub GetPswapDistributionAccountId: AccountId = AccountId32::from([3; 32]);
    pub const GetDefaultSubscriptionFrequency: BlockNumber = 10;
    pub GetIncentiveAssetId: AssetId = common::PSWAP.into();
}

impl pswap_distribution::Trait for Runtime {
    type Event = ();
    type GetIncentiveAssetId = GetIncentiveAssetId;
    type LiquidityProxy = ();
    type CompatBalance = Balance;
    type GetDefaultSubscriptionFrequency = GetDefaultSubscriptionFrequency;
    type GetTechnicalAccountId = GetPswapDistributionAccountId;
    type EnsureDEXManager = ();
}

pub type System = frame_system::Module<Runtime>;
pub type Balances = pallet_balances::Module<Runtime>;
pub type Tokens = tokens::Module<Runtime>;
pub type DEXAPI = Module<Runtime>;

type ReservesInit = Vec<(DEXId, AssetId, (Fixed, Fixed))>;

pub struct ExtBuilder {
    endowed_accounts: Vec<(AccountId, AssetId, Balance)>,
    reserves: ReservesInit,
    reserves_2: ReservesInit,
    reserves_3: ReservesInit,
    reserves_4: ReservesInit,
    dex_list: Vec<(DEXId, DEXInfo<AssetId>)>,
    initial_permission_owners: Vec<(u32, Scope, Vec<AccountId>)>,
    initial_permissions: Vec<(AccountId, Scope, Vec<u32>)>,
    source_types: Vec<LiquiditySourceType>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            endowed_accounts: vec![
                (alice(), XOR, balance!(1000000000000000000)),
                (bob(), DOT, balance!(1000000000000000000)),
            ],
            reserves: vec![
                (DEX_A_ID, DOT, (fixed!(5000), fixed!(7000))),
                (DEX_A_ID, KSM, (fixed!(5500), fixed!(4000))),
                (DEX_B_ID, DOT, (fixed!(100), fixed!(45))),
            ],
            reserves_2: vec![
                (DEX_A_ID, DOT, (fixed!(6000), fixed!(7000))),
                (DEX_A_ID, KSM, (fixed!(6500), fixed!(3000))),
                (DEX_B_ID, DOT, (fixed!(200), fixed!(45))),
            ],
            reserves_3: vec![
                (DEX_A_ID, DOT, (fixed!(7000), fixed!(7000))),
                (DEX_A_ID, KSM, (fixed!(7500), fixed!(2000))),
                (DEX_B_ID, DOT, (fixed!(300), fixed!(45))),
            ],
            reserves_4: vec![
                (DEX_A_ID, DOT, (fixed!(8000), fixed!(7000))),
                (DEX_A_ID, KSM, (fixed!(8500), fixed!(1000))),
                (DEX_B_ID, DOT, (fixed!(400), fixed!(45))),
            ],
            dex_list: vec![
                (
                    DEX_A_ID,
                    DEXInfo {
                        base_asset_id: GetBaseAssetId::get(),
                        is_public: true,
                    },
                ),
                (
                    DEX_B_ID,
                    DEXInfo {
                        base_asset_id: GetBaseAssetId::get(),
                        is_public: true,
                    },
                ),
            ],
            initial_permission_owners: vec![
                (INIT_DEX, Scope::Unlimited, vec![alice()]),
                (MANAGE_DEX, Scope::Limited(hash(&DEX_A_ID)), vec![alice()]),
                (MANAGE_DEX, Scope::Limited(hash(&DEX_B_ID)), vec![alice()]),
            ],
            initial_permissions: vec![
                (alice(), Scope::Unlimited, vec![INIT_DEX]),
                (alice(), Scope::Limited(hash(&DEX_A_ID)), vec![MANAGE_DEX]),
                (alice(), Scope::Limited(hash(&DEX_B_ID)), vec![MANAGE_DEX]),
            ],
            source_types: vec![
                LiquiditySourceType::MockPool,
                LiquiditySourceType::MockPool2,
                LiquiditySourceType::MockPool3,
                LiquiditySourceType::MockPool4,
            ],
        }
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        tokens::GenesisConfig::<Runtime> {
            endowed_accounts: self.endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        dex_manager::GenesisConfig::<Runtime> {
            dex_list: self.dex_list,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        permissions::GenesisConfig::<Runtime> {
            initial_permission_owners: self.initial_permission_owners,
            initial_permissions: self.initial_permissions,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        mock_liquidity_source::GenesisConfig::<Runtime, mock_liquidity_source::Instance1> {
            reserves: self.reserves,
            phantom: Default::default(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        mock_liquidity_source::GenesisConfig::<Runtime, mock_liquidity_source::Instance2> {
            reserves: self.reserves_2,
            phantom: Default::default(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        mock_liquidity_source::GenesisConfig::<Runtime, mock_liquidity_source::Instance3> {
            reserves: self.reserves_3,
            phantom: Default::default(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        mock_liquidity_source::GenesisConfig::<Runtime, mock_liquidity_source::Instance4> {
            reserves: self.reserves_4,
            phantom: Default::default(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        crate::GenesisConfig {
            source_types: self.source_types,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }
}
