use rand::Rng;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDER = 0x200;
const FONTSET_SIZE: usize = 80;


const FONTSET: [u8; FONTSET_SIZE] = [
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
0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

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
        let mut new_emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            15
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_registers: [0; NUM_REGS],
            i_registers: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        };
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
    }
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_registers = [0; NUM_REGS];
        self.i_registers = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }
    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();
        // Decode
        // Execute
        self.execute(op);
        }
    fn fetch(&mut self) -> u16 {
        let higher_bytes = self.ram[self.pc as usize] as u16;
        let lower_bytes = self.ram[(self.pc+1) as usize] as u16;
        let op = (higher_bytes << 8) | lower_bytes;
        self.pc += 2;
        op
    }
    pub fn tick_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // BEEP
            }
            self.sound_timer -= 1;
        }
    }
    fn execute(&mut self, op: u16) {
        let digit_1 = (op & 0xF000) >> 12;
        let digit_2 = (op & 0x0F00) >> 8;
        let digit_3 = (op & 0x00F0) >> 4;
        let digit_4 = op & 0x000F;
        
        match(digit_1, digit_2, digit_3, digit_4) {
            (0, 0, 0, 0) => return, // Do Nothing

            (0, 0, 0xE, 0) => {// Clear Screen
                self.screen = [false, SCREEN_WIDTH * SCREEN_HEIGHT];}, 

            (0, 0, 0xE, 0xE) => { // Return from Subroutine 
                let return_address = self.pop(); 
                self.pc = ret_address;}, 

            (1, _, _, _) => { // Jump
                let nnn = op & 0xFFF; 
                self.pc = nnn;}, 

            (2, _, _, _) => { // Call Subroutine
                let nnn = op & 0xFFF; 
                self.push(self.pc); 
                self.pc = nnn;},

            (3, _, _, _) => { // Skip if VX == NN
                let x = digit_2 as usize; 
                let nn = (op & 0xFF) as u8;
                if self.v_registers[x] == nn {
                    self.pc += 2;
                }
            },

            (4, _, _, _) => { // Skip if VX != NN
                let x = digit_2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_registers[x] != nn {
                    self.pc += 2;
                }
            },

            (5, _, _, 0) => { // Skip if VX == VY
                let x = digit_2 as usize;
                let y = digit_3 as usize;
                if self.v_registers[x] == self.v_registers[y] {
                    self.pc += 2;
                }
            },

            (6, _, _, _) => { // VX == NN
                let x = digit_2 as usize;
                let nn = (op & 0xFF) as u8;
                v_registers[x] = nn;
            },

            (7, _, _, _) => { // VX += NN
                let x = digit_2 as usize;
                let nn = (op & 0xFF) as u8;
                v_registers[x] = self.v_registers[x].wrapping_add(nn);
            },

            (8,_,_,0) => { // VX = VY
                let x = digit_2 as usize;
                let y = digit_3 as usize;
                self.v_registers[x] = self.v_registers[y];
            },

            (8, _, _, 1) => { // Bitwise Operations 8xy1
                let x = digit_2 as usize;
                let y = digit_3 as usize;
                self.v_registers[x] |= self.v_registers[y];
            },

            (8, _, _, 2) => { // Bitwise Operations 8xy2
                let x = digit_2 as usize;
                let y = digit_3 as usize;
                self.v_registers[x] |= self.v_registers[y];
            },

            (8, _, _, 3) => { // Bitwise Operations 8xy3
                let x = digit_2 as usize;
                let y = digit_3 as usize;
                self.v_registers[x] |= self.v_registers[y];
            },

            (8, _, _, 4) => { // VX += VY
                let x = digit_2 as usize;
                let y = digit_3 as usize;

                let (new_x,carry) = self.v_registers[x].overflowing_add(self.v_registers[y]);
                let new_f = if carry { 1 } else { 0 };

                self.v_registers[x] = new_x;
                self.v_registers[0xF] = new_f; 
            },

            (_,_,_,_) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }
}



