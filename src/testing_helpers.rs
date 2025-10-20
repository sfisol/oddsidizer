use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::ConversionError;

// --- Helper for comparing Decimals in tests ---
pub fn assert_decimal_eq(a: Decimal, b: Decimal) {
    let tolerance = dec!(0.001);
    assert!(
        (a - b).abs() < tolerance,
        "Expected {} to be close to {}",
        a,
        b
    );
}

pub fn assert_decimal_ok_eq(a: Result<Decimal, ConversionError>, b: Decimal) {
    assert!(a.is_ok());
    assert_decimal_eq(a.unwrap(), b);
}
