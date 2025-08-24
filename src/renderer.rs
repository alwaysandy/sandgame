use crate::{DOT_SIZE_IN_PXS, GRID_X_SIZE, GRID_Y_SIZE, gamecontext::*};
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

const PARTICLE_COLORS: [[u8; 3]; 5] = [
    [199, 229, 252], // Air
    [237, 207, 168], // Sand
    [200, 200, 200], // Wall
    [100, 100, 100], // Concrete
    [0, 0, 200],     // Water
];

pub struct Renderer<'t> {
    pub canvas: WindowCanvas,
    pub texture: Texture<'t>,
}

impl<'t> Renderer<'t> {
    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.draw_particles(context)?;
        let dst_rect = Rect::new(
            0,
            0,
            (GRID_X_SIZE * DOT_SIZE_IN_PXS) as u32,
            (GRID_Y_SIZE * DOT_SIZE_IN_PXS) as u32,
        );
        self.canvas.copy(&self.texture, dst_rect, None)?;
        self.canvas.present();
        Ok(())
    }

    fn draw_particles(&mut self, context: &GameContext) -> Result<(), String> {
        self.texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for (y, line) in context.grid.iter().enumerate() {
                    for (x, particle) in line.iter().enumerate() {
                        let offset = y * pitch + x * 4;
                        let colors: [u8; 3] = PARTICLE_COLORS[*particle as usize];
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
