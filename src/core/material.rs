use nalgebra::Vector3;

type Color = Vector3<f32>;

pub struct BsdfInfo {
    wi: Vector3<f32>,
    n: Vector3<f32>,
    wo: Vector3<f32>,
}

pub trait Material: Send + Sync {
    fn eval(&self, bsdf_info: &BsdfInfo) -> Color;
    fn sample(&self, bsdf_info: &mut BsdfInfo) -> ();
    fn pdf(&self, bsdf_info: &BsdfInfo) -> f32;
}

pub struct Mirror {}
pub struct Dielectric {}
pub struct Microfacet {}
