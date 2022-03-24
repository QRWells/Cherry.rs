use crate::core::renderer::Renderer;

pub struct PathTracer {}

impl Renderer for PathTracer {
    fn render(&self, scene: &crate::core::scene::Scene) -> image::DynamicImage {
        todo!()
    }
}
