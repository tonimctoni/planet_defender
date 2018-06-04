use sdl2;
use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;
type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

pub struct Textures<'a>{
    pub planet: Texture<'a>,
    pub satellite: Texture<'a>,
    pub energy_meter_flask: Texture<'a>,
    pub energy_meter_holder: Texture<'a>,
    pub shield: Texture<'a>,
    pub projectiles: [Texture<'a>;3],
    pub meteors: [Texture<'a>;3],
}

impl<'a> Textures<'a> {
    pub fn load(texture_creator: &'a TextureCreator) -> Textures<'a>{
        let mut energy_meter_flask=texture_creator.load_texture(Path::new("blender/energy_meter_flask.png")).unwrap();
        energy_meter_flask.set_blend_mode(sdl2::render::BlendMode::Add);
        let mut shield=texture_creator.load_texture(Path::new("blender/shield.png")).unwrap();
        shield.set_blend_mode(sdl2::render::BlendMode::Add);
        Textures{
            planet: texture_creator.load_texture(Path::new("blender/planet.png")).unwrap(),
            satellite: texture_creator.load_texture(Path::new("blender/dummy_satellite.png")).unwrap(),
            energy_meter_flask: energy_meter_flask,
            energy_meter_holder: texture_creator.load_texture(Path::new("blender/energy_meter_holder.png")).unwrap(),
            shield: shield,
            projectiles: [
                texture_creator.load_texture(Path::new("blender/dummy_projectile01.png")).unwrap(),
                texture_creator.load_texture(Path::new("blender/scatter_projectile.png")).unwrap(),
                texture_creator.load_texture(Path::new("blender/pellet.png")).unwrap(),
            ],
            meteors: [
            texture_creator.load_texture(Path::new("blender/meteor01.png")).unwrap(),
            texture_creator.load_texture(Path::new("blender/meteor02.png")).unwrap(),
            texture_creator.load_texture(Path::new("blender/meteor03.png")).unwrap(),
            ],
        }
    }
}