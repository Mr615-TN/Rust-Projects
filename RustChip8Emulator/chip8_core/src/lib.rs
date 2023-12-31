pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDER = 0x200;

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen_size: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_registers: [u8, NUM_REGISTERS],
    i_registers: u16,
    sp: u16,
    stack: [u16, STACK_SIZE],
    keys: [bool, NUM_KEYS]
    delay_timer: u8,
    sound_timer: u8,
}

impl Emu {
    pub fn new() -> Self {
        Self {
            pc: START_ADDER,
            ram: [0; RAM_SIZE],
            screen_size: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_registers: [0, NUM_REGISTERS],
            i_registers: 0,
            sp: 0,
            stack: [0, STACK_SIZE],
            keys: [false, NUM_KEYS]
            delay_timer: 0,
            sound_timer: 0,
        }
    }
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
