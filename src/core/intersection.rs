use nalgebra::{Point3, Vector3};

use super::material::Material;

pub struct Intersection<'a> {
    pub point: Point3<f32>,
    pub normal: Vector3<f32>,
    pub material: &'a Box<dyn Material>,
    pub distance: f32,
}
