use crate as iroha_migration; // for construct_runtime
use crate::{Config, TECH_ACCOUNT_MAIN, TECH_ACCOUNT_PREFIX};
use common::mock::ExistentialDeposits;
use common::prelude::Balance;
use common::{Amount, AssetId, AssetId32, AssetName, AssetSymbol, VAL};
use currencies::BasicCurrencyAdapter;
use frame_support::traits::GenesisBuild;
use frame_support::weights::Weight;
use frame_support::{construct_runtime, parameter_types};
use permissions::{Scope, MINT};
use sp_core::H256;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};
use sp_runtime::{self, Perbill};

type DEXId = common::DEXId;
type AccountId = u64;
type BlockNumber = u64;
type TechAccountId = common::TechAccountId<AccountId, TechAssetId, DEXId>;
type TechAssetId = common::TechAssetId<common::AssetId>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

pub const XOR: AssetId = AssetId::XOR;
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const MINTING_ACCOUNT: AccountId = 4;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    pub const GetBaseAssetId: AssetId32<AssetId> = AssetId32::from_asset_id(XOR);
    pub const ExistentialDeposit: u128 = 0;
    pub const DepositBase: u64 = 1;
    pub const DepositFactor: u64 = 1;
    pub const MaxSignatories: u16 = 4;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        Multisig: pallet_multisig::{Module, Call, Storage, Event<T>},
        Tokens: tokens::{Module, Call, Storage, Config<T>, Event<T>},
        Currencies: currencies::{Module, Call, Storage,  Event<T>},
        Assets: assets::{Module, Call, Storage, Config<T>, Event<T>},
        Technical: technical::{Module, Call, Config<T>, Event<T>},
        Permissions: permissions::{Module, Call, Storage, Config<T>, Event<T>},
        ReferralSystem: referral_system::{Module, Call, Storage, Config<T>},
        IrohaMigration: iroha_migration::{Module, Call, Storage, Config<T>, Event<T>}
    }
);

impl frame_system::Config for Runtime {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type PalletInfo = PalletInfo;
    type SS58Prefix = ();
}

impl technical::Config for Runtime {
    type Event = Event;
    type TechAssetId = TechAssetId;
    type TechAccountId = TechAccountId;
    type Trigger = ();
    type Condition = ();
    type SwapAction = ();
    type WeightInfo = ();
}

impl assets::Config for Runtime {
    type Event = Event;
    type ExtraAccountId = u64;
    type ExtraAssetRecordArg =
        common::AssetIdExtraAssetRecordArg<DEXId, common::LiquiditySourceType, u64>;
    type AssetId = common::AssetId32<AssetId>;
    type GetBaseAssetId = GetBaseAssetId;
    type Currency = currencies::Module<Runtime>;
    type WeightInfo = ();
}

impl common::Config for Runtime {
    type DEXId = DEXId;
    type LstId = common::LiquiditySourceType;
}

impl permissions::Config for Runtime {
    type Event = Event;
}

// Required by assets::Config
impl currencies::Config for Runtime {
    type Event = Event;
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = <Runtime as assets::Config>::GetBaseAssetId;
    type WeightInfo = ();
}

// Required by currencies::Config
impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
}

impl tokens::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = <Runtime as assets::Config>::AssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
}

impl referral_system::Config for Runtime {}

impl pallet_multisig::Config for Runtime {
    type Call = Call;
    type Event = Event;
    type Currency = Balances;
    type DepositBase = DepositBase;
    type DepositFactor = DepositFactor;
    type MaxSignatories = MaxSignatories;
    type WeightInfo = ();
}

impl Config for Runtime {
    type Event = Event;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let tech_account_id =
        TechAccountId::Generic(TECH_ACCOUNT_PREFIX.to_vec(), TECH_ACCOUNT_MAIN.to_vec());

    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();

    permissions::GenesisConfig::<Runtime> {
        initial_permission_owners: vec![(MINT, Scope::Unlimited, vec![MINTING_ACCOUNT])],
        initial_permissions: vec![(MINTING_ACCOUNT, Scope::Unlimited, vec![MINT])],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    assets::GenesisConfig::<Runtime> {
        endowed_assets: vec![(
            VAL,
            ALICE,
            AssetSymbol(b"VAL".to_vec()),
            AssetName(b"SORA Validator Token".to_vec()),
            18,
            Balance::from(0u32),
            true,
        )],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    tokens::GenesisConfig::<Runtime> {
        endowed_accounts: vec![(ALICE, VAL, 0u128.into())],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    technical::GenesisConfig::<Runtime> {
        account_ids_to_tech_account_ids: vec![(MINTING_ACCOUNT, tech_account_id.clone())],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    IrohaMigrationConfig {
        iroha_accounts: vec![
            (
                "did_sora_d9bda3688c6f608ab15c@sora".to_string(),
                Balance::from(0u128),
                None,
                1,
                vec![
                    "D9BDA3688c6f608ab15c03a55b171da0413788a40a25722b4ae4d3672890bcd7".to_string(),
                ],
            ),
            (
                "did_sora_balance@sora".to_string(),
                Balance::from(300u128),
                None,
                1,
                vec![
                    "9A685d77BCd3f60e6cc1e91eedc7a48e11bbcf1a036b920f3bae0372a78a5432".to_string(),
                ],
            ),
            (
                "did_sora_referral@sora".to_string(),
                Balance::from(0u128),
                Some("did_sora_referrer@sora".to_string()),
                1,
                vec![
                    "cba1c8c2eeaf287d734bd167b10d762e89c0ee8327a29e04f064ae94086ef1e9".to_string(),
                ],
            ),
            (
                "did_sora_referrer@sora".to_string(),
                Balance::from(0u128),
                None,
                1,
                vec![
                    "dd54e9efb95531154316cf3e28e2232abab349296dde94353febc9ebbb3ff283".to_string(),
                ],
            ),
            (
                "did_sora_multi_sig@sora".to_string(),
                Balance::from(1000u128),
                None,
                2,
                vec![
                    "f7d89d39d48a67e4741a612de10650234f9148e84fe9e8b2a9fad322b0d8e5bc".to_string(),
                    "f56b4880ed91a25b257144acab749f615855c4b1b6a5d7891e1a6cdd9fd695e9".to_string(),
                    "57571ec82cff710143eba60c05d88de14a22799048137162d63c534a8b02dc20".to_string(),
                ],
            ),
        ],
        account_id: MINTING_ACCOUNT,
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}
