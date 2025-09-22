use std::collections::HashMap;
use std::sync::OnceLock;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

// Accessor for DECIMAL_TO_FRACTION Map
static DECIMAL_TO_FRACTION: OnceLock<HashMap<Decimal, (u32, u32)>> = OnceLock::new();

/// Lookup table for conversion from decimal to fractional.
pub fn get_decimal_to_fraction_map() -> &'static HashMap<Decimal, (u32, u32)> {
    DECIMAL_TO_FRACTION.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(dec!(1.01), (1, 100));
        m.insert(dec!(1.2), (1, 5));
        m.insert(dec!(1.22), (2, 9));
        m.insert(dec!(1.25), (1, 4));
        m.insert(dec!(1.29), (2, 7));
        m.insert(dec!(1.3), (3, 10));
        m.insert(dec!(1.33), (1, 3));
        m.insert(dec!(1.36), (4, 11));
        m.insert(dec!(1.4), (2, 5));
        m.insert(dec!(1.44), (4, 9));
        m.insert(dec!(1.5), (1, 2));
        m.insert(dec!(1.53), (8, 15));
        m.insert(dec!(1.57), (4, 7));
        m.insert(dec!(1.62), (8, 13));
        m.insert(dec!(1.67), (4, 6));
        m.insert(dec!(1.73), (8, 11));
        m.insert(dec!(1.8), (4, 5));
        m.insert(dec!(1.83), (5, 6));
        m.insert(dec!(1.91), (10, 11));
        m.insert(dec!(2), (1, 1));
        m.insert(dec!(2.05), (21, 20));
        m.insert(dec!(2.1), (11, 10));
        m.insert(dec!(2.15), (23, 20));
        m.insert(dec!(2.2), (6, 5));
        m.insert(dec!(2.25), (5, 4));
        m.insert(dec!(2.38), (11, 8));
        m.insert(dec!(2.4), (7, 5));
        m.insert(dec!(2.5), (6, 4));
        m.insert(dec!(2.6), (8, 5));
        m.insert(dec!(2.62), (13, 8));
        m.insert(dec!(2.75), (7, 4));
        m.insert(dec!(2.8), (9, 5));
        m.insert(dec!(2.88), (15, 8));
        m.insert(dec!(3), (2, 1));
        m.insert(dec!(3.2), (11, 5));
        m.insert(dec!(3.25), (9, 4));
        m.insert(dec!(3.4), (12, 5));
        m.insert(dec!(3.5), (5, 2));
        m.insert(dec!(3.6), (13, 5));
        m.insert(dec!(3.75), (11, 4));
        m.insert(dec!(4), (3, 1));
        m.insert(dec!(4.2), (16, 5));
        m.insert(dec!(4.33), (10, 3));
        m.insert(dec!(4.5), (7, 2));
        m.insert(dec!(5), (4, 1));
        m.insert(dec!(5.5), (9, 2));
        m.insert(dec!(6), (5, 1));
        m.insert(dec!(6.5), (11, 2));
        m.insert(dec!(7), (6, 1));
        m.insert(dec!(7.5), (13, 2));
        m.insert(dec!(8), (7, 1));
        m.insert(dec!(8.5), (15, 2));
        m.insert(dec!(9), (8, 1));
        m.insert(dec!(10), (9, 1));
        m.insert(dec!(11), (10, 1));
        m.insert(dec!(12), (11, 1));
        m.insert(dec!(13), (12, 1));
        m.insert(dec!(14), (13, 1));
        m.insert(dec!(15), (14, 1));
        m.insert(dec!(16), (15, 1));
        m.insert(dec!(17), (16, 1));
        m.insert(dec!(19), (18, 1));
        m.insert(dec!(21), (20, 1));
        m.insert(dec!(26), (25, 1));
        m.insert(dec!(34), (33, 1));
        m.insert(dec!(51), (50, 1));
        m.insert(dec!(67), (66, 1));
        m.insert(dec!(101), (100, 1));
        m.insert(dec!(1001), (1000, 1));
        m
    })
}

// Accessor for AMERICAN_TO_FRACTION Map
static AMERICAN_TO_FRACTION: OnceLock<HashMap<i32, (u32, u32)>> = OnceLock::new();

