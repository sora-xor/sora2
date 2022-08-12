use crate::mock::{
    new_tester, AccountId, Assets, EthApp, Event, Origin, System, Test, BASE_NETWORK_ID,
};
use crate::{Addresses, Error};
use common::{balance, XOR};
use frame_support::assert_ok;
use frame_support::dispatch::DispatchError;
use sp_core::H160;
use sp_keyring::AccountKeyring as Keyring;

fn last_event() -> Event {
    System::events().pop().expect("Event expected").event
}

#[test]
fn mints_after_handling_ethereum_event() {
    new_tester().execute_with(|| {
        let peer_contract = H160::default();
        let sender = H160::repeat_byte(7);
        let recipient: AccountId = Keyring::Bob.into();
        let amount = balance!(10);
        let old_balance = Assets::total_balance(&XOR, &recipient).unwrap();
        assert_ok!(EthApp::mint(
            dispatch::RawOrigin(BASE_NETWORK_ID, Default::default(), peer_contract).into(),
            sender,
            recipient.clone(),
            amount.into()
        ));
        assert_eq!(
            Assets::total_balance(&XOR, &recipient).unwrap(),
            old_balance + amount
        );

        assert_eq!(
            Event::EthApp(crate::Event::<Test>::Minted(
                BASE_NETWORK_ID,
                sender,
                recipient,
                amount
            )),
            last_event()
        );
    });
}

#[test]
fn burn_should_emit_bridge_event() {
    new_tester().execute_with(|| {
        let recipient = H160::repeat_byte(2);
        let bob: AccountId = Keyring::Bob.into();
        let amount = balance!(20);
        assert_ok!(Assets::mint_to(&XOR, &bob, &bob, balance!(500)));

        assert_ok!(EthApp::burn(
            Origin::signed(bob.clone()),
            BASE_NETWORK_ID,
            recipient.clone(),
            amount.into()
        ));

        assert_eq!(
            Event::EthApp(crate::Event::<Test>::Burned(
                BASE_NETWORK_ID,
                bob,
                recipient,
                amount
            )),
            last_event()
        );
    });
}

#[test]
fn should_not_burn_on_commitment_failure() {
    new_tester().execute_with(|| {
        let sender: AccountId = Keyring::Eve.into();
        let recipient = H160::repeat_byte(9);
        let amount = balance!(20);

        assert_ok!(Assets::mint_to(
            &XOR,
            &Keyring::Bob.to_account_id(),
            &sender,
            balance!(500)
        ));

        common::assert_noop_transactional!(
            EthApp::burn(
                Origin::signed(sender.clone()),
                BASE_NETWORK_ID,
                recipient.clone(),
                amount
            ),
            DispatchError::Other("some error!")
        );
    });
}

#[test]
fn test_register_network() {
    new_tester().execute_with(|| {
        assert!(!Addresses::<Test>::contains_key(BASE_NETWORK_ID + 1));
        assert_ok!(EthApp::register_network_with_existing_asset(
            Origin::root(),
            BASE_NETWORK_ID + 1,
            XOR,
            H160::repeat_byte(12)
        ));
        assert!(Addresses::<Test>::contains_key(BASE_NETWORK_ID + 1));
    });
}

#[test]
fn test_existing_register_network() {
    new_tester().execute_with(|| {
        assert!(Addresses::<Test>::contains_key(BASE_NETWORK_ID));
        common::assert_noop_transactional!(
            EthApp::register_network_with_existing_asset(
                Origin::root(),
                BASE_NETWORK_ID,
                XOR,
                H160::repeat_byte(12)
            ),
            Error::<Test>::AppAlreadyExists
        );
        assert!(Addresses::<Test>::contains_key(BASE_NETWORK_ID));
    });
}
