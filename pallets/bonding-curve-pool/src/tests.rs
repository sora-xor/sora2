#[rustfmt::skip]
mod tests {
    use crate::{mock::*, DistributionAccountData, DistributionAccounts, Error, SwapKind};
    use common::{
        self, fixed, fixed_wrapper,
        AssetSymbol, DEXId, LiquiditySource, TechPurpose, USDT, VAL, XOR,
    };
    use common::prelude::{Balance, Fixed, SwapAmount, SwapOutcome, FixedWrapper};
    use common::prelude::fixnum::ops::Numeric;
    use frame_support::assert_err;
    use frame_support::storage::{with_transaction, TransactionOutcome};
    use sp_arithmetic::traits::{Bounded, Zero};
    use sp_runtime::DispatchError;

    #[test]
    fn should_calculate_price() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            assert_eq!(
                BondingCurvePool::buy_price_for_one_main_asset(&XOR)
                    .expect("failed to calculate buy price"),
                fixed!(100)
            );
            assert_eq!(
                BondingCurvePool::price_for_main_asset(&XOR, fixed!(100000), SwapKind::Buy)
                    .expect("failed to calculate buy assets price"),
                fixed!(10010000)
            );
            assert_eq!(
                BondingCurvePool::price_for_collateral_asset(&XOR, 10_010_000u32.into(), SwapKind::Buy)
                    .expect("failed to calculate buy assets price"),
                fixed!(100000)
            );
            assert_eq!(
                BondingCurvePool::sell_price_for_one_main_asset(&XOR)
                    .expect("failed to calculate sell price"),
                fixed!(80)
            );
            assert_eq!(
                BondingCurvePool::price_for_main_asset(&XOR, fixed!(100000), SwapKind::Sell)
                    .expect("failed to calculate sell assets price"),
                fixed!(7992000)
            );
            assert_eq!(
                BondingCurvePool::price_for_collateral_asset(&XOR, fixed!(7992000), SwapKind::Sell)
                    .expect("failed to calculate sell assets price"),
                fixed!(100000)
            );
            assert_eq!(
                BondingCurvePool::price_for_main_asset(&XOR, fixed!(0), SwapKind::Sell)
                    .expect("failed to calculate sell assets price"),
                fixed!(0)
            );
        });
    }

    #[test]
    fn inverse_calculation_for_buy_should_match_forward_price() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            for q in 0u32..10_000 {
                let direct_price = BondingCurvePool::price_for_main_asset(&XOR, q.into(), SwapKind::Buy)
                    .expect("failed to calculate buy assets price");
                let inverse_price = BondingCurvePool::price_for_collateral_asset(&XOR, direct_price.into(), SwapKind::Buy)
                    .expect("failed to calculate buy assets price");
                assert_eq!(Balance::from(q), Balance(inverse_price));
            }

            for q in (100u32..200_000).step_by(123) {
                let direct_price = BondingCurvePool::price_for_main_asset(&XOR, q.into(), SwapKind::Buy)
                    .expect("failed to calculate buy assets price");
                let inverse_price = BondingCurvePool::price_for_collateral_asset(&XOR, direct_price.into(), SwapKind::Buy)
                    .expect("failed to calculate buy assets price");
                assert_eq!(Balance::from(q), Balance(inverse_price));
            }
        });
    }

    #[test]
    fn inverse_calculation_for_sell_should_match_forward_price() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            for q in 0u32..10_000 {
                let direct_price = BondingCurvePool::price_for_main_asset(&XOR, q.into(), SwapKind::Sell)
                    .expect("failed to calculate buy assets price");
                let inverse_price = BondingCurvePool::price_for_collateral_asset(&XOR, direct_price.into(), SwapKind::Sell)
                    .expect("failed to calculate buy assets price (inverse)");
                assert_eq!(Balance::from(q), Balance(inverse_price));
            }

            for q in (100u32..200_000).step_by(123) {
                let direct_price = BondingCurvePool::price_for_main_asset(&XOR, q.into(), SwapKind::Sell)
                    .expect("failed to calculate buy assets price");
                let inverse_price = BondingCurvePool::price_for_collateral_asset(&XOR, direct_price.into(), SwapKind::Sell)
                    .expect("failed to calculate buy assets price (inverse)");
                assert_eq!(Balance::from(q), Balance(inverse_price));
            }
        });
    }

    #[test]
    fn should_not_calculate_price() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            assert_eq!(
                BondingCurvePool::price_for_main_asset(
                    &XOR,
                    Balance::max_value(),
                    SwapKind::Sell
                )
                .unwrap_err(),
                Error::<Runtime>::CalculatePriceFailed.into()
            );
        });
    }

    fn bonding_curve_pool_init(
        initial_reserves: Vec<(AssetId, Balance)>,
    ) -> Result<
        DistributionAccounts<DistributionAccountData<<Runtime as technical::Trait>::TechAccountId>>,
        DispatchError,
    > {
        let bonding_curve_tech_account_id = TechAccountId::Pure(
            DEXId::Polkaswap,
            TechPurpose::Identifier(b"bonding_curve_tech_account_id".to_vec()),
        );
        Technical::register_tech_account_id(bonding_curve_tech_account_id.clone())?;
        BondingCurvePool::set_reserves_account_id(bonding_curve_tech_account_id.clone())?;
        for (asset_id, balance) in initial_reserves {
            Technical::mint(&asset_id, &bonding_curve_tech_account_id, balance)?;
        }

        let val_holders_coefficient: Fixed = fixed!(0.5);
        let val_holders_xor_alloc_coeff = val_holders_coefficient * fixed_wrapper!(0.9);
        let val_holders_buy_back_coefficient = val_holders_coefficient * fixed_wrapper!(0.1);
        let projects_coefficient: FixedWrapper = fixed_wrapper!(1) - val_holders_coefficient;
        let projects_sora_citizens_coeff: FixedWrapper = projects_coefficient.clone() * fixed_wrapper!(0.01);
        let projects_stores_and_shops_coeff: FixedWrapper = projects_coefficient.clone() * fixed_wrapper!(0.04);
        let projects_parliament_and_development_coeff: FixedWrapper = projects_coefficient.clone() * fixed_wrapper!(0.05);
        let projects_other_coeff: FixedWrapper = projects_coefficient.clone() * fixed_wrapper!(0.9);

        debug_assert_eq!(
            Fixed::ONE,
            (val_holders_xor_alloc_coeff.clone()
                + projects_sora_citizens_coeff.clone()
                + projects_stores_and_shops_coeff.clone()
                + projects_parliament_and_development_coeff.clone()
                + projects_other_coeff.clone()
                + val_holders_buy_back_coefficient.clone()).get().unwrap()
        );

        let xor_allocation = DistributionAccountData::new(
            TechAccountId::Pure(
                DEXId::Polkaswap,
                TechPurpose::Identifier(b"xor_allocation".to_vec()),
            ),
            val_holders_xor_alloc_coeff.get().unwrap(),
        );
        let sora_citizens = DistributionAccountData::new(
            TechAccountId::Pure(
                DEXId::Polkaswap,
                TechPurpose::Identifier(b"sora_citizens".to_vec()),
            ),
            projects_sora_citizens_coeff.get().unwrap(),
        );
        let stores_and_shops = DistributionAccountData::new(
            TechAccountId::Pure(
                DEXId::Polkaswap,
                TechPurpose::Identifier(b"stores_and_shops".to_vec()),
            ),
            projects_stores_and_shops_coeff.get().unwrap(),
        );
        let parliament_and_development = DistributionAccountData::new(
            TechAccountId::Pure(
                DEXId::Polkaswap,
                TechPurpose::Identifier(b"parliament_and_development".to_vec()),
            ),
            projects_parliament_and_development_coeff.get().unwrap(),
        );
        let projects = DistributionAccountData::new(
            TechAccountId::Pure(
                DEXId::Polkaswap,
                TechPurpose::Identifier(b"projects".to_vec()),
            ),
            projects_other_coeff.get().unwrap(),
        );
        let val_holders = DistributionAccountData::new(
            TechAccountId::Pure(
                DEXId::Polkaswap,
                TechPurpose::Identifier(b"val_holders".to_vec()),
            ),
            val_holders_buy_back_coefficient.get().unwrap(),
        );
        let accounts = DistributionAccounts::<_> {
            xor_allocation,
            sora_citizens,
            stores_and_shops,
            parliament_and_development,
            projects,
            val_holders,
        };
        for tech_account in &accounts.xor_distribution_accounts_as_array() {
            Technical::register_tech_account_id((*tech_account).clone())?;
        }
        BondingCurvePool::set_distribution_accounts(accounts.clone());
        Ok(accounts)
    }

    #[test]
    fn should_exchange_with_empty_reserves() {
        let mut ext = ExtBuilder::new(vec![
            (
                alice(),
                USDT,
                10_000u32.into(),
                AssetSymbol(b"USDT".to_vec()),
                18,
            ),
            (alice(), XOR, 0u32.into(), AssetSymbol(b"XOR".to_vec()), 18),
            (alice(), VAL, 0u32.into(), AssetSymbol(b"VAL".to_vec()), 18),
        ])
        .build();
        ext.execute_with(|| {
            MockDEXApi::init().unwrap();
            let distribution_accounts = bonding_curve_pool_init(Vec::new()).unwrap();
            let distribution_accounts_array = distribution_accounts.xor_distribution_accounts_as_array();
            let alice = &alice();
            assert_eq!(
                BondingCurvePool::exchange(
                    alice,
                    alice,
                    &DEXId::Polkaswap.into(),
                    &USDT,
                    &XOR,
                    SwapAmount::with_desired_output(1u32.into(), Balance::max_value()),
                )
                .unwrap(),
                SwapOutcome::new(fixed!(0.999), fixed!(0.001))
            );
            for account_id in &distribution_accounts_array {
                assert_eq!(
                    Technical::total_balance(&XOR, account_id).unwrap(),
                    Balance::zero(),
                );
            }
            assert_eq!(
                BondingCurvePool::exchange(
                    alice,
                    alice,
                    &DEXId::Polkaswap.into(),
                    &XOR,
                    &USDT,
                    SwapAmount::with_desired_input(fixed!(0.999), Balance::zero()),
                )
                .unwrap(),
                SwapOutcome::new(
                    fixed!(79.2827970392023992),
                    fixed!(0.0793621591984008)
                )
            );
        });
    }

    #[test]
    fn should_exchange_with_nearly_full_reserves() {
        let mut ext = ExtBuilder::new(vec![
            (
                alice(),
                USDT,
                fixed!(10000),
                AssetSymbol(b"USDT".to_vec()),
                18,
            ),
            (alice(), XOR, fixed!(10), AssetSymbol(b"XOR".to_vec()), 18),
            (alice(), VAL, fixed!(0), AssetSymbol(b"VAL".to_vec()), 18),
        ])
        .build();
        ext.execute_with(|| {
            MockDEXApi::init().unwrap();
            let total_issuance = Assets::total_issuance(&XOR).unwrap();
            let reserve_amount_expected = Balance(
                BondingCurvePool::price_for_main_asset(&XOR, total_issuance, SwapKind::Sell)
                    .unwrap(),
            );
            let pool_usd_amount = reserve_amount_expected
                - Balance(BondingCurvePool::buy_price_for_one_main_asset(&XOR).unwrap())
                    / Balance(fixed!(2));
            let distribution_accounts =
                bonding_curve_pool_init(vec![(USDT, pool_usd_amount)]).unwrap();
            let distribution_accounts_array = distribution_accounts.xor_distribution_accounts_as_array();
            let alice = &alice();
            assert_eq!(
                BondingCurvePool::exchange(
                    alice,
                    alice,
                    &DEXId::Polkaswap.into(),
                    &USDT,
                    &XOR,
                    SwapAmount::with_desired_output(fixed!(1), Balance::max_value()),
                )
                .unwrap(),
                SwapOutcome::new(fixed!(0.999), fixed!(0.001))
            );
            let balances: Vec<Balance> = vec![
                fixed!(0.0445518521703),
                fixed!(0.00049502057967),
                fixed!(0.00198008231868),
                fixed!(0.00247510289835),
                fixed!(0.0445518521703),
            ];
            for (account_id, balance) in distribution_accounts_array
                .to_vec()
                .into_iter()
                .zip(balances)
            {
                assert_eq!(
                    Technical::total_balance(&XOR, &account_id).unwrap(),
                    balance,
                );
            }
            assert_eq!(
                BondingCurvePool::exchange(
                    alice,
                    alice,
                    &DEXId::Polkaswap.into(),
                    &XOR,
                    &USDT,
                    SwapAmount::with_desired_input(fixed!(0.999), Balance::zero()),
                )
                .unwrap(),
                SwapOutcome::new(
                    fixed!(79.2828130072183992),
                    fixed!(0.0793621751824008)
                )
            );
        });
    }

    #[test]
    fn should_exchange_with_full_reserves() {
        let mut ext = ExtBuilder::new(vec![
            (
                alice(),
                USDT,
                fixed!(10000),
                AssetSymbol(b"USDT".to_vec()),
                18,
            ),
            (alice(), XOR, fixed!(10), AssetSymbol(b"XOR".to_vec()), 18),
            (alice(), VAL, fixed!(0), AssetSymbol(b"VAL".to_vec()), 18),
        ])
        .build();
        ext.execute_with(|| {
            MockDEXApi::init().unwrap();
            let total_issuance = Assets::total_issuance(&XOR).unwrap();
            let reserve_amount_expected = Balance(
                BondingCurvePool::price_for_main_asset(&XOR, total_issuance, SwapKind::Sell)
                    .unwrap(),
            );
            let distribution_accounts =
                bonding_curve_pool_init(vec![(USDT, reserve_amount_expected)]).unwrap();
            let distribution_accounts_array = distribution_accounts.xor_distribution_accounts_as_array();
            let alice = &alice();
            assert_eq!(
                BondingCurvePool::exchange(
                    alice,
                    alice,
                    &DEXId::Polkaswap.into(),
                    &USDT,
                    &XOR,
                    SwapAmount::with_desired_output(fixed!(1), Balance::max_value()),
                )
                .unwrap(),
                SwapOutcome::new(fixed!(0.999), fixed!(0.001))
            );
            let balances: Vec<Balance> = vec![
                fixed!(0.0891037034433),
                fixed!(0.00099004114937),
                fixed!(0.00396016459748),
                fixed!(0.00495020574685),
                fixed!(0.0891037034433),
            ];
            for (account_id, balance) in distribution_accounts_array
                .to_vec()
                .into_iter()
                .zip(balances)
            {
                assert_eq!(
                    Technical::total_balance(&XOR, &account_id).unwrap(),
                    balance,
                );
            }
            assert_eq!(
                BondingCurvePool::exchange(
                    alice,
                    alice,
                    &DEXId::Polkaswap.into(),
                    &XOR,
                    &USDT,
                    SwapAmount::with_desired_input(fixed!(0.999), Balance::zero()),
                )
                .unwrap(),
                SwapOutcome::new(
                    fixed!(79.2828130072183992),
                    fixed!(0.0793621751824008)
                )
            );
        });
    }

    #[test]
    fn should_not_sell_without_reserves() {
        let mut ext = ExtBuilder::new(vec![
            (alice(), USDT, 0u32.into(), AssetSymbol(b"USDT".to_vec()), 18),
            (alice(), XOR, 1u32.into(), AssetSymbol(b"XOR".to_vec()), 18),
            (alice(), VAL, 0u32.into(), AssetSymbol(b"VAL".to_vec()), 18),
        ])
        .build();
        ext.execute_with(|| {
            MockDEXApi::init().unwrap();
            let _ = bonding_curve_pool_init(vec![]).unwrap();
            let alice = &alice();
            assert_err!(
                BondingCurvePool::exchange(
                    alice,
                    alice,
                    &DEXId::Polkaswap.into(),
                    &XOR,
                    &USDT,
                    SwapAmount::with_desired_input(fixed!(1), Balance::zero()),
                ),
                Error::<Runtime>::NotEnoughReserves
            );
        });
    }

    #[test]
    fn swaps_should_be_additive() {
        let mut ext = ExtBuilder::new(vec![
            (
                alice(),
                USDT,
                fixed!(10000),
                AssetSymbol(b"USDT".to_vec()),
                18,
            ),
            (alice(), XOR, fixed!(0), AssetSymbol(b"XOR".to_vec()), 18),
            (alice(), VAL, fixed!(0), AssetSymbol(b"VAL".to_vec()), 18),
        ])
        .build();
        ext.execute_with(|| {
            MockDEXApi::init().unwrap();
            let alice = &alice();
            let _ = bonding_curve_pool_init(Vec::new()).unwrap();
            let amount = 100_u32;
            let parts = 2;

            let whole_outcome = with_transaction(|| {
                let whole_outcome = BondingCurvePool::exchange(
                    alice,
                    alice,
                    &DEXId::Polkaswap.into(),
                    &USDT,
                    &XOR,
                    SwapAmount::with_desired_output(amount.into(), Balance::max_value()),
                )
                .unwrap();
                TransactionOutcome::Rollback(whole_outcome)
            });

            let cumulative_outcome = (0..parts)
                .into_iter()
                .map(|_i| {
                    BondingCurvePool::exchange(
                        alice,
                        alice,
                        &DEXId::Polkaswap.into(),
                        &USDT,
                        &XOR,
                        SwapAmount::with_desired_output(
                            (amount / parts).into(),
                            Balance::max_value(),
                        ),
                    )
                    .unwrap()
                })
                .fold(
                    SwapOutcome::new(Balance::zero(), Balance::zero()),
                    |acc, x| SwapOutcome {
                        amount: acc.amount + x.amount,
                        fee: acc.fee + x.fee,
                    },
                );
            assert_eq!(whole_outcome, cumulative_outcome);

            let whole_outcome = with_transaction(|| {
                let whole_outcome = BondingCurvePool::exchange(
                    alice,
                    alice,
                    &DEXId::Polkaswap.into(),
                    &XOR,
                    &USDT,
                    SwapAmount::with_desired_input(cumulative_outcome.amount, Balance::zero()),
                )
                .unwrap();
                TransactionOutcome::Rollback(whole_outcome)
            });

            let cumulative_outcome = (0..parts)
                .into_iter()
                .map(|_i| {
                    BondingCurvePool::exchange(
                        alice,
                        alice,
                        &DEXId::Polkaswap.into(),
                        &XOR,
                        &USDT,
                        SwapAmount::with_desired_input(
                            cumulative_outcome.amount / Balance::from(parts),
                            Balance::zero(),
                        ),
                    )
                    .unwrap()
                })
                .fold(
                    SwapOutcome::new(Balance::zero(), Balance::zero()),
                    |acc, x| SwapOutcome {
                        amount: acc.amount + x.amount,
                        fee: acc.fee + x.fee,
                    },
                );
            assert_eq!(whole_outcome, cumulative_outcome);
        });
    }
}
