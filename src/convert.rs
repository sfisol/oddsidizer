use rust_decimal::{Decimal, prelude::ToPrimitive};
use rust_decimal_macros::dec;

use crate::{
    ConversionConfig, FractionStrategy,
    lookup_tables::{
        get_american_to_decimal_map, get_american_to_fraction_map, get_decimal_to_fraction_map,
    },
};

/// Convert from american to decimal using default parameters.
pub fn american_to_decimal(value: i32) -> Result<Decimal, ConversionError> {
    american_to_decimal_custom(value, &ConversionConfig::default())
}

/// Convert from american to decimal using custom parameters.
pub fn american_to_decimal_custom(
    value: i32,
    config: &ConversionConfig,
) -> Result<Decimal, ConversionError> {
    if value == 0 {
        return Err(ConversionError::AmericanZero);
    }

    if config.use_lookup_tables
        && let Some(ret) = get_american_to_decimal_map().get(&value)
    {
        return Ok(*ret);
    }

    american_to_decimal_inner(value)
}

fn american_to_decimal_inner(value: i32) -> Result<Decimal, ConversionError> {
    let value_dec = Decimal::from(value);
    if value > 0 {
        Ok(value_dec / Decimal::ONE_HUNDRED + Decimal::ONE)
    } else if value < 0 {
        Ok(Decimal::ONE_HUNDRED / (-value_dec) + Decimal::ONE)
    } else {
        Err(ConversionError::AmericanZero)
    }
}

// Convert from fractional to decimal (doesn't use conversion parameters).
pub fn fractional_to_decimal(num: u32, den: u32) -> Result<Decimal, ConversionError> {
    if den == 0 {
        Err(ConversionError::DenominatorZero)
    } else {
        Ok(Decimal::from(num) / Decimal::from(den) + Decimal::ONE)
    }
}

// Convert from decimal to fractional using default parameters.
pub fn decimal_to_fractional(value: Decimal) -> Result<(u32, u32), ConversionError> {
    decimal_to_fractional_custom(value, &ConversionConfig::default())
}

// Convert from decimal to fractional using custom parameters.
pub fn decimal_to_fractional_custom(
    value: Decimal,
    config: &ConversionConfig,
) -> Result<(u32, u32), ConversionError> {
    if config.use_lookup_tables
        && let Some(ret) = get_decimal_to_fraction_map().get(&value)
    {
        return Ok(*ret);
    }

    match config.fraction_strategy {
        FractionStrategy::Plain => decimal_to_fractional_plain(value, config),
        FractionStrategy::Simplify => decimal_to_fractional_simplify(value),
    }
}

/// Convert from decimal to fractional with plain fractional strategy.
///
/// Bypasses look tables.
pub fn decimal_to_fractional_plain(
    value: Decimal,
    config: &ConversionConfig,
) -> Result<(u32, u32), ConversionError> {
    if value <= Decimal::ONE {
        return Err(ConversionError::InvalidDecimal);
    }

    let numerator = (value - Decimal::ONE) * Decimal::ONE_THOUSAND;
    let numerator = numerator
        .round_dp_with_strategy(0, config.rounding_strategy)
        .to_u64()
        .unwrap_or_default();

    let divisor: u64 = num_integer::gcd(numerator, 100000);

    let num = Decimal::from(numerator) / Decimal::from(divisor);
    let den = Decimal::ONE_THOUSAND / Decimal::from(divisor);

    Ok((
        num.to_u32().unwrap_or_default(),
        den.to_u32().unwrap_or_default(),
    ))
}

/// Conversion from decimal to fractional using a continued fraction algorithm to find the best rational approximation.
///
/// This usually produce simplified fractions. Bypasses look tables.
pub fn decimal_to_fractional_simplify(value: Decimal) -> Result<(u32, u32), ConversionError> {
    if value <= Decimal::ONE {
        return Err(ConversionError::InvalidDecimal);
    }

    let fractional_part = value - Decimal::ONE;

    // Set a practical limit for denominators in betting odds.
    const MAX_DENOMINATOR: u64 = 1000;

    // Epsilon for comparing decimals to handle precision errors from division.
    // Note that the epsilon is dependent on the value itself - the bigger the
    // value is, the bigger the epsilon can be to be more roundish-like,
    // 1.333 -> 1/3 (bigger epsilon)
    // 1.001 -> 1/1000 (smaller epsilon)
    let epsilon = (value - Decimal::ONE).min(dec!(0.05));

    // Standard algorithm to find best rational approximation.
    let mut a = fractional_part;
    let (mut num, mut den) = (1u64, 0u64);
    let (mut num_prev, mut den_prev) = (0u64, 1u64);

    loop {
        let a_floor = a.floor();
        // Prevent overflow if 'a' becomes unexpectedly large.
        let whole = match a_floor.to_u64() {
            Some(w) => w,
            None => break, // Value is too large, exit loop.
        };

        let num_next = whole.saturating_mul(num).saturating_add(num_prev);
        let den_next = whole.saturating_mul(den).saturating_add(den_prev);

        // Safety break for very large or complex decimals.
        if den_next > MAX_DENOMINATOR {
            break;
        }

        num_prev = num;
        den_prev = den;
        num = num_next;
        den = den_next;

        let remainder = a - a_floor;
        // If remainder is negligible, we've found our fraction.
        if remainder < epsilon {
            break;
        }

        a = Decimal::ONE / remainder;
    }

    // After the loop, `den` might be 0 if the input was an integer.
    if den == 0 {
        // If value was 2.0, fractional_part is 1.0. Loop gives num=1, den=0.
        // We should return num/1.
        den = 1;
    }

    Ok((num as u32, den as u32))
}

