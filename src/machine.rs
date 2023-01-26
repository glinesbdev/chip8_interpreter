use self::{audio::Audio, display::Display, keyboard::Keyboard};
use crate::{constants::INSTRUCTIONS_PER_SECOND, cpu::Cpu, types::Result};
use sdl2::gfx::framerate::FPSManager;
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
        let mut display = Display::init("CHIP-8 Interpreter", 640, 320, &context)?;
        let audio = Audio::init(&context)?;
        let mut cpu = Cpu::new();

        // Default 1000 instructions per second
        let instruction_time_ns = 1e9 as u128 / INSTRUCTIONS_PER_SECOND;

        cpu.init(&rom)?;

        // game loop
        while let Ok(keypad) = keyboard.poll() {
            let output = cpu.process(keypad, instruction_time_ns);

            if output.should_draw {
                display.draw(&output.vram, fps.get_framerate())?;
            }

            if output.should_beep {
                audio.play();
            } else {
                audio.pause();
            }
        }

        Ok(())
    }
}
