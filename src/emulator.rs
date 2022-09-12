use std::io::{Read, BufReader};
use std::fs::File;

use crate::cpu::Cpu;
use crate::ram::Ram;
use crate::display::Display;

static FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emulator {
    cpu: Cpu,
    ram: Ram,
    display: Display
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: Cpu::new(),
            ram: Ram::new(),
            display: Display::new()
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.ram, &mut self.display);
    }

    pub fn reset_draw_flag(&mut self) {
        self.cpu.reset_draw_flag();
    }

    pub fn has_drawn(&self) -> bool {
        self.cpu.get_draw_flag()
    }

    pub fn update_cpu_timers(&mut self) {
        self.cpu.update_timers();
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        // load FONTSET into memory.
        for i in 0..FONTSET.len() {
            self.ram.write(i as u16, FONTSET[i]);
        }

        let mut rom_buffer = Vec::new();

        match File::open(rom_path) {
            Ok(rom_file) => {
                let mut reader = BufReader::new(rom_file);

                match reader.read_to_end(&mut rom_buffer) {
                    Ok(_) => (),
                    Err(why) => {
                        println!("Error : Unable to read the given file !");
                        println!("Why -> {:?}", why);
                    }
                }
            },
            Err(why) => {
                println!("Error : Unable to open the given file !");
                println!("Why -> {:?}", why);
                std::process::exit(1);
            }
        }

        // load rom content into the memory.
        if rom_buffer.len() <= 0xFFF - 0x200 {
            for i in 0..rom_buffer.len() {
                self.ram.write(i as u16 + 0x200, rom_buffer[i]);
            }
        } else {
            println!("Error : This ROM file is too large to fit into memory !");
        }
    }

    pub fn pixel_is_on_at(&self, x: u8, y: u8) -> bool {
        self.display.pixel_is_on_at(x, y)
    }

    pub fn key(&mut self, n: u8, is_down: bool) {
        self.cpu.key(n, is_down);
    }
}
