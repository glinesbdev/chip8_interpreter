use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH, SPRITE_SCALE},
    types::Result,
};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn init(context: &Sdl) -> Result<Self> {
        let video = context.video()?;
        let window = video
            .window("CHIP-8 Interpreter", 640, 320)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().present_vsync().build()?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(Self { canvas })
    }

    pub fn draw(
        &mut self,
        vram_buffer: &[[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
        framerate: i32,
    ) -> Result<()> {
        self.canvas.present();
        self.canvas
            .window_mut()
            .set_title(&format!("CHIP-8 Interpreter - FPS: {}", framerate))?;

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
