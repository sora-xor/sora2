use super::*;
use currencies::BasicCurrencyAdapter;

use frame_support::dispatch::DispatchError;
use frame_support::traits::{Currency, Everything, GenesisBuild};
use frame_support::{assert_noop, assert_ok, parameter_types};
use sp_core::{H160, H256};
use sp_keyring::AccountKeyring as Keyring;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, Convert, IdentifyAccount, IdentityLookup, Verify};
use sp_runtime::{MultiSignature, Perbill};
use sp_std::convert::From;
use sp_std::marker::PhantomData;

use snowbridge_core::{Message, MessageDispatch, Proof};
use snowbridge_ethereum::{EthNetworkId, Log, U256};

use common::mock::ExistentialDeposits;
use common::{balance, Amount, AssetId32, AssetName, AssetSymbol, DEXId, XOR};
use hex_literal::hex;

use crate::inbound::Error;

use crate::inbound as incentivized_inbound_channel;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

const BASE_NETWORK_ID: EthNetworkId = 12123;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
        Assets: assets::{Pallet, Call, Storage, Event<T>},
        Tokens: tokens::{Pallet, Call, Config<T>, Storage, Event<T>},
        Currencies: currencies::{Pallet, Call, Storage, Event<T>},
        Permissions: permissions::{Pallet, Call, Config<T>, Storage, Event<T>},
        IncentivizedInboundChannel: incentivized_inbound_channel::{Pallet, Call, Storage, Event<T>},
    }
);

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Balance = u128;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
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
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    /// The ubiquitous event type.
    type Event = Event;
    type MaxLocks = MaxLocks;
    /// The type for recording an account's balance.
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

impl common::Config for Test {
    type DEXId = common::DEXId;
    type LstId = common::LiquiditySourceType;
}

impl permissions::Config for Test {
    type Event = Event;
}

impl tokens::Config for Test {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = <Test as assets::Config>::AssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
    type MaxLocks = ();
    type DustRemovalWhitelist = Everything;
}

impl currencies::Config for Test {
    type Event = Event;
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Test, Balances, Amount, u64>;
    type GetNativeCurrencyId = <Test as assets::Config>::GetBaseAssetId;
    type WeightInfo = ();
}
parameter_types! {
    pub const GetBaseAssetId: AssetId = XOR;
    pub GetTeamReservesAccountId: AccountId = Default::default();
}

type AssetId = AssetId32<common::PredefinedAssetId>;

impl assets::Config for Test {
    type Event = Event;
    type ExtraAccountId = [u8; 32];
    type ExtraAssetRecordArg =
        common::AssetIdExtraAssetRecordArg<DEXId, common::LiquiditySourceType, [u8; 32]>;
    type AssetId = AssetId;
    type GetBaseAssetId = GetBaseAssetId;
    type Currency = currencies::Pallet<Test>;
    type GetTeamReservesAccountId = GetTeamReservesAccountId;
    type WeightInfo = ();
    type GetTotalBalance = ();
}

// Mock verifier
pub struct MockVerifier;

impl Verifier for MockVerifier {
    fn verify(_: EthNetworkId, message: &Message) -> Result<Log, DispatchError> {
        let log: Log = rlp::decode(&message.data).unwrap();
        Ok(log)
    }
}

// Mock Dispatch
pub struct MockMessageDispatch;

impl MessageDispatch<Test, MessageId> for MockMessageDispatch {
    fn dispatch(_: EthNetworkId, _: H160, _: MessageId, _: &[u8]) {}

    #[cfg(feature = "runtime-benchmarks")]
    fn successful_dispatch_event(_: MessageId) -> Option<<Test as frame_system::Config>::Event> {
        None
    }
}

parameter_types! {
    pub SourceAccount: AccountId = Keyring::Eve.into();
    pub TreasuryAccount: AccountId = Keyring::Dave.into();
}

pub struct FeeConverter<T: Config>(PhantomData<T>);

impl<T: Config> Convert<U256, BalanceOf<T>> for FeeConverter<T> {
    fn convert(_: U256) -> BalanceOf<T> {
        100u32.into()
    }
}

impl incentivized_inbound_channel::Config for Test {
    type Event = Event;
    type Verifier = MockVerifier;
    type MessageDispatch = MockMessageDispatch;
    type FeeConverter = FeeConverter<Self>;
    type FeeAssetId = ();
    type UpdateOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type WeightInfo = ();
}

