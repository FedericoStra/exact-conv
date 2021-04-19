#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExactError {
    Inexact,
    Overflow,
}

pub trait ExactFrom<T>: Sized {
    type Error;
    fn exact_from(value: T) -> Result<Self, Self::Error>;
}

pub trait ExactInto<T> {
    type Error;
    fn exact_into(self) -> Result<T, Self::Error>;
}

impl<T, U> ExactInto<U> for T
where
    U: ExactFrom<T>,
{
    type Error = <U as ExactFrom<T>>::Error;
    fn exact_into(self) -> Result<U, Self::Error> {
        <U as ExactFrom<T>>::exact_from(self)
    }
}

macro_rules! float_to_int {
    ($($F:ty),* => $n:literal: $U:ty, $S:ty) => {$(
        impl ExactFrom<$F> for $U {
            type Error = ExactError;
            fn exact_from(value: $F) -> Result<$U, Self::Error> {
                let min: $F = 0 as $F;
                let max: $F = (2 as $F).powi($n);
                if min <= value && value < max {
                    if value.trunc() == value {
                        Ok(value as $U)
                    } else {
                        Err(ExactError::Inexact)
                    }
                } else {
                    Err(ExactError::Overflow)
                }
            }
        }
        impl ExactFrom<$F> for $S {
            type Error = ExactError;
            fn exact_from(value: $F) -> Result<$S, Self::Error> {
                let max: $F = (2 as $F).powi($n - 1);
                let min: $F = -max;
                if min <= value && value < max {
                    if value.trunc() == value {
                        Ok(value as $S)
                    } else {
                        Err(ExactError::Inexact)
                    }
                } else {
                    Err(ExactError::Overflow)
                }
            }
        }
    )*};
}

float_to_int!(f32, f64 => 8: u8, i8);
float_to_int!(f32, f64 => 16: u16, i16);
float_to_int!(f32, f64 => 32: u32, i32);
float_to_int!(f32, f64 => 64: u64, i64);
float_to_int!(f32, f64 => 128: u128, i128);

#[cfg(target_pointer_width = "16")]
float_to_int!(f32, f64 => 16: usize, isize);

#[cfg(target_pointer_width = "32")]
float_to_int!(f32, f64 => 32: usize, isize);

