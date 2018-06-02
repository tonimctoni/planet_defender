use sdl2;
use textures;
use sdl2::render::TextureQuery;
use actor::Actor;
use sdl2::rect::Rect;
use constants::SHIP_SPEED;
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;



#[derive(Debug)]
pub struct Satellite {
    pos: (f64, f64),
    width: u32,
    height: u32,
    angle: f64,
}

impl Satellite {
    pub fn new(textures: &textures::Textures) -> Satellite{
        let TextureQuery{width,height,..}=textures.satellite.query();
        Satellite{pos: (0.,0.), width: width, height: height, angle: 0.}
    }

    pub fn set_angle(&mut self, angle: f64){
        self.angle=angle;
    }

    pub fn draw(&self, canvas: &mut Canvas, textures: &textures::Textures){
        // canvas.copy(&textures.satellite, None, Some(Rect::new(self.pos.0 as i32, self.pos.1 as i32, self.width, self.height))).expect("Render failed");
        canvas.copy_ex(&textures.satellite, None, Some(Rect::new(self.pos.0 as i32, self.pos.1 as i32, self.width, self.height)), self.angle, None, false, false).expect("Render failed");
    }
}

impl Actor for Satellite {
    fn get_mut_pos(&mut self) -> &mut (f64,f64){
        &mut self.pos
    }

    fn get_pos(&self) -> (f64,f64){
        self.pos
    }

    fn get_dims(&self) -> (f64, f64) {
        (self.width as f64, self.height as f64)
    }

    fn get_speed(&self) -> (f64, f64){
        (SHIP_SPEED,0.)
    }
}
