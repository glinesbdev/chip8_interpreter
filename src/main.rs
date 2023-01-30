use machine::Machine;
use types::Result;

mod constants;
mod cpu;
mod machine;
mod rom;
mod timer;
mod types;
mod utils;

fn main() -> Result<()> {
    Machine::prepare()?;

    Ok(())
}
