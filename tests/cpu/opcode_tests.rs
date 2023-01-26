use super::*;
use crate::cpu::opcode_tests::opcode_helper::OpcodeHelper;
use all_asserts::{assert_false, assert_true};

#[path = "../helpers/opcode_helper.rs"]
mod opcode_helper;

#[test]
fn clear_screen() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    helper.draw_to_screen();

    let screen_empty = helper.cpu.vram.iter().flatten().all(|&pixel| pixel == 0_u8);

    assert_false!(screen_empty);

    helper.clear_screen();

    let screen_empty = helper.cpu.vram.iter().flatten().all(|&pixel| pixel == 0_u8);

    assert_true!(screen_empty);
}

#[test]
fn return_from_address() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };
    let pc = helper.cpu.pc;

    // SUB Vx, Vy
    helper.load_byte(0xA, 24);
    helper.load_byte(3, 15);
    helper.load_addr_ram(0x123, 0x8A35);
    helper.assert_pc_value(pc + 4);

    // CALL addr
    helper.call_addr(0x123);
    helper.assert_sp_value(1);
    helper.assert_pc_value(0x123);

    // Process opcode at pc
    helper.process_pc();
    helper.assert_register_value(0xA, 9);

    // Assert pc and sp after operations and return jump
    helper.func_return();
    helper.assert_sp_value(0);
    helper.assert_pc_value(pc + 8);
}

#[test]
fn jump_to_address() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    helper.load_addr_ram(0x400, 0x5120);
    helper.jump_addr(0x400);
    helper.assert_pc_value(0x400);
}

#[test]
fn call_address() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };
    let pc = helper.cpu.pc;

    // ADD Vx, byte
    helper.load_byte(1, 0x23);
    helper.load_addr_ram(0xA25, 0x7101);
    helper.assert_stack_value(0);
    helper.assert_pc_value(pc + 2);

    // CALL addr
    helper.call_addr(0xA25);
    helper.assert_sp_value(1);
    helper.assert_pc_value(0xA25);

    // Process opcode at pc
    helper.process_pc();
    helper.assert_register_value(1, 0x24);
    helper.assert_stack_value(pc + 4);
}

#[test]
fn skip_equal_byte() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };
    let pc = helper.cpu.pc;

    // LD Vx, byte
    helper.load_byte(1, 0xA);
    helper.assert_pc_value(pc + OPCODE_SIZE);

    // SE Vx, byte
    // Skips pc + OPCODE_SIZE * 2
    // (pc + OPCODE_SIZE * 1) * 2 == (pc + OPCODE_SIZE * 3)
    helper.skip_equal_byte(1, 0xA);
    helper.assert_pc_value(pc + OPCODE_SIZE * 3);

    helper.load_byte(4, 1);
    helper.assert_pc_value(pc + OPCODE_SIZE * 4);

    helper.skip_equal_byte(4, 0xF);
    helper.assert_pc_value(pc + OPCODE_SIZE * 5);
}

#[test]
fn skip_not_equal_byte() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };
    let pc = helper.cpu.pc;

    // LD Vx, byte
    helper.load_byte(7, 0xF);
    helper.assert_pc_value(pc + OPCODE_SIZE);

    // SNE Vx, byte
    // (pc + OPCODE_SIZE * 1) * 2 == (pc + OPCODE_SIZE * 3)
    helper.skip_not_equal_byte(7, 1);
    helper.assert_pc_value(pc + OPCODE_SIZE * 3);

    helper.load_byte(3, 4);
    helper.assert_pc_value(pc + OPCODE_SIZE * 4);

    helper.skip_not_equal_byte(3, 4);
    helper.assert_pc_value(pc + OPCODE_SIZE * 5);
}

#[test]
fn skip_equal_registers() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };
    let pc = helper.cpu.pc;

    // LD V[n], byte
    helper.load_byte(1, 0xB);
    helper.load_byte(2, 0xB);
    helper.assert_pc_value(pc + OPCODE_SIZE * 2);

    // SE Vx, Vy
    helper.skip_equal_registers(1, 2);
    helper.assert_pc_value(pc + OPCODE_SIZE * 4);

    helper.load_byte(3, 5);
    helper.assert_pc_value(pc + OPCODE_SIZE * 5);

    helper.skip_equal_registers(3, 1);
    helper.assert_pc_value(pc + OPCODE_SIZE * 6);
}

#[test]
fn skip_not_equal_registers() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };
    let pc = helper.cpu.pc;

    // LD V[n], byte
    helper.load_byte(1, 0xB);
    helper.load_byte(2, 1);
    helper.assert_pc_value(pc + OPCODE_SIZE * 2);

    // SNE Vx, Vy
    helper.skip_not_equal_registers(1, 2);
    helper.assert_pc_value(pc + OPCODE_SIZE * 4);

    helper.load_byte(3, 1);
    helper.assert_pc_value(pc + OPCODE_SIZE * 5);

    helper.skip_not_equal_registers(2, 3);
    helper.assert_pc_value(pc + OPCODE_SIZE * 6);
}

