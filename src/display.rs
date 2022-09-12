const RENDER_TABLE_WIDTH: usize = 64;
const RENDER_TABLE_HEIGHT: usize = 32;
const RENDER_TABLE_SIZE: usize = RENDER_TABLE_WIDTH * RENDER_TABLE_HEIGHT;

pub struct Display {
    render_table: [bool; RENDER_TABLE_SIZE]
}

impl Display {
    pub fn new() -> Display {
        Display {
            render_table: [false; RENDER_TABLE_SIZE]
        }
    }

    pub fn pixel_is_on_at(&self, x: u8, y: u8) -> bool {
        self.render_table[y as usize * RENDER_TABLE_WIDTH + x as usize]
    }

    pub fn set_pixel_state_at(&mut self, x: u8, y: u8, new_state: bool) {
        self.render_table[y as usize * RENDER_TABLE_WIDTH + x as usize] = new_state;
    }

    pub fn clear(&mut self) {
        for y in 0..RENDER_TABLE_HEIGHT {
            for x in 0..RENDER_TABLE_WIDTH {
                self.render_table[y as usize * RENDER_TABLE_WIDTH + x as usize] = false;
            }
        }
    }
}
