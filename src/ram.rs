const RAM_SIZE: usize = 0x1000;

pub struct Ram {
    ram: [u8; RAM_SIZE]
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            ram: [0x0; RAM_SIZE]
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        if addr > RAM_SIZE as u16 {
            println!("Memory Error : READ command out of range !");
            return 0x0;
        }

        return self.ram[addr as usize];
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if addr > RAM_SIZE as u16 {
            println!("Memory Error : WRITE command out of range !");
            return;
        }

        self.ram[addr as usize] = value;
    }
}