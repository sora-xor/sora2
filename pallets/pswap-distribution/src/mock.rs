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

use crate::{self as pswap_distribution, Config};
use common::mock::ExistentialDeposits;
use common::prelude::Balance;
use common::{
    balance, fixed, fixed_from_basis_points, AssetName, AssetSymbol, BalancePrecision, Fixed,
    FromGenericPair,
};
use currencies::BasicCurrencyAdapter;
use frame_support::traits::GenesisBuild;
use frame_support::weights::Weight;
use frame_support::{construct_runtime, parameter_types};
use frame_system;
use hex_literal::hex;
use permissions::Scope;
use sp_core::H256;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup, Zero};
use sp_runtime::{AccountId32, Perbill};

pub type AccountId = AccountId32;
pub type BlockNumber = u64;
pub type Amount = i128;
pub type AssetId = common::AssetId32<common::PredefinedAssetId>;
pub type TechAccountId = common::TechAccountId<AccountId, TechAssetId, DEXId>;
type TechAssetId = common::TechAssetId<common::PredefinedAssetId>;
type DEXId = common::DEXId;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

pub fn alice() -> AccountId {
    AccountId32::from([1u8; 32])
}

pub fn fees_account_a() -> AccountId {
    AccountId32::from([2u8; 32])
}

pub fn fees_account_b() -> AccountId {
    AccountId32::from([3u8; 32])
}

pub fn liquidity_provider_a() -> AccountId {
    AccountId32::from([4u8; 32])
}

pub fn liquidity_provider_b() -> AccountId {
    AccountId32::from([5u8; 32])
}

pub fn liquidity_provider_c() -> AccountId {
    AccountId32::from([6u8; 32])
}

pub const DEX_A_ID: DEXId = common::DEXId::Polkaswap;

parameter_types! {
    pub GetBaseAssetId: AssetId = common::XOR.into();
    pub GetIncentiveAssetId: AssetId = common::PSWAP.into();
    pub const PoolTokenAId: AssetId = common::AssetId32::from_bytes(hex!("0211110000000000000000000000000000000000000000000000000000000000"));
    pub const PoolTokenBId: AssetId = common::AssetId32::from_bytes(hex!("0222220000000000000000000000000000000000000000000000000000000000"));
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    pub const GetDefaultFee: u16 = 30;
    pub const GetDefaultProtocolFee: u16 = 0;
    pub GetPswapDistributionTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            crate::TECH_ACCOUNT_PREFIX.to_vec(),
            crate::TECH_ACCOUNT_MAIN.to_vec(),
        );
        tech_account_id
    };
    pub GetPswapDistributionAccountId: AccountId = {
        let tech_account_id = GetPswapDistributionTechAccountId::get();
        let account_id =
            technical::Module::<Runtime>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub const GetDefaultSubscriptionFrequency: BlockNumber = 10;
    pub const GetBurnUpdateFrequency: BlockNumber = 3;
    pub const ExistentialDeposit: u128 = 0;
    pub const TransferFee: u128 = 0;
    pub const CreationFee: u128 = 0;
    pub const TransactionByteFee: u128 = 1;
    pub GetFee: Fixed = fixed_from_basis_points(30u16);
    pub GetParliamentAccountId: AccountId = AccountId32::from([7u8; 32]);
    pub GetTeamReservesAccountId: AccountId = AccountId32::from([11; 32]);
}

construct_runtime! {
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        PswapDistribution: pswap_distribution::{Module, Call, Config<T>, Storage, Event<T>},
        Tokens: tokens::{Module, Call, Config<T>, Storage, Event<T>},
        Permissions: permissions::{Module, Call, Config<T>, Storage, Event<T>},
        Currencies: currencies::{Module, Call, Storage, Event<T>},
        Assets: assets::{Module, Call, Config<T>, Storage, Event<T>},
        Balances: pallet_balances::{Module, Call, Config<T>, Storage, Event<T>},
        Technical: technical::{Module, Call, Storage, Event<T>},
        DexManager: dex_manager::{Module, Call, Storage},
    }
}

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

