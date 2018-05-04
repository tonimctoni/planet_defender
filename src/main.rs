extern crate sdl2;

mod textures;
mod planet;
mod satellite;
mod actor;
mod constants;

use actor::Actor;
use constants::{SCREEN_WIDTH, SCREEN_HEIGHT};


use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{INIT_PNG, INIT_JPG};

struct Keys {
    left: bool,
    right: bool
}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
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

    let texture_creator = canvas.texture_creator();
    let textures=textures::Textures::load(&texture_creator);
    let planet=planet::Planet::new();
    let mut satellite=satellite::Satellite::new(&textures);
    satellite.set_center((SCREEN_WIDTH/2) as f64, SCREEN_HEIGHT as f64 - 80.);

    let mut event_pump=sdl_context.event_pump().unwrap();
    let mut keys=Keys{left: false, right: false};
    loop{
        for event in event_pump.poll_iter(){
            use Event::*;
            match event {
                Quit { .. } => return,
                KeyDown { keycode, .. } => {
                    use Keycode::*;
                    match keycode {
                        Some(Escape) => return,
                        Some(Left) => keys.left=true,
                        Some(Right) => keys.right=true,
                        _ => {}
                    }
                },
                KeyUp { keycode, .. } => {
                    use Keycode::*;
                    match keycode {
                        Some(Left) => keys.left=false,
                        Some(Right) => keys.right=false,
                        _ => {}
                    }
                },
                // MouseMotion {x,y, ..} => {
                //     satellite.set_center(x as f64,y as f64);
                //     // satellite.set_xmid(x);
                //     // // satellite.set_ymid(560);
                //     // // let dist_squared=((((SCREEN_WIDTH/2) as i32)-x)*(((SCREEN_WIDTH/2) as i32)-x)) as f64;
                //     // let dist=(SCREEN_WIDTH as f64)/2. - (x as f64);
                //     // let dist_squared=dist*dist;
                //     // // println!("{:?}", (360000.-dist_squared));
                //     // let height=(SCREEN_HEIGHT as f64)-(360000.-dist_squared).sqrt();

                //     // println!("{:?}", x);
                //     // satellite.set_ymid(height as i32);
                // },
                _ => {},
            }
        }

        if keys.left{
            satellite.move_by_minus_speed();
        }

        if keys.right{
            satellite.move_by_speed();
        }

        // planet.draw(&mut canvas);
        // satellite.draw(&mut canvas);
        planet.draw(&mut canvas, &textures);
        satellite.draw(&mut canvas, &textures);
        canvas.present();
    }
}

// x**2 + y**2 == r

// y == sqrt(r - x**2)