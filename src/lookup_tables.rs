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

// Accessor for DECIMAL_TO_FRACTION extended Map
static DECIMAL_TO_FRACTION_EXTENDED: OnceLock<HashMap<Decimal, (u32, u32)>> = OnceLock::new();

/// Extended lookup table for conversion from decimal to fractional.
pub fn get_decimal_to_fraction_extended_map() -> &'static HashMap<Decimal, (u32, u32)> {
    DECIMAL_TO_FRACTION_EXTENDED.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(dec!(1.0010), (1, 1000));
        m.insert(dec!(1.0013), (1, 750));
        m.insert(dec!(1.0020), (1, 500));
        m.insert(dec!(1.0025), (1, 400));
        m.insert(dec!(1.0030), (1, 300));
        m.insert(dec!(1.0040), (1, 250));
        m.insert(dec!(1.0050), (1, 200));
        m.insert(dec!(1.0070), (1, 150));
        m.insert(dec!(1.0120), (1, 80));
        m.insert(dec!(1.0150), (1, 66));
        m.insert(dec!(1.0200), (1, 50));
        m.insert(dec!(1.0250), (1, 40));
        m.insert(dec!(1.0300), (1, 33));
        m.insert(dec!(1.0400), (1, 25));
        m.insert(dec!(1.0500), (1, 20));
        m.insert(dec!(1.0550), (1, 18));
        m.insert(dec!(1.0600), (1, 16));
        m.insert(dec!(1.0700), (1, 14));
        m.insert(dec!(1.0800), (1, 12));
        m.insert(dec!(1.0900), (1, 11));
        m.insert(dec!(1.1000), (1, 10));
        m.insert(dec!(1.1100), (1, 9));
        m.insert(dec!(1.1200), (1, 8));
        m.insert(dec!(1.1300), (2, 15));
        m.insert(dec!(1.1400), (1, 7));
        m.insert(dec!(1.1500), (2, 13));
        m.insert(dec!(1.1600), (1, 6));
        m.insert(dec!(1.1800), (2, 11)); // D
        m.insert(dec!(1.1900), (19, 100)); // D
        m.insert(dec!(1.2100), (21, 100)); // D
        m.insert(dec!(1.2300), (23, 100)); // D
        m.insert(dec!(1.2400), (6, 25)); // D
        m.insert(dec!(1.2600), (13, 50)); // D
        m.insert(dec!(1.2700), (27, 100)); // D
        m.insert(dec!(1.3100), (31, 100)); // D
        m.insert(dec!(1.3200), (8, 25)); // D
        m.insert(dec!(1.3400), (17, 50)); // D
        m.insert(dec!(1.3500), (7, 20)); // D
        m.insert(dec!(1.3700), (37, 100)); // D
        m.insert(dec!(1.3800), (19, 50)); // D
        m.insert(dec!(1.3900), (39, 100)); // D
        m.insert(dec!(1.4100), (41, 100)); // D
        m.insert(dec!(1.4200), (21, 50)); // D
        m.insert(dec!(1.4300), (43, 100)); // D
        m.insert(dec!(1.4500), (9, 20)); // D
        m.insert(dec!(1.4600), (23, 50)); // D
        m.insert(dec!(1.4700), (40, 85));
        m.insert(dec!(1.4800), (12, 25)); // D
        m.insert(dec!(1.4900), (49, 100)); // D
        m.insert(dec!(1.5100), (51, 100)); // D
        m.insert(dec!(1.5200), (13, 25)); // D
        m.insert(dec!(1.5400), (27, 50)); // D
        m.insert(dec!(1.5500), (11, 20)); // D
        m.insert(dec!(1.5600), (14, 25)); // D
        m.insert(dec!(1.5800), (29, 50)); // D
        m.insert(dec!(1.5900), (59, 100)); // D
        m.insert(dec!(1.6000), (3, 5)); // D
        m.insert(dec!(1.6100), (8, 13));
        m.insert(dec!(1.6300), (63, 100)); // D
        m.insert(dec!(1.6400), (16, 25)); // D
        m.insert(dec!(1.6500), (13, 20)); // D
        m.insert(dec!(1.6600), (4, 6));
        m.insert(dec!(1.6800), (34, 50)); // D
        m.insert(dec!(1.6900), (69, 100));
        m.insert(dec!(1.7000), (7, 10)); // D
        m.insert(dec!(1.7100), (71, 100)); // D
        m.insert(dec!(1.7200), (8, 11));
        m.insert(dec!(1.7400), (37, 50)); // D
        m.insert(dec!(1.7500), (3, 4));
        m.insert(dec!(1.7600), (19, 25)); // D
        m.insert(dec!(1.7700), (77, 100)); // D
        m.insert(dec!(1.7800), (39, 50)); // D
        m.insert(dec!(1.7900), (79, 100)); // D
        m.insert(dec!(1.8100), (81, 100)); // D
        m.insert(dec!(1.8200), (41, 50)); // D
        m.insert(dec!(1.8400), (21, 25)); // D
        m.insert(dec!(1.8500), (17, 20)); // D
        m.insert(dec!(1.8600), (20, 23)); // D
        m.insert(dec!(1.8700), (87, 100)); // D
        m.insert(dec!(1.8800), (22, 25)); // D
        m.insert(dec!(1.8900), (89, 100)); // D
        m.insert(dec!(1.9000), (9, 10));
        m.insert(dec!(1.9200), (23, 25)); // D
        m.insert(dec!(1.9300), (93, 100)); // D
        m.insert(dec!(1.9400), (47, 50)); // D
        m.insert(dec!(1.9500), (20, 21));
        m.insert(dec!(1.9600), (24, 25)); // D
        m.insert(dec!(1.9700), (97, 100)); // D
        m.insert(dec!(1.9800), (49, 50)); // D
        m.insert(dec!(1.9900), (99, 100)); // D
        m.insert(dec!(2.0100), (101, 100)); // D
        m.insert(dec!(2.0200), (51, 50)); // D
        m.insert(dec!(2.0300), (103, 100)); // D
        m.insert(dec!(2.0400), (26, 25)); // D
        m.insert(dec!(2.0600), (53, 50)); // D
        m.insert(dec!(2.0700), (107, 100)); // D
        m.insert(dec!(2.0800), (27, 25)); // D
        m.insert(dec!(2.0900), (109, 100)); // D
        m.insert(dec!(2.1100), (111, 100)); // D
        m.insert(dec!(2.1200), (28, 25)); // D
        m.insert(dec!(2.1300), (113, 100)); // D
        m.insert(dec!(2.1400), (57, 50)); // D
        m.insert(dec!(2.1600), (29, 25)); // D
        m.insert(dec!(2.1700), (117, 100)); // D
        m.insert(dec!(2.1800), (59, 50)); // D
        m.insert(dec!(2.1900), (119, 100)); // D
        m.insert(dec!(2.2100), (121, 100)); // D
        m.insert(dec!(2.2200), (61, 50)); // D
        m.insert(dec!(2.2300), (123, 100)); // D
        m.insert(dec!(2.2400), (31, 25)); // D
        m.insert(dec!(2.2600), (63, 50)); // D
        m.insert(dec!(2.2700), (127, 100)); // D
        m.insert(dec!(2.2800), (32, 25)); // D
        m.insert(dec!(2.3000), (13, 10));
        m.insert(dec!(2.3200), (33, 25)); // D
        m.insert(dec!(2.3400), (67, 50)); // D
        m.insert(dec!(2.3500), (27, 20)); // D
        m.insert(dec!(2.3600), (34, 25)); // D
        m.insert(dec!(2.3700), (11, 8));
        m.insert(dec!(2.4200), (71, 50)); // D
        m.insert(dec!(2.4400), (36, 25)); // D
        m.insert(dec!(2.4500), (29, 20));
        m.insert(dec!(2.4600), (73, 50)); // D
        m.insert(dec!(2.4800), (37, 25)); // D
        m.insert(dec!(2.5200), (38, 25)); // D
        m.insert(dec!(2.5400), (77, 50)); // D
        m.insert(dec!(2.5600), (39, 25)); // D
        m.insert(dec!(2.5800), (79, 50)); // D
        m.insert(dec!(2.6400), (41, 25)); // D
        m.insert(dec!(2.6600), (83, 50)); // D
        m.insert(dec!(2.6800), (42, 25)); // D
        m.insert(dec!(2.7000), (17, 10));
        m.insert(dec!(2.7200), (43, 25)); // D
        m.insert(dec!(2.7400), (87, 50)); // D
        m.insert(dec!(2.7600), (44, 25)); // D
        m.insert(dec!(2.7800), (89, 50)); // D
        m.insert(dec!(2.8200), (91, 50)); // D
        m.insert(dec!(2.8400), (46, 25)); // D
        m.insert(dec!(2.8600), (93, 50)); // D
        m.insert(dec!(2.8700), (15, 8)); // D
        m.insert(dec!(2.9000), (19, 10));
        m.insert(dec!(2.9200), (48, 25)); // D
        m.insert(dec!(2.9400), (97, 50));
        m.insert(dec!(2.9600), (49, 25)); // D
        m.insert(dec!(2.9800), (99, 50)); // D
        m.insert(dec!(3.0500), (41, 20)); // D
        m.insert(dec!(3.1000), (21, 10));
        m.insert(dec!(3.1250), (85, 40));
        m.insert(dec!(3.1500), (43, 20)); // D
        m.insert(dec!(3.3000), (23, 10));
        m.insert(dec!(3.3500), (47, 20)); // D
        m.insert(dec!(3.4500), (49, 20)); // D
        m.insert(dec!(3.5500), (51, 20)); // D
        m.insert(dec!(3.6500), (53, 20)); // D
        m.insert(dec!(3.7000), (27, 10)); // D
        m.insert(dec!(3.8000), (14, 5));
        m.insert(dec!(3.8500), (57, 20)); // D
        m.insert(dec!(3.9500), (59, 20)); // D
        m.insert(dec!(4.0500), (61, 20)); // D
        m.insert(dec!(4.1000), (31, 10));
        m.insert(dec!(4.1500), (63, 20)); // D
        m.insert(dec!(4.2500), (13, 4)); // D
        m.insert(dec!(4.3000), (33, 10)); // D
        m.insert(dec!(4.3500), (67, 20)); // D
        m.insert(dec!(4.4000), (17, 5));
        m.insert(dec!(4.4500), (69, 20)); // D
        m.insert(dec!(4.5500), (71, 20)); // D
        m.insert(dec!(4.6000), (18, 5));
        m.insert(dec!(4.6500), (73, 20)); // D
        m.insert(dec!(4.7000), (37, 10)); // D
        m.insert(dec!(4.7500), (15, 4));
        m.insert(dec!(4.8000), (19, 5));
        m.insert(dec!(4.8500), (77, 20)); // D
        m.insert(dec!(4.9000), (39, 10)); // D
        m.insert(dec!(4.9500), (79, 20)); // D
        m.insert(dec!(5.1000), (41, 10)); // D
        m.insert(dec!(5.2000), (21, 5)); // D
        m.insert(dec!(5.3000), (43, 10)); // D
        m.insert(dec!(5.4000), (22, 5)); // D
        m.insert(dec!(5.6000), (23, 5)); // D
        m.insert(dec!(5.7000), (47, 10)); // D
        m.insert(dec!(5.8000), (24, 5)); // D
        m.insert(dec!(5.9000), (49, 10)); // D
        m.insert(dec!(6.2000), (26, 5)); // D
        m.insert(dec!(6.4000), (27, 5)); // D
        m.insert(dec!(6.6000), (28, 5)); // D
        m.insert(dec!(6.8000), (29, 5)); // D
        m.insert(dec!(7.2000), (31, 5)); // D
        m.insert(dec!(7.4000), (32, 5)); // D
        m.insert(dec!(7.6000), (33, 5)); // D
        m.insert(dec!(7.8000), (34, 5)); // D
        m.insert(dec!(8.2000), (36, 5)); // D
        m.insert(dec!(8.4000), (37, 5)); // D
        m.insert(dec!(8.6000), (38, 5)); // D
        m.insert(dec!(8.8000), (39, 5)); // D
        m.insert(dec!(9.2000), (41, 5)); // D
        m.insert(dec!(9.4000), (42, 5)); // D
        m.insert(dec!(9.5000), (17, 2));
        m.insert(dec!(9.6000), (43, 5)); // D
        m.insert(dec!(9.8000), (44, 5)); // D
        m.insert(dec!(23.0000), (22, 1));
        m.insert(dec!(29.0000), (28, 1));
        m.insert(dec!(31.0000), (30, 1));
        m.insert(dec!(36.0000), (35, 1));
        m.insert(dec!(41.0000), (40, 1));
        m.insert(dec!(46.0000), (45, 1));
        m.insert(dec!(56.0000), (55, 1));
        m.insert(dec!(61.0000), (60, 1));
        m.insert(dec!(71.0000), (70, 1));
        m.insert(dec!(76.0000), (75, 1)); // D
        m.insert(dec!(81.0000), (80, 1));
        m.insert(dec!(86.0000), (85, 1)); // D
        m.insert(dec!(91.0000), (90, 1));
        m.insert(dec!(96.0000), (95, 1)); // D
        m.insert(dec!(111.0000), (110, 1));
        m.insert(dec!(121.0000), (120, 1));
        m.insert(dec!(126.0000), (125, 1));
        m.insert(dec!(131.0000), (130, 1));
        m.insert(dec!(141.0000), (140, 1));
        m.insert(dec!(151.0000), (150, 1));
        m.insert(dec!(176.0000), (175, 1));
        m.insert(dec!(201.0000), (200, 1));
        m.insert(dec!(226.0000), (225, 1));
        m.insert(dec!(251.0000), (250, 1));
        m.insert(dec!(276.0000), (275, 1));
        m.insert(dec!(301.0000), (300, 1));
        m.insert(dec!(401.0000), (400, 1));
        m.insert(dec!(501.0000), (500, 1));
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

// Accessor for AMERICAN_TO_FRACTION extended Map
static AMERICAN_TO_FRACTION_EXTENDED: OnceLock<HashMap<i32, (u32, u32)>> = OnceLock::new();

/// Extended lookup table for conversion from american to fractional.
pub fn get_american_to_fraction_extended_map() -> &'static HashMap<i32, (u32, u32)> {
    AMERICAN_TO_FRACTION_EXTENDED.get_or_init(HashMap::new)
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

// Accessor for AMERICAN_TO_DECIMAL extended Map
static AMERICAN_TO_DECIMAL_EXTENDED: OnceLock<HashMap<i32, Decimal>> = OnceLock::new();

/// Extended lookup table for conversion from american to decimal.
pub fn get_american_to_decimal_extended_map() -> &'static HashMap<i32, Decimal> {
    // TODO
    AMERICAN_TO_DECIMAL_EXTENDED.get_or_init(HashMap::new)
}
