mod cpu;
mod display;
// !! https://github.com/DavidJowett/chip8-emulator !! got deets on cpu instructions

extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use std::collections::LinkedList;



fn main() {

    let mut memory = [0;4096];

    let mut file=File::open("resources/test_opcode.ch8").unwrap(); // ! dirty rom load, replace this when cartridge controller implemented
    let mut buf=[0u8;0xFF];
    file.read(&mut buf).unwrap();
    for x in 0..0xFF { // ! dirty rom into memeory merge, probs wont ever fix this
        memory[x + 0x200] = buf[x];
    };

    let mut main_cpu = cpu::Cpu::new(memory, [0;16], LinkedList::new(), [[false; 64]; 32], 0, 0x200);
    main_cpu.debug(); // ! make modifications to registers for testing

    
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = display::windowsetup(&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0x00, 0x0, 0x0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        main_cpu.tick();
        canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        for x in 0..63{ // gfx drawing
            for y in 0..31{
                if main_cpu.gfx[y][x] == true{
                    let rect = sdl2::rect::Rect::new((x * 10) as i32,(y*10) as i32,10,10);
                    canvas.fill_rect(rect).map_err(|err| println!("{:?}", err)).ok();
                }
            } 
        }



        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
