use std::f64::consts::PI;
use sdl2;
use textures;
use sdl2::render::TextureQuery;
use actor::Actor;
use sdl2::rect::Rect;
use constants::ProjectileKind;
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;



pub struct Projectile {
    pos: (f64, f64),
    width: u32,
    height: u32,
    angle: f64,
    speed: (f64,f64),
    kind: ProjectileKind,
}

impl Projectile {
    pub fn new(textures: &textures::Textures, kind: ProjectileKind, velocity: (f64, f64)) -> Projectile{
        let TextureQuery{width,height,..}=textures.projectiles[kind.get_ptexture_i()].query();
        let speed=kind.get_speed();

        let angle=if kind==ProjectileKind::P01{
            0.
        } else{
            360.-(-velocity.1).atan2(velocity.0)*(180./PI)
        };

        Projectile{
            pos: (0.,0.),
            width: width,
            height: height,
            angle: angle,
            speed: (speed*velocity.0,speed*velocity.1),
            kind}
    }

    pub fn get_damage(&self) -> f64{
        self.kind.get_damage()
    }

    pub fn is_kind(&self, kind: ProjectileKind) -> bool{
        self.kind==kind
    }

    pub fn draw(&self, canvas: &mut Canvas, textures: &textures::Textures){
        if self.angle==0.{
            canvas.copy(&textures.projectiles[self.kind.get_ptexture_i()], None, Some(Rect::new(self.pos.0 as i32, self.pos.1 as i32, self.width, self.height))).expect("Render failed");
        } else {
            canvas.copy_ex(&textures.projectiles[self.kind.get_ptexture_i()], None, Some(Rect::new(self.pos.0 as i32, self.pos.1 as i32, self.width, self.height)), self.angle, None, false, false).expect("Render failed");
        }
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


// (0.8717454193280995, -0.48995910429389533), 29.337893654673955
// (-0.8579458929422961, -0.5137400556540694), -30.913275682128514