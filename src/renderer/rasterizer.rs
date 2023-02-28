use crate::core::{
    renderer::{Rasterizer, Renderer},
    scene::Scene,
};

pub struct SimpleRasterizer {}

impl Rasterizer for SimpleRasterizer {
    fn rasterize(&self, scene: &Scene, pixel: nalgebra::Point2<usize>) -> nalgebra::Vector3<f32> {
        todo!()
    }
}
impl Renderer for SimpleRasterizer {
    fn render(&self, scene: &Scene) -> image::DynamicImage {
        self.rasterize_all(scene)
    }
}
