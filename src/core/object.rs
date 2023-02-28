use super::{intersection::Intersection, ray::Ray};

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
