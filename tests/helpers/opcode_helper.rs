use crate::cpu::Cpu;
use all_asserts::{assert_gt, assert_true};

pub struct OpcodeHelper {
    pub cpu: Cpu,
}

// Opcode Instructions
impl OpcodeHelper {
    pub fn clear_screen(&mut self) {
        self.process_opcode(0x00E0);
    }

    pub fn func_return(&mut self) {
        self.process_opcode(0x00EE);
    }

    pub fn jump_addr(&mut self, opcode: u16) {
        let opcode = 0x1000 | opcode;
        self.process_opcode(opcode);
    }

    pub fn call_addr(&mut self, opcode: u16) {
        let opcode = 0x2000 | opcode;
        self.process_opcode(opcode);
    }

    pub fn skip_equal_byte(&mut self, reg: usize, byte: usize) {
        let opcode = (0x3000 | reg << 8 | byte) as u16;
        self.process_opcode(opcode);
    }

    pub fn skip_not_equal_byte(&mut self, reg: usize, byte: usize) {
        let opcode = (0x4000 | reg << 8 | byte) as u16;
        self.process_opcode(opcode);
    }

    pub fn skip_equal_registers(&mut self, x_reg: usize, y_reg: usize) {
        let opcode = (0x5000 | x_reg << 8 | y_reg << 4) as u16;
        self.process_opcode(opcode);
    }

    pub fn load_byte(&mut self, reg: usize, value: usize) {
        let opcode = (0x6000 | reg << 8 | value) as u16;
        self.process_opcode(opcode);
    }

    pub fn add_register_bytes(&mut self, reg: usize, byte: usize) {
        let opcode = (0x7000 | reg << 8 | byte) as u16;
        self.process_opcode(opcode);
    }

    pub fn load_from_register(&mut self, x_reg: usize, y_reg: usize) {
        let opcode = (0x8000 | x_reg << 8 | y_reg << 4) as u16;
        self.process_opcode(opcode);
    }

    pub fn bitwise_or_registers(&mut self, x_reg: usize, y_reg: usize) {
        let opcode = (0x8001 | x_reg << 8 | y_reg << 4) as u16;
        self.process_opcode(opcode);
    }

    pub fn bitwise_and_registers(&mut self, x_reg: usize, y_reg: usize) {
        let opcode = (0x8002 | x_reg << 8 | y_reg << 4) as u16;
        self.process_opcode(opcode);
    }

    pub fn bitwise_xor_registers(&mut self, x_reg: usize, y_reg: usize) {
        let opcode = (0x8003 | x_reg << 8 | y_reg << 4) as u16;
        self.process_opcode(opcode);
    }

    pub fn add_registers(&mut self, x_reg: usize, y_reg: usize) {
        let opcode = (0x8004 | x_reg << 8 | y_reg << 4) as u16;
        self.process_opcode(opcode);
    }

    pub fn subtract_registers(&mut self, x_reg: usize, y_reg: usize) {
        let opcode = (0x8005 | x_reg << 8 | y_reg << 4) as u16;
        self.process_opcode(opcode);
    }

    pub fn shift_right_once(&mut self, reg: usize) {
        let opcode = (0x8006 | reg << 8) as u16;
        self.process_opcode(opcode);
    }

    pub fn subtract_n(&mut self, x_reg: usize, y_reg: usize) {
        let opcode = (0x8007 | x_reg << 8 | y_reg << 4) as u16;
        self.process_opcode(opcode);
    }

    pub fn shift_left_once(&mut self, reg: usize) {
        let opcode = (0x800E | reg << 8) as u16;
        self.process_opcode(opcode);
    }

    pub fn skip_not_equal_registers(&mut self, x_reg: usize, y_reg: usize) {
        let opcode = (0x9000 | x_reg << 8 | y_reg << 4) as u16;
        self.process_opcode(opcode);
    }

    pub fn load_i_register(&mut self, addr: u16) {
        let opcode = 0xA000 | addr;
        self.process_opcode(opcode);
    }

    pub fn jump_addr_v0_offset(&mut self, addr: u16) {
        let opcode = 0xB000 | addr;
        self.process_opcode(opcode);
    }

    pub fn load_random_byte(&mut self, reg: usize, byte: usize) {
        let opcode = (0xC000 | reg << 8 | byte) as u16;
        self.process_opcode(opcode);
    }

    pub fn draw(&mut self, x_reg: usize, y_reg: usize, byte_size: usize) {
        let opcode = (0xD000 | x_reg << 8 | y_reg << 4 | byte_size) as u16;
        self.process_opcode(opcode);
    }

    pub fn skip_keypressed(&mut self, reg: usize) {
        let opcode = (0xE09E | reg << 8) as u16;
        self.process_opcode(opcode);
    }

