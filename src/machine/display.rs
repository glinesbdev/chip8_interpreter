use crate::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH, SPRITE_SCALE, VRAM_HEIGHT, VRAM_WIDTH},
    rom::Rom,
    types::Result,
    utils::Utils,
};
use colors_transform::{Color, Rgb};
use glow::HasContext;
use imgui::{CollapsingHeader, Condition, Context};
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels,
    rect::Rect,
    render::Canvas,
    video::{GLProfile, Window},
    EventPump, Sdl,
};
use std::path::PathBuf;

#[derive(PartialEq)]
enum MachineScreen<'screen> {
    Roms,
    Game(&'screen str),
}

pub struct Display;

impl Display {
    pub fn splash_screen(
        sdl_context: &Sdl,
        imgui_context: &mut Context,
        roms: Vec<Rom>,
    ) -> Result<(Window, EventPump, PathBuf)> {
        let video_subsystem = sdl_context.video()?;

        {
            /* hint SDL to initialize an OpenGL 3.3 core profile context */
            let gl_attr = video_subsystem.gl_attr();

            gl_attr.set_context_version(3, 3);
            gl_attr.set_context_profile(GLProfile::Core);
        }

        let window = video_subsystem
            .window("CHIP-8", DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .allow_highdpi()
            .opengl()
            .position_centered()
            .build()?;

        /* create a new OpenGL context and make it current */
        let gl_context = window.gl_create_context()?;
        window.gl_make_current(&gl_context)?;

        /* enable vsync to cap framerate */
        window.subsystem().gl_set_swap_interval(1)?;

        /* create new glow and imgui contexts */
        let gl = Self::glow_context(&window);

        /* disable creation of files on disc */
        imgui_context.set_ini_filename(None);
        imgui_context.set_log_filename(None);

        /* setup platform and renderer, and fonts to imgui */
        imgui_context
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        /* create platform and renderer */
        let mut platform = SdlPlatform::init(imgui_context);
        let mut renderer = AutoRenderer::initialize(gl, imgui_context)?;

        let mut event_pump = sdl_context.event_pump()?;
        let (window_width, window_height) = window.size();

        let downloaded_roms = Utils::downloaded_roms()?;
        let (existing_roms, new_roms): (Vec<Rom>, Vec<Rom>) = roms
            .into_iter()
            .partition(|rom| downloaded_roms.contains(&rom.title));

        let mut screen = MachineScreen::Roms;
        let mut rom_name = "";

        'main: loop {
            for event in event_pump.poll_iter() {
                platform.handle_event(imgui_context, &event);

                match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    }
                    | Event::Quit { .. } => {
                        break 'main;
                    }
                    _ => {}
                }
            }

            /* call prepare_frame before calling imgui.new_frame() */
            platform.prepare_frame(imgui_context, &window, &event_pump);

            let ui = imgui_context.new_frame();

            ui.window("Roms from chip8Archive")
                .size(
                    [window_width as f32, window_height as f32],
                    Condition::FirstUseEver,
                )
                .position([0.0, 0.0], Condition::Always)
                .resizable(false)
                .collapsible(false)
                .build(|| {
                    if CollapsingHeader::new("Downloaded Roms")
                        .default_open(true)
                        .build(ui)
                    {
                        for rom in existing_roms.iter() {
                            ui.selectable(&rom.title);

                            if ui.is_item_hovered() {
                                ui.tooltip(|| {
                                    ui.text(&rom.desc);
                                });
                            }

                            if ui.is_item_clicked() && Utils::download_rom(&rom.title).is_ok() {
                                screen = MachineScreen::Game(&rom.title);
                            }
                        }
                    }

                    if CollapsingHeader::new("New Roms")
                        .default_open(true)
                        .build(ui)
                    {
                        for rom in new_roms.iter() {
                            ui.selectable(&rom.title);

                            if ui.is_item_hovered() {
                                ui.tooltip(|| {
                                    ui.text(&rom.desc);
                                });
                            }

                            if ui.is_item_clicked() && Utils::download_rom(&rom.title).is_ok() {
                                screen = MachineScreen::Game(&rom.title);
                            }
                        }
                    }
                });

            /* determine machine screen to show */
            if let MachineScreen::Game(rom) = screen {
                rom_name = rom;
                break 'main;
            }

            /* render */
            let draw_data = imgui_context.render();

            unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
            renderer.render(draw_data)?;

            window.gl_swap_window();
        }

        let mut rom_path = Utils::roms_dir()?;
        rom_path.push(&format!("{rom_name}.ch8"));

        Ok((window, event_pump, rom_path))
    }

    pub fn draw_game(
        canvas: &mut Canvas<Window>,
        vram_buffer: &[[u8; VRAM_WIDTH]; VRAM_HEIGHT],
        bg_color: Rgb,
        fg_color: Rgb,
    ) -> Result<()> {
        for (y, row) in vram_buffer.iter().enumerate() {
            for (x, &color) in row.iter().enumerate() {
                let color = if color == 0 {
                    pixels::Color::RGB(
                        bg_color.get_red() as u8,
                        bg_color.get_green() as u8,
                        bg_color.get_blue() as u8,
                    )
                } else {
                    pixels::Color::RGB(
                        fg_color.get_red() as u8,
                        fg_color.get_green() as u8,
                        fg_color.get_blue() as u8,
                    )
                };

                canvas.set_draw_color(color);
                canvas.fill_rect(Rect::new(
                    (x * SPRITE_SCALE as usize) as i32,
                    (y * SPRITE_SCALE as usize) as i32,
                    SPRITE_SCALE,
                    SPRITE_SCALE,
                ))?;
            }
        }

        canvas.present();

        Ok(())
    }

    fn glow_context(window: &Window) -> glow::Context {
        unsafe {
            glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
        }
    }
}
