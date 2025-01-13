use crate::vec3::Vec3;

pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {

    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        return self.orig + t * self.dir;
    }
}

#[cfg(test)]
mod ray_tests {
    use super::*;

    use std::f64::EPSILON;

    fn assert_float_eq(a: f64, b: f64) {
        assert!((a - b).abs() < EPSILON);
    }

    #[test]
    fn test_at1() {
        let v = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 1.0)
        );

        let t1 = v.at(1.0);
        assert_float_eq(t1.x, 1.0);
        assert_float_eq(t1.y, 1.0);
        assert_float_eq(t1.z, 1.0);



    }

}
