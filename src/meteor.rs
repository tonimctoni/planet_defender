use sdl2;
use textures;
use sdl2::render::TextureQuery;
use actor::Actor;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use constants::MeteorKind;
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

pub struct Meteor {
    pos: (f64, f64),
    width: u32,
    height: u32,
    angle: f64,
    vertical_speed: f64,
    hp: f64,
    kind: MeteorKind,
}

impl Meteor {
    pub fn new(textures: &textures::Textures, kind: MeteorKind) -> Meteor{
        let TextureQuery{width,height,..}=textures.meteors[kind.get_mtexture_i()].query();
        Meteor{pos: (0.,0.), width: width, height: height, angle: 0., vertical_speed: kind.get_speed(), hp: kind.get_max_hp(), kind: kind}
    }

    pub fn damage(&mut self, damage: f64) -> bool{
        self.hp-=damage;
        self.hp <= 0.
    }

    pub fn set_angle(&mut self, angle: f64){
        self.angle=angle;
    }

    pub fn set_speed_to_lowest(&mut self, other: &mut Meteor){
        self.vertical_speed=other.vertical_speed;
        if self.vertical_speed < other.vertical_speed{
            other.vertical_speed=self.vertical_speed;
        } else if other.vertical_speed < self.vertical_speed{
            self.vertical_speed=other.vertical_speed;
        };
    }

    pub fn step(&mut self){
        self.move_by_speed();
        if self.kind.get_rotation()!=0.{
            self.angle-=self.kind.get_rotation();
            if self.angle < 0.{
                self.angle=360.-self.angle;
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, textures: &textures::Textures){
        canvas.copy_ex(
            &textures.meteors[self.kind.get_mtexture_i()],
            None,
            Some(Rect::new(self.pos.0 as i32, self.pos.1 as i32, self.width, self.height)),
            self.angle,
            None,
            false,
            false
        ).expect("Render failed");
        if self.hp!=self.kind.get_max_hp(){
            let proportion_hp=self.hp/self.kind.get_max_hp();
            let hp_rect=Rect::new(self.pos.0 as i32+1, self.pos.1 as i32+1, (((self.width-2) as f64)*proportion_hp) as u32, 4);
            let h=proportion_hp*proportion_hp;
            canvas.set_draw_color(Color::RGB(((1.-h)*255.) as u8, (h*255.) as u8, 0));
            canvas.fill_rect(hp_rect).expect("Render failed");
        }
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

    fn get_radius(&self) -> f64{
        let (width,height)=self.get_dims();
        (width+height)/4.-4.
    }
}
