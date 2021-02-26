#![cfg_attr(not(feature = "std"), no_std)]

use core::convert::TryInto;

use common::{fixed, prelude::*, Fixed, LiquiditySourceFilter, LiquiditySourceType};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    traits::{Currency, Get, Imbalance},
};
use liquidity_proxy::LiquidityProxyTrait;
use pallet_staking::ValBurnedNotifier;
use pallet_transaction_payment::OnTransactionPayment;

pub const TECH_ACCOUNT_PREFIX: &[u8] = b"xor-fee";
pub const TECH_ACCOUNT_MAIN: &[u8] = b"main";

type NegativeImbalanceOf<T> = <<T as Trait>::XorCurrency as Currency<
    <T as frame_system::Trait>::AccountId,
>>::NegativeImbalance;

type BalanceOf<T> =
    <<T as Trait>::XorCurrency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

type Technical<T> = technical::Module<T>;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait:
    frame_system::Trait + referral_system::Trait + assets::Trait + common::Trait + technical::Trait
{
    type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;

    /// XOR - The native currency of this blockchain.
    type XorCurrency: Currency<Self::AccountId> + Send + Sync;

    type XorId: Get<Self::AssetId>;

    type ValId: Get<Self::AssetId>;

    type ReferrerWeight: Get<u32>;

    type XorBurnedWeight: Get<u32>;

    type XorIntoValBurnedWeight: Get<u32>;

    type DEXIdValue: Get<Self::DEXId>;

    type LiquidityProxy: LiquidityProxyTrait<Self::DEXId, Self::AccountId, Self::AssetId>;

    type ValBurnedNotifier: ValBurnedNotifier<Balance>;
}

decl_storage! {
    trait Store for Module<T: Trait> as XorFee {}
}

decl_event!(
    pub enum Event {}
);

decl_error! {
    pub enum Error for Module<T: Trait> {}
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;
    }
}

impl<T: Trait> Module<T> {}

impl<T: Trait> OnTransactionPayment<T::AccountId, NegativeImbalanceOf<T>, BalanceOf<T>>
    for Module<T>
{
    fn on_payment(
        from_account: T::AccountId,
        fee: NegativeImbalanceOf<T>,
        tip: NegativeImbalanceOf<T>,
    ) {
        let amount = fee.merge(tip);
        let (referrer_xor, amount) = amount.ration(
            T::ReferrerWeight::get(),
            T::XorBurnedWeight::get() + T::XorIntoValBurnedWeight::get(),
        );
        if let Some(referrer) = referral_system::Module::<T>::referrer_account(from_account) {
            let _result = T::XorCurrency::resolve_into_existing(&referrer, referrer_xor);
        }
        // TODO: decide what should be done with XOR if there is no referrer.
        // Burn XOR for now
        let (_xor_burned, xor_to_val) =
            amount.ration(T::XorBurnedWeight::get(), T::XorIntoValBurnedWeight::get());
        // Convert `NegativeImbalanceOf<T>` value to a `Fixed` type through its inner representation
        // This method is saturation-free as long as `xor_to_val` doesn't exceed 13.106511852580896768
        // which is the case for fees (that are usually of the order of ~10^-4)
        let xor_to_val: usize = xor_to_val.peek().try_into().unwrap_or(0);
        let xor_to_val: Fixed = Fixed::from_bits(xor_to_val as i128);
        let tech_account_id = T::TechAccountId::from_generic_pair(
            TECH_ACCOUNT_PREFIX.to_vec(),
            TECH_ACCOUNT_MAIN.to_vec(),
        );
        // Trying to mint the `xor_to_val` tokens amount to `tech_account_id` of this pallet. Tokens were initially withdrawn as part of the fee.
        if Technical::<T>::mint(&T::XorId::get(), &tech_account_id, Balance(xor_to_val)).is_ok() {
            let account_id = Technical::<T>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
            // Trying to swap XOR with VAL.
            // If swap goes through, VAL will be burned (for more in-depth look read VAL tokenomics), otherwise remove XOR from the tech account.
            match T::LiquidityProxy::exchange(
                &account_id,
                &account_id,
                &T::XorId::get(),
                &T::ValId::get(),
                SwapAmount::WithDesiredInput {
                    desired_amount_in: Balance(xor_to_val),
                    min_amount_out: Balance(fixed!(0)),
                },
                LiquiditySourceFilter::with_allowed(
                    T::DEXIdValue::get(),
                    [LiquiditySourceType::XYKPool].into(),
                ),
            ) {
                Ok(swap_outcome) => {
                    let val_to_burn = Balance::from(swap_outcome.amount);
                    if Technical::<T>::burn(&T::ValId::get(), &tech_account_id, val_to_burn.clone())
                        .is_ok()
                    {
                        T::ValBurnedNotifier::notify_val_burned(val_to_burn);
                    };
                }
                Err(_) => {
                    let _ =
                        Technical::<T>::burn(&T::XorId::get(), &tech_account_id, xor_to_val.into());
                }
            }
        }
    }
}
