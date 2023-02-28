use crate::core::{
    renderer::{Integrator, Renderer},
    scene::Scene,
};

pub struct NormalIntegrator {}

impl Integrator for NormalIntegrator {
    fn li(&self, scene: &Scene, ray: &crate::core::ray::Ray) -> nalgebra::Vector3<f32> {
        match scene.intersect(ray) {
            Some(i) => i.normal.abs(),
            None => nalgebra::Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
impl Renderer for NormalIntegrator {
    fn render(&self, scene: &Scene) -> image::DynamicImage {
        self.integrate_all(scene)
    }
}
