extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("Some Title", 800, 600)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

    // let renderer = window
    // .renderer()
    // .accelerated()
    // .build()
    // .unwrap();


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
                _ => {},
            }
        }
    }
}
