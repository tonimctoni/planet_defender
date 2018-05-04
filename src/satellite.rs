use sdl2;
use textures;
use sdl2::render::TextureQuery;
use actor::Actor;
use sdl2::rect::Rect;
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;



pub struct Satellite {
    pos: (f64, f64),
    width: u32,
    height: u32
}

impl Satellite {
    pub fn new(textures: &textures::Textures) -> Satellite{
        let TextureQuery{width,height,..}=textures.satellite.query();
        Satellite{pos: (0.,0.), width: width, height: height}
    }

    pub fn draw(&self, canvas: &mut Canvas, textures: &textures::Textures){
        canvas.copy(&textures.satellite, None, Some(Rect::new(self.pos.0 as i32, self.pos.1 as i32, self.width, self.height))).expect("Render failed");
    }
}

impl Actor for Satellite {
    fn get_mut_pos(&mut self) -> &mut (f64,f64){
        &mut self.pos
    }

    fn get_dims(&self) -> (f64, f64) {
        (self.width as f64, self.height as f64)
    }

    fn get_speed(&self) -> (f64, f64){
        (2.5,0.)
    }
}
