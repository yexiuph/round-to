pub struct RoundTo {}

impl RoundTo {
    fn round(method: fn(f32) -> f32, number: f32, precision: i32) -> f32 {
        let factor = 10_f32.powi(precision);
        let abs_number = number.abs();
        (method(abs_number * factor) / factor) * if number < 0.0 { -1.0 } else { 1.0 }
    }

    pub fn round_to(number: f32, precision: i32) -> f32 {
        Self::round(f32::round, number, precision)
    }

    pub fn round_to_ceil(number: f32, precision: i32) -> f32 {
        Self::round(f32::ceil, number, precision)
    }

    pub fn round_to_floor(number: f32, precision: i32) -> f32 {
        Self::round(f32::floor, number, precision)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_to() {
        // Test rounding positive numbers
        assert_eq!(RoundTo::round_to(3.14159, 2), 3.14);
        assert_eq!(RoundTo::round_to(3.14159, 3), 3.142);
        assert_eq!(RoundTo::round_to(1234.5678, 0), 1235.0);
        
        // Test rounding negative numbers
        assert_eq!(RoundTo::round_to(-3.14159, 2), -3.14);
        assert_eq!(RoundTo::round_to(-3.14159, 3), -3.142);
        assert_eq!(RoundTo::round_to(-1234.5678, 0), -1235.0);
    }

    #[test]
    fn test_round_to_ceil() {
        // Test rounding up positive numbers
        assert_eq!(RoundTo::round_to_ceil(3.14159, 2), 3.15);
        assert_eq!(RoundTo::round_to_ceil(3.14159, 3), 3.142);
        assert_eq!(RoundTo::round_to_ceil(1234.5678, 0), 1235.0);
        
        // Test rounding up negative numbers
        assert_eq!(RoundTo::round_to_ceil(-3.14159, 2), -3.14);
        assert_eq!(RoundTo::round_to_ceil(-3.14159, 3), -3.141);
        assert_eq!(RoundTo::round_to_ceil(-1234.5678, 0), -1234.0);
    }

    #[test]
    fn test_round_to_floor() {
        // Test rounding down positive numbers
        assert_eq!(RoundTo::round_to_floor(3.14159, 2), 3.14);
        assert_eq!(RoundTo::round_to_floor(3.14159, 3), 3.141);
        assert_eq!(RoundTo::round_to_floor(1234.5678, 0), 1234.0);
        
        // Test rounding down negative numbers
        assert_eq!(RoundTo::round_to_floor(-3.14159, 2), -3.15);
        assert_eq!(RoundTo::round_to_floor(-3.14159, 3), -3.142);
        assert_eq!(RoundTo::round_to_floor(-1234.5678, 0), -1235.0);
    }
}
