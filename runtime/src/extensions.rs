use codec::{Decode, Encode};
use frame_support::dispatch::{DispatchInfo, Dispatchable, PostDispatchInfo};
use frame_support::pallet_prelude::InvalidTransaction;
use frame_support::unsigned::TransactionValidityError;
use frame_support::weights::Pays;
use pallet_transaction_payment as ptp;
use pallet_utility::Call as UtilityCall;
use sp_runtime::traits::{DispatchInfoOf, SignedExtension};
use sp_runtime::FixedPointOperand;
use sp_std::borrow::Cow;
use xor_fee::IsCalledByBridgePeer;

type PtpBalanceOf<T> =
    <<T as ptp::Config>::OnChargeTransaction as ptp::OnChargeTransaction<T>>::Balance;

/// The copy of pallet_transaction_payment::ChargeTransactionPayment, but the tip is always 0.
/// We don't want some users to have leverage over other because it could be abused in trading
#[derive(Encode, Clone, Eq, PartialEq, scale_info::TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ChargeTransactionPayment<T: ptp::Config>(ptp::ChargeTransactionPayment<T>);

impl<T: ptp::Config> ChargeTransactionPayment<T>
where
    PtpBalanceOf<T>: Send + Sync + FixedPointOperand,
    T::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
{
    pub fn new() -> Self {
        Self(ptp::ChargeTransactionPayment::<T>::from(0u32.into()))
    }
}

impl<T: ptp::Config> sp_std::fmt::Debug for ChargeTransactionPayment<T> {
    fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        Ok(())
    }
}

impl<T: ptp::Config> Decode for ChargeTransactionPayment<T>
where
    PtpBalanceOf<T>: Send + Sync + FixedPointOperand,
    T::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
{
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        // The input needs to be checked, but the result is irrelevant
        let _ = ptp::ChargeTransactionPayment::<T>::decode(input)?;
        Ok(Self(ptp::ChargeTransactionPayment::<T>::from(0u32.into())))
    }
}

// Copied from pallet-transaction-payment
impl<T: ptp::Config + eth_bridge::Config> SignedExtension for ChargeTransactionPayment<T>
where
    T: frame_system::Config<Call = crate::Call>,
    PtpBalanceOf<T>: Send + Sync + From<u64> + FixedPointOperand,
    <T as frame_system::Config>::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>
        + IsCalledByBridgePeer<T::AccountId>,
{
    const IDENTIFIER: &'static str =
        <ptp::ChargeTransactionPayment<T> as SignedExtension>::IDENTIFIER;

    type AccountId = <ptp::ChargeTransactionPayment<T> as SignedExtension>::AccountId;

    type Call = crate::Call;

    type AdditionalSigned = <ptp::ChargeTransactionPayment<T> as SignedExtension>::AdditionalSigned;

    type Pre = <ptp::ChargeTransactionPayment<T> as SignedExtension>::Pre;

    fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        self.0.additional_signed()
    }

    fn validate(
        &self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize,
    ) -> sp_api::TransactionValidity {
        call.check_for_swap_in_batch()?;
        let info = Self::pre_dispatch_info(who, call, info);
        self.0.validate(who, call, &*info, len)
    }

    fn pre_dispatch(
        self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        call.check_for_swap_in_batch()?;
        let info = Self::pre_dispatch_info(who, call, info);
        self.0.pre_dispatch(who, call, &*info, len)
    }

    fn post_dispatch(
        pre: Option<Self::Pre>,
        info: &sp_runtime::traits::DispatchInfoOf<Self::Call>,
        post_info: &sp_runtime::traits::PostDispatchInfoOf<Self::Call>,
        len: usize,
        result: &sp_runtime::DispatchResult,
    ) -> Result<(), TransactionValidityError> {
        <ptp::ChargeTransactionPayment<T> as SignedExtension>::post_dispatch(
            pre, info, post_info, len, result,
        )
    }
}

impl crate::Call {
    // Filter batch calls containing at least a swap call
    fn check_for_swap_in_batch(&self) -> Result<(), TransactionValidityError> {
        if let Self::Utility(UtilityCall::batch { calls })
        | Self::Utility(UtilityCall::batch_all { calls }) = self
        {
            if calls.iter().any(|call| {
                matches!(
                    call,
                    Self::LiquidityProxy(liquidity_proxy::Call::swap { .. })
                        | Self::LiquidityProxy(liquidity_proxy::Call::swap_transfer { .. })
                )
            }) {
                return Err(TransactionValidityError::Invalid(InvalidTransaction::Call));
            }
        }

        Ok(())
    }
}

impl<T: ptp::Config + eth_bridge::Config> ChargeTransactionPayment<T>
where
    <T as frame_system::Config>::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>
        + IsCalledByBridgePeer<T::AccountId>,
{
    /// Returns dispatch info for the call for `validate` and `pre_dispatch` methods based on the
    /// given one.
    fn pre_dispatch_info<'a>(
        who: &'a <T as frame_system::Config>::AccountId,
        call: &'a <T as frame_system::Config>::Call,
        info: &'a DispatchInfoOf<<T as frame_system::Config>::Call>,
    ) -> Cow<'a, DispatchInfoOf<<T as frame_system::Config>::Call>> {
        // In eth-bridge we can't check that the call was called by a peer, since `origin` is not
        // accessible in the `pallet::weight` attribute, so we perform the check here and set
        // `pays_fee` to `Pays::No` if the extrinsic was called by a bridge peer.
        if call.is_called_by_bridge_peer(who) {
            let mut info: DispatchInfo = info.clone().into();
            info.pays_fee = Pays::No;
            Cow::Owned(info)
        } else {
            Cow::Borrowed(info)
        }
    }
}

