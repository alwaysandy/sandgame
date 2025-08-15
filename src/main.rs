extern crate sdl2;

mod gamecontext;
mod particle;
mod point;
mod renderer;

use crate::{gamecontext::*, particle::*, point::*, renderer::*};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use std::time::Duration;

const GRID_X_SIZE: usize = 75;
const GRID_Y_SIZE: usize = 75;
const DOT_SIZE_IN_PXS: usize = 10;

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

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::ARGB8888,
            GRID_X_SIZE as u32,
            GRID_Y_SIZE as u32,
        )
        .map_err(|e| e.to_string())?;
    let mut renderer = Renderer { canvas, texture };
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
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => context.placing_particle = Particle::default(),
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => context.placing_particle = Particle::concrete(),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => context.placing_particle = Particle::sand(),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => context.placing_particle = Particle::wall(),
                Event::MouseButtonDown { .. } => mouse_down = true,
                Event::MouseButtonUp { .. } => mouse_down = false,
                _ => {}
            }
        }

        frame_counter += 1;
        if frame_counter % 5 == 0 {
            if frame_counter % 15 == 0 {
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
            }

            context.next_tick();
        }

        renderer.draw(&context)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32) / 144);
    }

    Ok(())
}