/// Convert from american to fractional with default parameters.
pub fn american_to_fractional(value: i32) -> Result<(u32, u32), ConversionError> {
    american_to_fractional_custom(value, &ConversionConfig::default())
}

/// Convert from american to fractional with custom parameters.
pub fn american_to_fractional_custom(
    value: i32,
    config: &ConversionConfig,
) -> Result<(u32, u32), ConversionError> {
    if config.use_lookup_tables
        && let Some(ret) = get_american_to_fraction_map().get(&value)
    {
        return Ok(*ret);
    }

    let decimal = american_to_decimal_inner(value)?;
    decimal_to_fractional(decimal)
}

/// Convert from decimal to american with default parameters.
pub fn decimal_to_american(decimal: Decimal) -> Result<i32, ConversionError> {
    decimal_to_american_custom(decimal, &ConversionConfig::default())
}

/// Convert from decimal to american with custom parameters.
pub fn decimal_to_american_custom(
    decimal: Decimal,
    config: &ConversionConfig,
) -> Result<i32, ConversionError> {
    if decimal >= Decimal::TWO {
        ((decimal - Decimal::ONE) * Decimal::ONE_HUNDRED)
            .round_dp_with_strategy(0, config.rounding_strategy)
            .to_i32()
            .ok_or(ConversionError::DecimalOverflow)
            .map(normalize_american_odds)
    } else if decimal > Decimal::ONE {
        (-Decimal::ONE_HUNDRED / (decimal - Decimal::ONE))
            .round_dp_with_strategy(0, config.rounding_strategy)
            .to_i32()
            .ok_or(ConversionError::DecimalOverflow)
    } else {
        Err(ConversionError::InvalidDecimal)
    }
}

/// Convert from fractional to american with default parameters.
pub fn fractional_to_american(num: u32, den: u32) -> Result<i32, ConversionError> {
    fractional_to_american_custom(num, den, &ConversionConfig::default())
}

/// Convert from fractional to american with custom parameters.
pub fn fractional_to_american_custom(
    num: u32,
    den: u32,
    config: &ConversionConfig,
) -> Result<i32, ConversionError> {
    if den == 0 {
        return Err(ConversionError::DenominatorZero);
    }
    let decimal = Decimal::from(num) / Decimal::from(den) + Decimal::ONE;
    decimal_to_american_custom(decimal, config)
}

/// Normalize american odds (converts 1-99 to negative values, -1-99 to positive values).
pub fn normalize_american_odds(odds: i32) -> i32 {
    if odds > 0 && odds < 100 {
        // 1-99 -> -xxx
        -((100 * 100) / odds)
    } else if odds < 0 && odds > -100 {
        // -1 to -99 -> +xxx
        (100 * 100) / (-odds)
    } else {
        odds
    }
}

#[derive(Debug, PartialEq)]
pub enum ConversionError {
    /// American odds value cannot be zero.
    AmericanZero,
    /// Denominator in fractional odds cannot be zero.
    DenominatorZero,
    /// Ran into overflow while computing decimal from or to decimal value.
    DecimalOverflow,
    /// Decimal odds cannot be less or equal 1.0
    InvalidDecimal,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::testing_helpers::assert_decimal_eq;

    use super::*;

    // --- Tests for Individual Conversion Functions ---

