use std::ops::{Add, Sub, Mul, Div};


#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D { x, y }
    }

    pub fn dot(&self, other: &Vector2D) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        if length == 0.0 {
            panic!("Can't normalize zero vector");
        }
        self.x /= length;
        self.y /= length;
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y }
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul<f32> for Vector2D {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Vector2D {
            x: self.x * other,
            y: self.y * other
        }
    }
}

impl Div<f32> for Vector2D {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        if other == 0.0 {
            panic!("Can't divide by zero");
        }
        Vector2D {
            x: self.x / other,
            y: self.y / other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_add() {
        let a = Vector2D::new(1.0, 2.0);
        let b = Vector2D::new(3.0, 4.0);
        let c = a + b;
        assert_eq!(c.x, 4.0);
        assert_eq!(c.y, 6.0);
    }

    #[test]
    fn test_add_negative_positive() {
        let a = Vector2D::new(-1.0, -2.0);
        let b = Vector2D::new(3.0, 5.0);
        let c = a + b;
        assert_eq!(c.x, 2.0);
        assert_eq!(c.y, 3.0);
    }

    #[test]
    fn test_add_negative_only() {
        let a = Vector2D::new(-1.0, -2.0);
        let b = Vector2D::new(-3.0, -5.0);
        let c = a + b;
        assert_eq!(c.x, -4.0);
        assert_eq!(c.y, -7.0);
    }

    #[test]
    fn test_add_large_numbers() {
        let a = Vector2D::new(1648736.0, 64267326.0);
        let b = Vector2D::new(684573.0, 958357333.0);
        let c = a + b;
        assert_eq!(c.x, 2333309.0);
        assert_eq!(c.y, 1022624640.0);
    }

    #[test]
    fn test_add_tiny_numbers() {
        let a = Vector2D::new(0.0000005, 0.0000043);
        let b = Vector2D::new(0.0000076, 0.000043);
        let c = a + b;
        assert_eq!(c.x, 0.0000081);
        assert_eq!(c.y, 0.0000473);
    }

    #[test]
    fn test_sub() {
        let a = Vector2D::new(1.0, 2.0);
        let b = Vector2D::new(3.0, 4.0);
        let c = a - b;
        assert_eq!(c.x, -2.0);
        assert_eq!(c.y, -2.0);
    }

    #[test]
    fn test_sub_negative_positive() {
        let a = Vector2D::new(-1.0, -2.0);
        let b = Vector2D::new(3.0, 5.0);
        let c = a - b;
        assert_eq!(c.x, -4.0);
        assert_eq!(c.y, -7.0);
    }

    #[test]
    fn test_sub_negative_only() {
        let a = Vector2D::new(-1.0, -2.0);
        let b = Vector2D::new(-3.0, -5.0);
        let c = a - b;
        assert_eq!(c.x, 2.0);
        assert_eq!(c.y, 3.0);
    }

    #[test]
    fn test_sub_large_numbers() {
        let a = Vector2D::new(1648736.0, 64267326.0);
        let b = Vector2D::new(684573.0, 958357333.0);
        let c = a - b;
        assert_eq!(c.x, 964163.0);
        assert_eq!(c.y, -894090000.0);
    }

    #[test]
    fn test_sub_tiny_numbers() {
        let a = Vector2D::new(0.0000005, 0.0000043);
        let b = Vector2D::new(0.0000076, 0.000043);
        let c = a - b;
        assert_eq!(c.x, -0.0000071);
        assert_eq!(c.y, -0.0000387);
    }

    #[test]
    fn test_mul() {
        let a = Vector2D::new(1.0, 2.0);
        let b = a * 3.0;
        assert_eq!(b.x, 3.0);
        assert_eq!(b.y, 6.0);
    }

    #[test]
    fn test_div() {
        let a = Vector2D::new(1.0, 2.0);
        let b = a / 3.0;
        assert_eq!(b.x, 0.3333333333333333);
        assert_eq!(b.y, 0.6666666666666666);
    }

    #[test]
    fn test_normalize() {
        let mut a = Vector2D::new(1.0, 2.0);
        a.normalize();
        assert_eq!(a.x, 0.4472135954999579);
        assert_eq!(a.y, 0.8944271909999158);
    }

    #[test]
    fn test_length() {
        let a = Vector2D::new(1.0, 2.0);
        assert_eq!(a.length(), 2.23606797749979);
    }

    #[test]
    fn test_zero_vector() {
        let zero = Vector2D::new(0.0, 0.0);
        let a = Vector2D::new(1.0, 2.0);
        let c = a + zero;
        let d = a - zero;
        assert_eq!(c.x, a.x);
        assert_eq!(c.y, a.y);
        assert_eq!(d.x, a.x);
        assert_eq!(d.y, a.y);
    }

    #[test]
    fn test_dot() {
        let a = Vector2D::new(1.0, 2.0);
        let b = Vector2D::new(3.0, 4.0);
        assert_eq!(a.dot(&b), 11.0);
    }

    #[test]
    fn test_dot_product_perpendicular() {
        let a = Vector2D::new(1.0, 0.0);
        let b = Vector2D::new(0.0, 1.0);
        assert_eq!(a.dot(&b), 0.0);
    }

    #[test]
    fn test_dot_product_parallel() {
        let a = Vector2D::new(2.0, 0.0);
        let b = Vector2D::new(3.0, 0.0);
        assert_eq!(a.dot(&b), 6.0);
    }

    #[test]
    fn test_normalize_unit_vector() {
        let mut a = Vector2D::new(1.0, 0.0);
        a.normalize();
        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_normalize_zero_vector() {
        let mut zero = Vector2D::new(0.0, 0.0);
        zero.normalize();
    }

    #[test]
    fn test_mul_zero() {
        let a = Vector2D::new(1.0, 2.0);
        let b = a * 0.0;
        assert_eq!(b.x, 0.0);
        assert_eq!(b.y, 0.0);
    }

    #[test]
    fn test_mul_negative() {
        let a = Vector2D::new(1.0, 2.0);
        let b = a * -1.0;
        assert_eq!(b.x, -1.0);
        assert_eq!(b.y, -2.0);
    }

    #[test]
    #[should_panic]
    fn test_div_by_zero() {
        let a = Vector2D::new(1.0, 2.0);
        let _b = a / 0.0;
    }

    #[test]
    fn test_div_by_negative() {
        let a = Vector2D::new(1.0, 2.0);
        let b = a / -2.0;
        assert_eq!(b.x, -0.5);
        assert_eq!(b.y, -1.0);
    }

    #[test]
    fn test_zero_vector_length() {
        let zero = Vector2D::new(0.0, 0.0);
        assert_eq!(zero.length(), 0.0);
    }

    #[test]
    fn test_unit_vector_length() {
        let unit = Vector2D::new(1.0, 0.0);
        assert_eq!(unit.length(), 1.0);
    }

    #[test]
    fn test_negative_components_length() {
        let v = Vector2D::new(-3.0, -4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_vector_copy() {
        let a = Vector2D::new(1.0, 2.0);
        let b = a;
        assert_eq!(a.x, 1.0);
        assert_eq!(b.x, 1.0);
    }

    #[test]
    fn test_vector_clone() {
        let a = Vector2D::new(1.0, 2.0);
        let b = a.clone();
        assert_eq!(a.x, b.x);
        assert_eq!(a.y, b.y);
    }

    fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() < epsilon
    }


    fn vector_approx_eq(a: &Vector2D, b: &Vector2D, epsilon: f32) -> bool {
        approx_eq(a.x, b.x, epsilon) && approx_eq(a.y, b.y, epsilon)
    }

    #[test]
    fn test_normalize_precision() {
        let mut v = Vector2D::new(123.456, 789.012);
        v.normalize();
        let length = v.length();
        assert!(approx_eq(length, 1.0, f32::EPSILON),
                "Expected length 1.0, got {}", length);
    }

    #[test]
    fn test_normalize_various_magnitudes() {
        let test_cases = [
            Vector2D::new(1000000.0, 1000000.0),
            Vector2D::new(0.000001, 0.000001),
            Vector2D::new(1.0, 0.000001),
        ];

        for mut v in test_cases {
            v.normalize();
            let length = v.length();
            assert!(approx_eq(length, 1.0, f32::EPSILON * 100.0),
                    "Vector {:?} after normalization has length {}", v, length);
        }
    }

    #[test]
    fn test_dot_product_precision() {
        let a = Vector2D::new(1234.567, 89.012);
        let b = Vector2D::new(34.567, 890.123);
        let actual_dot = a.dot(&b);
        let expected_dot = 1234.567 * 34.567 + 89.012 * 890.123;
        assert!(approx_eq(actual_dot, expected_dot, f32::EPSILON * actual_dot.abs()),
                "Expected dot product {}, got {}", expected_dot, actual_dot);
    }

    #[test]
    fn test_perpendicular_vectors_dot() {
        let mut a = Vector2D::new(2.0, 1.0);
        let mut b = Vector2D::new(-1.0, 2.0);
        a.normalize();
        b.normalize();
        let dot = a.dot(&b);
        assert!(approx_eq(dot, 0.0, f32::EPSILON),
                "Perpendicular vectors should have dot product 0, got {}", dot);
    }

    #[test]
    fn test_vector_addition_precision() {
        let tiny = Vector2D::new(1e-6, 1e-6);
        let large = Vector2D::new(1e6, 1e6);
        let sum = tiny + large;
        assert!(approx_eq(sum.x, large.x, f32::EPSILON * large.x.abs()),
                "Expected x ≈ {}, got {}", large.x, sum.x);
        assert!(approx_eq(sum.y, large.y, f32::EPSILON * large.y.abs()),
                "Expected y ≈ {}, got {}", large.y, sum.y);
    }

    #[test]
    fn test_multiplication_precision() {
        let v = Vector2D::new(123.456, 789.012);
        let scalar = 0.001;
        let result = v * scalar;
        assert!(approx_eq(result.x, v.x * scalar, f32::EPSILON * result.x.abs()),
                "Expected x ≈ {}, got {}", v.x * scalar, result.x);
        assert!(approx_eq(result.y, v.y * scalar, f32::EPSILON * result.y.abs()),
                "Expected y ≈ {}, got {}", v.y * scalar, result.y);
    }

    #[test]
    fn test_division_precision() {
        let v = Vector2D::new(123.456, 789.012);
        let scalar = 1000.0;
        let result = v / scalar;
        assert!(approx_eq(result.x, v.x / scalar, f32::EPSILON * (v.x / scalar).abs()),
                "Expected x ≈ {}, got {}", v.x / scalar, result.x);
        assert!(approx_eq(result.y, v.y / scalar, f32::EPSILON * (v.y / scalar).abs()),
                "Expected y ≈ {}, got {}", v.y / scalar, result.y);
    }

    #[test]
    fn test_length_precision() {
        // Test with Pythagorean triple to verify precision
        let v = Vector2D::new(3.0, 4.0);
        assert!(approx_eq(v.length(), 5.0, f32::EPSILON),
                "Expected length 5.0, got {}", v.length());
    }

    #[test]
    fn test_chain_operations_precision() {
        let a = Vector2D::new(1.0, 2.0);
        let b = Vector2D::new(3.0, 4.0);

        // Test (a + b) * 2.0 / 2.0 ≈ (a + b)
        let result = (a + b) * 2.0 / 2.0;
        assert!(vector_approx_eq(&result, &(a + b), f32::EPSILON),
                "Chain operation precision test failed");
    }

    #[test]
    fn test_normalize_small_vector() {
        let mut v = Vector2D::new(1e-15, 1e-15);
        v.normalize();
        assert!(approx_eq(v.length(), 1.0, f32::EPSILON * 100.0),
                "Small vector normalization failed, length = {}", v.length());
    }

}