#[cfg(test)]
mod tests {
    use frame_support::weights::{DispatchInfo, Pays};
    use pallet_utility::Call as UtilityCall;
    use sp_core::H256;
    use sp_runtime::traits::SignedExtension;
    use sp_runtime::AccountId32;

    use common::{balance, VAL, XOR};

    use crate::extensions::ChargeTransactionPayment;
    use crate::{Call, GetBaseAssetId, Runtime};

    #[test]
    fn check_calls_from_bridge_peers_pays_yes() {
        let call: &<Runtime as frame_system::Config>::Call =
            &Call::EthBridge(eth_bridge::Call::transfer_to_sidechain {
                asset_id: XOR.into(),
                to: Default::default(),
                amount: Default::default(),
                network_id: 0,
            });

        let dispatch_info = DispatchInfo::default();
        let who = AccountId32::from([0; 32]);

        let pre_info =
            ChargeTransactionPayment::<Runtime>::pre_dispatch_info(&who, call, &dispatch_info);
        assert_eq!(pre_info.pays_fee, Pays::Yes);
    }

    #[test]
    #[ignore] // TODO: fix check_calls_from_bridge_peers_pays_no test
    fn check_calls_from_bridge_peers_pays_no() {
        framenode_chain_spec::ext().execute_with(|| {
            let call: &<Runtime as frame_system::Config>::Call =
                &Call::EthBridge(eth_bridge::Call::finalize_incoming_request {
                    hash: H256::zero(),
                    network_id: 0,
                });

            let dispatch_info = DispatchInfo::default();
            let who = eth_bridge::BridgeAccount::<Runtime>::get(0).unwrap();

            let pre_info =
                ChargeTransactionPayment::<Runtime>::pre_dispatch_info(&who, call, &dispatch_info);
            assert_eq!(pre_info.pays_fee, Pays::No);
        });
    }

    #[test]
    fn simple_call_should_pass() {
        let call = Call::Assets(assets::Call::transfer {
            asset_id: GetBaseAssetId::get(),
            to: From::from([1; 32]),
            amount: balance!(100),
        });

        assert!(call.check_for_swap_in_batch().is_ok());
    }

    #[test]
    fn regular_batch_should_pass() {
        let batch_calls = vec![
            assets::Call::transfer {
                asset_id: GetBaseAssetId::get(),
                to: From::from([1; 32]),
                amount: balance!(100),
            }
            .into(),
            assets::Call::transfer {
                asset_id: GetBaseAssetId::get(),
                to: From::from([1; 32]),
                amount: balance!(100),
            }
            .into(),
        ];

        let call_batch = Call::Utility(UtilityCall::batch {
            calls: batch_calls.clone(),
        });
        let call_batch_all = Call::Utility(UtilityCall::batch_all { calls: batch_calls });

        assert!(call_batch.check_for_swap_in_batch().is_ok());
        assert!(call_batch_all.check_for_swap_in_batch().is_ok());
    }

    fn test_swap_in_batch(call: Call) {
        let batch_calls = vec![
            assets::Call::transfer {
                asset_id: GetBaseAssetId::get(),
                to: From::from([1; 32]),
                amount: balance!(100),
            }
            .into(),
            call,
        ];

        let call_batch = Call::Utility(UtilityCall::batch {
            calls: batch_calls.clone(),
        });
        let call_batch_all = Call::Utility(UtilityCall::batch_all { calls: batch_calls });

        assert!(call_batch.check_for_swap_in_batch().is_err());
        assert!(call_batch_all.check_for_swap_in_batch().is_err());

        let who = AccountId32::from([0; 32]);
        let dispatch_info = DispatchInfo::default();
        let len = 10;

        let pre_batch = ChargeTransactionPayment::<Runtime>::new().pre_dispatch(
            &who,
            &call_batch,
            &dispatch_info,
            len,
        );
        let pre_batch_all = ChargeTransactionPayment::<Runtime>::new().pre_dispatch(
            &who,
            &call_batch_all,
            &dispatch_info,
            len,
        );
        let val_batch = ChargeTransactionPayment::<Runtime>::new().validate(
            &who,
            &call_batch,
            &dispatch_info,
            len,
        );
        let val_batch_all = ChargeTransactionPayment::<Runtime>::new().validate(
            &who,
            &call_batch_all,
            &dispatch_info,
            len,
        );

        assert!(pre_batch.is_err());
        assert!(pre_batch_all.is_err());
        assert!(val_batch.is_err());
        assert!(val_batch_all.is_err());
    }

    #[test]
    fn swap_in_batch_should_fail() {
        test_swap_in_batch(
            liquidity_proxy::Call::swap {
                dex_id: 0,
                input_asset_id: VAL,
                output_asset_id: XOR,
                swap_amount: common::prelude::SwapAmount::WithDesiredInput {
                    desired_amount_in: crate::balance!(100),
                    min_amount_out: crate::balance!(100),
                },
                selected_source_types: vec![],
                filter_mode: common::FilterMode::Disabled,
            }
            .into(),
        );
    }

    #[test]
    fn swap_transfer_in_batch_should_fail() {
        test_swap_in_batch(
            liquidity_proxy::Call::swap_transfer {
                receiver: From::from([1; 32]),
                dex_id: 0,
                input_asset_id: VAL,
                output_asset_id: XOR,
                swap_amount: common::prelude::SwapAmount::WithDesiredInput {
                    desired_amount_in: crate::balance!(100),
                    min_amount_out: crate::balance!(100),
                },
                selected_source_types: vec![],
                filter_mode: common::FilterMode::Disabled,
            }
            .into(),
        );
    }
}
