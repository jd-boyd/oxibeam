use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len == 0.0 {
            *self
        } else {
            Vec3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        }
    }
}

// Implement basic arithmetic operations
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        if scalar == 0.0 {
            panic!("Division by zero");
        }
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    fn assert_float_eq(a: f64, b: f64) {
        assert!((a - b).abs() < EPSILON);
    }

    fn assert_vec3_eq(a: Vec3, b: Vec3) {
        assert_float_eq(a.x, b.x);
        assert_float_eq(a.y, b.y);
        assert_float_eq(a.z, b.z);
    }

    #[test]
    fn test_creation() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_float_eq(v.x, 1.0);
        assert_float_eq(v.y, 2.0);
        assert_float_eq(v.z, 3.0);
    }

    #[test]
    fn test_zero() {
        let v = Vec3::zero();
        assert_vec3_eq(v, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_float_eq(v1.dot(&v2), 32.0); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let result = v1.cross(&v2);
        assert_vec3_eq(result, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert_float_eq(v.length(), 5.0);
    }

    #[test]
    fn test_normalize() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let normalized = v.normalize();
        assert_vec3_eq(normalized, Vec3::new(0.6, 0.8, 0.0));
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_vec3_eq(result, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(4.0, 5.0, 6.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let result = v1 - v2;
        assert_vec3_eq(result, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = v * 2.0;
        assert_vec3_eq(result, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_div_scalar() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let result = v / 2.0;
        assert_vec3_eq(result, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_div_by_zero() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let _result = v / 0.0;
    }
}
