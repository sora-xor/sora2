use core::ops::{Shl, Shr};

use codec::{CompactAs, Decode, Encode};
use derive_more::From;
use fixnum::ops::{CheckedAdd, CheckedSub, Numeric, RoundMode::*, RoundingDiv, RoundingMul};
use num_traits::{CheckedNeg, Num, One, Unsigned, Zero};
#[cfg(feature = "std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sp_arithmetic::traits::{
    Bounded, CheckedDiv, CheckedMul, CheckedShl, CheckedShr, IntegerSquareRoot, Saturating,
};
use sp_runtime::FixedPointOperand;
use sp_std::convert::TryFrom;
use sp_std::fmt::Display;
use sp_std::ops::*;
use sp_std::str::FromStr;
use static_assertions::_core::fmt::Formatter;

use crate::{fixed, Amount, Fixed, FixedInner};

/// Fixed-point balance type.
#[derive(
    CompactAs, Encode, Debug, Clone, Copy, Decode, Default, From, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Balance(pub Fixed);

#[cfg(feature = "std")]
impl FromStr for Balance {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s
            .parse::<FixedInner>()
            .map_err(|_| "Parsing fixedpoint number failed.")?;
        Ok(Balance(Fixed::from_bits(val)))
    }
}

#[cfg(feature = "std")]
impl Display for Balance {
    fn fmt(&self, f: &mut Formatter<'_>) -> sp_std::fmt::Result {
        write!(f, "{}", self.0.as_bits())
    }
}

#[cfg(feature = "std")]
impl Serialize for Balance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for Balance {
    fn deserialize<D>(deserializer: D) -> Result<Balance, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Balance::from_str(&s).map_err(|str_err| serde::de::Error::custom(str_err))
    }
}

impl Rem for Balance {
    type Output = Balance;

    /// Division always occurs without a remainder.
    fn rem(self, _: Self) -> Self::Output {
        Balance::zero()
    }
}

impl Add for Balance {
    type Output = Balance;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.cadd(rhs.0).unwrap())
    }
}

impl Mul for Balance {
    type Output = Balance;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.rmul(rhs.0, Floor).unwrap())
    }
}

impl Div for Balance {
    type Output = Balance;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.rdiv(rhs.0, Floor).unwrap())
    }
}

impl Sub for Balance {
    type Output = Balance;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.csub(rhs.0).unwrap())
    }
}

impl Shl<u32> for Balance {
    type Output = Balance;

    fn shl(self, _rhs: u32) -> Self::Output {
        // TODO: implement `Shl` for `Balance`
        self
    }
}

impl Shr<u32> for Balance {
    type Output = Balance;

    fn shr(self, _rhs: u32) -> Self::Output {
        // TODO: implement `Shr` for `Balance`
        self
    }
}

impl AddAssign for Balance {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0.cadd(rhs.0).unwrap());
    }
}

impl SubAssign for Balance {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0.csub(rhs.0).unwrap());
    }
}

impl MulAssign for Balance {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self(self.0.rmul(rhs.0, Floor).unwrap());
    }
}

impl DivAssign for Balance {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self(self.0.rdiv(rhs.0, Floor).unwrap());
    }
}

impl RemAssign for Balance {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl Bounded for Balance {
    fn min_value() -> Self {
        Self(Fixed::MIN)
    }

    fn max_value() -> Self {
        Self(Fixed::MAX)
    }
}

impl Zero for Balance {
    fn zero() -> Self {
        const ZERO: Fixed = Fixed::from_bits(0);
        Self(ZERO)
    }

    fn is_zero(&self) -> bool {
        const ZERO: Fixed = Fixed::from_bits(0);
        self.0 == ZERO
    }
}

impl One for Balance {
    fn one() -> Self {
        fixed!(1)
    }

    fn is_one(&self) -> bool {
        self.0 == fixed!(1)
    }
}

impl IntegerSquareRoot for Balance {
    fn integer_sqrt_checked(&self) -> Option<Self>
    where
        Self: Sized,
    {
        // TODO: implement `IntegerSquareRoot` for `Balance`
        None
    }
}

impl sp_arithmetic::traits::CheckedAdd for Balance {
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        self.0.cadd(rhs.0).map(Self).ok()
    }
}

impl sp_arithmetic::traits::CheckedSub for Balance {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        self.0.csub(rhs.0).map(Self).ok()
    }
}

impl CheckedMul for Balance {
    fn checked_mul(&self, rhs: &Self) -> Option<Self> {
        self.0.rmul(rhs.0, Floor).map(Self).ok()
    }
}

