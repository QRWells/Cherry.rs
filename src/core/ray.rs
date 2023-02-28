use nalgebra::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f32>,
    pub dir: Vector3<f32>,
}

impl Ray {
    pub fn origin(&self) -> &Point3<f32> {
        &self.origin
    }

    pub fn dir(&self) -> &Vector3<f32> {
        &self.dir
    }

    pub fn dir_reciprocal(&self) -> Vector3<f32> {
        Vector3::new(1.0 / self.dir.x, 1.0 / self.dir.y, 1.0 / self.dir.z)
    }

    pub fn point_at(&self, distance: f32) -> Vector3<f32> {
        self.origin.coords + self.dir * distance
    }
}