    pub fn skip_no_keypressed(&mut self, reg: usize) {
        let opcode = (0xE0A1 | reg << 8) as u16;
        self.process_opcode(opcode);
    }

    pub fn read_delay_timer(&mut self, reg: usize) {
        let opcode = (0xF007 | reg << 8) as u16;
        self.process_opcode(opcode);
    }

    pub fn wait_keypress(&mut self, reg: usize) {
        let opcode = (0xF00A | reg << 8) as u16;
        self.process_opcode(opcode);
    }

    pub fn set_delay_timer(&mut self, reg: usize) {
        let opcode = (0xF015 | reg << 8) as u16;
        self.process_opcode(opcode);
    }

    pub fn set_sound_timer(&mut self, reg: usize) {
        let opcode = (0xF018 | reg << 8) as u16;
        self.process_opcode(opcode);
    }

    pub fn add_i_register(&mut self, reg: usize) {
        let opcode = (0xF01E | reg << 8) as u16;
        self.process_opcode(opcode);
    }

    pub fn load_i_from_sprite(&mut self, reg: usize) {
        let opcode = (0xF029 | reg << 8) as u16;
        self.process_opcode(opcode);
    }

    pub fn store_bcd_of_register(&mut self, reg: usize) {
        let opcode = (0xF033 | reg << 8) as u16;
        self.process_opcode(opcode);
    }

    pub fn store_at_i(&mut self, to_reg: usize) {
        let opcode = (0xF055 | to_reg << 8) as u16;
        self.process_opcode(opcode);
    }

    pub fn read_from_i(&mut self, to_reg: usize) {
        let opcode = (0xF065 | to_reg << 8) as u16;
        self.process_opcode(opcode);
    }
}

// Opcode assertions
impl OpcodeHelper {
    pub fn assert_pc_value(&self, value: usize) {
        assert_eq!(self.cpu.pc, value);
    }

    pub fn assert_sp_value(&self, value: usize) {
        assert_eq!(self.cpu.sp, value);
    }

    pub fn assert_stack_value(&self, value: usize) {
        assert_eq!(self.cpu.stack[self.cpu.sp], value);
    }

    pub fn assert_register_value(&self, reg: usize, value: u8) {
        assert_eq!(self.cpu.v[reg], value);
    }

    pub fn assert_register_has_value(&self, reg: usize) {
        assert_gt!(self.cpu.v[reg], 0);
    }

    pub fn assert_i_register_value(&self, value: usize) {
        assert_eq!(self.cpu.i, value);
    }

    pub fn assert_borrow(&self, borrow: bool) {
        assert_eq!(self.cpu.v[0xF], borrow as u8);
    }

    pub fn assert_keypressed(&self, key: usize) {
        assert_true!(self.cpu.keypad[key]);
    }

    pub fn assert_dt_value(&self, value: u8) {
        assert_eq!(self.cpu.delay_timer, value);
    }

    pub fn assert_st_value(&self, value: u8) {
        assert_eq!(self.cpu.sound_timer, value);
    }

    pub fn assert_ram_addr_value(&self, addr: usize, value: usize) {
        assert_eq!(self.cpu.ram[addr], value as u8);
    }
}

// Helper methods
impl OpcodeHelper {
    // Load some data at some address in ram
    pub fn load_addr_ram(&mut self, addr: usize, value: u16) {
        let (hi, lo) = self.split_u16(value);

        self.cpu.ram[addr] = hi;
        self.cpu.ram[addr + 1] = lo;
    }

    pub fn process_opcode(&mut self, opcode: u16) {
        self.load_addr_ram(self.cpu.pc, opcode);
        self.cpu.process([false; 16]);
    }

    pub fn process_pc(&mut self) {
        self.cpu.process([false; 16]);
    }

    pub fn press_key(&mut self, key: usize) {
        self.cpu.keypad[key] = true;
    }

    pub fn draw_to_screen(&mut self) {
        self.cpu.load_fonts();
        self.load_byte(0, 0);
        self.load_byte(1, 0);
        self.load_i_register(0);
        self.draw(0, 1, 5);
    }

    pub fn reset_st(&mut self) {
        self.cpu.sound_timer = 0;
    }

    // conveineice method to get binary-coded decimal of hex value
    pub fn bcd(&mut self) -> (u8, u8, u8) {
        (
            self.cpu.ram[self.cpu.i],
            self.cpu.ram[self.cpu.i + 1],
            self.cpu.ram[self.cpu.i + 2],
        )
    }

    fn split_u16(&self, byte: u16) -> (u8, u8) {
        let hi = ((byte & 0xFF00) >> 8) as u8;
        let lo = (byte & 0xFF) as u8;

        (hi, lo)
    }
}
