use crate::mock::*;
use common::{
    fixed, prelude::SwapAmount, LiquidityRegistry, LiquiditySource, LiquiditySourceFilter,
    LiquiditySourceId, LiquiditySourceType, DOT, XOR,
};

#[test]
fn test_filter_empty_should_pass() {
    let mut ext = ExtBuilder::default().build();
    ext.execute_with(|| {
        let list =
            DEXAPI::list_liquidity_sources(&XOR, &DOT, LiquiditySourceFilter::empty(DEX_A_ID))
                .expect("Failed to list available sources.");
        assert_eq!(
            &list,
            &[
                LiquiditySourceId::new(DEX_A_ID, LiquiditySourceType::MockPool),
                LiquiditySourceId::new(DEX_A_ID, LiquiditySourceType::MockPool2),
                LiquiditySourceId::new(DEX_A_ID, LiquiditySourceType::MockPool3),
                LiquiditySourceId::new(DEX_A_ID, LiquiditySourceType::MockPool4),
            ]
        );
    })
}

#[test]
fn test_filter_with_forbidden_existing_should_pass() {
    let mut ext = ExtBuilder::default().build();
    ext.execute_with(|| {
        let list = DEXAPI::list_liquidity_sources(
            &XOR,
            &DOT,
            LiquiditySourceFilter::with_forbidden(
                DEX_A_ID,
                [
                    LiquiditySourceType::MockPool,
                    LiquiditySourceType::MockPool3,
                ]
                .into(),
            ),
        )
        .expect("Failed to list available sources.");
        assert_eq!(
            &list,
            &[
                LiquiditySourceId::new(DEX_A_ID, LiquiditySourceType::MockPool2),
                LiquiditySourceId::new(DEX_A_ID, LiquiditySourceType::MockPool4),
            ]
        );
    })
}

#[test]
fn test_filter_with_allowed_existing_should_pass() {
    let mut ext = ExtBuilder::default().build();
    ext.execute_with(|| {
        let list = DEXAPI::list_liquidity_sources(
            &XOR,
            &DOT,
            LiquiditySourceFilter::with_allowed(
                DEX_A_ID,
                [
                    LiquiditySourceType::MockPool,
                    LiquiditySourceType::MockPool2,
                ]
                .into(),
            ),
        )
        .expect("Failed to list available sources.");
        assert_eq!(
            &list,
            &[
                LiquiditySourceId::new(DEX_A_ID, LiquiditySourceType::MockPool),
                LiquiditySourceId::new(DEX_A_ID, LiquiditySourceType::MockPool2),
            ]
        );
    })
}

#[test]
fn test_different_reserves_should_pass() {
    let mut ext = ExtBuilder::default().build();
    ext.execute_with(|| {
        let res1 = crate::Module::<Runtime>::quote(
            &LiquiditySourceId::new(DEX_A_ID, LiquiditySourceType::MockPool),
            &XOR,
            &DOT,
            SwapAmount::with_desired_input(fixed!(100), fixed!(0)),
        );
        assert_eq!(
            res1.unwrap().amount,
            fixed!(136.851187324744592819) // for reserves: 5000 XOR, 7000 DOT, 30bp fee
        );
        let res2 = crate::Module::<Runtime>::quote(
            &LiquiditySourceId::new(DEX_A_ID, LiquiditySourceType::MockPool2),
            &XOR,
            &DOT,
            SwapAmount::with_desired_input(fixed!(100), fixed!(0)),
        );
        assert_eq!(
            res2.unwrap().amount,
            fixed!(114.415463055560109513) // for reserves: 6000 XOR, 7000 DOT, 30bp fee
        );
    })
}