pub fn new_tester(source_channel: H160) -> sp_io::TestExternalities {
    new_tester_with_config(incentivized_inbound_channel::GenesisConfig {
        networks: vec![(
            BASE_NETWORK_ID,
            vec![(source_channel, Default::default(), Default::default())],
        )],
        reward_fraction: Perbill::from_percent(80),
        treasury_account: Default::default(),
    })
}

pub fn new_tester_with_config(
    config: incentivized_inbound_channel::GenesisConfig<Test>,
) -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    GenesisBuild::<Test>::assimilate_storage(&config, &mut storage).unwrap();

    let bob: AccountId = Keyring::Bob.into();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(bob.clone(), balance!(1))],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    assets::GenesisConfig::<Test> {
        endowed_assets: vec![(
            XOR.into(),
            bob,
            AssetSymbol(b"XOR".to_vec()),
            AssetName(b"SORA".to_vec()),
            18,
            0,
            true,
            None,
            None,
        )],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}

// The originating channel address for the messages below
const SOURCE_CHANNEL_ADDR: [u8; 20] = hex!["4130819912a398f4eb84e7f16ed443232ba638b5"];

// Message with nonce = 1
const MESSAGE_DATA_0: [u8; 317] = hex!(
    "
	f9013a944130819912a398f4eb84e7f16ed443232ba638b5e1a05e9ae1d7c484
	f74d554a503aa825e823725531d97e784dd9b1aacdb58d1f7076b90100000000
	000000000000000000c2c5d46481c291be111d5e3a0b52114bdf212a01000000
	0000000000000000000000000000000000000000000000000000000001000000
	0000000000000000000000000000000000000000000de0b6b3a7640000000000
	0000000000000000000000000000000000000000000000000000000080000000
	00000000000000000000000000000000000000000000000000000000570c0182
	13dae5f9c236beab905c8305cb159c5fa1aae500d43593c715fdd31c61141abd
	04a99fd6822c8558854ccde39a5684e7a56da27d0000d9e9ac2d780300000000
	0000000000000000000000000000000000000000000000000000000000
"
);

// Message with nonce = 2
const MESSAGE_DATA_1: [u8; 317] = hex!(
    "
	f9013a944130819912a398f4eb84e7f16ed443232ba638b5e1a05e9ae1d7c484
	f74d554a503aa825e823725531d97e784dd9b1aacdb58d1f7076b90100000000
	000000000000000000c2c5d46481c291be111d5e3a0b52114bdf212a01000000
	0000000000000000000000000000000000000000000000000000000002000000
	0000000000000000000000000000000000000000000de0b6b3a7640000000000
	0000000000000000000000000000000000000000000000000000000080000000
	00000000000000000000000000000000000000000000000000000000570c0182
	13dae5f9c236beab905c8305cb159c5fa1aae500d43593c715fdd31c61141abd
	04a99fd6822c8558854ccde39a5684e7a56da27d0000d9e9ac2d780300000000
	0000000000000000000000000000000000000000000000000000000000
"
);

#[test]
fn test_submit_with_invalid_source_channel() {
    new_tester(H160::zero()).execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();
        let origin = Origin::signed(relayer);

        // Submit message
        let message = Message {
            data: MESSAGE_DATA_0.into(),
            proof: Proof {
                block_hash: Default::default(),
                tx_index: Default::default(),
                data: Default::default(),
            },
        };
        assert_noop!(
            IncentivizedInboundChannel::submit(origin.clone(), BASE_NETWORK_ID, message.clone()),
            Error::<Test>::InvalidSourceChannel
        );
    });
}

#[test]
fn test_submit() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();
        let origin = Origin::signed(relayer);

        // Submit message 1
        let message_1 = Message {
            data: MESSAGE_DATA_0.into(),
            proof: Proof {
                block_hash: Default::default(),
                tx_index: Default::default(),
                data: Default::default(),
            },
        };
        assert_ok!(IncentivizedInboundChannel::submit(
            origin.clone(),
            BASE_NETWORK_ID,
            message_1
        ));
        let nonce: u64 =
            <ChannelNonces<Test>>::get(BASE_NETWORK_ID, H160::from(SOURCE_CHANNEL_ADDR));
        assert_eq!(nonce, 1);

        // Submit message 2
        let message_2 = Message {
            data: MESSAGE_DATA_1.into(),
            proof: Proof {
                block_hash: Default::default(),
                tx_index: Default::default(),
                data: Default::default(),
            },
        };
        assert_ok!(IncentivizedInboundChannel::submit(
            origin.clone(),
            BASE_NETWORK_ID,
            message_2
        ));
        let nonce: u64 =
            <ChannelNonces<Test>>::get(BASE_NETWORK_ID, H160::from(SOURCE_CHANNEL_ADDR));
        assert_eq!(nonce, 2);
    });
}

