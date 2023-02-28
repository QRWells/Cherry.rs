use super::camera::Camera;
use super::light::Light;
use super::object::Object;

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
}

impl Scene {
    fn add_object(&mut self, obj: Box<dyn Object>) {
        self.objects.push(obj);
    }

    fn clear_objects(&mut self) {
        self.objects.clear();
    }

    fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    fn clear_lights(&mut self) {
        self.lights.clear();
    }

    pub fn intersect(
        &self,
        ray: &crate::core::ray::Ray,
    ) -> Option<crate::core::intersection::Intersection> {
        todo!()
    }
}