#[test]
fn loads_byte_into_registers() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, byte
    helper.load_byte(2, 0xFF);
    helper.assert_register_value(2, 0xFF);
    helper.assert_pc_value(0x202);

    helper.load_byte(0, 0xA3);
    helper.assert_register_value(0, 0xA3);
    helper.assert_pc_value(0x204);
}

#[test]
fn add_regster_bytes() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, byte
    helper.load_byte(1, 0xA);

    // ADD Vx, byte
    helper.add_register_bytes(1, 1);
    helper.assert_register_value(1, 0xB);
}

#[test]
fn loads_register_into_register() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, byte
    helper.load_byte(1, 0xA);

    // LD Vx, Vy
    helper.load_from_register(3, 1);
    helper.assert_register_value(3, 0xA);
}

#[test]
fn bitwise_or_registers() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, Vy
    helper.load_byte(6, 0xB);
    helper.load_byte(0xE, 0xF);

    // OR Vx, Vy
    helper.bitwise_or_registers(6, 0xE);
    helper.assert_register_value(6, 0xF);
}

#[test]
fn bitwise_and_registers() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, Vy
    helper.load_byte(1, 0xB);
    helper.load_byte(5, 5);

    // OR Vx, Vy
    helper.bitwise_and_registers(1, 5);
    helper.assert_register_value(1, 1);
}

#[test]
fn bitwise_xor_registers() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, Vy
    helper.load_byte(8, 0xC);
    helper.load_byte(0xA, 0xE);

    // OR Vx, Vy
    helper.bitwise_xor_registers(8, 0xA);
    helper.assert_register_value(8, 2);
}

#[test]
fn adds_registers() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD V[n], byte
    helper.load_byte(5, 1);
    helper.load_byte(0xB, 5);

    // ADD Vx, Vy
    helper.add_registers(5, 0xB);
    helper.assert_register_value(5, 6);
}

#[test]
fn add_overflow() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD V[n], byte
    helper.load_byte(0xA, 0xFE);
    helper.load_byte(3, 3);

    // ADD Vx, Vy
    helper.add_registers(0xA, 3);
    helper.assert_register_value(0xA, 1);

    // ADD Vx, bytes
    helper.load_byte(1, 1);
    helper.add_register_bytes(1, 0xFF);
    helper.assert_register_value(1, 0);
}

#[test]
fn subtract_registers() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD V[n], byte
    helper.load_byte(0, 0xC);
    helper.load_byte(1, 0xA);

    // SUB Vx, Vy
    helper.subtract_registers(0, 1);
    helper.assert_register_value(0, 2);
}

#[test]
fn subtract_registers_overflow() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD V[n], byte
    helper.load_byte(0xB, 0xF);
    helper.load_byte(4, 0xFF);

    // SUB Vx, Vy
    helper.subtract_registers(0xB, 4);
    helper.assert_register_value(0xB, 0x10);
    helper.assert_borrow(false);
}

#[test]
fn subtract_registers_borrow() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD V[n], byte
    helper.load_byte(3, 0xFF);
    helper.load_byte(4, 0xFE);

    // SUB Vx, Vy
    helper.subtract_registers(3, 4);
    helper.assert_register_value(3, 1);
    helper.assert_borrow(true);
}

#[test]
fn shift_right_once() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD V[n], byte
    helper.load_byte(3, 0x65);

    // SHR Vx {, Vy}
    helper.shift_right_once(3);
    helper.assert_register_value(3, 0x32);
    helper.assert_borrow(true);

    helper.load_byte(1, 10);
    helper.shift_right_once(1);
    helper.assert_register_value(1, 5);
    helper.assert_borrow(false);
}

#[test]
fn subtract_n() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD V[n], byte
    helper.load_byte(5, 0xB);
    helper.load_byte(0xC, 0xF);

    // SUBN Vx, Vy
    helper.subtract_n(5, 0xC);
    helper.assert_register_value(5, 4);
    helper.assert_borrow(true);

    helper.load_byte(5, 0xF);
    helper.load_addr_ram(0xC, 0xB);
    helper.subtract_n(5, 0xC);
    helper.assert_register_value(5, 0);
    helper.assert_borrow(false);
}

#[test]
fn shift_left_once() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, byte
    helper.load_byte(8, 0xFF);

    // SHL Vx {, Vy}
    helper.shift_left_once(8);
    helper.assert_register_value(8, 0xFE);
    helper.assert_borrow(true);

    helper.load_byte(1, 5);
    helper.shift_left_once(1);
    helper.assert_register_value(1, 10);
    helper.assert_borrow(false);
}

#[test]
fn load_i_register_address() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD I, addr
    helper.load_i_register(0xA15);
    helper.assert_i_register_value(0xA15);
}

#[test]
fn jump_addr_v0_offset() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD V0, byte
    helper.load_byte(0, 0xF);

    // JP V0, addr
    helper.jump_addr_v0_offset(0x2C5);
    helper.assert_pc_value(0x2D4);
}

#[test]
fn load_random_byte_bitwise_and_register() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, byte
    helper.load_byte(9, 9);

    // RND Vx, byte
    helper.load_random_byte(9, 0xF);
    helper.assert_register_has_value(9);
}

