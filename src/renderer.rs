use crate::{DOT_SIZE_IN_PXS, gamecontext::*, point::*};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * DOT_SIZE_IN_PXS as i32,
            y * DOT_SIZE_IN_PXS as i32,
            DOT_SIZE_IN_PXS as u32,
            DOT_SIZE_IN_PXS as u32,
        ))?;
        Ok(())
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.draw_background();
        self.draw_particles(context)?;
        self.canvas.present();
        Ok(())
    }

    fn draw_particles(&mut self, context: &GameContext) -> Result<(), String> {
        for (y, line) in context.grid.iter().enumerate() {
            for (x, particle) in line.iter().enumerate() {
                match particle.particle_type {
                    ParticleType::Air => (),
                    ParticleType::Sand => {
                        self.canvas.set_draw_color(Color::RED);
                        self.draw_dot(&Point(x as i32, y as i32))?;
                    },
                    ParticleType::Wall => {
                        self.canvas.set_draw_color(Color::GRAY);
                        self.draw_dot(&Point(x as i32, y as i32))?;
                    }
                }
            }
        }

        Ok(())
    }

    fn draw_background(&mut self) {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
    }
}
