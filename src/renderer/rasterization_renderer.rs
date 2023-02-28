use crate::core::{
    renderer::{Rasterizer, Renderer},
    scene::Scene,
};

pub struct RasterizationRenderer {}

impl Renderer for RasterizationRenderer {
    fn render(&self, scene: &Scene) -> image::DynamicImage {
        todo!()
    }
}

impl Rasterizer for RasterizationRenderer {}
