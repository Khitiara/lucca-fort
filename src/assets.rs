use std::io::Cursor;

#[derive(Debug)]
pub enum ResourceKey {
    Image(String)
}

pub enum Resource {
    Image(glium::Texture2d),
}

pub fn load_resource(key: ResourceKey, d: &glium::Display) -> Resource {
    match key {
        ResourceKey::Image(path) => {
            let image = image::load(Cursor::new(path), image::PNG).unwrap().to_rgba();
            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
            let texture = glium::Texture2d::new(d, image).unwrap();
            Resource::Image(texture)
        }
    }
}