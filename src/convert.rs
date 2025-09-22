use rust_decimal::{Decimal, prelude::ToPrimitive};
use rust_decimal_macros::dec;

use crate::{
    ConversionConfig, FractionStrategy, LookupVariant,
    lookup_tables::{
        get_american_to_decimal_extended_map, get_american_to_decimal_map,
        get_american_to_fraction_extended_map, get_american_to_fraction_map,
        get_decimal_to_fraction_extended_map, get_decimal_to_fraction_map,
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

    match config.lookup_tables_variant {
        LookupVariant::Basic => {
            if let Some(ret) = get_american_to_decimal_map().get(&value) {
                return Ok(*ret);
            }
        }
        LookupVariant::Extended => {
            if let Some(ret) = get_american_to_decimal_map()
                .get(&value)
                .or(get_american_to_decimal_extended_map().get(&value))
            {
                return Ok(*ret);
            }
        }
        _ => (),
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
    match config.lookup_tables_variant {
        LookupVariant::Basic => {
            if let Some(ret) = get_decimal_to_fraction_map().get(&value) {
                return Ok(*ret);
            }
        }
        LookupVariant::Extended => {
            if let Some(ret) = get_decimal_to_fraction_map()
                .get(&value)
                .or(get_decimal_to_fraction_extended_map().get(&value))
            {
                return Ok(*ret);
            }
        }
        _ => (),
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
    let epsilon = (value - Decimal::ONE).min(dec!(0.01));

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
    match config.lookup_tables_variant {
        LookupVariant::Basic => {
            if let Some(ret) = get_american_to_fraction_map().get(&value) {
                return Ok(*ret);
            }
        }
        LookupVariant::Extended => {
            if let Some(ret) = get_american_to_fraction_map()
                .get(&value)
                .or(get_american_to_fraction_extended_map().get(&value))
            {
                return Ok(*ret);
            }
        }
        _ => (),
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
            american_to_decimal_custom(-150, &ConversionConfig::default().no_lookup()).unwrap(),
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
            american_to_fractional_custom(-150, &ConversionConfig::default().no_lookup()),
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
                &ConversionConfig::default().plain_fraction_strategy()
            ),
            Ok((1, 3))
        );

        // Gives 33/100 with lookup tables disabled
        assert_eq!(
            super::decimal_to_fractional_custom(
                dec!(1.33),
                &ConversionConfig::default()
                    .plain_fraction_strategy()
                    .no_lookup()
            ),
            Ok((33, 100))
        );

        // No lookup for 1.333
        assert_eq!(
            super::decimal_to_fractional_custom(
                dec!(1.333),
                &ConversionConfig::default().plain_fraction_strategy()
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

    #[test]
    #[rustfmt::skip]
    fn extended_lookup_test() {
        let config = &ConversionConfig::default().extended_lookup();
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0010), config), Ok((1, 1000)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0013), config), Ok((1, 750)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0020), config), Ok((1, 500)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0025), config), Ok((1, 400)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0030), config), Ok((1, 300)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0040), config), Ok((1, 250)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0050), config), Ok((1, 200)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0070), config), Ok((1, 150)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0100), config), Ok((1, 100)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0120), config), Ok((1, 80)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0150), config), Ok((1, 66)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0200), config), Ok((1, 50)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0250), config), Ok((1, 40)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0300), config), Ok((1, 33)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0400), config), Ok((1, 25)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0500), config), Ok((1, 20)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0550), config), Ok((1, 18)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0600), config), Ok((1, 16)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0700), config), Ok((1, 14)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0800), config), Ok((1, 12)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.0900), config), Ok((1, 11)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.1000), config), Ok((1, 10)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.1100), config), Ok((1, 9)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.1200), config), Ok((1, 8)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.1300), config), Ok((2, 15)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.1400), config), Ok((1, 7)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.1500), config), Ok((2, 13)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.1600), config), Ok((1, 6)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.1800), config), Ok((2, 11))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.1900), config), Ok((19, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.2000), config), Ok((1, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.2100), config), Ok((21, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.2200), config), Ok((2, 9)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.2300), config), Ok((23, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.2400), config), Ok((6, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.2500), config), Ok((1, 4)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.2600), config), Ok((13, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.2700), config), Ok((27, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.2900), config), Ok((2, 7)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.3000), config), Ok((3, 10)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.3100), config), Ok((31, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.3200), config), Ok((8, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.3300), config), Ok((1, 3)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.3400), config), Ok((17, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.3500), config), Ok((7, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.3600), config), Ok((4, 11)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.3700), config), Ok((37, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.3800), config), Ok((19, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.3900), config), Ok((39, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.4000), config), Ok((2, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.4100), config), Ok((41, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.4200), config), Ok((21, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.4300), config), Ok((43, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.4400), config), Ok((4, 9)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.4500), config), Ok((9, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.4600), config), Ok((23, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.4700), config), Ok((40, 85)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.4800), config), Ok((12, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.4900), config), Ok((49, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.5000), config), Ok((1, 2)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.5100), config), Ok((51, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.5200), config), Ok((13, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.5300), config), Ok((8, 15)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.5400), config), Ok((27, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.5500), config), Ok((11, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.5600), config), Ok((14, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.5700), config), Ok((4, 7)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.5800), config), Ok((29, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.5900), config), Ok((59, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.6000), config), Ok((3, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.6100), config), Ok((8, 13)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.6300), config), Ok((63, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.6400), config), Ok((16, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.6500), config), Ok((13, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.6600), config), Ok((4, 6)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.6800), config), Ok((34, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.6900), config), Ok((69, 100)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.7000), config), Ok((7, 10))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.7100), config), Ok((71, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.7200), config), Ok((8, 11)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.7400), config), Ok((37, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.7500), config), Ok((3, 4)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.7600), config), Ok((19, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.7700), config), Ok((77, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.7800), config), Ok((39, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.7900), config), Ok((79, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.8000), config), Ok((4, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.8100), config), Ok((81, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.8200), config), Ok((41, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.8300), config), Ok((5, 6)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.8400), config), Ok((21, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.8500), config), Ok((17, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.8600), config), Ok((20, 23))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.8700), config), Ok((87, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.8800), config), Ok((22, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.8900), config), Ok((89, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.9000), config), Ok((9, 10)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.9100), config), Ok((10, 11)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.9200), config), Ok((23, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.9300), config), Ok((93, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.9400), config), Ok((47, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.9500), config), Ok((20, 21)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.9600), config), Ok((24, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.9700), config), Ok((97, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.9800), config), Ok((49, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(1.9900), config), Ok((99, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.0000), config), Ok((1, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.0100), config), Ok((101, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.0200), config), Ok((51, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.0300), config), Ok((103, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.0400), config), Ok((26, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.0500), config), Ok((21, 20)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.0600), config), Ok((53, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.0700), config), Ok((107, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.0800), config), Ok((27, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.0900), config), Ok((109, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.1000), config), Ok((11, 10)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.1100), config), Ok((111, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.1200), config), Ok((28, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.1300), config), Ok((113, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.1400), config), Ok((57, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.1500), config), Ok((23, 20)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.1600), config), Ok((29, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.1700), config), Ok((117, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.1800), config), Ok((59, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.1900), config), Ok((119, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.2000), config), Ok((6, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.2100), config), Ok((121, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.2200), config), Ok((61, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.2300), config), Ok((123, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.2400), config), Ok((31, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.2500), config), Ok((5, 4)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.2600), config), Ok((63, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.2700), config), Ok((127, 100))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.2800), config), Ok((32, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.3000), config), Ok((13, 10)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.3200), config), Ok((33, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.3400), config), Ok((67, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.3500), config), Ok((27, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.3600), config), Ok((34, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.3700), config), Ok((11, 8)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.4000), config), Ok((7, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.4200), config), Ok((71, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.4400), config), Ok((36, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.4500), config), Ok((29, 20)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.4600), config), Ok((73, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.4800), config), Ok((37, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.5000), config), Ok((6, 4)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.5200), config), Ok((38, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.5400), config), Ok((77, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.5600), config), Ok((39, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.5800), config), Ok((79, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.6000), config), Ok((8, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.6200), config), Ok((13, 8)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.6400), config), Ok((41, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.6600), config), Ok((83, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.6800), config), Ok((42, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.7000), config), Ok((17, 10)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.7200), config), Ok((43, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.7400), config), Ok((87, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.7500), config), Ok((7, 4)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.7600), config), Ok((44, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.7800), config), Ok((89, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.8000), config), Ok((9, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.8200), config), Ok((91, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.8400), config), Ok((46, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.8600), config), Ok((93, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.8700), config), Ok((15, 8))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.8800), config), Ok((15, 8))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.9000), config), Ok((19, 10)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.9200), config), Ok((48, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.9400), config), Ok((97, 50)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.9600), config), Ok((49, 25))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(2.9800), config), Ok((99, 50))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.0000), config), Ok((2, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.0500), config), Ok((41, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.1000), config), Ok((21, 10)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.1250), config), Ok((85, 40)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.1500), config), Ok((43, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.2000), config), Ok((11, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.2500), config), Ok((9, 4)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.3000), config), Ok((23, 10)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.3500), config), Ok((47, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.4000), config), Ok((12, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.4500), config), Ok((49, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.5000), config), Ok((5, 2)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.5500), config), Ok((51, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.6000), config), Ok((13, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.6500), config), Ok((53, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.7000), config), Ok((27, 10))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.7500), config), Ok((11, 4)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.8000), config), Ok((14, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.8500), config), Ok((57, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(3.9500), config), Ok((59, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.0000), config), Ok((3, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.0500), config), Ok((61, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.1000), config), Ok((31, 10)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.1500), config), Ok((63, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.2000), config), Ok((16, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.2500), config), Ok((13, 4))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.3000), config), Ok((33, 10))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.3300), config), Ok((10, 3)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.3500), config), Ok((67, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.4000), config), Ok((17, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.4500), config), Ok((69, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.5000), config), Ok((7, 2)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.5500), config), Ok((71, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.6000), config), Ok((18, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.6500), config), Ok((73, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.7000), config), Ok((37, 10))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.7500), config), Ok((15, 4)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.8000), config), Ok((19, 5)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.8500), config), Ok((77, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.9000), config), Ok((39, 10))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(4.9500), config), Ok((79, 20))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(5.0000), config), Ok((4, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(5.1000), config), Ok((41, 10))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(5.2000), config), Ok((21, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(5.3000), config), Ok((43, 10))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(5.4000), config), Ok((22, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(5.5000), config), Ok((9, 2)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(5.6000), config), Ok((23, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(5.7000), config), Ok((47, 10))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(5.8000), config), Ok((24, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(5.9000), config), Ok((49, 10))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(6.0000), config), Ok((5, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(6.2000), config), Ok((26, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(6.4000), config), Ok((27, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(6.5000), config), Ok((11, 2)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(6.6000), config), Ok((28, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(6.8000), config), Ok((29, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(7.0000), config), Ok((6, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(7.2000), config), Ok((31, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(7.4000), config), Ok((32, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(7.5000), config), Ok((13, 2)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(7.6000), config), Ok((33, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(7.8000), config), Ok((34, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(8.0000), config), Ok((7, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(8.2000), config), Ok((36, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(8.4000), config), Ok((37, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(8.5000), config), Ok((15, 2)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(8.6000), config), Ok((38, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(8.8000), config), Ok((39, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(9.0000), config), Ok((8, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(9.2000), config), Ok((41, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(9.4000), config), Ok((42, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(9.5000), config), Ok((17, 2)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(9.6000), config), Ok((43, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(9.8000), config), Ok((44, 5))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(10.0000), config), Ok((9, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(11.0000), config), Ok((10, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(12.0000), config), Ok((11, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(13.0000), config), Ok((12, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(14.0000), config), Ok((13, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(15.0000), config), Ok((14, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(16.0000), config), Ok((15, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(17.0000), config), Ok((16, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(19.0000), config), Ok((18, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(21.0000), config), Ok((20, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(23.0000), config), Ok((22, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(26.0000), config), Ok((25, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(29.0000), config), Ok((28, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(31.0000), config), Ok((30, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(34.0000), config), Ok((33, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(36.0000), config), Ok((35, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(41.0000), config), Ok((40, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(46.0000), config), Ok((45, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(51.0000), config), Ok((50, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(56.0000), config), Ok((55, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(61.0000), config), Ok((60, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(67.0000), config), Ok((66, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(71.0000), config), Ok((70, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(76.0000), config), Ok((75, 1))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(81.0000), config), Ok((80, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(86.0000), config), Ok((85, 1))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(91.0000), config), Ok((90, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(96.0000), config), Ok((95, 1))); //disabled
        assert_eq!(super::decimal_to_fractional_custom(dec!(101.0000), config), Ok((100, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(111.0000), config), Ok((110, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(121.0000), config), Ok((120, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(126.0000), config), Ok((125, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(131.0000), config), Ok((130, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(141.0000), config), Ok((140, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(151.0000), config), Ok((150, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(176.0000), config), Ok((175, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(201.0000), config), Ok((200, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(226.0000), config), Ok((225, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(251.0000), config), Ok((250, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(276.0000), config), Ok((275, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(301.0000), config), Ok((300, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(401.0000), config), Ok((400, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(501.0000), config), Ok((500, 1)));
        assert_eq!(super::decimal_to_fractional_custom(dec!(1001.0000), config), Ok((1000, 1)));
    }
}
