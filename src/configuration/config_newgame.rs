use std::ops::Range;

use crate::FitzError;

#[derive(Debug, PartialEq)]
pub struct ConfigNew {
    pub height: u16,
    pub width: u16,
}

impl ConfigNew {
    pub fn build(height: &str, width: &str) -> Result<ConfigNew, FitzError> {
        // Generate error message
        let game_dimensions_invalid: FitzError = FitzError {
            code: 5,
            message: "Invalid dimensions".to_string(),
        };

        let Ok(height) = height.parse::<u16>() else {
            return Err(game_dimensions_invalid);
        };

        let Ok(width) = width.parse::<u16>() else {
            return Err(game_dimensions_invalid);
        };

        // Ensure width and height are in valid range
        let valid_range: Range<u16> = 1..1000;
        if !valid_range.contains(&height) || !valid_range.contains(&width) {
            return Err(game_dimensions_invalid);
        }

        Ok(ConfigNew { height, width })
    }
}

#[cfg(test)]
mod ut {
    use super::*;

    #[test]
    fn test_1_1_valid_dimensions() {
        // 1.1
        let expected: ConfigNew = ConfigNew {
            height: 69,
            width: 420,
        };
        assert_eq!(ConfigNew::build("69", "420"), Ok(expected));
    }

    #[test]
    fn test_1_2_valid_dimensions() {
        let expected: ConfigNew = ConfigNew {
            height: 1,
            width: 999,
        };
        assert_eq!(ConfigNew::build("1", "999"), Ok(expected));
    }

    #[test]
    fn test_1_3_valid_dimensions() {
        let expected: ConfigNew = ConfigNew {
            height: 999,
            width: 1,
        };
        assert_eq!(ConfigNew::build("999", "1"), Ok(expected));
    }

    #[test]
    fn test_2_1_invalid_dimensions() {
        let expected = FitzError {
            code: 5,
            message: "Invalid dimensions".to_string(),
        };

        assert_eq!(ConfigNew::build("0", "999"), Err(expected));
    }

    #[test]
    fn test_2_2_invalid_dimensions() {
        let expected = FitzError {
            code: 5,
            message: "Invalid dimensions".to_string(),
        };

        assert_eq!(ConfigNew::build("1", "1000"), Err(expected));
    }
}
