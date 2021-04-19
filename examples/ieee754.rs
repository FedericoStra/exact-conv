use exact_conv::*;

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
}
