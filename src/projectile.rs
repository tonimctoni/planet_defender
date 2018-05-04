use sdl2;
use textures;
use sdl2::render::TextureQuery;
use actor::Actor;
use sdl2::rect::Rect;
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;



pub struct Projectile {
    pos: (f64, f64),
    width: u32,
    height: u32,
    angle: f64,
    speed: (f64,f64)
}

impl Projectile {
    pub fn new(textures: &textures::Textures) -> Projectile{
        let TextureQuery{width,height,..}=textures.projectile.query();
        Projectile{pos: (0.,0.), width: width, height: height, angle: 0., speed: (0.,0.)}
    }

    // pub fn set_angle(&mut self, angle: f64){ // set it using speed
    //     self.angle=angle;
    // }

    pub fn set_speed(&mut self, speed_x: f64, speed_y: f64){
        self.speed=(speed_x, speed_y);
    }

    pub fn draw(&self, canvas: &mut Canvas, textures: &textures::Textures){
        // canvas.copy(&textures.Projectile, None, Some(Rect::new(self.pos.0 as i32, self.pos.1 as i32, self.width, self.height))).expect("Render failed");
        canvas.copy_ex(&textures.projectile, None, Some(Rect::new(self.pos.0 as i32, self.pos.1 as i32, self.width, self.height)), self.angle, None, false, false).expect("Render failed");
    }
}

impl Actor for Projectile {
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
        self.speed
    }
}

// impl Drop for Projectile {
//     fn drop(&mut self){
//         println!("drop");
//     }
// }