use super::camera::Camera;
use super::light::Light;
use super::object::Object;

struct Scene {
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
}
