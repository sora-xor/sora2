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

use crate as iroha_migration; // for construct_runtime
use crate::{Config, TECH_ACCOUNT_MAIN, TECH_ACCOUNT_PREFIX};
use common::mock::ExistentialDeposits;
use common::prelude::Balance;
use common::{
    balance, Amount, AssetId32, AssetName, AssetSymbol, PredefinedAssetId,
    DEFAULT_BALANCE_PRECISION, PSWAP, VAL, XST,
};
use currencies::BasicCurrencyAdapter;
use frame_support::traits::{Everything, GenesisBuild};
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
type TechAssetId = common::TechAssetId<common::PredefinedAssetId>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

pub const XOR: PredefinedAssetId = PredefinedAssetId::XOR;
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const MINTING_ACCOUNT: AccountId = 4;
pub const REFERRALS_RESERVES_ACC: AccountId = 22;
pub const BUY_BACK_ACCOUNT: AccountId = 23;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = Weight::from_ref_time(1024);
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    pub const GetBaseAssetId: AssetId32<PredefinedAssetId> = AssetId32::from_asset_id(XOR);
    pub const ExistentialDeposit: u128 = 0;
    pub const DepositBase: u64 = 1;
    pub const DepositFactor: u64 = 1;
    pub const MaxSignatories: u16 = 4;
    pub const ReferralsReservesAcc: AccountId = REFERRALS_RESERVES_ACC;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Multisig: pallet_multisig::{Pallet, Call, Storage, Event<T>},
        Tokens: tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
        Currencies: currencies::{Pallet, Call, Storage},
        Assets: assets::{Pallet, Call, Storage, Config<T>, Event<T>},
        Technical: technical::{Pallet, Call, Config<T>, Event<T>},
        Permissions: permissions::{Pallet, Call, Storage, Config<T>, Event<T>},
        Referrals: referrals::{Pallet, Call, Storage, Config<T>},
        IrohaMigration: iroha_migration::{Pallet, Call, Storage, Config<T>, Event<T>}
    }
);

impl frame_system::Config for Runtime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type PalletInfo = PalletInfo;
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<65536>;
}

impl technical::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type TechAssetId = TechAssetId;
    type TechAccountId = TechAccountId;
    type Trigger = ();
    type Condition = ();
    type SwapAction = ();
}

parameter_types! {
    pub const GetBuyBackAssetId: common::AssetId32<PredefinedAssetId> = XST;
    pub GetBuyBackSupplyAssets: Vec<common::AssetId32<PredefinedAssetId>> = vec![VAL, PSWAP];
    pub const GetBuyBackPercentage: u8 = 10;
    pub const GetBuyBackAccountId: AccountId = BUY_BACK_ACCOUNT;
    pub const GetBuyBackDexId: DEXId = DEXId::Polkaswap;
}

impl assets::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ExtraAccountId = u64;
    type ExtraAssetRecordArg =
        common::AssetIdExtraAssetRecordArg<DEXId, common::LiquiditySourceType, u64>;
    type AssetId = common::AssetId32<PredefinedAssetId>;
    type GetBaseAssetId = GetBaseAssetId;
    type GetBuyBackAssetId = GetBuyBackAssetId;
    type GetBuyBackSupplyAssets = GetBuyBackSupplyAssets;
    type GetBuyBackPercentage = GetBuyBackPercentage;
    type GetBuyBackAccountId = GetBuyBackAccountId;
    type GetBuyBackDexId = GetBuyBackDexId;
    type BuyBackLiquidityProxy = ();
    type Currency = currencies::Pallet<Runtime>;
    type GetTotalBalance = ();
    type WeightInfo = ();
}

impl common::Config for Runtime {
    type DEXId = DEXId;
    type LstId = common::LiquiditySourceType;
}

impl permissions::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

// Required by assets::Config
impl currencies::Config for Runtime {
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = <Runtime as assets::Config>::GetBaseAssetId;
    type WeightInfo = ();
}

// Required by currencies::Config
impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
}

impl tokens::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = <Runtime as assets::Config>::AssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type CurrencyHooks = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
    type DustRemovalWhitelist = Everything;
}

impl referrals::Config for Runtime {
    type ReservesAcc = ReferralsReservesAcc;
    type WeightInfo = ();
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

impl Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

pub fn test_ext(add_iroha_accounts: bool) -> sp_io::TestExternalities {
    let tech_account_id =
        TechAccountId::Generic(TECH_ACCOUNT_PREFIX.to_vec(), TECH_ACCOUNT_MAIN.to_vec());

    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();

    pallet_balances::GenesisConfig::<Runtime> {
        balances: vec![
            (ALICE, 0u128.into()),
            (BOB, 0u128.into()),
            (MINTING_ACCOUNT, 0u128.into()),
        ],
    }
    .assimilate_storage(&mut t)
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
            DEFAULT_BALANCE_PRECISION,
            Balance::from(0u32),
            true,
            None,
            None,
        )],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let eth_bridge_tech_account_id = TechAccountId::Generic(
        eth_bridge::TECH_ACCOUNT_PREFIX.to_vec(),
        eth_bridge::TECH_ACCOUNT_MAIN.to_vec(),
    );
    let eth_bridge_account_id =
        Technical::tech_account_id_to_account_id(&eth_bridge_tech_account_id).unwrap();

    tokens::GenesisConfig::<Runtime> {
        balances: vec![
            (ALICE, VAL, 0u128.into()),
            (eth_bridge_account_id, VAL, balance!(1000)),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    technical::GenesisConfig::<Runtime> {
        register_tech_accounts: vec![
            (MINTING_ACCOUNT, tech_account_id.clone()),
            (eth_bridge_account_id, eth_bridge_tech_account_id),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let iroha_accounts = if add_iroha_accounts {
        vec![
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
        ]
    } else {
        Vec::new()
    };

    IrohaMigrationConfig {
        iroha_accounts,
        account_id: Some(MINTING_ACCOUNT),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    test_ext(true)
}
