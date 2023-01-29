use machine::Machine;
use types::Result;
use utils::Utils;

mod constants;
mod cpu;
mod machine;
mod rom;
mod timer;
mod types;
mod utils;

fn main() -> Result<()> {
    let loaded_rom = Utils::load_rom_direct()?;
    Machine::prepare(loaded_rom)?;

    Ok(())
}
