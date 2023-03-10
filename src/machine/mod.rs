use self::{audio::Audio, display::Display, keyboard::Keyboard};
use crate::{cpu::Cpu, rom::Rom, types::Result, utils::Utils};
use colors_transform::Rgb;
use imgui::Context;
use sdl2::{pixels::Color, video::Window, EventPump, Sdl};
use snailquote::unescape;
use std::path::Path;

mod audio;
mod display;
mod keyboard;

pub struct Machine {
    sdl_context: Sdl,
    imgui_context: Context,
}

impl Machine {
    pub fn prepare() -> Result<()> {
        let mut roms = Utils::fetch_rom_list()?;
        roms.sort_by(|a, b| a.title.cmp(&b.title));

        Self::init()?.boot(roms)?;

        Ok(())
    }

    pub fn init() -> Result<Self> {
        Ok(Self {
            sdl_context: sdl2::init()?,
            imgui_context: Context::create(),
        })
    }

    pub fn boot(&mut self, roms: Vec<Rom>) -> Result<()> {
        let (window, event_pump, rom_path) =
            Display::splash_screen(&self.sdl_context, &mut self.imgui_context, roms)?;

        self.start(window, event_pump, rom_path.as_path())?;

        Ok(())
    }

    pub fn start(&mut self, window: Window, mut event_pump: EventPump, rom: &Path) -> Result<()> {
        let mut cpu = Cpu::new();
        cpu.init(rom)?;

        let audio = Audio::init(&self.sdl_context)?;
        let filename = rom.with_extension("");
        let filename = filename.file_name().unwrap();
        let rom = Utils::find_rom(filename.to_str().unwrap())?;

        let mut bg_color = Rgb::from(75.0, 75.0, 75.0);
        let mut fg_color = Rgb::from(0.0, 0.0, 0.0);
        let mut tickrate = Utils::instruction_time_ns();

        if let Some(background_color) = rom.options.background_color {
            if let Ok(bg_result) = Rgb::from_hex_str(&unescape(&background_color)?) {
                bg_color = bg_result;
            }
        }

        if let Some(foreground_color) = rom.options.fill_color {
            if let Ok(fg_result) = Rgb::from_hex_str(&unescape(&foreground_color)?) {
                fg_color = fg_result;
            }
        }

        if rom.options.tickrate > 0 {
            tickrate = rom.options.tickrate;
        }

        let mut canvas = window.into_canvas().build()?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        while let Ok(result) = Keyboard::poll(&mut event_pump) {
            let output = cpu.process(result, tickrate);

            if output.should_draw {
                Display::draw_game(&mut canvas, output.vram, bg_color, fg_color)?;
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
