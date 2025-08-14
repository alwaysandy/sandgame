extern crate sdl2;

mod gamecontext;
mod point;
mod renderer;

use crate::{gamecontext::*, point::*, renderer::*};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

const GRID_X_SIZE: usize = 75;
const GRID_Y_SIZE: usize = 75;
const DOT_SIZE_IN_PXS: usize = 5;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "Dunes",
            GRID_X_SIZE as u32 * DOT_SIZE_IN_PXS as u32,
            GRID_Y_SIZE as u32 * DOT_SIZE_IN_PXS as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window)?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_counter = 0;
    let mut context = GameContext::new();
    let mut mouse_down = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { .. } => mouse_down = true,
                Event::MouseButtonUp { .. } => mouse_down = false,
                _ => {}
            }
        }

        frame_counter += 1;
        if frame_counter % 5 == 0 {
            if mouse_down {
                let x = event_pump.mouse_state().x();
                let y = event_pump.mouse_state().y();
                if x as usize / DOT_SIZE_IN_PXS < GRID_X_SIZE
                    && y as usize / DOT_SIZE_IN_PXS < GRID_Y_SIZE
                {
                    context.add_particle(Point(
                        x / DOT_SIZE_IN_PXS as i32,
                        y / DOT_SIZE_IN_PXS as i32,
                    ));
                }
            }

            frame_counter = 0;
            context.next_tick();
        }

        renderer.draw(&context)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32) / 144);
    }

    Ok(())
}