/// Lookup table for conversion from american to fractional.
pub fn get_american_to_fraction_map() -> &'static HashMap<i32, (u32, u32)> {
    AMERICAN_TO_FRACTION.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(-10000, (1, 100));
        m.insert(-500, (1, 5));
        m.insert(-450, (2, 9));
        m.insert(-400, (1, 4));
        m.insert(-350, (2, 7));
        m.insert(-333, (3, 10));
        m.insert(-300, (1, 3));
        m.insert(-275, (4, 11));
        m.insert(-250, (2, 5));
        m.insert(-225, (4, 9));
        m.insert(-200, (1, 2));
        m.insert(-188, (8, 15));
        m.insert(-175, (4, 7));
        m.insert(-163, (8, 13));
        m.insert(-150, (4, 6));
        m.insert(-138, (8, 11));
        m.insert(-125, (4, 5));
        m.insert(-120, (5, 6));
        m.insert(-110, (10, 11));
        m.insert(100, (1, 1));
        m.insert(105, (21, 20));
        m.insert(110, (11, 10));
        m.insert(115, (23, 20));
        m.insert(120, (6, 5));
        m.insert(125, (5, 4));
        m.insert(138, (11, 8));
        m.insert(140, (7, 5));
        m.insert(150, (6, 4));
        m.insert(160, (8, 5));
        m.insert(163, (13, 8));
        m.insert(175, (7, 4));
        m.insert(180, (9, 5));
        m.insert(188, (15, 8));
        m.insert(200, (2, 1));
        m.insert(220, (11, 5));
        m.insert(225, (9, 4));
        m.insert(240, (12, 5));
        m.insert(250, (5, 2));
        m.insert(260, (13, 5));
        m.insert(275, (11, 4));
        m.insert(300, (3, 1));
        m.insert(320, (16, 5));
        m.insert(333, (10, 3));
        m.insert(350, (7, 2));
        m.insert(400, (4, 1));
        m.insert(450, (9, 2));
        m.insert(500, (5, 1));
        m.insert(550, (11, 2));
        m.insert(600, (6, 1));
        m.insert(650, (13, 2));
        m.insert(700, (7, 1));
        m.insert(750, (15, 2));
        m.insert(800, (8, 1));
        m.insert(900, (9, 1));
        m.insert(1000, (10, 1));
        m.insert(1100, (11, 1));
        m.insert(1200, (12, 1));
        m.insert(1300, (13, 1));
        m.insert(1400, (14, 1));
        m.insert(1500, (15, 1));
        m.insert(1600, (16, 1));
        m.insert(1800, (18, 1));
        m.insert(2000, (20, 1));
        m.insert(2500, (25, 1));
        m.insert(3300, (33, 1));
        m.insert(5000, (50, 1));
        m.insert(6600, (66, 1));
        m.insert(10000, (100, 1));
        m.insert(100000, (1000, 1));
        m
    })
}

// Accessor for AMERICAN_TO_DECIMAL Map
static AMERICAN_TO_DECIMAL: OnceLock<HashMap<i32, Decimal>> = OnceLock::new();

/// Lookup table for conversion from american to decimal.
pub fn get_american_to_decimal_map() -> &'static HashMap<i32, Decimal> {
    AMERICAN_TO_DECIMAL.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(-10000, dec!(1.01));
        m.insert(-500, dec!(1.2));
        m.insert(-450, dec!(1.22));
        m.insert(-400, dec!(1.25));
        m.insert(-350, dec!(1.29));
        m.insert(-333, dec!(1.3));
        m.insert(-300, dec!(1.33));
        m.insert(-275, dec!(1.36));
        m.insert(-250, dec!(1.4));
        m.insert(-225, dec!(1.44));
        m.insert(-200, dec!(1.5));
        m.insert(-188, dec!(1.53));
        m.insert(-175, dec!(1.57));
        m.insert(-163, dec!(1.62));
        m.insert(-150, dec!(1.67));
        m.insert(-138, dec!(1.73));
        m.insert(-125, dec!(1.8));
        m.insert(-120, dec!(1.83));
        m.insert(-110, dec!(1.91));
        m.insert(100, dec!(2));
        m.insert(105, dec!(2.05));
        m.insert(110, dec!(2.1));
        m.insert(115, dec!(2.15));
        m.insert(120, dec!(2.2));
        m.insert(125, dec!(2.25));
        m.insert(138, dec!(2.38));
        m.insert(140, dec!(2.4));
        m.insert(150, dec!(2.5));
        m.insert(160, dec!(2.6));
        m.insert(163, dec!(2.62));
        m.insert(175, dec!(2.75));
        m.insert(180, dec!(2.8));
        m.insert(188, dec!(2.88));
        m.insert(200, dec!(3));
        m.insert(220, dec!(3.2));
        m.insert(225, dec!(3.25));
        m.insert(240, dec!(3.4));
        m.insert(250, dec!(3.5));
        m.insert(260, dec!(3.6));
        m.insert(275, dec!(3.75));
        m.insert(300, dec!(4));
        m.insert(320, dec!(4.2));
        m.insert(333, dec!(4.33));
        m.insert(350, dec!(4.5));
        m.insert(400, dec!(5));
        m.insert(450, dec!(5.5));
        m.insert(500, dec!(6));
        m.insert(550, dec!(6.5));
        m.insert(600, dec!(7));
        m.insert(650, dec!(7.5));
        m.insert(700, dec!(8));
        m.insert(750, dec!(8.5));
        m.insert(800, dec!(9));
        m.insert(900, dec!(10));
        m.insert(1000, dec!(11));
        m.insert(1100, dec!(12));
        m.insert(1200, dec!(13));
        m.insert(1300, dec!(14));
        m.insert(1400, dec!(15));
        m.insert(1500, dec!(16));
        m.insert(1600, dec!(17));
        m.insert(1800, dec!(19));
        m.insert(2000, dec!(21));
        m.insert(2500, dec!(26));
        m.insert(3300, dec!(34));
        m.insert(5000, dec!(51));
        m.insert(6600, dec!(67));
        m.insert(10000, dec!(101));
        m.insert(100000, dec!(1001));
        m
    })
}