#[cfg(target_pointer_width = "64")]
float_to_int!(f32, f64 => 64: usize, isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_to_unsigned() {
        assert_eq!(0_f32.exact_into(), Ok(0_u8));
        assert_eq!(0_f32.exact_into(), Ok(0_u16));
        assert_eq!(0_f32.exact_into(), Ok(0_u32));
        assert_eq!(0_f32.exact_into(), Ok(0_u64));
        assert_eq!(0_f32.exact_into(), Ok(0_u128));

        assert_eq!(255_f32.exact_into(), Ok(u8::MAX));
        assert_eq!(65535_f32.exact_into(), Ok(u16::MAX));
        assert_eq!(4294967040_f32.exact_into(), Ok(4294967040_u32));
        assert_eq!(
            18446742974197923840_f32.exact_into(),
            Ok(18446742974197923840_u64)
        );
        assert_eq!(
            340282346638528859811704183484516925440_f32.exact_into(),
            Ok(340282346638528859811704183484516925440_u128)
        );
    }

    #[test]
    fn f32_to_signed() {
        assert_eq!((-128_f32).exact_into(), Ok(i8::MIN));
        assert_eq!((-32768_f32).exact_into(), Ok(i16::MIN));
        assert_eq!((-2147483648_f32).exact_into(), Ok(-2147483648_i32));
        assert_eq!(
            (-9223372036854775808_f32).exact_into(),
            Ok(-9223372036854775808_i64)
        );
        assert_eq!(
            (-170141183460469231731687303715884105728_f32).exact_into(),
            Ok(-170141183460469231731687303715884105728_i128)
        );

        assert_eq!(127_f32.exact_into(), Ok(i8::MAX));
        assert_eq!(32767_f32.exact_into(), Ok(i16::MAX));
        assert_eq!(2147483520_f32.exact_into(), Ok(2147483520_i32));
        assert_eq!(
            9223371487098961920_f32.exact_into(),
            Ok(9223371487098961920_i64)
        );
        assert_eq!(
            170141173319264429905852091742258462720_f32.exact_into(),
            Ok(170141173319264429905852091742258462720_i128)
        );
    }

    #[test]
    fn f64_to_unsigned() {
        assert_eq!(0_f64.exact_into(), Ok(0_u8));
        assert_eq!(0_f64.exact_into(), Ok(0_u16));
        assert_eq!(0_f64.exact_into(), Ok(0_u32));
        assert_eq!(0_f64.exact_into(), Ok(0_u64));
        assert_eq!(0_f64.exact_into(), Ok(0_u128));

        assert_eq!(255_f64.exact_into(), Ok(u8::MAX));
        assert_eq!(65535_f64.exact_into(), Ok(u16::MAX));
        assert_eq!(4294967295_f64.exact_into(), Ok(u32::MAX));
        assert_eq!(
            18446744073709549568_f64.exact_into(),
            Ok(18446744073709549568_u64)
        );
        assert_eq!(
            340282366920938425684442744474606501888_f64.exact_into(),
            Ok(340282366920938425684442744474606501888_u128)
        );
    }

    #[test]
    fn f64_to_signed() {
        assert_eq!((-128_f64).exact_into(), Ok(i8::MIN));
        assert_eq!((-32768_f64).exact_into(), Ok(i16::MIN));
        assert_eq!((-2147483648_f64).exact_into(), Ok(i32::MIN));
        assert_eq!(
            (-9223372036854775808_f64).exact_into(),
            Ok(-9223372036854775808_i64)
        );
        assert_eq!(
            (-170141183460469231731687303715884105728_f64).exact_into(),
            Ok(-170141183460469231731687303715884105728_i128)
        );

        assert_eq!(127_f64.exact_into(), Ok(i8::MAX));
        assert_eq!(32767_f64.exact_into(), Ok(i16::MAX));
        assert_eq!(2147483647_f64.exact_into(), Ok(i32::MAX));
        assert_eq!(
            9223372036854774784_f64.exact_into(),
            Ok(9223372036854774784_i64)
        );
        assert_eq!(
            170141183460469212842221372237303250944_f64.exact_into(),
            Ok(170141183460469212842221372237303250944_i128)
        );
    }

    #[test]
    fn inexact() {
        assert_eq!(i8::exact_from(3.14_f32), Err(ExactError::Inexact));
    }

    #[test]
    fn f32_to_unsigned_overflow() {
        assert_eq!(u8::exact_from(-1_f32), Err(ExactError::Overflow));
        assert_eq!(u16::exact_from(-1_f32), Err(ExactError::Overflow));
        assert_eq!(u32::exact_from(-1_f32), Err(ExactError::Overflow));
        assert_eq!(u64::exact_from(-1_f32), Err(ExactError::Overflow));
        assert_eq!(u128::exact_from(-1_f32), Err(ExactError::Overflow));

        assert_eq!(u8::exact_from((2_f32).powi(8)), Err(ExactError::Overflow));
        assert_eq!(u16::exact_from((2_f32).powi(16)), Err(ExactError::Overflow));
        assert_eq!(u32::exact_from((2_f32).powi(32)), Err(ExactError::Overflow));
        assert_eq!(u64::exact_from((2_f32).powi(64)), Err(ExactError::Overflow));
        assert_eq!(
            u128::exact_from((2_f32).powi(128)),
            Err(ExactError::Overflow)
        );
    }

    #[test]
    fn f32_to_signed_overflow() {
        assert_eq!(
            i8::exact_from(-(2_f32).powi(8 - 1) - 1_f32),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i16::exact_from(-(2_f32).powi(16 - 1) - 1_f32),
            Err(ExactError::Overflow)
        );
        assert_eq!(i32::exact_from(-2147483904_f32), Err(ExactError::Overflow));
        assert_eq!(
            i64::exact_from(-9223373136366403584_f32),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i128::exact_from(-170141203742878835383357727663135391744_f32),
            Err(ExactError::Overflow)
        );

        assert_eq!(
            i8::exact_from((2_f32).powi(8 - 1)),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i16::exact_from((2_f32).powi(16 - 1)),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i32::exact_from((2_f32).powi(32 - 1)),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i64::exact_from((2_f32).powi(64 - 1)),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i128::exact_from((2_f32).powi(128 - 1)),
            Err(ExactError::Overflow)
        );
    }

    #[test]
    fn f64_to_unsigned_overflow() {
        assert_eq!(u8::exact_from(-1_f64), Err(ExactError::Overflow));
        assert_eq!(u16::exact_from(-1_f64), Err(ExactError::Overflow));
        assert_eq!(u32::exact_from(-1_f64), Err(ExactError::Overflow));
        assert_eq!(u64::exact_from(-1_f64), Err(ExactError::Overflow));
        assert_eq!(u128::exact_from(-1_f64), Err(ExactError::Overflow));

        assert_eq!(u8::exact_from((2_f64).powi(8)), Err(ExactError::Overflow));
        assert_eq!(u16::exact_from((2_f64).powi(16)), Err(ExactError::Overflow));
        assert_eq!(u32::exact_from((2_f64).powi(32)), Err(ExactError::Overflow));
        assert_eq!(u64::exact_from((2_f64).powi(64)), Err(ExactError::Overflow));
        assert_eq!(
            u128::exact_from((2_f64).powi(128)),
            Err(ExactError::Overflow)
        );
    }

    #[test]
    fn f64_to_signed_overflow() {
        assert_eq!(
            i8::exact_from(-(2_f64).powi(8 - 1) - 1_f64),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i16::exact_from(-(2_f64).powi(16 - 1) - 1_f64),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i32::exact_from(-(2_f64).powi(32 - 1) - 1_f64),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i64::exact_from(-9223372036854777856_f64),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i128::exact_from(-170141183460469269510619166673045815296_f64),
            Err(ExactError::Overflow)
        );

        assert_eq!(
            i8::exact_from((2_f64).powi(8 - 1)),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i16::exact_from((2_f64).powi(16 - 1)),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i32::exact_from((2_f64).powi(32 - 1)),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i64::exact_from((2_f64).powi(64 - 1)),
            Err(ExactError::Overflow)
        );
        assert_eq!(
            i128::exact_from((2_f64).powi(128 - 1)),
            Err(ExactError::Overflow)
        );
    }
}

