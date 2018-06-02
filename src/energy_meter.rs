use sdl2;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use constants::{SCREEN_HEIGHT_F64, ENERGY_RECOVERY_PER_FRAME, COOLDOWN_FRAMES};
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;




// pub struct EnergyMeter(f64);
pub struct EnergyMeter{
    energy: f64,
    cd: isize,
}


impl EnergyMeter {
    pub fn new() -> EnergyMeter{
        // EnergyMeter(1.)
        EnergyMeter{
            energy: 1.,
            cd: 0
        }
    }

    pub fn step(&mut self){
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

    pub fn draw(&self, canvas: &mut Canvas){
        let energy_rect=Rect::new(0, ((1.-self.energy)*SCREEN_HEIGHT_F64) as i32, 20, (self.energy*SCREEN_HEIGHT_F64) as u32);
        canvas.set_draw_color(Color::RGB(((1.-self.energy)*200.) as u8+55, (self.energy*200.) as u8+55, 0));
        canvas.fill_rect(energy_rect).expect("Render failed");
    }
}