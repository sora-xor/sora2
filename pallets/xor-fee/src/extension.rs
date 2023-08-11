use core::fmt::Debug;

use codec::{Decode, Encode};
use frame_support::dispatch::{DispatchClass, DispatchInfo, PostDispatchInfo};
use pallet_transaction_payment as ptp;
use ptp::OnChargeTransaction;
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{DispatchInfoOf, Dispatchable, PostDispatchInfoOf, SignedExtension},
    transaction_validity::{
        TransactionPriority, TransactionValidity, TransactionValidityError, ValidTransaction,
    },
    DispatchResult,
};

use crate::{BalanceOf, Config, CustomFeeDetailsOf, LiquidityInfo};

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ChargeTransactionPayment<T: Config> {
    #[codec(compact)]
    tip: BalanceOf<T>,
}

impl<T: Config> From<u128> for ChargeTransactionPayment<T>
where
    BalanceOf<T>: From<u128>,
{
    fn from(value: u128) -> Self {
        Self { tip: value.into() }
    }
}

impl<T: Config> Debug for ChargeTransactionPayment<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XorFeeChargeTransactionPayment")
            .field(&self.tip)
            .finish()
    }
}

impl<T: Config> Default for ChargeTransactionPayment<T>
where
    BalanceOf<T>: Default,
{
    fn default() -> Self {
        ChargeTransactionPayment {
            tip: Default::default(),
        }
    }
}

type CallOf<T> = <T as frame_system::Config>::RuntimeCall;

impl<T: Config> ChargeTransactionPayment<T>
where
    T: ptp::Config<OnChargeTransaction = crate::Pallet<T>>,
    BalanceOf<T>: Into<u128>,
    CallOf<T>: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
{
    pub fn new() -> Self {
        Default::default()
    }

    #[allow(clippy::type_complexity)] // This function can only be called in this module
    fn withdraw_fee(
        &self,
        who: &T::AccountId,
        call: &CallOf<T>,
        info: &DispatchInfoOf<CallOf<T>>,
        len: usize,
    ) -> Result<
        (
            BalanceOf<T>,
            LiquidityInfo<T>,
            Option<CustomFeeDetailsOf<T>>,
        ),
        TransactionValidityError,
    > {
        let tip = self.tip;
        let (fee, fee_details) = crate::Pallet::<T>::compute_fee(len as u32, call, info, tip);
        let liquidity_info = T::OnChargeTransaction::withdraw_fee(who, call, info, fee, tip)?;
        Ok((fee, liquidity_info, fee_details))
    }
}

impl<T: Config> SignedExtension for ChargeTransactionPayment<T>
where
    BalanceOf<T>: Send + Sync + Into<u128>,
    T: ptp::Config<OnChargeTransaction = crate::Pallet<T>>,
    CallOf<T>: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
{
    const IDENTIFIER: &'static str = "ChargeTransactionPayment";
    type AccountId = T::AccountId;
    type Call = CallOf<T>;
    type AdditionalSigned = ();
    type Pre = (
        // tip
        BalanceOf<T>,
        // who paid the fee - this is an option to allow for a Default impl.
        Self::AccountId,
        // additional data
        LiquidityInfo<T>,
        // transaction fee kind
        Option<CustomFeeDetailsOf<T>>,
    );
    fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
        Ok(())
    }

    fn validate(
        &self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize,
    ) -> TransactionValidity {
        let (final_fee, _, _) = self.withdraw_fee(who, call, info, len)?;
        let priority = match info.class {
            DispatchClass::Normal => TransactionPriority::default(),
            DispatchClass::Operational | DispatchClass::Mandatory => {
                ptp::ChargeTransactionPayment::<T>::get_priority(info, len, self.tip, final_fee)
            }
        };
        Ok(ValidTransaction {
            priority,
            ..Default::default()
        })
    }

    fn pre_dispatch(
        self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        let (_, liquidity_info, fee_kind) = self.withdraw_fee(who, call, info, len)?;
        Ok((self.tip, who.clone(), liquidity_info, fee_kind))
    }

    fn post_dispatch(
        maybe_pre: Option<Self::Pre>,
        info: &DispatchInfoOf<Self::Call>,
        post_info: &PostDispatchInfoOf<Self::Call>,
        len: usize,
        result: &DispatchResult,
    ) -> Result<(), TransactionValidityError> {
        if let Some((tip, who, imbalance, custom_fee_details)) = maybe_pre {
            let actual_fee = crate::Pallet::<T>::compute_actual_fee(
                len as u32,
                info,
                post_info,
                result,
                tip,
                custom_fee_details,
            );
            T::OnChargeTransaction::correct_and_deposit_fee(
                &who, info, post_info, actual_fee, tip, imbalance,
            )?;
            let event: <T as ptp::Config>::RuntimeEvent = ptp::Event::<T>::TransactionFeePaid {
                who,
                actual_fee,
                tip,
            }
            .into();
            frame_system::Pallet::<T>::deposit_event(event);
        }
        Ok(())
    }
}
