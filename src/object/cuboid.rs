use nalgebra::{Point3, Vector3};

use crate::{
    core::{intersection::Intersection, material::Material, object::Object},
    util::math::{max_element, min_element},
};

pub struct Cuboid<'a> {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
    pub area: f32,
    pub material: &'a Box<dyn Material>,
}

impl Object for Cuboid<'_> {
    fn intersect(&self, ray: &crate::core::ray::Ray) -> Option<Intersection> {
        let t_min = (self.min - ray.origin()).component_mul(&ray.dir_reciprocal());
        let t_max = (self.max - ray.origin()).component_mul(&ray.dir_reciprocal());

        let t0 = min_element(t_min, t_max);
        let t1 = max_element(t_min, t_max);

        let t_enter = t0.x.max(t0.y).max(t0.z);
        let t_exit = t1.x.min(t1.y).min(t1.z);
        if (t_enter >= t_exit) || t_exit.is_sign_negative() {
            return None;
        }
        if t_enter < 0.5 {
            return None;
        }
        let mut result = Intersection {
            point: ray.point_at(t_enter).into(),
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: self.material,
            distance: t_enter,
        };

        if (result.point.x - self.min.x).abs() < 1e-2 {
            result.normal = Vector3::new(-1.0, 0.0, 0.0);
        } else if (result.point.x - self.max.x).abs() < 1e-2 {
            result.normal = Vector3::new(1.0, 0.0, 0.0);
        } else if (result.point.y - self.min.y).abs() < 1e-2 {
            result.normal = Vector3::new(0.0, -1.0, 0.0);
        } else if (result.point.y - self.max.y).abs() < 1e-2 {
            result.normal = Vector3::new(0.0, 1.0, 0.0);
        } else if (result.point.z - self.min.z).abs() < 1e-2 {
            result.normal = Vector3::new(0.0, 0.0, -1.0);
        } else if (result.point.z - self.max.z).abs() < 1e-2 {
            result.normal = Vector3::new(0.0, 0.0, 1.0);
        } else {
            return None;
        }
        Some(result)
    }
}