    #[test]
    fn test_american_to_decimal() {
        // Real-world examples (Favorites)
        assert_decimal_eq(american_to_decimal(-110).unwrap(), dec!(1.91));

        assert_decimal_eq(american_to_decimal(-150).unwrap(), dec!(1.67));

        assert_decimal_eq(
            american_to_decimal_custom(-150, ConversionConfig::default().no_lookup()).unwrap(),
            dec!(1.666),
        );

        assert_decimal_eq(american_to_decimal(-200).unwrap(), dec!(1.5));
        assert_decimal_eq(american_to_decimal(-500).unwrap(), dec!(1.2));

        // Real-world examples (Underdogs)
        assert_decimal_eq(american_to_decimal(100).unwrap(), dec!(2.0));
        assert_decimal_eq(american_to_decimal(250).unwrap(), dec!(3.5));
        assert_decimal_eq(american_to_decimal(900).unwrap(), dec!(10.0));
        assert_decimal_eq(american_to_decimal(1200).unwrap(), dec!(13.0));

        // Unrealistic / Edge cases
        assert_decimal_eq(american_to_decimal(50000).unwrap(), dec!(501.0));
        assert_decimal_eq(american_to_decimal(-50000).unwrap(), dec!(1.002));

        // Invalid cases
        assert!(american_to_decimal(0).is_err());
    }

    #[test]
    fn test_fractional_to_decimal() {
        // Real-world examples (Favorites)
        assert_decimal_eq(fractional_to_decimal(1, 2).unwrap(), dec!(1.5));
        assert_decimal_eq(fractional_to_decimal(4, 5).unwrap(), dec!(1.8));
        assert_decimal_eq(fractional_to_decimal(2, 3).unwrap(), dec!(1.666));

        // Real-world examples (Underdogs)
        assert_decimal_eq(fractional_to_decimal(1, 1).unwrap(), dec!(2.0));
        assert_decimal_eq(fractional_to_decimal(5, 2).unwrap(), dec!(3.5));
        assert_decimal_eq(fractional_to_decimal(20, 1).unwrap(), dec!(21.0));
        assert_decimal_eq(fractional_to_decimal(9, 5).unwrap(), dec!(2.8));

        // Unrealistic / Edge cases
        assert_decimal_eq(fractional_to_decimal(1000, 1).unwrap(), dec!(1001.0));
        assert_decimal_eq(fractional_to_decimal(1, 1000).unwrap(), dec!(1.001));
        assert_decimal_eq(fractional_to_decimal(0, 1).unwrap(), dec!(1.0));

        // Invalid cases
        assert!(fractional_to_decimal(10, 0).is_err());
    }

    #[test]
    fn test_decimal_to_american() {
        // Real-world examples (Favorites: > 1 and < 2)
        assert_eq!(decimal_to_american(dec!(1.5)).unwrap(), -200);
        assert_eq!(decimal_to_american(dec!(1.2)).unwrap(), -500);
        assert_eq!(decimal_to_american(dec!(1.8)).unwrap(), -125);
        assert_eq!(decimal_to_american(dec!(1.666)).unwrap(), -150);

        // Real-world examples (Underdogs: >= 2)
        assert_eq!(decimal_to_american(dec!(2.0)).unwrap(), 100);
        assert_eq!(decimal_to_american(dec!(3.5)).unwrap(), 250);
        assert_eq!(decimal_to_american(dec!(10.0)).unwrap(), 900);
        assert_eq!(decimal_to_american(dec!(21.0)).unwrap(), 2000);

        // Testing normalization path (decimal that results in american odds between -99 and 99)
        assert_eq!(decimal_to_american(dec!(1.05)).unwrap(), -2000); // 1.05 -> +5, which is normalized to -2000

        // Unrealistic / Edge cases
        assert_eq!(decimal_to_american(dec!(501.0)).unwrap(), 50000);
        assert_eq!(decimal_to_american(dec!(1.002)).unwrap(), -50000);

        // Invalid cases
        assert!(decimal_to_american(dec!(1.0)).is_err());
        assert!(decimal_to_american(dec!(0.99)).is_err());
        assert!(decimal_to_american(dec!(-5.0)).is_err());
    }

    #[test]
    fn test_fractional_to_american() {
        // Real-world examples (Favorites)
        assert_eq!(fractional_to_american(1, 2).unwrap(), -200);
        assert_eq!(fractional_to_american(2, 3).unwrap(), -150);
        assert_eq!(fractional_to_american(4, 5).unwrap(), -125);

        // Real-world examples (Underdogs)
        assert_eq!(fractional_to_american(1, 1).unwrap(), 100);
        assert_eq!(fractional_to_american(5, 2).unwrap(), 250);
        assert_eq!(fractional_to_american(9, 1).unwrap(), 900);
        assert_eq!(fractional_to_american(30, 1).unwrap(), 3000);

        // Unrealistic / Edge cases
        assert_eq!(fractional_to_american(1000, 1).unwrap(), 100000);
        assert_eq!(fractional_to_american(1, 1000).unwrap(), -100000);
        assert_eq!(fractional_to_american(1, 20).unwrap(), -2000); // tests normalization path

        // Invalid cases
        assert_eq!(
            fractional_to_american(10, 0),
            Err(ConversionError::DenominatorZero)
        );
    }

