extern crate sdl2;

use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureQuery};
type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;


struct Textures<'a>{
    planet: Texture<'a>,
    satellite: Texture<'a>,
}

impl<'a> Textures<'a> {
    fn load(texture_creator: &'a TextureCreator) -> Textures<'a>{
        Textures{
            planet: texture_creator.load_texture(Path::new("blender/planet.png")).unwrap(),
            satellite: texture_creator.load_texture(Path::new("blender/dummy_satellite.png")).unwrap(),
        }
    }
}

struct Planet<'a> {
    texture: &'a Texture<'a>
}

impl<'a> Planet<'a> {
    fn new(texture: &'a Textures) -> Planet<'a>{
        Planet{texture: &texture.planet}
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        canvas.copy(self.texture, None, None).expect("Render failed");
    }
}

struct Satellite<'a> {
    texture: &'a sdl2::render::Texture<'a>,
    rect: Rect,
}

impl<'a> Satellite<'a> {
    fn new(texture: &'a Textures) -> Satellite<'a>{
        let TextureQuery{width,height,..}=texture.satellite.query();
        Satellite{texture: &texture.satellite, rect: Rect::new(0,0,width,height)}
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        canvas.copy(self.texture, None, Some(self.rect)).expect("Render failed");
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let _ = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

    let window = video.window("Some Title", 1200, 800)
    .position_centered()
    .fullscreen_desktop()
    .opengl()
    .build()
    .unwrap();

    let mut canvas = window
    .into_canvas()
    .software()
    .build()
    .unwrap();

    let texture_creator = canvas.texture_creator();
    let textures=Textures::load(&texture_creator);
    let planet=Planet::new(&textures);
    let satellite=Satellite::new(&textures);
    // drop(canvas);
    // let ()=texture_creator;
    // let textures = Textures::init(&texture_creator);

    // let texture = texture_creator.load_texture(Path::new("blender/planet.png")).unwrap();

    // let mf=MeteorFactory::init(texture_creator.load_texture(Path::new("dummy_satellite.png")).unwrap());
    // canvas.copy(&texture, None, None).expect("Render failed");
    // canvas.present();

    // drop(texture_creator);
    // drop(window);

    planet.draw(&mut canvas);
    satellite.draw(&mut canvas);
    canvas.present();
    // canvas.copy(&textures.planet, None, None).expect("Render failed");
    let mut event_pump=sdl_context.event_pump().unwrap();
    loop{
        for event in event_pump.poll_iter(){
            use Event::*;
            match event {
                Quit { .. } => return,
                KeyDown { keycode, .. } => {
                    use Keycode::*;
                    match keycode {
                        Some(Escape) => return,
                        _ => {}
                    }
                },
                _ => {},
            }
        }
    }
}
