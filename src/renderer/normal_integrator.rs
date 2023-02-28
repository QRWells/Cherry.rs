use crate::core::{
    renderer::{Integrator, Renderer},
    scene::Scene,
};

pub struct NormalIntegrator {}

impl Renderer for NormalIntegrator {
    fn render(&self, _scene: &Scene) -> image::DynamicImage {
        todo!()
    }
}

impl Integrator for NormalIntegrator {}
