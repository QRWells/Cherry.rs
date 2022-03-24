use crate::core::{renderer::Renderer, scene::Scene};

pub struct RasterizationRenderer {}

impl Renderer for RasterizationRenderer {
    fn render(&self, scene: &Scene) -> image::DynamicImage {
        todo!()
    }
}