pub trait IEEE754 {
    const BITS: i16;
    const EXP_BITS: i16;
    const SIG_BITS: i16;
    type Unsigned;
    const EXP_MASK: Self::Unsigned; // = ((1 << Self::EXP_BITS) - 1) << Self::SIG_BITS;
    const SIG_MASK: Self::Unsigned; // = (1 << Self::SIG_MASK) - 1;
    const EXP_BIAS: Self::Unsigned;
}

impl IEEE754 for f32 {
    const BITS: i16 = 32;
    const EXP_BITS: i16 = 8;
    const SIG_BITS: i16 = 23;
    type Unsigned = u32;
    const EXP_MASK: Self::Unsigned = ((1 << Self::EXP_BITS) - 1) << Self::SIG_BITS;
    const SIG_MASK: Self::Unsigned = (1 << Self::SIG_BITS) - 1;
    const EXP_BIAS: Self::Unsigned = (1 << (Self::EXP_BITS - 1)) - 1;
}

impl IEEE754 for f64 {
    const BITS: i16 = 64;
    const EXP_BITS: i16 = 11;
    const SIG_BITS: i16 = 52;
    type Unsigned = u64;
    const EXP_MASK: Self::Unsigned = ((1 << Self::EXP_BITS) - 1) << Self::SIG_BITS;
    const SIG_MASK: Self::Unsigned = (1 << Self::SIG_BITS) - 1;
    const EXP_BIAS: Self::Unsigned = (1 << (Self::EXP_BITS - 1)) - 1;
}
