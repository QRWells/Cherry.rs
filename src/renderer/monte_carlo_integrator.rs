use crate::core::{
    renderer::{Integrator, Renderer},
    scene::Scene,
};

pub struct MonteCarloIntegrator {}

impl Renderer for MonteCarloIntegrator {
    fn render(&self, scene: &Scene) -> image::DynamicImage {
        todo!()
    }
}

impl Integrator for MonteCarloIntegrator {}
