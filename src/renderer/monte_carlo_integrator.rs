use crate::core::{
    renderer::{Integrator, Renderer},
    scene::Scene,
};

pub struct MonteCarloIntegrator {}

impl Integrator for MonteCarloIntegrator {
    fn li(&self, scene: &Scene, ray: &crate::core::ray::Ray) -> nalgebra::Vector3<f32> {
        todo!()
    }
}

impl Renderer for MonteCarloIntegrator {
    fn render(&self, scene: &Scene) -> image::DynamicImage {
        self.integrate_all(scene)
    }
}
