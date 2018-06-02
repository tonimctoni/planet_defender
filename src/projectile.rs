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

        Projectile{
            pos: (0.,0.),
            width: width,
            height: height,
            angle: 0.,
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
        // canvas.set_draw_color(Color::RGB(255, 0, 0));
        // canvas.draw_rect(Rect::new(self.pos.0 as i32, self.pos.1 as i32, self.width, self.height)).expect("Render failed");
        // let radius=((self.width+self.height) as f64)/4.;
        // let inner=radius/((2f64).sqrt());
        // let (xcenter, ycenter)=self.get_center();
        // canvas.draw_rect(Rect::new((xcenter-inner) as i32, (ycenter-inner) as i32, (inner*2.) as u32, (inner*2.) as u32)).expect("Render failed");
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
