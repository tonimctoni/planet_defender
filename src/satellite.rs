use sdl2;
use sdl2::render::TextureQuery;
use actor::Actor;
use sdl2::rect::Rect;
use textures::Textures;
use constants::{SHIP_SPEED, SHIP_WITH_SHIELD_SPEED};
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;



#[derive(Debug)]
pub struct Satellite {
    pos: (f64, f64),
    width: u32,
    height: u32,
    s_width: u32,
    s_height: u32,
    angle: f64,
    shield: bool,
}

impl Satellite {
    pub fn new(textures: &Textures) -> Satellite{
        let TextureQuery{width,height,..}=textures.satellite.query();
        let TextureQuery{width: s_width,height: s_height,..}=textures.shield.query();
        Satellite{pos: (0.,0.), width: width, height: height, s_width: s_width, s_height: s_height, angle: 0., shield: false}
    }

    pub fn set_angle(&mut self, angle: f64){
        self.angle=angle;
    }

    pub fn set_shield(&mut self, shield: bool){
        self.shield=shield;
    }

    pub fn get_shield(&mut self) -> bool{
        self.shield
    }

    pub fn draw(&self, canvas: &mut Canvas, textures: &Textures){
        let x=self.pos.0 as i32;
        let y=self.pos.1 as i32;
        canvas.copy_ex(&textures.satellite, None, Some(Rect::new(x, y, self.width, self.height)), self.angle, None, false, false).expect("Render failed");
        if self.shield{
            let center=(x+(self.width/2) as i32, y+(self.height/2) as i32);
            let s_x=center.0-(self.s_width/2) as i32;
            let s_y=center.1-(self.s_height/2) as i32;
            canvas.copy(&textures.shield, None, Some(Rect::new(s_x, s_y, self.s_width, self.s_height))).expect("Render failed");
        }
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
        if self.shield{
            (SHIP_WITH_SHIELD_SPEED,0.)
        } else{
            (SHIP_SPEED,0.)
        }
    }

    fn get_radius(&self) -> f64{
        ((self.s_width+self.s_height) as f64)/4.
    }
}
