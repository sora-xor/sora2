#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#[warn(clippy::too_many_arguments)]
use codec::{Decode, Encode};
use common::Balance;

#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct UserLendingPosition<BlockNumberFor> {
    pub lending_amount: Balance,
    pub lending_interest: Balance,
    pub last_lending_block: BlockNumberFor,
}

#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct UserBorrowingPosition<AssetId, BlockNumberFor> {
    pub collateral_token: AssetId,
    pub collateral_amount: Balance,
    pub borrowing_amount: Balance,
    pub borrowing_interest: Balance,
    pub last_borrowing_block: BlockNumberFor,
    pub borrowing_rewards: Balance,
}

#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PoolInfo {
    pub total_liquidity: Balance,
    pub total_borrowed: Balance,
    pub total_collateral: Balance,
    pub lending_rate: Balance,
    pub borrowing_rate: Balance,
    pub loan_to_value: Balance,
    pub liquidation_threshold: Balance,
    pub optimal_utilization_rate: Balance,
    pub base_rate: Balance,
    pub slope_rate_1: Balance,
    pub slope_rate_2: Balance,
    pub reserve_factor: Balance,
    pub rewards: Balance,
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::{PoolInfo, UserBorrowingPosition, UserLendingPosition};
    use common::prelude::{Balance, FixedWrapper};
    use common::{balance, PriceVariant};
    use common::{CERES_ASSET_ID, DAI, XOR};
    use frame_support::pallet_prelude::*;
    use frame_support::pallet_prelude::{OptionQuery, ValueQuery};
    use frame_support::sp_runtime::traits::AccountIdConversion;
    use frame_support::transactional;
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;
    use hex_literal::hex;
    use sp_runtime::traits::UniqueSaturatedInto;

    const PALLET_ID: PalletId = PalletId(*b"apollolb");

    #[pallet::config]
    pub trait Config: frame_system::Config + assets::Config + price_tools::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    type Assets<T> = assets::Pallet<T>;
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type AssetIdOf<T> = <T as assets::Config>::AssetId;
    pub type PriceTools<T> = price_tools::Pallet<T>;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::storage]
    #[pallet::getter(fn user_lending_info)]
    pub type UserLendingInfo<T: Config> = StorageDoubleMap<
        _,
        Identity,
        AccountIdOf<T>,
        Identity,
        AssetIdOf<T>,
        UserLendingPosition<BlockNumberFor<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn user_borrowing_info)]
    pub type UserBorrowingInfo<T: Config> = StorageDoubleMap<
        _,
        Identity,
        AccountIdOf<T>,
        Identity,
        AssetIdOf<T>,
        UserBorrowingPosition<AssetIdOf<T>, BlockNumberFor<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn pool_info)]
    pub type PoolData<T: Config> = StorageMap<_, Identity, AssetIdOf<T>, PoolInfo, OptionQuery>;

    #[pallet::type_value]
    pub fn DefaultForAuthorityAccount<T: Config>() -> AccountIdOf<T> {
        let bytes = hex!("96ea3c9c0be7bbc7b0656a1983db5eed75210256891a9609012362e36815b132");
        AccountIdOf::<T>::decode(&mut &bytes[..]).unwrap()
    }

    /// Account which has permissions for creating a poll
    #[pallet::storage]
    #[pallet::getter(fn authority_account)]
    pub type AuthorityAccount<T: Config> =
        StorageValue<_, AccountIdOf<T>, ValueQuery, DefaultForAuthorityAccount<T>>;

    #[pallet::type_value]
    pub fn FixedLendingRewards<T: Config>() -> Balance {
        balance!(200000)
    }

    /// Default lending rewards
    #[pallet::storage]
    #[pallet::getter(fn lending_rewards)]
    pub type LendingRewards<T: Config> =
        StorageValue<_, Balance, ValueQuery, FixedLendingRewards<T>>;

    #[pallet::type_value]
    pub fn FixedBorrowingRewards<T: Config>() -> Balance {
        balance!(100000)
    }

    /// Default borrowing rewards
    #[pallet::storage]
    #[pallet::getter(fn borrowing_rewards)]
    pub type BorrowingRewards<T: Config> =
        StorageValue<_, Balance, ValueQuery, FixedBorrowingRewards<T>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Pool added [who, asset_id]
        PoolAdded(AccountIdOf<T>, AssetIdOf<T>),
        /// Lended [who, asset_id, amount]
        Lended(AccountIdOf<T>, AssetIdOf<T>, Balance),
        /// Borrowed [who, borrow_asset, collateral_asset, borrow_amount, collateral_amount]
        Borrowed(AccountIdOf<T>, AssetIdOf<T>, AssetIdOf<T>, Balance, Balance),
        /// ClaimedLendingRewards [who, asset_id, amount]
        ClaimedLendingRewards(AccountIdOf<T>, AssetIdOf<T>, Balance),
        /// ClaimedBorrowingRewards [who, asset_id, amount]
        ClaimedBorrowingRewards(AccountIdOf<T>, AssetIdOf<T>, Balance),
        /// Withdrawn [who, asset_id, amount]
        Withdrawn(AccountIdOf<T>, AssetIdOf<T>, Balance),
        /// Repaid [who, asset_id, amount]
        Repaid(AccountIdOf<T>, AssetIdOf<T>, Balance),
        //// ChangedRewardsAmount [who, is_lending, amount]
        ChangedRewardsAmount(AccountIdOf<T>, bool, Balance),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Unauthorized
        Unauthorized,
        /// Token already exists
        TokenAlreadyExists,
        /// Invalid pool parameters
        InvalidPoolParameters,
        /// Asset is not listed
        AssetIsNotListed,
        /// Token does not exists
        TokenDoesNotExists,
        /// Collateral token does not exists
        CollateralTokenDoesNotExists,
        /// No lending amount to borrow
        NoLendingAmountToBorrow,
        /// No liquidity for borrowing asset
        NoLiquidityForBorrowingAsset,
        /// Nothing lended
        NothingLended,
        /// Borrowing amount exceeds
        BorrowingAmountExceeds,
        /// Invalid collateral amount
        InvalidCollateralAmount,
        /// Can not transfer borrowing amount
        CanNotTransferBorrowingAmount,
        /// Can not transfer collateral amount
        CanNotTransferCollateralAmount,
        /// No rewards to claim
        NoRewardsToClaim,
        /// Unable to transfer rewards
        UnableToTransferRewards,
        /// No lending amount
        NoLendingAmount,
        /// Lending amount exceeded
        LendingAmountExceeded,
        /// Can not transfer lending amount
        CanNotTransferLendingAmount,
        /// Nothing borrowed
        NothingBorrowed,
        /// Nothing to repay
        NothingToRepay,
        /// Can not transfer lending interest
        CanNotTransferLendingInterest,
        /// Unable to transfer collateral
        UnableToTransferCollateral,
        /// Unable to transfer amount to repay
        UnableToTransferAmountToRepay,
        /// Can not withdraw lending amount
        CanNotWithdrawLendingAmount,
        /// Can not transfer borrowing rewards
        CanNotTransferBorrowingRewards,
        /// Can not transfer amount to repay
        CanNotTransferAmountToRepay,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add pool
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn add_pool(
            origin: OriginFor<T>,
            asset_id: AssetIdOf<T>,
            loan_to_value: Balance,
            liquidation_threshold: Balance,
            optimal_utilization_rate: Balance,
            base_rate: Balance,
            slope_rate_1: Balance,
            slope_rate_2: Balance,
            reserve_factor: Balance,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            if user != AuthorityAccount::<T>::get() {
                return Err(Error::<T>::Unauthorized.into());
            }

            ensure!(
                !<PoolData<T>>::contains_key(asset_id),
                Error::<T>::TokenAlreadyExists
            );

            if loan_to_value > balance!(1)
                || liquidation_threshold > balance!(1)
                || optimal_utilization_rate > balance!(1)
                || reserve_factor > balance!(1)
            {
                return Err(Error::<T>::InvalidPoolParameters.into());
            }

            let pool_info = PoolInfo {
                total_liquidity: 0,
                total_borrowed: 0,
                total_collateral: 0,
                lending_rate: 0,
                borrowing_rate: 0,
                loan_to_value,
                liquidation_threshold,
                optimal_utilization_rate,
                base_rate,
                slope_rate_1,
                slope_rate_2,
                reserve_factor,
                rewards: 0,
            };

            <PoolData<T>>::insert(asset_id, pool_info);
            //Emit event
            Self::deposit_event(Event::PoolAdded(user, asset_id));

            Ok(().into())
        }

        /// Lend token
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn lend(
            origin: OriginFor<T>,
            lending_token: AssetIdOf<T>,
            lending_amount: Balance,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            ensure!(
                <PoolData<T>>::contains_key(lending_token),
                Error::<T>::TokenDoesNotExists
            );
            let mut pool_info =
                <PoolData<T>>::get(lending_token).ok_or(Error::<T>::AssetIsNotListed)?;

            if let Some(mut user_info) = <UserLendingInfo<T>>::get(user.clone(), lending_token) {
                // Calculate interest in APOLLO token
                let calculated_interest = Self::calculate_lending_earnings(&user, lending_token);
                user_info.lending_interest += calculated_interest;
                user_info.lending_amount += lending_amount;
                user_info.last_lending_block = <frame_system::Pallet<T>>::block_number();
                <UserLendingInfo<T>>::insert(user.clone(), lending_token, user_info);
            } else {
                let new_user_info = UserLendingPosition {
                    lending_amount,
                    lending_interest: 0,
                    last_lending_block: <frame_system::Pallet<T>>::block_number(),
                };
                <UserLendingInfo<T>>::insert(user.clone(), lending_token, new_user_info);
            }

            Assets::<T>::transfer_from(&lending_token, &user, &Self::account_id(), lending_amount)?;

            pool_info.total_liquidity += lending_amount;
            <PoolData<T>>::insert(lending_token, pool_info);
            //Emit event
            Self::deposit_event(Event::Lended(user, lending_token, lending_amount));

            Ok(().into())
        }

        #[transactional]
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn borrow(
            origin: OriginFor<T>,
            borrowing_token: AssetIdOf<T>,
            collateral_token: AssetIdOf<T>,
            borrowing_amount: Balance,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            ensure!(
                <PoolData<T>>::contains_key(borrowing_token),
                Error::<T>::TokenDoesNotExists
            );

            ensure!(
                <PoolData<T>>::contains_key(collateral_token),
                Error::<T>::CollateralTokenDoesNotExists
            );

            let mut borrow_pool_info =
                <PoolData<T>>::get(borrowing_token).ok_or(Error::<T>::AssetIsNotListed)?;
            let mut collateral_pool_info =
                <PoolData<T>>::get(collateral_token).ok_or(Error::<T>::AssetIsNotListed)?;
            let mut user_lending_info = <UserLendingInfo<T>>::get(user.clone(), collateral_token)
                .ok_or(Error::<T>::NothingLended)?;
            let xor_price = PriceTools::<T>::spot_price(&DAI.into()).unwrap();
            let collateral_asset_price = Self::get_price(&collateral_token);

            let coll_amount_in_dollars: u128;
            let collateral_amount: Balance;

            if borrowing_token == XOR.into() {
                coll_amount_in_dollars = ((FixedWrapper::from(borrowing_amount)
                    / FixedWrapper::from(borrow_pool_info.loan_to_value))
                    * FixedWrapper::from(xor_price))
                .try_into_balance()
                .unwrap_or(0);
            } else {
                coll_amount_in_dollars = ((FixedWrapper::from(borrowing_amount)
                    / FixedWrapper::from(borrow_pool_info.loan_to_value))
                    * FixedWrapper::from(collateral_asset_price))
                .try_into_balance()
                .unwrap_or(0);
            }

            if collateral_token == XOR.into() {
                collateral_amount = coll_amount_in_dollars
                    / FixedWrapper::from(xor_price)
                        .try_into_balance()
                        .unwrap_or(0);
            } else {
                collateral_amount = coll_amount_in_dollars
                    / FixedWrapper::from(collateral_asset_price)
                        .try_into_balance()
                        .unwrap_or(0);
            }

            ensure!(
                collateral_amount <= user_lending_info.lending_amount,
                Error::<T>::InvalidCollateralAmount
            );

            ensure!(
                borrowing_amount <= borrow_pool_info.total_liquidity,
                Error::<T>::NoLiquidityForBorrowingAsset
            );

            if let Some(mut user_info) = <UserBorrowingInfo<T>>::get(user.clone(), borrowing_token)
            {
                let calculated_interest =
                    Self::calculate_borrowing_interest(&user, borrowing_token);
                user_info.borrowing_interest += calculated_interest;
                user_info.collateral_amount += collateral_amount;
                user_info.borrowing_amount += borrowing_amount;
                user_info.last_borrowing_block = <frame_system::Pallet<T>>::block_number();
                user_lending_info.lending_amount -= collateral_amount;
                user_lending_info.last_lending_block = <frame_system::Pallet<T>>::block_number();
                borrow_pool_info.total_liquidity -= borrowing_amount;
                borrow_pool_info.total_borrowed += borrowing_amount;
                collateral_pool_info.total_collateral += collateral_amount;

                <UserBorrowingInfo<T>>::insert(user.clone(), borrowing_token, user_info);
                <UserLendingInfo<T>>::insert(user.clone(), collateral_token, user_lending_info);
                <PoolData<T>>::insert(borrowing_token, borrow_pool_info);
                <PoolData<T>>::insert(collateral_token, collateral_pool_info);
            } else {
                let new_user_info = UserBorrowingPosition {
                    collateral_token,
                    collateral_amount,
                    borrowing_amount,
                    borrowing_interest: 0,
                    last_borrowing_block: <frame_system::Pallet<T>>::block_number(),
                    borrowing_rewards: 0,
                };
                user_lending_info.lending_amount -= collateral_amount;
                user_lending_info.last_lending_block = <frame_system::Pallet<T>>::block_number();
                borrow_pool_info.total_liquidity -= borrowing_amount;
                borrow_pool_info.total_borrowed += borrowing_amount;
                collateral_pool_info.total_collateral += collateral_amount;

                <UserBorrowingInfo<T>>::insert(user.clone(), borrowing_token, new_user_info);
                <UserLendingInfo<T>>::insert(user.clone(), collateral_token, user_lending_info);
                <PoolData<T>>::insert(borrowing_token, borrow_pool_info);
                <PoolData<T>>::insert(collateral_token, collateral_pool_info);
            }

            Assets::<T>::transfer_from(
                &borrowing_token,
                &Self::account_id(),
                &user,
                borrowing_amount,
            )
            .map_err(|_| Error::<T>::CanNotTransferBorrowingAmount)?;

            Assets::<T>::transfer_from(
                &collateral_token,
                &user,
                &Self::account_id(),
                collateral_amount,
            )
            .map_err(|_| Error::<T>::CanNotTransferCollateralAmount)?;

            //Emit event
            Self::deposit_event(Event::Borrowed(
                user,
                borrowing_token,
                collateral_token,
                borrowing_amount,
                collateral_amount,
            ));

            Ok(().into())
        }

        /// Get rewards
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn get_rewards(
            origin: OriginFor<T>,
            asset_id: AssetIdOf<T>,
            is_lending: bool,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            let mut lend_user_info = <UserLendingInfo<T>>::get(user.clone(), asset_id)
                .ok_or(Error::<T>::NothingLended)?;

            let mut borrow_user_info = <UserBorrowingInfo<T>>::get(user.clone(), asset_id)
                .ok_or(Error::<T>::NothingBorrowed)?;

            if is_lending == true {
                ensure!(
                    lend_user_info.lending_interest > 0,
                    Error::<T>::NoRewardsToClaim
                );

                Assets::<T>::transfer_from(
                    &CERES_ASSET_ID.into(),
                    &Self::account_id(),
                    &user,
                    lend_user_info.lending_interest.clone(),
                )
                .map_err(|_| Error::<T>::UnableToTransferRewards)?;

                lend_user_info.lending_interest = 0;
                <UserLendingInfo<T>>::insert(user.clone(), asset_id, &lend_user_info);

                //Emit event
                Self::deposit_event(Event::ClaimedLendingRewards(
                    user,
                    asset_id,
                    lend_user_info.lending_interest,
                ));
            } else {
                ensure!(
                    borrow_user_info.borrowing_rewards > 0,
                    Error::<T>::NoRewardsToClaim
                );

                Assets::<T>::transfer_from(
                    &CERES_ASSET_ID.into(),
                    &Self::account_id(),
                    &user,
                    borrow_user_info.borrowing_rewards,
                )
                .map_err(|_| Error::<T>::UnableToTransferRewards)?;

                borrow_user_info.borrowing_rewards = 0;
                <UserBorrowingInfo<T>>::insert(user.clone(), asset_id, &borrow_user_info);

                //Emit event
                Self::deposit_event(Event::ClaimedBorrowingRewards(
                    user,
                    asset_id,
                    borrow_user_info.borrowing_rewards,
                ));
            }
            Ok(().into())
        }

        /// Withdraw
        #[transactional]
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn withdraw(
            origin: OriginFor<T>,
            lending_token: AssetIdOf<T>,
            lending_amount: Balance,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            let mut user_info = <UserLendingInfo<T>>::get(user.clone(), lending_token)
                .ok_or(Error::<T>::NothingLended)?;

            let mut pool_info =
                <PoolData<T>>::get(lending_token).ok_or(Error::<T>::AssetIsNotListed)?;

            ensure!(user_info.lending_amount > 0, Error::<T>::NoLendingAmount);

            ensure!(
                lending_amount < pool_info.total_liquidity,
                Error::<T>::CanNotTransferLendingAmount
            );

            let calculated_interest = Self::calculate_lending_earnings(&user, lending_token);
            user_info.lending_interest += calculated_interest;
            <UserLendingInfo<T>>::insert(user.clone(), lending_token, &user_info);

            if lending_amount < user_info.lending_amount {
                user_info.lending_amount -= lending_amount;
                user_info.last_lending_block = <frame_system::Pallet<T>>::block_number();
                pool_info.total_liquidity -= lending_amount;

                Assets::<T>::transfer_from(
                    &lending_token,
                    &Self::account_id(),
                    &user,
                    lending_amount,
                )
                .map_err(|_| Error::<T>::CanNotTransferLendingAmount)?;

                <UserLendingInfo<T>>::insert(user.clone(), lending_token, user_info);
                <PoolData<T>>::insert(lending_token, pool_info);
            } else if lending_amount == user_info.lending_amount {
                Assets::<T>::transfer_from(
                    &lending_token,
                    &Self::account_id(),
                    &user,
                    lending_amount,
                )
                .map_err(|_| Error::<T>::CanNotTransferLendingAmount)?;

                Assets::<T>::transfer_from(
                    &CERES_ASSET_ID.into(),
                    &Self::account_id(),
                    &user,
                    user_info.lending_interest,
                )
                .map_err(|_| Error::<T>::CanNotTransferLendingInterest)?;

                pool_info.total_liquidity -= lending_amount;

                <UserLendingInfo<T>>::remove(user.clone(), lending_token);
                <PoolData<T>>::insert(lending_token, pool_info);
            } else {
                return Err(Error::<T>::LendingAmountExceeded.into());
            }

            //Emit event
            Self::deposit_event(Event::Withdrawn(user, lending_token, lending_amount));

            Ok(().into())
        }

        #[transactional]
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn repay(
            origin: OriginFor<T>,
            borrowing_token: AssetIdOf<T>,
            amount_to_repay: Balance,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            let mut user_info = <UserBorrowingInfo<T>>::get(user.clone(), borrowing_token)
                .ok_or(Error::<T>::NothingBorrowed)?;
            let mut borrow_pool_info =
                <PoolData<T>>::get(borrowing_token).ok_or(Error::<T>::AssetIsNotListed)?;
            let mut collateral_pool_info = <PoolData<T>>::get(user_info.collateral_token)
                .ok_or(Error::<T>::CollateralTokenDoesNotExists)?;

            ensure!(user_info.borrowing_interest > 0, Error::<T>::NothingToRepay);

            let calculated_interest = Self::calculate_borrowing_interest(&user, borrowing_token);
            user_info.borrowing_interest += calculated_interest;
            <UserBorrowingInfo<T>>::insert(user.clone(), borrowing_token, &user_info);

            if amount_to_repay <= user_info.borrowing_interest {
                let reserve_allocation = amount_to_repay;

                // Reserve allocation

                user_info.borrowing_interest -= amount_to_repay;
                user_info.last_borrowing_block = <frame_system::Pallet<T>>::block_number();
                borrow_pool_info.total_liquidity += amount_to_repay;
                <UserBorrowingInfo<T>>::insert(user.clone(), borrowing_token, user_info);
                <PoolData<T>>::insert(borrowing_token, borrow_pool_info);
            } else if amount_to_repay > user_info.borrowing_interest
                && amount_to_repay < user_info.borrowing_interest + user_info.borrowing_amount
            {
                let reserve_allocation = user_info.borrowing_interest;

                // Reserve allocation

                let remaining_amount = amount_to_repay - user_info.borrowing_interest;
                user_info.borrowing_amount -= remaining_amount;
                user_info.borrowing_interest = 0;
                user_info.last_borrowing_block = <frame_system::Pallet<T>>::block_number();
                borrow_pool_info.total_borrowed -= remaining_amount;
                borrow_pool_info.total_liquidity += remaining_amount;

                Assets::<T>::transfer_from(
                    &borrowing_token.into(),
                    &Self::account_id(),
                    &user,
                    remaining_amount,
                )
                .map_err(|_| Error::<T>::CanNotTransferAmountToRepay)?;

                <UserBorrowingInfo<T>>::insert(user.clone(), borrowing_token, user_info);
                <PoolData<T>>::insert(borrowing_token, borrow_pool_info);
            } else if amount_to_repay == user_info.borrowing_interest + user_info.borrowing_amount {
                let reserve_allocation = user_info.borrowing_interest;

                // Reserve allocation

                borrow_pool_info.total_borrowed -= user_info.borrowing_amount;
                borrow_pool_info.total_liquidity += user_info.borrowing_amount;
                collateral_pool_info.total_collateral -= user_info.collateral_amount;
                // user_info.borrowing_interest = 0;
                // user_info.last_borrowing_block = <frame_system::Pallet<T>>::block_number();
                <PoolData<T>>::insert(borrowing_token, borrow_pool_info);
                <PoolData<T>>::insert(user_info.collateral_token, collateral_pool_info);

                Assets::<T>::transfer_from(
                    &user_info.collateral_token,
                    &Self::account_id(),
                    &user,
                    user_info.collateral_amount.clone(),
                )
                .map_err(|_| Error::<T>::UnableToTransferCollateral)?;

                Assets::<T>::transfer_from(
                    &borrowing_token.into(),
                    &Self::account_id(),
                    &user,
                    user_info.borrowing_amount,
                )
                .map_err(|_| Error::<T>::CanNotTransferBorrowingAmount)?;

                Assets::<T>::transfer_from(
                    &CERES_ASSET_ID.into(),
                    &Self::account_id(),
                    &user,
                    user_info.borrowing_rewards,
                )
                .map_err(|_| Error::<T>::CanNotTransferBorrowingRewards)?;

                <UserBorrowingInfo<T>>::remove(user.clone(), borrowing_token);
            } else {
                return Err(Error::<T>::BorrowingAmountExceeds.into());
            }

            //Emit event
            Self::deposit_event(Event::Repaid(user, borrowing_token, amount_to_repay));

            Ok(().into())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn change_rewards_amount(
            origin: OriginFor<T>,
            is_lending: bool,
            amount: Balance,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            if user != AuthorityAccount::<T>::get() {
                return Err(Error::<T>::Unauthorized.into());
            }

            if is_lending == true {
                <LendingRewards<T>>::put(amount);
            } else {
                <BorrowingRewards<T>>::put(amount);
            }

            //Emit event
            Self::deposit_event(Event::ChangedRewardsAmount(user, is_lending, amount));

            Ok(().into())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(now: T::BlockNumber) -> Weight {
            let rates = Self::calculate_rate(now);
            //let liquidation = Self::liquidation(now);

            //rates.saturating_add(liquidation) // ??????
            rates
        }
    }

    impl<T: Config> Pallet<T> {
        /// The account ID of pallet
        fn account_id() -> T::AccountId {
            PALLET_ID.into_account_truncating()
        }

        fn get_price(asset_id: &AssetIdOf<T>) -> Balance {
            //Get XOR price from spot price function in PriceTools pallet
            let xor_price = PriceTools::<T>::spot_price(&DAI.into()).unwrap();

            // Get price from price-tools pallet
            let buy_price =
                PriceTools::<T>::get_average_price(&XOR.into(), asset_id.into(), PriceVariant::Buy)
                    .unwrap();

            let sell_price = PriceTools::<T>::get_average_price(
                &XOR.into(),
                asset_id.into(),
                PriceVariant::Sell,
            )
            .unwrap();

            // Average price in dollars
            let average_price = (FixedWrapper::from(xor_price * (buy_price + sell_price))
                / FixedWrapper::from(2))
            .try_into_balance()
            .unwrap_or(0);

            return average_price;
        }

        fn calculate_rate(_current_block: T::BlockNumber) -> Weight {
            let mut counter: u64 = 0;
            for (asset_id, mut pool_info) in PoolData::<T>::iter() {
                let utilization_rate = FixedWrapper::from(pool_info.total_borrowed)
                    / FixedWrapper::from(pool_info.total_liquidity);

                if utilization_rate < pool_info.optimal_utilization_rate.into() {
                    pool_info.borrowing_rate = FixedWrapper::from(
                        pool_info.base_rate
                            + (utilization_rate.clone() / pool_info.optimal_utilization_rate)
                                * pool_info.slope_rate_1,
                    )
                    .try_into_balance()
                    .unwrap_or(0);
                } else {
                    pool_info.borrowing_rate = FixedWrapper::from(
                        pool_info.base_rate
                            + pool_info.slope_rate_1
                            + ((utilization_rate.clone() - pool_info.optimal_utilization_rate)
                                / (balance!(1) - pool_info.optimal_utilization_rate))
                                * pool_info.slope_rate_2,
                    )
                    .try_into_balance()
                    .unwrap_or(0);
                }
                pool_info.lending_rate = utilization_rate.try_into_balance().unwrap_or(0);

                <PoolData<T>>::insert(asset_id, pool_info);
                counter += 1;
            }

            T::DbWeight::get()
                .reads(counter + 1)
                .saturating_add(T::DbWeight::get().writes(counter))
        }

        fn calculate_lending_earnings(user: &AccountIdOf<T>, asset_id: AssetIdOf<T>) -> Balance {
            let blocks = 5_256_000 as u128; // 1 year
            let block_number = <frame_system::Pallet<T>>::block_number();
            let user_info = UserLendingInfo::<T>::get(user, asset_id).unwrap();
            let pool_info = PoolData::<T>::get(&asset_id).unwrap();

            let totla_lending_blocks: u128 =
                (block_number - user_info.last_lending_block).unique_saturated_into();
            let lending_interest_per_block = (FixedWrapper::from(user_info.lending_amount)
                * FixedWrapper::from(pool_info.lending_rate))
                / FixedWrapper::from(blocks);
            let lending_interest =
                lending_interest_per_block * FixedWrapper::from(totla_lending_blocks);

            return lending_interest.try_into_balance().unwrap_or(0);
        }

        fn calculate_borrowing_interest(user: &AccountIdOf<T>, asset_id: AssetIdOf<T>) -> Balance {
            let blocks = 5_256_000 as u128; // 1 year
            let block_number = <frame_system::Pallet<T>>::block_number();
            let user_info = UserBorrowingInfo::<T>::get(user, asset_id).unwrap();
            let pool_info = PoolData::<T>::get(&asset_id).unwrap();

            let totla_borrowing_blocks: u128 =
                (block_number - user_info.last_borrowing_block).unique_saturated_into();
            let borrowing_interest_per_block = (FixedWrapper::from(user_info.borrowing_amount)
                * FixedWrapper::from(pool_info.borrowing_rate))
                / FixedWrapper::from(blocks);
            let borrowing_interest =
                borrowing_interest_per_block * FixedWrapper::from(totla_borrowing_blocks);

            return borrowing_interest.try_into_balance().unwrap_or(0);
        }

        fn liquidation(_current_block: T::BlockNumber) -> Weight {
            let counter: u64 = 0;

            for (pool_asset_id, pool_info) in PoolData::<T>::iter() {
                for (user, asset_id, mut user_info) in UserBorrowingInfo::<T>::iter() {
                    let liquidation_threshold: u128;
                    let health_factor: u128;
                    let xor_price = PriceTools::<T>::spot_price(&DAI.into()).unwrap();
                    let total_borrows_in_dollars: u128;
                    let asset_price = Self::get_price(&user_info.collateral_token);

                    if user_info.collateral_token == XOR.into() {
                        let collateral_in_dollars =
                            (FixedWrapper::from(user_info.collateral_amount)
                                * FixedWrapper::from(xor_price))
                            .try_into_balance()
                            .unwrap_or(0);

                        liquidation_threshold = ((collateral_in_dollars
                            * FixedWrapper::from(pool_info.liquidation_threshold))
                            / (FixedWrapper::from(pool_info.total_collateral)
                                * FixedWrapper::from(xor_price)))
                        .try_into_balance()
                        .unwrap_or(0);

                        if pool_asset_id == XOR.into() {
                            total_borrows_in_dollars =
                                (FixedWrapper::from(pool_info.total_borrowed)
                                    * FixedWrapper::from(xor_price))
                                .try_into_balance()
                                .unwrap_or(0);
                        } else {
                            total_borrows_in_dollars =
                                (FixedWrapper::from(pool_info.total_borrowed)
                                    * FixedWrapper::from(asset_price))
                                .try_into_balance()
                                .unwrap_or(0);
                        }

                        health_factor = ((collateral_in_dollars
                            * FixedWrapper::from(pool_info.liquidation_threshold))
                            / total_borrows_in_dollars)
                            .try_into_balance()
                            .unwrap_or(0);
                    } else {
                        let collateral_in_dollars =
                            (FixedWrapper::from(user_info.collateral_amount)
                                * FixedWrapper::from(asset_price))
                            .try_into_balance()
                            .unwrap_or(0);

                        liquidation_threshold = ((collateral_in_dollars
                            * FixedWrapper::from(pool_info.liquidation_threshold))
                            / (FixedWrapper::from(pool_info.total_collateral)
                                * FixedWrapper::from(asset_price)))
                        .try_into_balance()
                        .unwrap_or(0);

                        if pool_asset_id == XOR.into() {
                            total_borrows_in_dollars =
                                (FixedWrapper::from(pool_info.total_borrowed)
                                    * FixedWrapper::from(xor_price))
                                .try_into_balance()
                                .unwrap_or(0);
                        } else {
                            total_borrows_in_dollars =
                                (FixedWrapper::from(pool_info.total_borrowed)
                                    * FixedWrapper::from(asset_price))
                                .try_into_balance()
                                .unwrap_or(0);
                        }

                        health_factor = ((collateral_in_dollars
                            * FixedWrapper::from(pool_info.liquidation_threshold))
                            / total_borrows_in_dollars)
                            .try_into_balance()
                            .unwrap_or(0);
                    }

                    if liquidation_threshold > pool_info.liquidation_threshold
                        || health_factor < balance!(1)
                    {
                        user_info.collateral_amount = 0;
                        user_info.borrowing_amount = 0;
                        user_info.borrowing_interest = 0;
                        <UserBorrowingInfo<T>>::remove(user.clone(), asset_id);

                        // Reserve factor
                    }
                }
            }

            T::DbWeight::get()
                .reads(counter + 1)
                .saturating_add(T::DbWeight::get().writes(counter))
        }
    }
}
