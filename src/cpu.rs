use crate::constants::*;
use rand::Rng;
use std::path::Path;

enum Operation {
    Noop,
    Next,
    Skip,
    Jump(usize),
}

impl Operation {
    fn skip_if(predicate: bool) -> Self {
        if predicate {
            Self::Skip
        } else {
            Self::Next
        }
    }
}

pub struct CpuOutput<'a> {
    pub should_beep: bool,
    pub should_draw: bool,
    pub vram: &'a [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

pub struct Cpu {
    vram: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
    ram: [u8; RAM_SIZE],
    v: [u8; 16],
    i: usize,
    stack: [usize; 12],
    sp: usize,
    pc: usize,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; 16],
    keypad_wait_input: usize,
    should_draw: bool,
    should_keypad_wait: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            vram: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            ram: [0; RAM_SIZE],
            v: [0; 16],
            i: 0,
            stack: [0; 12],
            sp: 0,
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            keypad_wait_input: 0,
            should_draw: true,
            should_keypad_wait: false,
        }
    }

    pub fn init(&mut self, filepath: &Path) -> std::result::Result<(), Box<dyn std::error::Error>> {
        self.load_fonts();
        self.load_rom(filepath)?;

        Ok(())
    }

    pub fn process(&mut self, keypad: [bool; 16]) -> CpuOutput {
        self.keypad = keypad;
        self.should_draw = false;

        if self.should_keypad_wait {
            for key in 0..keypad.len() {
                if keypad[key] {
                    self.should_keypad_wait = false;
                    self.v[self.keypad_wait_input] = key as u8;
                    break;
                }
            }
        } else {
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }

            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            let opcode = self.get_opcode();
            self.exec_opcode(opcode);
        }

        CpuOutput {
            should_beep: self.sound_timer > 0,
            should_draw: self.should_draw,
            vram: &self.vram,
        }
    }

    fn get_opcode(&self) -> u16 {
        (self.ram[self.pc] as u16) << 8 | (self.ram[self.pc + 1] as u16)
    }

    fn load_fonts(&mut self) {
        self.ram[0..80].copy_from_slice(&FONT);
    }

    fn load_rom(&mut self, filepath: &Path) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let rom = std::fs::read(filepath)?;
        let rom_end = self.pc + rom.len();

        if (self.pc + rom_end) > MAX_ROM_MEMORY {
            return Err(format!(
                "ROM filesize too big! Cannot be bigger than {} bytes!",
                MAX_ROM_MEMORY
            )
            .into());
        }

        self.ram[self.pc..rom_end].copy_from_slice(&rom.as_slice());

        Ok(())
    }

    fn exec_opcode(&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );

        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        let operation: Operation = match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.op_00e0(),  // CLS
            (0x0, 0x0, 0xE, 0xE) => self.op_00ee(),  // RET
            (0x1, _, _, _) => self.op_1nnn(nnn),     // JP addr
            (0x2, _, _, _) => self.op_2nnn(nnn),     // CALL addr
            (0x3, _, _, _) => self.op_3xkk(x, kk),   // SE Vx, byte
            (0x4, _, _, _) => self.op_4xkk(x, kk),   // SNE Vx, byte
            (0x5, _, _, 0x0) => self.op_5xy0(x, y),  // SE Vx, Vy
            (0x6, _, _, _) => self.op_6xkk(x, kk),   // LD Vx, byte
            (0x7, _, _, _) => self.op_7xkk(x, kk),   // ADD Vx, byte
            (0x8, _, _, 0x0) => self.op_8xy0(x, y),  // LD Vx, Vy
            (0x8, _, _, 0x1) => self.op_8xy1(x, y),  // OR Vx, Vy
            (0x8, _, _, 0x2) => self.op_8xy2(x, y),  // AND Vx, Vy
            (0x8, _, _, 0x3) => self.op_8xy3(x, y),  // XOR Vx, Vy
            (0x8, _, _, 0x4) => self.op_8xy4(x, y),  // ADD Vx, Vy
            (0x8, _, _, 0x5) => self.op_8xy5(x, y),  // SUB Vx, Vy
            (0x8, _, _, 0x6) => self.op_8x06(x),     // SHR Vx {, Vy}
            (0x8, _, _, 0x7) => self.op_8xy7(x, y),  // SUBN Vx, Vy
            (0x8, _, _, 0xE) => self.op_8x0e(x),     // SHL Vx {, Vy}
            (0x9, _, _, 0x0) => self.op_9xy0(x, y),  // SNE Vx, Vy
            (0xA, _, _, _) => self.op_annn(nnn),     // LD I, addr
            (0xB, _, _, _) => self.op_bnnn(nnn),     // JP V0, addr
            (0xC, _, _, _) => self.op_cxkk(x, kk),   // RND Vx, byte
            (0xD, _, _, _) => self.op_dxyn(x, y, n), // DRW Vx, Vy, nibble
            (0xE, _, 0x9, 0xE) => self.op_ex9e(x),   // SKP Vx
            (0xE, _, 0xA, 0x1) => self.op_exa1(x),   // SKNP Vx
            (0xF, _, 0x0, 0x7) => self.op_fx07(x),   // LD Vx, DT
            (0xF, _, 0x0, 0xA) => self.op_fx0a(x),   // LD Vx {, K}
            (0xF, _, 0x1, 0x5) => self.op_fx15(x),   // LD DT, Vx
            (0xF, _, 0x1, 0x8) => self.op_fx18(x),   // LD ST, Vx
            (0xF, _, 0x1, 0xE) => self.op_fx1e(x),   // ADD I, Vx
            (0xF, _, 0x2, 0x9) => self.op_fx29(x),   // LD F, Vx
            (0xF, _, 0x3, 0x3) => self.op_fx33(x),   // LD B, Vx
            (0xF, _, 0x5, 0x5) => self.op_fx55(x),   // LD [I], Vx
            (0xF, _, 0x6, 0x5) => self.op_fx65(x),   // LD Vx, [I]
            _ => Operation::Next,
        };

        match operation {
            Operation::Noop => {},
            Operation::Next => self.pc += OPCODE_SIZE,
            Operation::Skip => self.pc += OPCODE_SIZE * 2,
            Operation::Jump(addr) => self.pc = addr,
        }
    }

    /// CLS
    ///
    /// Clear the display.
    fn op_00e0(&mut self) -> Operation {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.vram[y][x] = 0
            }
        }

        Operation::Next
    }

    /// RET
    ///
    /// Return from a subroutine.
    /// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    fn op_00ee(&mut self) -> Operation {
        self.pc = self.stack[self.sp];
        self.sp -= 1;
        Operation::Noop
    }

    /// JP addr
    ///
    /// Jump to location nnn.
    ///The interpreter sets the program counter to nnn.
    fn op_1nnn(&mut self, addr: usize) -> Operation {
        Operation::Jump(addr)
    }

    /// CALL addr
    ///
    /// Call subroutine at nnn.
    /// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    fn op_2nnn(&mut self, addr: usize) -> Operation {
        self.sp += 1;
        self.stack[self.sp] = self.pc + OPCODE_SIZE;
        Operation::Jump(addr)
    }

    /// SE Vx, byte
    ///
    /// Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn op_3xkk(&mut self, x: usize, kk: u8) -> Operation {
        Operation::skip_if(self.v[x] == kk)
    }

    /// SNE Vx, byte
    ///
    /// Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn op_4xkk(&mut self, x: usize, kk: u8) -> Operation {
        Operation::skip_if(self.v[x] != kk)
    }

    /// SE Vx, Vy
    ///
    /// Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn op_5xy0(&mut self, x: usize, y: usize) -> Operation {
        Operation::skip_if(self.v[x] == self.v[y])
    }

    /// LD Vx, byte
    ///
    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    fn op_6xkk(&mut self, x: usize, kk: u8) -> Operation {
        self.v[x] = kk as u8;
        Operation::Next
    }

    /// ADD Vx, byte
    ///
    /// Set Vx = Vx + kk.
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn op_7xkk(&mut self, x: usize, kk: u8) -> Operation {
        let vx = self.v[x] as u16;
        let val = kk as u16;
        let result = vx + val;
        self.v[x] = result as u8;

        Operation::Next
    }

    /// LD Vx, Vy
    ///
    /// Set Vx = Vy.
    /// Stores the value of register Vy in register Vx.
    fn op_8xy0(&mut self, x: usize, y: usize) -> Operation {
        self.v[x] = self.v[y];
        Operation::Next
    }

    /// OR Vx, Vy
    ///
    /// Set Vx = Vx OR Vy.
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1.
    /// Otherwise, it is 0.
    fn op_8xy1(&mut self, x: usize, y: usize) -> Operation {
        self.v[x] |= self.v[y];
        Operation::Next
    }

    /// AND Vx, Vy
    ///
    /// Set Vx = Vx AND Vy.
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1.
    /// Otherwise, it is 0.
    fn op_8xy2(&mut self, x: usize, y: usize) -> Operation {
        self.v[x] &= self.v[y];
        Operation::Next
    }

    /// XOR Vx, Vy
    ///
    /// Set Vx = Vx XOR Vy.
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    /// An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1.
    /// Otherwise, it is 0.
    fn op_8xy3(&mut self, x: usize, y: usize) -> Operation {
        self.v[x] ^= self.v[y];
        Operation::Next
    }

    /// ADD Vx, Vy
    ///
    /// Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    /// Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn op_8xy4(&mut self, x: usize, y: usize) -> Operation {
        let vx = self.v[x] as u16;
        let vy = self.v[y] as u16;
        let result = vx + vy;
        self.v[x] = result as u8;
        self.v[0xF] = if result > 0xFF { 1 } else { 0 };

        Operation::Next
    }

    /// SUB Vx, Vy
    ///
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn op_8xy5(&mut self, x: usize, y: usize) -> Operation {
        self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        Operation::Next
    }

    /// SHR Vx {, Vy}
    ///
    /// Set Vx = Vx SHR 1.
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    fn op_8x06(&mut self, x: usize) -> Operation {
        self.v[0xF] = self.v[x] & 1;
        self.v[x] >>= 1;
        Operation::Next
    }

    /// SUBN Vx, Vy
    ///
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    fn op_8xy7(&mut self, x: usize, y: usize) -> Operation {
        self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        Operation::Next
    }

    /// SHL Vx {, Vy}
    ///
    /// Set Vx = Vx SHL 1.
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    fn op_8x0e(&mut self, x: usize) -> Operation {
        self.v[0xF] = (self.v[x] & 0xFF) >> 7;
        self.v[x] <<= 1;
        Operation::Next
    }

    /// SNE Vx, Vy
    ///
    /// Skip next instruction if Vx != Vy.
    /// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    fn op_9xy0(&mut self, x: usize, y: usize) -> Operation {
        Operation::skip_if(self.v[x] != self.v[y])
    }

    /// LD I, addr
    /// Set I = nnn.
    /// The value of register I is set to nnn.
    fn op_annn(&mut self, nnn: usize) -> Operation {
        self.i = nnn;
        Operation::Next
    }

    /// JP V0, addr
    ///
    /// Jump to location nnn + V0.
    /// The program counter is set to nnn plus the value of V0.
    fn op_bnnn(&mut self, nnn: usize) -> Operation {
        Operation::Jump(self.v[0] as usize + nnn)
    }

    // RND Vx, byte
    ///
    /// Set Vx = random byte AND kk.
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
    /// The results are stored in Vx. See instruction [`Self::op_8xy2`] for more information on AND.
    fn op_cxkk(&mut self, x: usize, kk: u8) -> Operation {
        let mut rng = rand::thread_rng();
        self.v[x] = rng.gen::<u8>() & kk;
        Operation::Next
    }

    /// DRW Vx, Vy, nibble
    ///
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    /// The interpreter reads n bytes from memory, starting at the address stored in I.
    /// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen.
    /// If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    /// If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen.
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) -> Operation {
        self.v[0xF] = 0;

        for byte in 0..n {
            let y = (self.v[y] as usize + byte) % SCREEN_HEIGHT;

            for bit in 0..8 {
                let x = (self.v[x] as usize + bit) % SCREEN_WIDTH;
                let color = (self.ram[self.i + byte] >> (7 - bit)) & 1;
                self.v[0xF] |= color & self.vram[y][x];
                self.vram[y][x] ^= color;
            }
        }

        self.should_draw = true;
        Operation::Next
    }

    /// SKP Vx
    ///
    /// Skip next instruction if key with the value of Vx is pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    fn op_ex9e(&self, x: usize) -> Operation {
        Operation::skip_if(self.keypad[self.v[x] as usize])
    }

    /// SKNP Vx
    ///
    /// Skip next instruction if key with the value of Vx is not pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    fn op_exa1(&self, x: usize) -> Operation {
        Operation::skip_if(!self.keypad[self.v[x] as usize])
    }

    /// LD Vx, DT
    ///
    /// Set Vx = delay timer value.
    /// The value of DT is placed into Vx.
    fn op_fx07(&mut self, x: usize) -> Operation {
        self.v[x] = self.delay_timer;
        Operation::Next
    }

    /// LD Vx {, K}
    ///
    /// Wait for a key press, store the value of the key in Vx.
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    fn op_fx0a(&mut self, x: usize) -> Operation {
        self.should_keypad_wait = true;
        self.keypad_wait_input = x;

        Operation::Next
    }

    /// LD DT, Vx
    ///
    /// Set delay timer = Vx.
    /// DT is set equal to the value of Vx.
    fn op_fx15(&mut self, x: usize) -> Operation {
        self.delay_timer = self.v[x];
        Operation::Next
    }

    /// LD ST, Vx
    ///
    /// Set sound timer = Vx.
    /// ST is set equal to the value of Vx.
    /// The minimum value that the timer will respond to is 0x02.
    fn op_fx18(&mut self, x: usize) -> Operation {
        if self.v[x] < 0x2 {
            return Operation::Next;
        }

        self.sound_timer = self.v[x];
        Operation::Next
    }

    /// ADD I, Vx
    ///
    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in I.
    fn op_fx1e(&mut self, x: usize) -> Operation {
        self.i += self.v[x] as usize;
        Operation::Next
    }

    /// LD F, Vx
    ///
    /// Set I = location of sprite for digit Vx.
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx
    fn op_fx29(&mut self, x: usize) -> Operation {
        self.i = (self.v[x] as usize) * 5;
        Operation::Next
    }

    /// LD B, Vx
    ///
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
    /// the tens digit at location I+1, and the ones digit at location I+2.
    fn op_fx33(&mut self, x: usize) -> Operation {
        self.ram[self.i] = self.v[x] / 100;
        self.ram[self.i + 1] = (self.v[x] % 100) / 10;
        self.ram[self.i + 2] = self.v[x] % 10;

        Operation::Next
    }

    /// LD [I], Vx
    ///
    /// Store registers V0 through Vx in memory starting at location I.
    /// The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    fn op_fx55(&mut self, x: usize) -> Operation {
        for i in 0..x + 1 {
            self.ram[self.i + i] = self.v[i];
        }

        Operation::Next
    }

    /// LD Vx, [I]
    ///
    /// Read registers V0 through Vx from memory starting at location I.
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn op_fx65(&mut self, x: usize) -> Operation {
        for i in 0..x + 1 {
            self.v[i] = self.ram[self.i + i];
        }

        Operation::Next
    }
}

#[cfg(test)]
#[path = "../tests/cpu/cpu_tests.rs"]
mod cpu_tests;

#[cfg(test)]
#[path = "../tests/cpu/opcode_tests.rs"]
mod opcode_tests;
