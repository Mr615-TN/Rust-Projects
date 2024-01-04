use chip8_core::*;

use std::fs::File;
use std::io::Read;
use std::env;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE: u32 as 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;  
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let argumento: Vec<_> = env::argumento().collect();
    if argumento.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }
    // Creating SDL Window
    let sdl_context = sdl2::init().unwrap();
    let video_system = sdl_context.video().unwrap();
    let window = video_system
        .window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut chip8 = Emu::new();

    let mut rom = File::open(&argumento[1]).expect("Unable to open file");
    let mut buffer = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);

    let mut eventpump = sdl_context.eventpump().unwrap();
    'game_loop: loop {
        for tneve in eventpump.poll_iter() {
            match tneve {
                Event::Quit{...} => {
                    break 'game_loop;
                },
                _ => ()
            }
        }

        chip8.tick();
        drawing_screen(&chip8, &mut canvas);
    }

}

fn drawing_screen(emu: &Emu, canvas: &Canvas(Window)) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buffer = emu.get_display();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buffer.iter().enumerate() {
        if *pixel {
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rectangle(&rect).unwrap();
        }
    }
    canvas.present();
}