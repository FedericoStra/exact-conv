#![feature(int_bits_const)]

use exact_conv::*;

macro_rules! float_to_unsigned_int_limits {
    ($F:ty => $U:ty) => {
        let min: $F = 0 as $F;
        let max: $F = (2 as $F).powi(<$U>::BITS as i32);
        println!(
            "f{} => u{:<4}: [{:.0?},{:.0?})",
            <$F>::BITS,
            <$U>::BITS,
            min,
            max
        );
    };
}

macro_rules! float_to_signed_int_limits {
    ($F:ty => $S:ty) => {
        let max: $F = (2 as $F).powi(<$S>::BITS as i32 - 1);
        let min: $F = -max;
        println!(
            "f{} => i{:<4}: [{:.0?},{:.0?})",
            <$F>::BITS,
            <$S>::BITS,
            min,
            max
        );
    };
}

fn main() {
    println!(
        "BITS = {}  EXP_BITS = {}  SIG_BITS = {}  EXP_BIAS = {}",
        f32::BITS,
        f32::EXP_BITS,
        f32::SIG_BITS,
        f32::EXP_BIAS
    );
    println!("EXP_MASK = {:032b}", f32::EXP_MASK);
    println!("SIG_MASK = {:032b}", f32::SIG_MASK);

    println!(
        "BITS = {}  EXP_BITS = {}  SIG_BITS = {}  EXP_BIAS = {}",
        f64::BITS,
        f64::EXP_BITS,
        f64::SIG_BITS,
        f64::EXP_BIAS
    );
    println!("EXP_MASK = {:064b}", f64::EXP_MASK);
    println!("SIG_MASK = {:064b}", f64::SIG_MASK);

    println!();
    println!("Ranges float -> int");
    println!();

    float_to_unsigned_int_limits!(f32 => u8);
    float_to_unsigned_int_limits!(f32 => u16);
    float_to_unsigned_int_limits!(f32 => u32);
    float_to_unsigned_int_limits!(f32 => u64);
    float_to_unsigned_int_limits!(f32 => u128);

    float_to_signed_int_limits!(f32 => i8);
    float_to_signed_int_limits!(f32 => i16);
    float_to_signed_int_limits!(f32 => i32);
    float_to_signed_int_limits!(f32 => i64);
    float_to_signed_int_limits!(f32 => i128);
}