#[test]
fn test_submit_with_invalid_nonce() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();
        let origin = Origin::signed(relayer);

        // Submit message
        let message = Message {
            data: MESSAGE_DATA_0.into(),
            proof: Proof {
                block_hash: Default::default(),
                tx_index: Default::default(),
                data: Default::default(),
            },
        };
        assert_ok!(IncentivizedInboundChannel::submit(
            origin.clone(),
            BASE_NETWORK_ID,
            message.clone()
        ));
        let nonce: u64 =
            <ChannelNonces<Test>>::get(BASE_NETWORK_ID, H160::from(SOURCE_CHANNEL_ADDR));
        assert_eq!(nonce, 1);

        // Submit the same again
        assert_noop!(
            IncentivizedInboundChannel::submit(origin.clone(), BASE_NETWORK_ID, message.clone()),
            Error::<Test>::InvalidNonce
        );
    });
}

#[test]
#[ignore] // TODO: fix test_handle_fee test
fn test_handle_fee() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();

        let _ = Balances::deposit_creating(&SourceAccount::get(), 100000000000); // 10 DOT
        let _ = Balances::deposit_creating(&TreasuryAccount::get(), Balances::minimum_balance());
        let _ = Balances::deposit_creating(&relayer, Balances::minimum_balance());

        let fee = 10000000000; // 1 DOT

        let source_account =
            <SourceAccounts<Test>>::get(BASE_NETWORK_ID, H160::from(SOURCE_CHANNEL_ADDR));
        IncentivizedInboundChannel::handle_fee(fee, &relayer, &source_account);
        assert_eq!(Balances::free_balance(&TreasuryAccount::get()), 2000000001);
        assert_eq!(Balances::free_balance(&relayer), 8000000001);
    });
}

#[test]
fn test_set_reward_fraction_not_authorized() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        let bob: AccountId = Keyring::Bob.into();
        assert_noop!(
            IncentivizedInboundChannel::set_reward_fraction(
                Origin::signed(bob),
                Perbill::from_percent(60)
            ),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn test_submit_with_invalid_network_id() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();
        let origin = Origin::signed(relayer);

        // Submit message
        let message = Message {
            data: MESSAGE_DATA_0.into(),
            proof: Proof {
                block_hash: Default::default(),
                tx_index: Default::default(),
                data: Default::default(),
            },
        };
        assert_noop!(
            IncentivizedInboundChannel::submit(
                origin.clone(),
                BASE_NETWORK_ID + 1,
                message.clone()
            ),
            Error::<Test>::InvalidSourceChannel
        );
    });
}

#[test]
fn test_register_channel() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        let owner: AccountId = Keyring::Charlie.into();
        let source: AccountId = Keyring::Dave.into();

        assert_ok!(IncentivizedInboundChannel::register_channel(
            Origin::signed(owner.clone()),
            BASE_NETWORK_ID + 1,
            H160::from(SOURCE_CHANNEL_ADDR),
            source.clone(),
        ));

        assert_eq!(
            ChannelOwners::<Test>::get(BASE_NETWORK_ID + 1, H160::from(SOURCE_CHANNEL_ADDR)),
            Some(owner.clone())
        );

        assert_eq!(
            SourceAccounts::<Test>::get(BASE_NETWORK_ID + 1, H160::from(SOURCE_CHANNEL_ADDR)),
            source.clone()
        );
    });
}

#[test]
fn test_register_existing_channel() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        let owner: AccountId = Keyring::Charlie.into();
        let source: AccountId = Keyring::Dave.into();

        assert_noop!(
            IncentivizedInboundChannel::register_channel(
                Origin::signed(owner.clone()),
                BASE_NETWORK_ID,
                H160::from(SOURCE_CHANNEL_ADDR),
                source.clone(),
            ),
            Error::<Test>::ContractExists
        );

        assert_eq!(
            ChannelOwners::<Test>::get(BASE_NETWORK_ID, H160::from(SOURCE_CHANNEL_ADDR)),
            Some(Default::default())
        );

        assert_eq!(
            SourceAccounts::<Test>::get(BASE_NETWORK_ID, H160::from(SOURCE_CHANNEL_ADDR)),
            Default::default()
        );
    });
}
