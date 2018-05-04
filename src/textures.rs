use sdl2;
use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;
type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

pub struct Textures<'a>{
    pub planet: Texture<'a>,
    pub satellite: Texture<'a>,
    pub projectile: Texture<'a>,
}

impl<'a> Textures<'a> {
    pub fn load(texture_creator: &'a TextureCreator) -> Textures<'a>{
        Textures{
            planet: texture_creator.load_texture(Path::new("blender/planet.png")).unwrap(),
            satellite: texture_creator.load_texture(Path::new("blender/dummy_satellite.png")).unwrap(),
            projectile: texture_creator.load_texture(Path::new("blender/dummy_projectile.png")).unwrap(),
        }
    }
}

// trait Drawable {
//     // add code here
// }