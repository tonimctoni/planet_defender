use sdl2;
use textures;
use sdl2::render::TextureQuery;
use actor::Actor;
use sdl2::rect::Rect;
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;


pub struct Meteor {
    pos: (f64, f64),
    width: u32,
    height: u32,
    angle: f64,
    vertical_speed: f64,
}

impl Meteor {
    pub fn new(textures: &textures::Textures) -> Meteor{
        let TextureQuery{width,height,..}=textures.meteor.query();
        Meteor{pos: (0.,0.), width: width, height: height, angle: 0., vertical_speed: 0.}
    }

    pub fn set_angle(&mut self, angle: f64){
        self.angle=angle;
    }

    pub fn set_speed(&mut self, vertical_speed: f64){
        self.vertical_speed=vertical_speed;
    }

    pub fn draw(&self, canvas: &mut Canvas, textures: &textures::Textures){
        canvas.copy_ex(&textures.meteor, None, Some(Rect::new(self.pos.0 as i32, self.pos.1 as i32, self.width, self.height)), self.angle, None, false, false).expect("Render failed");
    }
}

impl Actor for Meteor {
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
        (0., self.vertical_speed)
    }
}
