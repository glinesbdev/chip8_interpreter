use super::*;

#[test]
fn loads_fonts_top_of_ram() {
    let mut cpu = Cpu::new();
    cpu.load_fonts();

    let zero = &cpu.ram[0..5];
    let f = &cpu.ram[75..80];

    assert_eq!(zero, [0xF0, 0x90, 0x90, 0x90, 0xF0]);
    assert_eq!(f, [0xF0, 0x80, 0xF0, 0x80, 0x80]);
}

#[test]
fn loads_rom_at_pc() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut cpu = Cpu::new();
    cpu.load_rom(Path::new("roms/Particle Demo [zeroZshadow, 2008].ch8"))?;

    let pc_start = &cpu.ram[cpu.pc..(cpu.pc + 5)];

    assert_ne!(pc_start, [0xF0, 0x90, 0x90, 0x90, 0xF0]);
    assert_eq!(pc_start, [0xA3, 0x21, 0x60, 0x0, 0x61]);

    Ok(())
}
