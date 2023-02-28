use nalgebra::{Point3, Vector3};

use crate::{
    core::{intersection::Intersection, material::Material, object::Object, ray::Ray},
    util::math::solve_quadratic,
};

pub struct Sphere<'a> {
    pub center: Point3<f32>,
    pub radius: f32,
    pub material: &'a Box<dyn Material>,
}

impl Object for Sphere<'_> {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let l = ray.origin() - self.center;
        let a = ray.dir().norm_squared();
        let b = 2.0 * ray.dir().dot(&l);
        let c = l.norm_squared() - self.radius * self.radius;
        let t0: f32;

        match solve_quadratic(a, b, c) {
            Some((t0_, _)) => {
                t0 = t0_;
            }
            None => return None,
        }

        let mut result = Intersection {
            point: ray.origin() + ray.dir() * t0,
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: self.material,
            distance: t0,
        };

        result.normal = (result.point - self.center).normalize();

        Some(result)
    }
}
