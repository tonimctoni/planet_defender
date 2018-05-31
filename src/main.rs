extern crate sdl2;
extern crate rand;

mod textures;
mod satellite;
mod actor;
mod constants;
mod timer;
mod projectile;
mod meteor;
mod actor_manager;

use constants::{SCREEN_WIDTH, SCREEN_HEIGHT};

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

    let mut timer=timer::Timer::init(timer);

    let texture_creator = canvas.texture_creator();
    let textures=textures::Textures::load(&texture_creator);
    let mut actor_manager=actor_manager::ActorManager::init(&textures);

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
                        Some(Q) => {
                            actor_manager.shoot_projectile(mouse_pos, &textures);
                        },
                        Some(E) => {
                            actor_manager.drop_meteor(&textures);
                        },
                        _ => {}
                    }
                },
                MouseMotion {x, y,..} => {
                    mouse_pos=(x as f64,y as f64);
                },
                _ => {},
            }
        }

        // Actor logic that is not a response to an event
        actor_manager.step(mouse_pos);

        // Draw
        canvas.copy(&textures.planet, None, None).expect("Render failed");
        actor_manager.draw(&mut canvas, &textures);
        timer.cap_fps();
        canvas.present();
    }
}



// Maybe add life bars to meteors that don't have full hp
// a2+b2=c2
// a2+a2=c2
// 2(a2)=c2
// a2=(c2)/2
// a=sqrt(c2)/sqrt(2)
// a=c/sqrt(2)