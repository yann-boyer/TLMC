mod ram;
mod display;
mod cpu;
mod emulator;
mod sound_system;

extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

use std::env;

use crate::emulator::Emulator;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const PIXEL_SCALE: usize = 10;

const WINDOW_WIDTH: u32 = (CHIP8_WIDTH * PIXEL_SCALE) as u32;
const WINDOW_HEIGHT: u32 = (CHIP8_HEIGHT * PIXEL_SCALE) as u32;

const CPU_CLOCK_DELAY: u64 = 1;
const TIMER_DIVISION_CLOCK: u8 = 9;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage : ./tlmc <chip8-rom>");
        std::process::exit(1);
    }

    let rom_path = &args[1];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("TLMC Chip8 Emulator by Yann BOYER", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mut emulator = Emulator::new();
    emulator.load_rom(rom_path);

    let mut running = true;

    let mut div_cycles: u8 = 0;
    while running {
        emulator.run_instruction();
        div_cycles += 1;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                },
                Event::KeyDown { keycode, .. } => {
                    match keycode.unwrap() {
                        Keycode::Escape => {
                            running = false;
                        },
                        Keycode::Num1 => {
                            emulator.key(0x1, true);
                        },
                        Keycode::Num2 => {
                            emulator.key(0x2, true);
                        },
                        Keycode::Num3 => {
                            emulator.key(0x3, true);
                        },
                        Keycode::Num4 => {
                            emulator.key(0xC, true);
                        },
                        Keycode::Q => {
                            emulator.key(0x4, true);
                        },
                        Keycode::W => {
                            emulator.key(0x5, true);
                        },
                        Keycode::E => {
                            emulator.key(0x6, true);
                        },
                        Keycode::R => {
                            emulator.key(0xD, true);
                        },
                        Keycode::A => {
                            emulator.key(0x7, true);
                        },
                        Keycode::S => {
                            emulator.key(0x8, true);
                        },
                        Keycode::D => {
                            emulator.key(0x9, true);
                        },
                        Keycode::F => {
                            emulator.key(0xE, true);
                        },
                        Keycode::Z => {
                            emulator.key(0xA, true);
                        },
                        Keycode::X => {
                            emulator.key(0x0, true);
                        },
                        Keycode::C => {
                            emulator.key(0xB, true);
                        },
                        Keycode::V => {
                            emulator.key(0xF, true);
                        },
                        _ => (),
                    }
                },
                Event::KeyUp {keycode, ..} => {
                    match keycode.unwrap() {
                        Keycode::Num1 => {
                            emulator.key(0x1, false);
                        },
                        Keycode::Num2 => {
                           emulator.key(0x2, false);
                        },
                        Keycode::Num3 => {
                            emulator.key(0x3, false);
                        },
                        Keycode::Num4 => {
                            emulator.key(0xC, false);
                        },
                        Keycode::Q => {
                            emulator.key(0x4, false);
                        },
                        Keycode::W => {
                            emulator.key(0x5, false);
                        },
                        Keycode::E => {
                            emulator.key(0x6, false);
                        },
                        Keycode::R => {
                            emulator.key(0xD, false);
                        },
                        Keycode::A => {
                            emulator.key(0x7, false);
                        },
                        Keycode::S => {
                            emulator.key(0x8, false);
                        },
                        Keycode::D => {
                            emulator.key(0x9, false);
                        },
                        Keycode::F => {
                            emulator.key(0xE, false);
                        },
                        Keycode::Z => {
                            emulator.key(0xA, false);
                        },
                        Keycode::X => {
                            emulator.key(0x0, false);
                        },
                        Keycode::C => {
                            emulator.key(0xB, false);
                        },
                        Keycode::V => {
                            emulator.key(0xF, false);
                        },
                        _ => (),
                    }
                },
                _ => {}
            }
        }
        
        if emulator.has_drawn() {
            for y in 0..CHIP8_HEIGHT {
                for x in 0..CHIP8_WIDTH {
                    if emulator.pixel_is_on_at(x as u8, y as u8) {
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                    } else {
                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                    }

                    let pixel = Rect::new(x as i32 * PIXEL_SCALE as i32, y as i32 * PIXEL_SCALE as i32, PIXEL_SCALE as u32, PIXEL_SCALE as u32);
                    canvas.fill_rect(pixel).unwrap();
                }
            }

            emulator.reset_draw_flag();

            canvas.present();
        }

        if div_cycles == TIMER_DIVISION_CLOCK {
            emulator.update_cpu_timers();
            div_cycles = 0;
        }

        std::thread::sleep(Duration::from_millis(CPU_CLOCK_DELAY));
    }
}

