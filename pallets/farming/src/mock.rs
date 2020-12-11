use crate::{Module, Trait};
use common::prelude::Balance;
use common::{self, Amount, AssetId32, XOR};
use currencies::BasicCurrencyAdapter;
use frame_support::{impl_outer_event, impl_outer_origin, parameter_types, weights::Weight};
use frame_system as system;
use permissions::{Scope, CLAIM_FROM_FARM, CREATE_FARM, INVEST_TO_FARM, TRANSFER};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

pub type AccountId = u128;
pub type BlockNumber = u64;
pub type TechAccountId = common::TechAccountId<AccountId, TechAssetId, DEXId>;
type TechAssetId = common::TechAssetId<common::AssetId, DEXId>;
type DEXId = common::DEXId;
pub type FarmsModule = Module<Test>;
pub type System = frame_system::Module<Test>;
pub type Balances = pallet_balances::Module<Test>;
pub type Tokens = tokens::Module<Test>;
pub type Assets = assets::Module<Test>;
type AssetId = AssetId32<common::AssetId>;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const NICK: AccountId = 3;

impl_outer_origin! {
    pub enum Origin for Test {}
}

impl_outer_event! {
    pub enum Event for Test {
        frame_system<T>,
        pallet_balances<T>,
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Test;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    pub const GetBaseAssetId: AssetId = XOR;
    pub const ExistentialDeposit: u128 = 0;
    pub const MinimumPeriod: u64 = 5;
}

impl system::Trait for Test {
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

impl common::Trait for Test {
    type DEXId = DEXId;
}

impl technical::Trait for Test {
    type Event = ();
    type TechAssetId = TechAssetId;
    type TechAccountId = TechAccountId;
    type Trigger = ();
    type Condition = ();
    type SwapAction = ();
    type WeightInfo = ();
}

impl currencies::Trait for Test {
    type Event = ();
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Test, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = <Test as assets::Trait>::GetBaseAssetId;
    type WeightInfo = ();
}

impl assets::Trait for Test {
    type Event = ();
    type AssetId = AssetId;
    type GetBaseAssetId = GetBaseAssetId;
    type Currency = currencies::Module<Test>;
    type WeightInfo = ();
}

impl permissions::Trait for Test {
    type Event = ();
}

impl pallet_balances::Trait for Test {
    type Balance = Balance;
    type Event = ();
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
}

impl tokens::Trait for Test {
    type Event = ();
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = <Test as assets::Trait>::AssetId;
    type OnReceived = ();
    type WeightInfo = ();
}

impl pallet_timestamp::Trait for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl Trait for Test {
    type Event = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    ExtBuilder::default().build()
}

pub struct ExtBuilder {
    initial_permission_owners: Vec<(u32, Scope, Vec<AccountId>)>,
    initial_permissions: Vec<(AccountId, Scope, Vec<u32>)>,
    endowed_accounts: Vec<(AccountId, AssetId, Balance)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            initial_permission_owners: vec![
                (CREATE_FARM, Scope::Unlimited, vec![ALICE]),
                (TRANSFER, Scope::Unlimited, vec![BOB]),
                (INVEST_TO_FARM, Scope::Unlimited, vec![BOB]),
                (CLAIM_FROM_FARM, Scope::Unlimited, vec![BOB]),
            ],
            initial_permissions: vec![
                (ALICE, Scope::Unlimited, vec![CREATE_FARM]),
                (BOB, Scope::Unlimited, vec![INVEST_TO_FARM, CLAIM_FROM_FARM]),
                (NICK, Scope::Unlimited, vec![INVEST_TO_FARM]),
            ],
            endowed_accounts: vec![
                (ALICE, XOR, 1_000_000_u128.into()),
                (BOB, XOR, 1_000_000_u128.into()),
            ],
        }
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        permissions::GenesisConfig::<Test> {
            initial_permission_owners: self.initial_permission_owners,
            initial_permissions: self.initial_permissions,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        tokens::GenesisConfig::<Test> {
            endowed_accounts: self.endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }
}
