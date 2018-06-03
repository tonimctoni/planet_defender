use sdl2;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use textures::Textures;
use constants::{SCREEN_HEIGHT, ENERGY_RECOVERY_PER_FRAME, COOLDOWN_FRAMES};
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;




// pub struct EnergyMeter(f64);
pub struct EnergyMeter{
    energy: f64,
    cd: isize,
    triple_shot: bool,
}


impl EnergyMeter {
    pub fn new() -> EnergyMeter{
        // EnergyMeter(1.)
        EnergyMeter{
            energy: 1.,
            cd: 0,
            triple_shot: false,
        }
    }

    pub fn set_triple_shot(&mut self, triple_shot: bool){
        self.triple_shot=triple_shot;
    }

    pub fn step(&mut self){
        // if self.triple_shot{
        //     self.energy+=ENERGY_RECOVERY_PER_FRAME/2.;
        // } else{
        //     self.energy+=ENERGY_RECOVERY_PER_FRAME;
        // }

        self.energy+=ENERGY_RECOVERY_PER_FRAME;
        if self.energy>1.{
            self.energy=1.
        }

        self.cd-=1;
        if self.cd < 0{
            self.cd=0
        }
    }

    pub fn consume(&mut self, energy: f64) -> bool{
        if self.cd==0 && self.energy>=energy{
            self.energy-=energy;
            self.cd=COOLDOWN_FRAMES;
            true
        } else {
            false
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, textures: &Textures){
        let pos_x=(SCREEN_HEIGHT as i32)/2-200;
        let bar_height=(self.energy*292.) as u32;
        let energy_rect=Rect::new(2, pos_x+54+(292-bar_height) as i32, 45, bar_height as u32);

        if self.triple_shot{
            canvas.set_draw_color(Color::RGB(150,50,0));
        } else {
            canvas.set_draw_color(Color::RGB(50,100,0));
        }

        canvas.fill_rect(energy_rect).expect("Render failed");
        canvas.copy(&textures.energy_meter_flask, None, Some(Rect::new(0,pos_x,50,400))).expect("Render failed");
        canvas.copy(&textures.energy_meter_holder, None, Some(Rect::new(0,pos_x,50,400))).expect("Render failed");
    }
}