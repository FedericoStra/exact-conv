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
    fn exact() {
        assert_eq!((3.0f32).exact_into(), Ok(3i8));
        assert_eq!((-128.0f32).exact_into(), Ok(-128i8));
        assert_eq!((127.0f32).exact_into(), Ok(127i8));
    }

    #[test]
    fn inexact() {
        assert_eq!(
            <i8 as ExactFrom<f32>>::exact_from(3.14f32),
            Err(ExactError::Inexact)
        );
        assert_eq!(i128::exact_from(f32::INFINITY), Err(ExactError::Inexact));
        assert_eq!(
            i128::exact_from(f32::NEG_INFINITY),
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
