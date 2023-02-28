use super::scene::Scene;

pub trait Renderer {
    fn render(&self, scene: &Scene) -> image::DynamicImage;
}

pub trait Integrator: Renderer {}

pub trait Rasterizer: Renderer {}
