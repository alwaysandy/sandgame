use crate::gamecontext::*;
use sdl2::render::{Texture, WindowCanvas};

const PARTICLE_COLORS: [[u8; 3]; 4] = [
    [199, 229, 252], // Air
    [237, 207, 168], // Sand
    [200, 200, 200], // Wall
    [100, 100, 100], // Concrete
];

pub struct Renderer<'t> {
    pub canvas: WindowCanvas,
    pub texture: Texture<'t>,
}

impl<'t> Renderer<'t> {
    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.draw_particles(context)?;
        self.canvas.copy(&self.texture, None, None)?;
        self.canvas.present();
        Ok(())
    }

    fn draw_particles(&mut self, context: &GameContext) -> Result<(), String> {
        self.texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for (y, line) in context.grid.iter().enumerate() {
                    for (x, particle) in line.iter().enumerate() {
                        let offset = y * pitch + x * 4;
                        let colors: [u8; 3] = PARTICLE_COLORS[particle.particle_type as usize];
                        buffer[offset + 0] = colors[2]; // B
                        buffer[offset + 1] = colors[1]; // G
                        buffer[offset + 2] = colors[0]; // R
                        buffer[offset + 3] = 255; // A
                    }
                }
            })?;
        Ok(())
    }
}
