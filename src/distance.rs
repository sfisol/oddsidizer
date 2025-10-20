use std::fmt;

/// Represents a race distance in miles, furlongs, and yards.
#[derive(Debug, PartialEq)]
pub struct RaceDistance {
    pub miles: u32,
    pub furlongs: u32,
    pub yards: u32,
}

impl RaceDistance {
    /// Creates a `RaceDistance` instance from a total distance in yards.
    pub fn from_yards(total_yards: u32) -> Self {
        const YARDS_IN_MILE: u32 = 1760;
        const YARDS_IN_FURLONG: u32 = 220;

        let miles = total_yards / YARDS_IN_MILE;
        let yards_after_miles = total_yards % YARDS_IN_MILE;

        let furlongs = yards_after_miles / YARDS_IN_FURLONG;
        let yards = yards_after_miles % YARDS_IN_FURLONG;

        Self {
            miles,
            furlongs,
            yards,
        }
    }
}

impl fmt::Display for RaceDistance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.miles == 0 && self.furlongs == 0 && self.yards == 0 {
            return write!(f, "0y");
        }

        let mut parts = Vec::new();

        if self.miles > 0 {
            parts.push(format!("{}m", self.miles));
        }
        if self.furlongs > 0 {
            parts.push(format!("{}f", self.furlongs));
        }
        if self.yards > 0 {
            parts.push(format!("{}y", self.yards));
        }

        write!(f, "{}", parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_conversions_with_yards() {
        let test_cases = [
            // Sprints
            (1100, 0, 5, 0), // 5f
            (1320, 0, 6, 0), // 6f
            (1540, 0, 7, 0), // 7f
            // Middle Distances
            (1760, 1, 0, 0), // 1m
            (1980, 1, 1, 0), // 1m 1f
            (2200, 1, 2, 0), // 1m 2f
            (2420, 1, 3, 0), // 1m 3f
            // Routes / Stayers
            (2640, 1, 4, 0), // 1m 4f
            (3080, 1, 6, 0), // 1m 6f
            (3520, 2, 0, 0), // 2m
            // Edge cases with remaining yards
            (0, 0, 0, 0),      // Zero yards
            (219, 0, 0, 219),  // Just under one furlong
            (1759, 0, 7, 219), // Just under one mile
            (2000, 1, 1, 20),
        ];

        for &(input_yards, expected_miles, expected_furlongs, expected_yards) in &test_cases {
            let expected = RaceDistance {
                miles: expected_miles,
                furlongs: expected_furlongs,
                yards: expected_yards,
            };
            // Updated to use the new associated function syntax.
            let result = RaceDistance::from_yards(input_yards);

            assert_eq!(result, expected, "Failed on input of {} yards", input_yards);
        }
    }

    #[test]
    fn test_display_formatting_with_yards() {
        fn rd(miles: u32, furlongs: u32, yards: u32) -> RaceDistance {
            RaceDistance {
                miles,
                furlongs,
                yards,
            }
        }

        // Test various combinations to check the formatting logic.
        assert_eq!(rd(1, 6, 0).to_string(), "1m 6f");
        assert_eq!(rd(0, 7, 219).to_string(), "7f 219y");
        assert_eq!(rd(2, 0, 50).to_string(), "2m 50y");
        assert_eq!(rd(0, 6, 0).to_string(), "6f");
        assert_eq!(rd(0, 0, 100).to_string(), "100y");
        assert_eq!(rd(0, 0, 0).to_string(), "0y");

        let converted_dist = RaceDistance::from_yards(2000);
        assert_eq!(format!("{}", converted_dist), "1m 1f 20y");
    }
}
