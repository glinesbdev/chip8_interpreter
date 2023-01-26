use self::{display::Display, keyboard::Keyboard};
use crate::{cpu::Cpu, types::Result};
use sdl2::{gfx::framerate::FPSManager};
use std::path::Path;

mod audio;
mod display;
mod keyboard;

pub struct Machine;

impl Machine {
    pub fn start(rom: &Path) -> Result<()> {
        let mut fps = FPSManager::new();
        fps.set_framerate(60)?;

        let context = sdl2::init()?;
        let mut keyboard = Keyboard::init(&context)?;
        let mut display = Display::init(&context)?;
        let mut cpu = Cpu::new();

        cpu.init(&rom)?;

        // game loop
        while let Ok(keypad) = keyboard.poll() {
            let output = cpu.process(keypad);

            if output.should_draw {
                display.draw(&output.vram, fps.get_framerate())?;
            }

            if output.should_beep {
                println!("BEEP!");
            }
        }

        Ok(())
    }
}
