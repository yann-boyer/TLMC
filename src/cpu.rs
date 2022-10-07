use rand::{Rng, rngs::ThreadRng};

use crate::display::Display;
use crate::ram::Ram;
use crate::sound_system::SoundSystem;

const REGISTERS_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const KEYS_COUNT: usize = 16;
const PC_START: u16 = 0x200;

pub struct Cpu<'a> {
    v: [u8; REGISTERS_COUNT],
    stack: [u16; STACK_SIZE],
    keys: [u8; KEYS_COUNT],
    index_reg: u16,
    delay_timer: u8,
    sound_timer: u8,
    sp: u16,
    pc: u16,
    rng: ThreadRng,
    sound_system: SoundSystem<'a>,
    draw_flag: bool
}

impl <'a> Cpu<'a> {
    pub fn new() -> Cpu<'a> {
        Cpu {
            v: [0x0; REGISTERS_COUNT],
            stack: [0x0; STACK_SIZE],
            keys: [0x0; KEYS_COUNT],
            index_reg: 0x0,
            delay_timer: 0x0,
            sound_timer: 0x0,
            sp: 0x0,
            pc: PC_START,
            rng: rand::thread_rng(),
            sound_system: SoundSystem::new(),
            draw_flag: false
        }
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0x0 {self.delay_timer -= 1;}
        if self.sound_timer > 0x0 {
            self.sound_timer -= 1;
            if self.sound_timer == 0x1 {
                self.sound_system.play_beep_sound();
            }
        }
    }

    pub fn key(&mut self, n: u8, is_down: bool) {
        if is_down {
            self.keys[n as usize] = 1;
        } else {
            self.keys[n as usize] = 0;
        }
    }

    pub fn reset_draw_flag(&mut self) {
        self.draw_flag = false;
    }

    pub fn get_draw_flag(&self) -> bool {
        self.draw_flag
    }

    fn write_reg(&mut self, reg_index: u8, reg_value: u8) {
        self.v[reg_index as usize] = reg_value;
    }

    fn read_reg(&self, reg_index: u8) -> u8 {
        self.v[reg_index as usize]
    }

    fn fetch_next_opcode(&self, ram: &Ram) -> u16 {
        let msb = ram.read(self.pc) as u16;
        let lsb = ram.read(self.pc + 1) as u16;

        msb << 8 | lsb
    }

    pub fn run_instruction(&mut self, ram: &mut Ram, display: &mut Display) {
        let opcode = self.fetch_next_opcode(ram);

        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x00FF {
                    0x00E0 => {
                        display.clear();
                        self.draw_flag = true;
                        self.pc += 2;
                    },
                    0x00EE => {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                        self.pc += 2;
                    },
                    _ => panic!("Unknown opcode -> 0x{:x}", opcode)
                }
            },
            0x1000 => {
                self.pc = nnn;
            },
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            },
            0x3000 => {
                if self.read_reg(x) == nn {self.pc += 4;}
                else {self.pc += 2;}
            },
            0x4000 => {
                if self.read_reg(x) != nn {self.pc += 4;}
                else {self.pc += 2;}
            },
            0x5000 => {
                if self.read_reg(x) == self.read_reg(y) {self.pc += 4;}
                else {self.pc += 2;}
            },
            0x6000 => {
                self.write_reg(x, nn);
                self.pc += 2;
            },
            0x7000 => {
                self.write_reg(x, nn + self.read_reg(x));
                self.pc += 2;
            },
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        self.write_reg(x, self.read_reg(y));
                        self.pc += 2;
                    },
                    0x0001 => {
                        self.write_reg(x, self.read_reg(x) | self.read_reg(y));
                        self.pc += 2;
                    },
                    0x0002 => {
                        self.write_reg(x, self.read_reg(x) & self.read_reg(y));
                        self.pc += 2;
                    },
                    0x0003 => {
                        self.write_reg(x, self.read_reg(x) ^ self.read_reg(y));
                        self.pc += 2;
                    },
                    0x0004 => {
                        let r = self.read_reg(x) as u16 + self.read_reg(y) as u16;

                        if r > 0xFF {self.write_reg(0xF, 1);}
                        else {self.write_reg(0xF, 0);}

                        self.write_reg(x, (r & 0xFF) as u8);
                        self.pc += 2;
                    },
                    0x0005 => {
                        if self.read_reg(x) > self.read_reg(y) {self.write_reg(0xF, 1);}
                        else {self.write_reg(0xF, 0);}

                        self.write_reg(x, self.read_reg(x) - self.read_reg(y));
                        self.pc += 2;
                    },
                    0x0006 => {
                        self.write_reg(0xF, self.read_reg(x) & 0x1);
                        self.write_reg(x, self.read_reg(x) << 1);
                        self.pc += 2;
                    },
                    0x0007 => {
                        if self.read_reg(y) > self.read_reg(x) {self.write_reg(0xF, 1);}
                        else {self.write_reg(0xF, 0);}

                        self.write_reg(x, self.read_reg(y) - self.read_reg(x));
                        self.pc += 2;
                    },
                    0x000E => {
                        self.write_reg(0xF, (self.read_reg(x) & 128) >> 7);
                        self.write_reg(x, self.read_reg(x) >> 1);
                        self.pc += 2;
                    },
                    _ => panic!("Unknown opcode -> 0x{:x}", opcode)
                }
            },
            0x9000 => {
                if self.read_reg(x) != self.read_reg(y) {self.pc += 4;}
                else {self.pc += 2;}
            },
            0xA000 => {
                self.index_reg = nnn;
                self.pc += 2;
            },
            0xB000 => {
                self.pc = self.read_reg(0x0) as u16 + nnn;
            },
            0xC000 => {
                let number = self.rng.gen_range(0x0..0xFF);
                self.write_reg(x, number & nn);
                self.pc += 2;
            },
            0xD000 => {
                let origin_x = self.read_reg(x);
                let origin_y = self.read_reg(y);

                self.write_reg(0xF, 0);
                for y_coord in 0..n {
                    let pixel = ram.read(self.index_reg + y_coord as u16);
                    for x_coord in 0..8 {
                        if pixel & (0x80 >> x_coord) != 0 {
                            let pixel_x = (origin_x.wrapping_add(x_coord)) % 64;
                            let pixel_y = (origin_y.wrapping_add(y_coord)) % 32;

                            if display.pixel_is_on_at(pixel_x, pixel_y) {
                                display.set_pixel_state_at(pixel_x, pixel_y, false);
                                self.write_reg(0xF, 1);
                            } else {
                                display.set_pixel_state_at(pixel_x, pixel_y, true);
                            }
                        }
                    }
                }

                self.draw_flag = true;
                self.pc += 2;
            },
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => {
                        if self.keys[self.read_reg(x) as usize] == 1 {self.pc += 4;}
                        else {self.pc += 2;}
                    },
                    0x00A1 => {
                        if self.keys[self.read_reg(x) as usize] == 0 {self.pc += 4;}
                        else {self.pc += 2;}
                    },
                    _ => panic!("Unknown opcode -> 0x{:x}", opcode)
                }
            },
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => {
                    self.write_reg(x, self.delay_timer);
                    self.pc += 2;
                    },
                    0x000A => {
                        let mut pressed = false;
                        for i in 0..KEYS_COUNT as u8 {
                            if self.keys[i as usize] != 0 {
                                self.write_reg(x, i);
                                pressed = true;
                            }
                        }

                        if !pressed {
                            return;
                        }

                        self.pc += 2;
                    },
                    0x0015 => {
                        self.delay_timer = self.read_reg(x);
                        self.pc += 2;
                    },
                    0x0018 => {
                        self.sound_timer = self.read_reg(x);
                        self.pc += 2;
                    },
                    0x001E => {
                        self.index_reg += self.read_reg(x) as u16;
                        self.pc += 2;
                    },
                    0x0029 => {
                        self.index_reg = self.read_reg(x) as u16 * 5;
                        self.pc += 2;
                    },
                    0x0033 => {
                        // Hundreds.
                        ram.write(self.index_reg, self.read_reg(x) / 100);
                        // Tens.
                        ram.write(self.index_reg + 1, (self.read_reg(x) % 100) / 10);
                        // Units.
                        ram.write(self.index_reg + 2, self.read_reg(x) % 10);

                        self.pc += 2;
                    },
                    0x0055 => {
                        for i in 0..(x + 1) {
                            let reg_value = self.read_reg(i);
                            ram.write(self.index_reg + i as u16, reg_value);
                        }

                        self.pc += 2;
                    },
                    0x0065 => {
                        for i in 0..(x + 1) {
                            let reg_value = ram.read(self.index_reg + i as u16);
                            self.write_reg(i, reg_value);
                        }

                        self.pc += 2;
                    },
                    _ => panic!("Unknown opcode -> 0x{:x}", opcode)
                }
            },
            _ => panic!("Unknown opcode -> 0x{:x}", opcode)
        }
    }
}