    #[test]
    fn test_american_to_fractional() {
        // Real-world examples (Favorites)
        assert_eq!(american_to_fractional(-200), Ok((1, 2)));
        assert_eq!(american_to_fractional(-500), Ok((1, 5)));

        // Traditional UK fraction
        assert_eq!(american_to_fractional(-150), Ok((4, 6)));

        // The same without lookup table
        assert_eq!(
            american_to_fractional_custom(-150, ConversionConfig::default().no_lookup()),
            Ok((2, 3))
        );

        // Real-world examples (Underdogs)
        assert_eq!(american_to_fractional(100), Ok((1, 1)));
        assert_eq!(american_to_fractional(250), Ok((5, 2)));
        assert_eq!(american_to_fractional(900), Ok((9, 1)));
        assert_eq!(american_to_fractional(1200), Ok((12, 1)));

        // Unrealistic / Edge cases
        assert_eq!(american_to_fractional(50000), Ok((500, 1)));
        assert_eq!(american_to_fractional(-110), Ok((10, 11))); // common case
        assert_eq!(american_to_fractional(-1000), Ok((1, 10)));

        // Note: american_to_fractional(0) will currently cause a panic
        // because of `unwrap_or(Decimal::ZERO)` followed by a conversion
        // that assumes a positive decimal. A robust implementation would handle this.
        assert_eq!(
            american_to_fractional(0),
            Err(ConversionError::AmericanZero)
        );
    }

    #[test]
    fn test_decimal_to_fractional() {
        // Existing tests
        assert_eq!(super::decimal_to_fractional(dec!(1.3)), Ok((3, 10)));
        assert_eq!(super::decimal_to_fractional(dec!(1.33)), Ok((1, 3)));
        assert_eq!(super::decimal_to_fractional(dec!(1.333)), Ok((1, 3)));
        assert_eq!(super::decimal_to_fractional(dec!(1.3333)), Ok((1, 3)));
        assert_eq!(super::decimal_to_fractional(dec!(1.3337)), Ok((1, 3)));
        assert_eq!(super::decimal_to_fractional(dec!(1.25)), Ok((1, 4)));
        assert_eq!(super::decimal_to_fractional(dec!(4.1)), Ok((31, 10)));
        assert_eq!(super::decimal_to_fractional(dec!(100.5)), Ok((199, 2)));

        // Gives 1/3 from lookup tables
        assert_eq!(
            super::decimal_to_fractional_custom(
                dec!(1.33),
                ConversionConfig::default().plain_fraction_strategy()
            ),
            Ok((1, 3))
        );

        // Gives 33/100 with lookup tables disabled
        assert_eq!(
            super::decimal_to_fractional_custom(
                dec!(1.33),
                ConversionConfig::default()
                    .plain_fraction_strategy()
                    .no_lookup()
            ),
            Ok((33, 100))
        );

        // No lookup for 1.333
        assert_eq!(
            super::decimal_to_fractional_custom(
                dec!(1.333),
                ConversionConfig::default().plain_fraction_strategy()
            ),
            Ok((333, 1000))
        );

        // Additional real-world cases
        assert_eq!(super::decimal_to_fractional(dec!(1.5)), Ok((1, 2)));
        assert_eq!(super::decimal_to_fractional(dec!(2.0)), Ok((1, 1)));
        assert_eq!(super::decimal_to_fractional(dec!(3.5)), Ok((5, 2)));
        assert_eq!(super::decimal_to_fractional(dec!(1.8)), Ok((4, 5)));
        assert_eq!(super::decimal_to_fractional(dec!(11.0)), Ok((10, 1)));

        // Edge cases
        assert_eq!(super::decimal_to_fractional(dec!(1.001)), Ok((1, 1000)));
        assert_eq!(
            super::decimal_to_fractional(dec!(1.0)),
            Err(ConversionError::InvalidDecimal)
        );
    }

    #[test]
    fn test_normalize_american_odds() {
        // Should not change
        assert_eq!(normalize_american_odds(100), 100);
        assert_eq!(normalize_american_odds(-100), -100);
        assert_eq!(normalize_american_odds(500), 500);
        assert_eq!(normalize_american_odds(-500), -500);
        assert_eq!(normalize_american_odds(0), 0);

        // Positive values < 100 should become negative
        assert_eq!(normalize_american_odds(50), -200);
        assert_eq!(normalize_american_odds(20), -500);
        assert_eq!(normalize_american_odds(1), -10000);
        assert_eq!(normalize_american_odds(99), -101);

        // Negative values > -100 should become positive
        assert_eq!(normalize_american_odds(-50), 200);
        assert_eq!(normalize_american_odds(-20), 500);
        assert_eq!(normalize_american_odds(-1), 10000);
        assert_eq!(normalize_american_odds(-99), 101);
    }
}
