extern crate sdl2;
extern crate core;

mod events;
mod gamestate;
mod sdl_engine;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

use std::path::Path;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use events::{EventStatus};
use gamestate::{GameState};

fn main() {
    // Game size

    let gfx_scale: usize = 4;
    let life_width: usize = 300;
    let life_height: usize = 300;

    // Initialize SDL.

    let mut engine = sdl_engine::new("Life RS", (gfx_scale * life_width) as u32, (gfx_scale * life_height) as u32);

    // Load a font

    let sdl_ttf = sdl2::ttf::init().unwrap();
    let font = sdl_ttf.load_font(Path::new("./fonts/UbuntuMono-R.ttf"), 32).unwrap();

    // Game state.

    let mut game_state = GameState::new(life_width, life_height);

    // For timing purposes to report FPS.

    let mut last_loop_duration: f64;
    let mut last_loop_fps: f64 = 0.0;

    let mut iteration: u64 = 0;

    // Event pump.

    let mut event_status = EventStatus::new();

    // Drive the game.

    loop {
        let loop_start = Instant::now();

        // Blank the renderer.

        {
            let r = engine.renderer();

            r.set_draw_color(Color::RGB(0, 0, 0));
            r.clear();
        }

        // Poll current events.

        event_status.pump(engine.event_pump(), true);

        // Update the gamestate. GameStates will return a new GameState for scene transitions.

        game_state.render(&mut engine, gfx_scale);

        game_state = game_state.update_events(&event_status);
        game_state = game_state.next_state();

        // Draw current diagnostics.

        {
            let r = engine.renderer();

            let pos = format!("FPS {:.3}", last_loop_fps);
            let position_text_surface = font.render(pos.as_str()).blended(Color::RGBA(255, 0, 0, 255)).unwrap();
            let mut position_text_texture = r.create_texture_from_surface(&position_text_surface).unwrap();
            let TextureQuery { width, height, .. } = position_text_texture.query();
            let (_, window_h) = r.output_size().unwrap();

            r.copy(&mut position_text_texture, None, Some(Rect::new(5, (window_h-height-5) as i32, width, height))).unwrap();
        }

        {
            iteration += 1;
            println!("iteration {}", iteration);
        }

        // Display.

        engine.renderer().present();

        // Sleep for a consistent amount of time.

        let loop_end = Instant::now();
        let loop_duration = loop_end.duration_since(loop_start);

        last_loop_duration =
            ( loop_duration.as_secs() as f64 )
            + ( loop_duration.subsec_nanos() as f64 / 1000000000.0 );

        // let standard_loop_delay = 0.01666666667;
        let standard_loop_delay = 0.025;

        let required_delay = standard_loop_delay - last_loop_duration;

        if last_loop_duration <= standard_loop_delay {
            last_loop_fps = 1.0 / standard_loop_delay;
        } else {
            last_loop_fps = 1.0 / ( last_loop_duration + 0.001 );
        }

        if required_delay > 0.0 {
            let delay_secs = required_delay as u64;
            let delay_nanos = ( ( ( required_delay * 1000000000.0 ) as u64 ) % 1000000000 ) as u32;
            let delay = Duration::new(delay_secs, delay_nanos);

            thread::sleep(delay);
        } else {
            thread::sleep(Duration::from_millis(1));
        }
    }
}