impl CheckedDiv for Balance {
    fn checked_div(&self, rhs: &Self) -> Option<Self> {
        self.0.rdiv(rhs.0, Floor).map(Self).ok()
    }
}

impl CheckedShl for Balance {
    fn checked_shl(&self, _rhs: u32) -> Option<Self> {
        // TODO: implement `CheckedShl` for Balance
        None
    }
}

impl CheckedShr for Balance {
    fn checked_shr(&self, _rhs: u32) -> Option<Self> {
        // TODO: implement `CheckedShr` for Balance
        None
    }
}

impl Saturating for Balance {
    fn saturating_add(self, rhs: Self) -> Self {
        let lhs = *self.0.as_bits();
        let rhs = *rhs.0.as_bits();
        Fixed::from_bits(lhs.saturating_add(rhs)).into()
    }

    fn saturating_sub(self, rhs: Self) -> Self {
        let lhs = *self.0.as_bits();
        let rhs = *rhs.0.as_bits();
        Fixed::from_bits(lhs.saturating_sub(rhs)).into()
    }

    fn saturating_mul(self, rhs: Self) -> Self {
        let lhs = self.0;
        let rhs = rhs.0;
        let result = match lhs.rmul(rhs, Floor) {
            Ok(value) => value,
            Err(_) => {
                let left_is_negative = lhs < fixed!(0);
                let right_is_negative = rhs < fixed!(0);
                if left_is_negative == right_is_negative {
                    Fixed::MAX
                } else {
                    Fixed::MIN
                }
            }
        };
        result.into()
    }

    fn saturating_pow(self, exp: usize) -> Self {
        let lhs = *self.0.as_bits();
        Fixed::from_bits(lhs.saturating_pow(exp as u32)).into()
    }
}

impl Num for Balance {
    type FromStrRadixErr = ();

    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        // TODO: implement `Num` for `Balance`
        Err(())
    }
}

impl Unsigned for Balance {}

macro_rules! impl_primitive_conversions {
    ($( $t:ty ),+) => ($(
        impl_primitive_conversions!{@single $t}
    )*);
    (@single $t:ty) => {
        impl From<$t> for Balance {
            fn from(value: $t) -> Self {
                Fixed::try_from(value).unwrap().into()
            }
        }

        impl From<Balance> for $t {
            fn from(value: Balance) -> Self {
                value.0.rounding_to_i64() as Self
            }
        }
    };
}

impl_primitive_conversions!(u8, u16, u32, u64, u128);

impl From<usize> for Balance {
    fn from(v: usize) -> Balance {
        Balance(Fixed::try_from(v as FixedInner).unwrap())
    }
}

impl From<Balance> for usize {
    fn from(value: Balance) -> usize {
        *value.0.as_bits() as usize
    }
}

impl From<Balance> for Fixed {
    fn from(value: Balance) -> Fixed {
        value.0
    }
}

impl Into<Amount> for Balance {
    fn into(self) -> Amount {
        <Self as Into<u64>>::into(self) as i128
    }
}

impl TryFrom<Amount> for Balance {
    type Error = ();

    fn try_from(amount: Amount) -> Result<Self, Self::Error> {
        if amount < 0 {
            Err(())
        } else {
            Ok(Self::from(amount as u128))
        }
    }
}

impl CheckedNeg for Balance {
    fn checked_neg(&self) -> Option<Self> {
        None
    }
}

impl FixedPointOperand for Balance {}

#[cfg(test)]
mod tests {
    use super::Balance;
    use crate::fixed;
    use codec::CompactAs;
    use num_traits::Bounded;
    use num_traits::One;

    #[test]
    fn balance_encode_as_should_equal_fixed_inner() {
        let balance = Balance::one();
        assert_eq!(&balance.0, balance.encode_as());
    }

    #[test]
    fn should_serialize_and_deserialize_balance_properly_with_string() {
        let test_for_number = |value: Balance, json_str: &str| {
            assert_eq!(serde_json::to_string(&value).unwrap(), json_str);
            assert_eq!(serde_json::from_str::<Balance>(json_str).unwrap(), value);
            // should not panic
            serde_json::to_value(&value).unwrap();
        };
        test_for_number(fixed!(1.5), r#""1500000000000000000""#);
        test_for_number(
            Balance::from(1u32) / Balance::from(1_000_000_000_000_000_000u128),
            r#""1""#,
        );
        test_for_number(
            Balance::from(1u32) / Balance::from(10_000_000_000_000_000_000u128),
            r#""0""#,
        );
        test_for_number(
            Balance::max_value(),
            r#""170141183460469231731687303715884105727""#,
        );
        test_for_number(Balance::from(10u32), r#""10000000000000000000""#)
    }
}
