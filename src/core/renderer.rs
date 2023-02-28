use image::GenericImage;
use nalgebra::{Point2, Vector2, Vector3};

use super::scene::Scene;

pub trait Renderer {
    fn render(&self, scene: &Scene) -> image::DynamicImage;
}

pub trait Integrator: Renderer {
    fn li(&self, scene: &Scene, ray: &crate::core::ray::Ray) -> Vector3<f32>;

    fn integrate_all(&self, scene: &Scene) -> image::DynamicImage {
        todo!()
    }
}

pub trait Rasterizer: Renderer {
    fn rasterize(&self, scene: &Scene, pixel: Point2<usize>) -> Vector3<f32>;

    fn rasterize_all(&self, scene: &Scene) -> image::DynamicImage {
        todo!()
    }
}
