use super::*;

use frame_support::dispatch::DispatchError;
use frame_support::{assert_noop, assert_ok, parameter_types};
use sp_core::{H160, H256};
use sp_keyring::AccountKeyring as Keyring;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Keccak256, Verify};
use sp_runtime::MultiSignature;
use sp_std::convert::From;

use snowbridge_core::{AssetId, SingleAsset};

use crate::outbound as basic_outbound_channel;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Event<T>},
        BasicOutboundChannel: basic_outbound_channel::{Pallet, Call, Storage, Event},
    }
);

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
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
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

parameter_types! {
    pub const MaxMessagePayloadSize: usize = 128;
    pub const MaxMessagesPerCommit: usize = 5;
    pub const Ether: AssetId = AssetId::ETH;
}

impl basic_outbound_channel::Config for Test {
    const INDEXING_PREFIX: &'static [u8] = b"commitment";
    type Event = Event;
    type Hashing = Keccak256;
    type MaxMessagePayloadSize = MaxMessagePayloadSize;
    type MaxMessagesPerCommit = MaxMessagesPerCommit;
    type WeightInfo = ();
}

pub fn new_tester() -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    let config: BasicOutboundChannelConfig<T> = BasicOutboundChannelConfig { interval: 1u64 };
    config.assimilate_storage(&mut storage).unwrap();

    let mut ext: sp_io::TestExternalities = storage.into();

    ext.execute_with(|| System::set_block_number(1));
    ext
}

#[test]
fn test_submit() {
    new_tester().execute_with(|| {
        let target = H160::zero();
        let who: AccountId = Keyring::Bob.into();

        assert_ok!(BasicOutboundChannel::submit(&who, target, &vec![0, 1, 2]));
        assert_eq!(Nonce::get(), 1);

        assert_ok!(BasicOutboundChannel::submit(&who, target, &vec![0, 1, 2]));
        assert_eq!(Nonce::get(), 2);
    });
}

#[test]
fn test_submit_fees_burned() {
    new_tester().execute_with(|| {
        let target = H160::zero();
        let who: AccountId = Keyring::Bob.into();

        assert_ok!(BasicOutboundChannel::submit(&who, target, &vec![0, 1, 2]));
    })
}

#[test]
fn test_submit_exceeds_queue_limit() {
    new_tester().execute_with(|| {
        let target = H160::zero();
        let who: AccountId = Keyring::Bob.into();

        let max_messages = MaxMessagesPerCommit::get();
        (0..max_messages)
            .for_each(|_| BasicOutboundChannel::submit(&who, target, &vec![0, 1, 2]).unwrap());

        assert_noop!(
            BasicOutboundChannel::submit(&who, target, &vec![0, 1, 2]),
            Error::<Test>::QueueSizeLimitReached,
        );
    })
}

#[test]
fn test_set_fee_not_authorized() {
    new_tester().execute_with(|| {
        let bob: AccountId = Keyring::Bob.into();
        assert_noop!(
            BasicOutboundChannel::set_fee(Origin::signed(bob), 1000.into()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn test_submit_exceeds_payload_limit() {
    new_tester().execute_with(|| {
        let target = H160::zero();
        let who: AccountId = Keyring::Bob.into();

        let max_payload_bytes = MaxMessagePayloadSize::get();
        let payload: Vec<u8> = (0..).take(max_payload_bytes + 1).collect();

        assert_noop!(
            BasicOutboundChannel::submit(&who, target, payload.as_slice()),
            Error::<Test>::PayloadTooLarge,
        );
    })
}

#[test]
fn test_submit_fails_on_nonce_overflow() {
    new_tester().execute_with(|| {
        let target = H160::zero();
        let who: AccountId = Keyring::Bob.into();

        Nonce::set(u64::MAX);
        assert_noop!(
            BasicOutboundChannel::submit(&who, target, &vec![0, 1, 2]),
            Error::<Test>::Overflow,
        );
    });
}
