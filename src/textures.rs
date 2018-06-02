use sdl2;
use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;
type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

pub struct Textures<'a>{
    pub planet: Texture<'a>,
    pub satellite: Texture<'a>,
    // pub projectile01: Texture<'a>,
    // pub projectile02: Texture<'a>,
    // pub pellet: Texture<'a>,
    pub meteor: Texture<'a>,
    // textures: Vec<Texture<'a>>,
    pub projectiles: [Texture<'a>;3],
}

impl<'a> Textures<'a> {
    pub fn load(texture_creator: &'a TextureCreator) -> Textures<'a>{
        Textures{
            planet: texture_creator.load_texture(Path::new("blender/planet.png")).unwrap(),
            satellite: texture_creator.load_texture(Path::new("blender/dummy_satellite.png")).unwrap(),
            // projectile01: texture_creator.load_texture(Path::new("blender/dummy_projectile01.png")).unwrap(),
            // projectile02: texture_creator.load_texture(Path::new("blender/dummy_projectile02.png")).unwrap(),
            // pellet: texture_creator.load_texture(Path::new("blender/dummy_pellet.png")).unwrap(),
            meteor: texture_creator.load_texture(Path::new("blender/meteor01.png")).unwrap(),
            projectiles: [
                texture_creator.load_texture(Path::new("blender/dummy_projectile01.png")).unwrap(),
                texture_creator.load_texture(Path::new("blender/dummy_projectile02.png")).unwrap(),
                texture_creator.load_texture(Path::new("blender/dummy_pellet.png")).unwrap(),
            ]
        }
    }
}