#[test]
fn draw() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    helper.cpu.load_fonts();

    // X position on the screen to start drawing to
    helper.load_byte(0, 0);
    // Y position on the screen to start drawing to
    helper.load_byte(1, 0);
    // Point I register at font data
    helper.load_i_register(0);

    // DRW Vx, Vy, nibble
    helper.draw(0, 1, 5);

    let mut pixels: Vec<u8> = vec![];

    for y in 0..SCREEN_HEIGHT {
        pixels.append(
            &mut helper.cpu.vram[y]
                .into_iter()
                .filter(|&b| b == 1u8)
                .collect::<Vec<u8>>(),
        );
    }

    assert_eq!(pixels.len(), 14);
}

#[test]
fn skip_keypressed() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };
    let pc = helper.cpu.pc;

    // LD Vx, byte
    helper.load_byte(0, 0xF);
    helper.assert_pc_value(pc + OPCODE_SIZE);

    helper.press_key(0xF);
    helper.assert_keypressed(0xF);

    // SKP Vx
    helper.skip_keypressed(0);
    helper.assert_pc_value(pc + OPCODE_SIZE * 3);

    helper.load_byte(1, 4);
    helper.press_key(4);
    helper.assert_keypressed(4);

    helper.skip_keypressed(1);
    helper.assert_pc_value(pc + OPCODE_SIZE * 6);
}

#[test]
fn skip_not_keypressed() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };
    let pc = helper.cpu.pc;

    // LD Vx, byte
    helper.load_byte(7, 0xA);
    helper.assert_pc_value(pc + OPCODE_SIZE);

    helper.press_key(9);
    helper.assert_keypressed(9);

    // SKNP Vx
    helper.skip_no_keypressed(7);
    helper.assert_pc_value(pc + OPCODE_SIZE * 3);

    helper.press_key(0xC);
    helper.assert_keypressed(0xC);

    helper.skip_no_keypressed(7);
    helper.assert_pc_value(pc + OPCODE_SIZE * 5);
}

#[test]
fn read_delay_timer() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, byte
    helper.load_byte(3, 0xB);
    helper.set_delay_timer(3);
    helper.assert_dt_value(0xB);

    // LD Vx, DT
    helper.read_delay_timer(5);
    helper.assert_register_value(5, 0xB);
}

#[test]
fn wait_keypress() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    helper.wait_keypress(5);
    // This is currently a hard-coded value.
    // Re-write test when event loop is implemented.
    helper.press_key(0xF);
    helper.assert_register_value(5, 0xF);
}

#[test]
fn set_delay_timer() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, byte
    helper.load_byte(7, 1);
    helper.set_delay_timer(7);
    helper.assert_dt_value(1);
}

#[test]
fn set_sound_timer() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, byte
    helper.load_byte(3, 0xA);

    // LD ST, Vx
    helper.set_sound_timer(3);
    helper.assert_st_value(0xA);

    // A value with less than 0x02 has no effect
    helper.load_byte(0xE, 0x01);
    helper.set_sound_timer(0xE);
    helper.assert_st_value(0xA);

    // Reset sound timer to 0
    helper.reset_st();

    // A value with less than 0x02 has no effect
    helper.load_byte(9, 0x01);
    helper.set_sound_timer(9);
    helper.assert_st_value(0);
}

#[test]
fn add_set_i_to_register() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, byte
    helper.load_byte(0xA, 0xFF);

    // ADD I, Vx
    helper.add_i_register(0xA);
    helper.assert_i_register_value(0xFF);

    // LD I, addr
    helper.load_i_register(0xABC);
    helper.assert_i_register_value(0xABC);

    helper.add_i_register(0xA);
    helper.assert_i_register_value(0xBBB);
}

#[test]
fn load_i_from_sprite() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    helper.draw_to_screen();

    // LD Vx, byte
    helper.load_byte(1, 0xA);

    // LD F, Vx
    helper.load_i_from_sprite(1);
    helper.assert_i_register_value(0x32);
}

#[test]
fn store_bcd_of_register() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // LD Vx, byte
    helper.load_byte(9, 123);
    helper.load_i_register(0xB45);

    // LD B, Vx
    helper.store_bcd_of_register(9);

    let (b, c, d) = helper.bcd();

    assert_eq!(b, 1);
    assert_eq!(c, 2);
    assert_eq!(d, 3);
}

#[test]
fn store_at_i() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    // load registers
    for n in 0..0xE {
        helper.load_byte(n, n * 2);
    }

    helper.load_i_register(0x220);

    // LD [I], Vx
    helper.store_at_i(0xE);

    for n in 0..0xE {
        helper.assert_ram_addr_value(helper.cpu.i + n, n * 2);
    }
}

#[test]
fn read_from_i() {
    let mut helper = OpcodeHelper { cpu: Cpu::new() };

    for (index, n) in (0..0xE).enumerate() {
        helper.cpu.ram[0x300 + index] = n + 5;
    }

    helper.load_i_register(0x300);

    // LD Vx, [I]
    helper.read_from_i(0xE);

    for n in 0..0xE {
        helper.assert_register_value(n, (n + 5) as u8);
    }
}
