use crate::types::Result;
use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

pub struct Keyboard {
    events: EventPump,
}

impl Keyboard {
    pub fn init(context: &Sdl) -> Result<Self> {
        let events = context.event_pump()?;

        Ok(Self { events })
    }

    pub fn poll(&mut self) -> std::result::Result<[bool; 16], ()> {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Err(());
                },
                _ => {}
            }
        }

        let keys: Vec<Keycode> = self
            .events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut keypad = [false; 16];

        for key in keys {
            let pressed = match key {
                Keycode::Num1 => Some(0x0),
                Keycode::Num2 => Some(0x1),
                Keycode::Num3 => Some(0x2),
                Keycode::Num4 => Some(0x3),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0x7),
                Keycode::A => Some(0x8),
                Keycode::S => Some(0x9),
                Keycode::D => Some(0xA),
                Keycode::F => Some(0xB),
                Keycode::Z => Some(0xC),
                Keycode::X => Some(0xD),
                Keycode::C => Some(0xE),
                Keycode::V => Some(0xF),
                _ => None,
            };

            if let Some(key_index) = pressed {
                keypad[key_index] = true;
            }
        }

        Ok(keypad)
    }
}
