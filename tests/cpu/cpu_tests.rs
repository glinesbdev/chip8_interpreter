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
