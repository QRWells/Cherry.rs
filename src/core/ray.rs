use nalgebra::Vector3;

pub struct Ray {
    origin: Vector3<f32>,
    dir: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, dir: Vector3<f32>) -> Self {
        Ray { origin, dir }
    }

    pub fn origin(&self) -> &Vector3<f32> {
        &self.origin
    }

    pub fn dir(&self) -> &Vector3<f32> {
        &self.dir
    }

    pub fn point_at(&self, distance: f32) -> Vector3<f32> {
        (self.dir * distance) + self.origin
    }
}
