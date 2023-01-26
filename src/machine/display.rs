use crate::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH, SPRITE_SCALE},
    types::Result,
};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn init(title: &str, width: u32, height: u32, context: &Sdl) -> Result<Self> {
        let video = context.video()?;
        let window = video
            .window(title, width, height)
            .position_centered()
            .allow_highdpi()
            .vulkan()
            .build()?;

        let mut canvas = window.into_canvas().build()?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(Self { canvas })
    }

    pub fn draw(&mut self, vram_buffer: &[[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) -> Result<()> {
        self.canvas.present();

        for (y, row) in vram_buffer.iter().enumerate() {
            for (x, &color) in row.iter().enumerate() {
                let color = if color == 0 {
                    Color::GRAY
                } else {
                    Color::RGB(74, 4, 4)
                };

                self.canvas.set_draw_color(color);
                self.canvas.fill_rect(Rect::new(
                    (x * SPRITE_SCALE as usize) as i32,
                    (y * SPRITE_SCALE as usize) as i32,
                    SPRITE_SCALE,
                    SPRITE_SCALE,
                ))?;
            }
        }

        self.canvas.present();

        Ok(())
    }
}