impl Config for Runtime {
    type Event = Event;
    type GetIncentiveAssetId = GetIncentiveAssetId;
    type LiquidityProxy = ();
    type CompatBalance = Balance;
    type GetDefaultSubscriptionFrequency = GetDefaultSubscriptionFrequency;
    type GetBurnUpdateFrequency = GetBurnUpdateFrequency;
    type GetTechnicalAccountId = GetPswapDistributionAccountId;
    type EnsureDEXManager = DexManager;
    type OnPswapBurnedAggregator = ();
    type WeightInfo = ();
    type GetParliamentAccountId = GetParliamentAccountId;
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

impl permissions::Config for Runtime {
    type Event = Event;
}

impl currencies::Config for Runtime {
    type Event = Event;
    type MultiCurrency = Tokens;
    type NativeCurrency =
        BasicCurrencyAdapter<Runtime, pallet_balances::Module<Runtime>, Amount, BlockNumber>;
    type GetNativeCurrencyId = <Runtime as assets::Config>::GetBaseAssetId;
    type WeightInfo = ();
}

impl assets::Config for Runtime {
    type Event = Event;
    type ExtraAccountId = [u8; 32];
    type ExtraAssetRecordArg =
        common::AssetIdExtraAssetRecordArg<common::DEXId, common::LiquiditySourceType, [u8; 32]>;
    type AssetId = AssetId;
    type GetBaseAssetId = GetBaseAssetId;
    type Currency = currencies::Module<Runtime>;
    type GetTeamReservesAccountId = GetTeamReservesAccountId;
    type WeightInfo = ();
}

impl common::Config for Runtime {
    type DEXId = DEXId;
    type LstId = common::LiquiditySourceType;
}

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
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

impl dex_manager::Config for Runtime {}

pub struct ExtBuilder {
    endowed_accounts: Vec<(AccountId, AssetId, Balance)>,
    endowed_assets: Vec<(
        AssetId,
        AccountId,
        AssetSymbol,
        AssetName,
        BalancePrecision,
        Balance,
        bool,
    )>,
    initial_permission_owners: Vec<(u32, Scope, Vec<AccountId>)>,
    initial_permissions: Vec<(AccountId, Scope, Vec<u32>)>,
    subscribed_accounts: Vec<(AccountId, (DEXId, AssetId, BlockNumber, BlockNumber))>,
    burn_info: (Fixed, Fixed, Fixed),
}

impl ExtBuilder {
    pub fn uninitialized() -> Self {
        Self {
            endowed_accounts: Vec::new(),
            endowed_assets: vec![(
                PoolTokenAId::get(),
                alice(),
                AssetSymbol(b"POOL".to_vec()),
                AssetName(b"Pool Token".to_vec()),
                18,
                Balance::from(0u32),
                true,
            )],
            initial_permission_owners: Vec::new(),
            initial_permissions: Vec::new(),
            subscribed_accounts: Vec::new(),
            burn_info: (fixed!(0), fixed!(0.10), fixed!(0.30)),
        }
    }
}

impl ExtBuilder {
    pub fn with_accounts(accounts: Vec<(AccountId, AssetId, Balance)>) -> Self {
        let permissioned_account_id = GetPswapDistributionAccountId::get();
        Self {
            endowed_accounts: accounts,
            endowed_assets: vec![
                (
                    common::XOR.into(),
                    alice(),
                    AssetSymbol(b"XOR".to_vec()),
                    AssetName(b"SORA".to_vec()),
                    18,
                    Balance::zero(),
                    true,
                ),
                (
                    common::PSWAP.into(),
                    alice(),
                    AssetSymbol(b"PSWAP".to_vec()),
                    AssetName(b"Polkaswap".to_vec()),
                    10,
                    Balance::zero(),
                    true,
                ),
                (
                    PoolTokenAId::get(),
                    alice(),
                    AssetSymbol(b"POOLA".to_vec()),
                    AssetName(b"Pool A".to_vec()),
                    18,
                    Balance::zero(),
                    true,
                ),
                (
                    PoolTokenBId::get(),
                    alice(),
                    AssetSymbol(b"POOLB".to_vec()),
                    AssetName(b"Pool B".to_vec()),
                    18,
                    Balance::zero(),
                    true,
                ),
            ],
            initial_permission_owners: vec![],
            initial_permissions: vec![(
                permissioned_account_id,
                Scope::Unlimited,
                vec![permissions::MINT, permissions::BURN],
            )],
            subscribed_accounts: vec![
                (fees_account_a(), (DEX_A_ID, PoolTokenAId::get(), 5, 0)),
                (fees_account_b(), (DEX_A_ID, PoolTokenBId::get(), 7, 0)),
            ],
            burn_info: (fixed!(0.1), fixed!(0.10), fixed!(0.40)),
        }
    }
}

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder::with_accounts(vec![
            (fees_account_a(), common::XOR.into(), balance!(1)),
            (fees_account_a(), common::PSWAP.into(), balance!(6)),
            (liquidity_provider_a(), PoolTokenAId::get(), balance!(3)),
            (liquidity_provider_b(), PoolTokenAId::get(), balance!(2)),
            (liquidity_provider_c(), PoolTokenAId::get(), balance!(1)),
            (liquidity_provider_a(), PoolTokenBId::get(), balance!(10)),
            (liquidity_provider_b(), PoolTokenBId::get(), balance!(10)),
            (liquidity_provider_c(), PoolTokenBId::get(), balance!(10)),
        ])
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = SystemConfig::default().build_storage::<Runtime>().unwrap();

        let mut vec = self
            .endowed_accounts
            .iter()
            .map(|(acc, ..)| (acc.clone(), 0))
            .chain(vec![
                (alice(), 0),
                (fees_account_a(), 0),
                (fees_account_b(), 0),
                (GetPswapDistributionAccountId::get(), 0),
                (GetParliamentAccountId::get(), 0),
            ])
            .collect::<Vec<_>>();

        vec.sort_by_key(|x| x.0.clone());
        vec.dedup_by(|x, y| x.0 == y.0);
        BalancesConfig { balances: vec }
            .assimilate_storage(&mut t)
            .unwrap();

        PermissionsConfig {
            initial_permissions: self.initial_permissions,
            initial_permission_owners: self.initial_permission_owners,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        TokensConfig {
            endowed_accounts: self.endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        AssetsConfig {
            endowed_assets: self.endowed_assets,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        PswapDistributionConfig {
            subscribed_accounts: self.subscribed_accounts,
            burn_info: self.burn_info,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }
}
