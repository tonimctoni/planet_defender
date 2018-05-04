use sdl2;
use textures;
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;



pub struct Planet {
}

impl Planet {
    pub fn new() -> Planet{
        Planet{}
    }

    pub fn draw(&self, canvas: &mut Canvas, textures: &textures::Textures){
        canvas.copy(&textures.planet, None, None).expect("Render failed");
    }
}
