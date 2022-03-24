use crate::core::{renderer::Renderer, scene::Scene};

pub struct NormalIntegrator {}

impl Renderer for NormalIntegrator {
    fn render(&self, _scene: &Scene) -> image::DynamicImage {
        todo!()
    }
}
