use crate::{point::*, GRID_X_SIZE, GRID_Y_SIZE};

use std::collections::BinaryHeap;
use fastrand;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParticleType {
    Sand,
    Air
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Particle {
    pub particle_type: ParticleType,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            particle_type: ParticleType::Air
        }
    }
}

pub struct GameContext {
    to_update: BinaryHeap<Point>,
    pub grid: [[Particle; GRID_X_SIZE]; GRID_Y_SIZE],
}

impl Default for GameContext {
    fn default() -> Self {
        GameContext {
            to_update: BinaryHeap::new(),
            grid: [[Particle::default(); GRID_X_SIZE]; GRID_Y_SIZE],
        }
    }
}

impl GameContext {
    pub fn new() -> Self {
        GameContext::default()
    }

    pub fn add_particle(&mut self, particle_type: ParticleType, point: Point) -> bool {
        if self.grid[point.1 as usize][point.0 as usize].particle_type != ParticleType::Air {
            return false;
        }

        self.grid[point.1 as usize][point.0 as usize].particle_type = particle_type;
        self.to_update.push(point);
        true
    }

    pub fn next_tick(&mut self) {
        let mut next_updates: BinaryHeap<Point> = BinaryHeap::new();
        while self.to_update.len() > 0 {
            let point = self.to_update.pop().unwrap();
            if let Some(below) = point + Point(0, 1) {
                if self.grid[below.1 as usize][below.0 as usize].particle_type == ParticleType::Air {
                    self.grid[point.1 as usize][point.0 as usize].particle_type = ParticleType::Air;
                    self.grid[below.1 as usize][below.0 as usize].particle_type = ParticleType::Sand;
                    next_updates.push(below);
                    continue;
                }
            }

            let can_move_down_left = if let Some(left) = point + Point(-1, 1) {
                self.grid[left.1 as usize][left.0 as usize].particle_type == ParticleType::Air
            } else {
                false
            };

            let can_move_down_right = if let Some(right) = point + Point(1, 1) {
                self.grid[right.1 as usize][right.0 as usize].particle_type == ParticleType::Air
            } else {
                false
            };

            if can_move_down_left && can_move_down_right {
                let moveleft = fastrand::bool();
                if moveleft {
                    let left = (point + Point(-1, 1)).unwrap();
                    self.grid[point.1 as usize][point.0 as usize].particle_type = ParticleType::Air;
                    self.grid[left.1 as usize][left.0 as usize].particle_type = ParticleType::Sand;
                    next_updates.push(left);
                } else {
                    let right = (point + Point(1, 1)).unwrap();
                    self.grid[point.1 as usize][point.0 as usize].particle_type = ParticleType::Air;
                    self.grid[right.1 as usize][right.0 as usize].particle_type = ParticleType::Sand;
                    next_updates.push(right);
                }
            } else if can_move_down_left {
                let left = (point + Point(-1, 1)).unwrap();
                self.grid[point.1 as usize][point.0 as usize].particle_type = ParticleType::Air;
                self.grid[left.1 as usize][left.0 as usize].particle_type = ParticleType::Sand;
                next_updates.push(left);
            } else if can_move_down_right {
                let right = (point + Point(1, 1)).unwrap();
                self.grid[point.1 as usize][point.0 as usize].particle_type = ParticleType::Air;
                self.grid[right.1 as usize][right.0 as usize].particle_type = ParticleType::Sand;
                next_updates.push(right);
            }
        }

        self.to_update = next_updates;
    }
}
