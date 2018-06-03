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
mod energy_meter;
mod game_events;


use constants::{SCREEN_WIDTH, SCREEN_HEIGHT, ProjectileKind};

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
    // canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    // println!("{:?}", canvas.blend_mode());
    let textures=textures::Textures::load(&texture_creator);
    // println!("{:?}", textures.energy_meter.blend_mode());
    let mut actor_manager=actor_manager::ActorManager::init(&textures);

    let mut mouse_pos=(0f64, 0f64);
    let mut paused=false;
    let mut energy_meter=energy_meter::EnergyMeter::new();
    let mut game_events=game_events::GameEvents::new();
    let mut pressed_q=false;
    let mut pressed_w=false;
    let mut pressedt_=false;

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
                            pressed_q=true;
                        },
                        Some(W) => {
                            pressed_w=true;
                        },
                        Some(T) => {
                            pressedt_=true;
                        },
                        Some(P) => {
                            paused=!paused;
                        },
                        _ => {}
                    }
                },
                KeyUp { keycode, .. } => {
                    use Keycode::*;
                    match keycode {
                        Some(Q) => {
                            pressed_q=false;
                        },
                        Some(W) => {
                            pressed_w=false;
                        },
                        Some(T) => {
                            pressedt_=false;
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

        if !paused{
            // Manage keys in pressed state
            if pressed_q{
                let kind=ProjectileKind::P01;
                if energy_meter.consume(kind.get_energy_cost()){
                    actor_manager.shoot_projectile(mouse_pos, &textures, kind);
                }
            } else if pressed_w{
                let kind=ProjectileKind::P02;
                if energy_meter.consume(kind.get_energy_cost()){
                    actor_manager.shoot_projectile(mouse_pos, &textures, kind);
                }
            } else if pressedt_{
                if energy_meter.consume(1.){
                    actor_manager.set_triple_shot(true);
                    energy_meter.set_triple_shot(true);
                    game_events.sched_disable_triple_shot();
                }
            }

            // Actor logic that is not a response to an event
            actor_manager.step(mouse_pos, &textures);
            energy_meter.step();
            game_events.step(&mut actor_manager, &mut energy_meter, &textures);
        }

        // Draw
        canvas.copy(&textures.planet, None, None).expect("Render failed");
        actor_manager.draw(&mut canvas, &textures);
        energy_meter.draw(&mut canvas, &textures);
        timer.cap_fps();
        canvas.present();
    }
}



// import numpy as np
// def sc(n):
//     n*=np.pi/180.
//     return np.cos(n), np.sin(n)
//     return ((np.cos(n), -np.sin(n)),(np.sin(n), np.cos(n)))
// Projectile that kills everything, continues on after impact and travels fast (10 or so)
// Projectile that creates an explosion/black hole that kills meteors on impact