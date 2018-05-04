extern crate sdl2;

mod textures;
mod planet;
mod satellite;
mod actor;
mod constants;
mod timer_manager;

use actor::Actor;
use constants::{SCREEN_WIDTH, SCREEN_HEIGHT, SHIP_EPSILON};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{INIT_PNG, INIT_JPG};


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let timer = sdl_context.timer().unwrap();
    let _ = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

    let window = video.window("Some Title", SCREEN_WIDTH, SCREEN_HEIGHT)
    .position_centered()
    // .fullscreen_desktop()
    // .fullscreen()
    .opengl()
    .build()
    .unwrap();

    let mut canvas = window
    .into_canvas()
    .software()
    .build()
    .unwrap();

    let mut timer_manager=timer_manager::TimerManager::init(timer);

    let texture_creator = canvas.texture_creator();
    let textures=textures::Textures::load(&texture_creator);
    let planet=planet::Planet::new();
    let mut satellite=satellite::Satellite::new(&textures);
    satellite.set_center((SCREEN_WIDTH/2) as f64, SCREEN_HEIGHT as f64 - 160.);

    let mut mouse_pos=(0f64, 0f64);

    let mut event_pump=sdl_context.event_pump().unwrap();
    loop{
        for event in event_pump.poll_iter(){
            use Event::*;
            match event {
                Quit { .. } => return,
                KeyDown { keycode, .. } => {
                    use Keycode::*;
                    match keycode {
                        Some(Escape) => return,
                        _ => {}
                    }
                },
                MouseMotion {x, y,..} => {
                    mouse_pos=(x as f64,y as f64);
                },
                _ => {},
            }
        }

        let satellite_direction=mouse_pos.0-satellite.get_x_center();
        if satellite_direction.abs()>SHIP_EPSILON{
            if satellite_direction > 0.{
                satellite.move_horizontally_by_speed();
            } else if satellite_direction < 0.{
                satellite.move_horizontally_by_minus_speed();
            }
        }

        let satellite_pos=satellite.get_center();
        let gradient=-(mouse_pos.0 - satellite_pos.0)/(mouse_pos.1 - satellite_pos.1);
        let angle=gradient.atan()*180./std::f64::consts::PI;
        satellite.set_angle(angle);




        planet.draw(&mut canvas, &textures);
        satellite.draw(&mut canvas, &textures);
        timer_manager.cap_fps();
        canvas.present();
    }
}
