pub trait Renderer {
    fn render(&self) -> image::DynamicImage;
}
