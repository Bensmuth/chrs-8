extern crate sdl2; 

use sdl2::pixels::Color;

pub fn windowsetup(sdl_context : &sdl2::Sdl) -> sdl2::render::Canvas<sdl2::video::Window>{
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("chrs-8", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    return canvas
}