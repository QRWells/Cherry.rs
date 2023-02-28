use nalgebra::{Vector2, Vector3};

use crate::util::{math::*, sampler::*};

use super::ray::Ray;

pub struct Camera {
    aperture: f32,
    focal_distance: f32,
    fov: f32,
    aspect_ratio: f32,
    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,
    position: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    top_left: Vector3<f32>,
}

impl Camera {
    pub fn new(
        look_from: Vector3<f32>,
        look_at: Vector3<f32>,
        view_up: Vector3<f32>,
        fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focal_distance: f32,
    ) -> Self {
        let theta = deg_to_rad(fov);
        let h = (theta / 2f32).tan();
        let viewport_height = 2f32 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).normalize();
        let u = view_up.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = viewport_width * u * focal_distance;
        let vertical = -viewport_height * v * focal_distance;
        let top_left = look_from - horizontal / 2f32 - vertical / 2f32 - w * focal_distance;

        Camera {
            aperture,
            focal_distance,
            fov,
            aspect_ratio,
            u,
            v,
            w,
            position: look_from,
            horizontal,
            vertical,
            top_left,
        }
    }

    pub fn generate_ray(&self, uv: Vector2<f32>) -> Ray {
        let lens_radius = self.aperture / 2f32;
        let dist = get_random_within_circle() * lens_radius;
        let offset = dist.x * self.u + dist.y * self.v;
        Ray::new(
            self.position + offset,
            self.top_left + uv.x * self.horizontal + uv.y * self.vertical - self.position - offset,
        )
    }
}
