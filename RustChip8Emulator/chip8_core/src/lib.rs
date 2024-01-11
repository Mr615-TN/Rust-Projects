use rand::Rng;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDER: u16 = 0x200;
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
    v_registers: [u8; NUM_REGISTERS],
    i_registers: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    delay_timer: u8,
    sound_timer: u8,
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: START_ADDER,
            ram: [0; RAM_SIZE],
            screen_size: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_registers: [0; NUM_REGISTERS],
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
        self.pc = START_ADDER;
        self.ram = [0; RAM_SIZE];
        self.screen_size = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_registers = [0; NUM_REGISTERS];
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
    pub fn get_display(&self) -> &[bool] {
        &self.screen_size
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDER as usize;
        let end = (START_ADDER as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    fn execute(&mut self, op: u16) {
        let digit_1 = (op & 0xF000) >> 12;
        let digit_2 = (op & 0x0F00) >> 8;
        let digit_3 = (op & 0x00F0) >> 4;
        let digit_4 = op & 0x000F;
        
        match(digit_1, digit_2, digit_3, digit_4) {
            (0, 0, 0, 0) => return, // Do Nothing

            (0, 0, 0xE, 0) => {// Clear Screen
                self.screen_size = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }, 

            (0, 0, 0xE, 0xE) => { // Return from Subroutine 
                let return_address = self.pop(); 
                self.pc = return_address;
            }, 

            (1, _, _, _) => { // Jump
                let nnn = op & 0xFFF; 
                self.pc = nnn;
            }, 

            (2, _, _, _) => { // Call Subroutine
                let nnn = op & 0xFFF; 
                self.push(self.pc); 
                self.pc = nnn;
            },

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
                self.v_registers[x] = nn;
            },

            (7, _, _, _) => { // VX += NN
                let x = digit_2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_registers[x] = self.v_registers[x].wrapping_add(nn);
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

            (8, _, _, 5) => { // VX -= VY
                let x = digit_2 as usize;
                let y = digit_3 as usize;

                let (new_x,borrow) = self.v_registers[x].overflowing_sub(self.v_registers[y]);
                let new_f = if borrow { 0 } else { 1 };

                self.v_registers[x] = new_x;
                self.v_registers[0xF] = new_f; 
            },

            (8, _, _, 6) => { // Right Shift by 1
                let x = digit_2 as usize;
                let right_shift = self.v_registers[x] & 1;
                self.v_registers[x] >>= 1;
                self.v_registers[0xF] = right_shift
            },

            (8, _, _, 7) => { // VY - VX
                let x = digit_2 as usize;
                let y = digit_3 as usize;

                let (new_x,borrow) = self.v_registers[y].overflowing_sub(self.v_registers[x]);
                let new_f = if borrow { 0 } else { 1 };

                self.v_registers[x] = new_x;
                self.v_registers[0xF] = new_f; 
            },

            (8, _, _, 0xE) => { // Left Shift by 1
                let x = digit_2 as usize;
                let left_shift = (self.v_registers[x] >> 7) & 1;
                self.v_registers[x] <<= 1;
                self.v_registers[x] = left_shift;
            },

            (9, _, _, 0) => { // VX != VY
                let x = digit_2 as usize;
                let y = digit_3 as usize;
                if self.v_registers[x] != self.v_registers[y] {
                    self.pc += 2;
                }
            },

            (0xA, _, _, _) => { // I = NNN
                let nnn = op & 0xFFF;
                self.i_registers = nnn;
            },

            (0xB, _, _, _) => { // JMP V0 + NNN
                let nnn = op & 0xFFF;
                self.i_registers = (self.v_registers[0] as u16) + nnn;
            },
            
            (0xC, _, _, _) => { // VX = random number and NN
                let x = digit_2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = rand::thread_rng().gen();
                self.v_registers[x] = rng & nn;
            },

            (0xD, _, _, _) => { // Drawing sprites
                let x_coordinate = self.v_registers[digit_2 as usize] as u16;
                let y_coordinate = self.v_registers[digit_3 as usize] as u16;

                let number_rows = digit_4;
                
                let mut flip = false;

                for y_line in 0..number_rows {

                    let address = self.i_registers + y_line as u16;
                    let pixel = self.ram[address as usize];

                    for x_line in 0..8 {
                        if (pixel & (0b1000_0000 >> x_line)) != 0 {

                            let x = (x_coordinate + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coordinate + y_line) as usize % SCREEN_HEIGHT;

                            let index = x + SCREEN_WIDTH * y;

                            flip |= self.screen_size[index];
                            self.screen_size[index] ^= true;
                        }
                    }
                }

                if flip {
                    self.v_registers[0xF] = 1;
                }
                else {
                    self.v_registers[0xF] = 0;
                }
            },

            (0xE, _, 9, 0xE) => { // Skip if Key Pressed
                let x = digit_2 as usize;
                let v_reg_x = self.v_registers[x];
                let key_press = self.keys[v_reg_x as usize];
                if key_press {
                    self.pc += 2;
                }
            },

            (0xE, _, 0xA, 1) => { // Skip if Key no Pressed
                let x = digit_2 as usize;
                let v_reg_x = self.v_registers[x];
                let key_press = self.keys[v_reg_x as usize];
                if !key_press {
                    self.pc += 2;
                } 
            },

            (0xF, _, 0, 7) => { // VX is Delay Timer
                let x = digit_2 as usize;
                self.v_registers[x] = self.delay_timer;
            },

            (0xF, _, 0, 0xA) => { // Wait if key press or no
                let x = digit_2 as usize;
                let mut press_key = false;

                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_registers[x] = i as u8;
                        press_key = true;
                        break;
                    }
                }
                
                if !press_key {
                    self.pc -= 2;
                }
            },

            (0xF, _, 1, 5) => { // Delay Timer is VX
                let x = digit_2 as usize;
                self.delay_timer = self.v_registers[x];
            },

            (0xF, _, 1, 8) => { // Sound Timer is VX
                let x = digit_2 as usize;
                self.sound_timer = self.v_registers[x];
            },

            (0xF, _, 1, 0xE) => { // I += VX
                let x = digit_2 as usize;
                let v_reg_x = self.v_registers[x] as u16;
                self.i_registers = self.i_registers.wrapping_add(v_reg_x);
            },

            (0xF, _, 2, 9) => { // I = Font
                let x = digit_2 as usize;
                let font_i = self.v_registers[x] as u16;
                self.i_registers = font_i * 5;
            },

            (0xF, _, 3, 3) => { // Binary Convert Decimal
                let x = digit_2 as usize;
                let v_reg_x = self.v_registers[x] as f32;
                let hundreds = (v_reg_x / 100.0).floor() as u8;
                let tens = ((v_reg_x / 10.0) % 10.0).floor() as u8;
                let ones = (v_reg_x % 10.0) as u8;
                self.ram[self.i_registers as usize] = hundreds;
                self.ram[(self.i_registers + 1) as usize] = tens;
                self.ram[(self.i_registers + 2) as usize] = ones;
            },

            (0xF, _, 5, 5) => { // Storing V0 through VX including VX
                let x = digit_2 as usize;
                let i = self.i_registers as usize;
                for index in 0..=x {
                    self.ram[i+index] = self.v_registers[index];
                }
            },

            (0xF, _, 6, 5)=> { // Loading I into V0 through VX
                let x = digit_2 as usize;
                let i = self.i_registers as usize;
                for index in 0..=x {
                    self.v_registers[index] = self.ram[i+index];
                }
            },

            (_,_,_,_) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }
}



