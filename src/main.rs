use machine::Machine;
use types::Result;
use std::path::Path;

mod constants;
mod cpu;
mod machine;
mod types;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filepath = args.get(1).ok_or_else(|| "Couldn't load rom at filepath")?;
    let filepath = Path::new(filepath);

    Machine::start(filepath)?;

    Ok(())
}
