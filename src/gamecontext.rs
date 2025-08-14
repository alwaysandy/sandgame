use crate::{GRID_X_SIZE, GRID_Y_SIZE, point::*};

use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParticleType {
    Sand,
    Air,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Particle {
    pub particle_type: ParticleType,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            particle_type: ParticleType::Air,
        }
    }
}

pub struct GameContext {
    to_update: BinaryHeap<Point>,
    to_update_set: HashSet<Point>,
    pub grid: [[Particle; GRID_X_SIZE]; GRID_Y_SIZE],
    pub placing_particle: ParticleType,
}

impl Default for GameContext {
    fn default() -> Self {
        GameContext {
            to_update: BinaryHeap::new(),
            to_update_set: HashSet::new(),
            grid: [[Particle::default(); GRID_X_SIZE]; GRID_Y_SIZE],
            placing_particle: ParticleType::Sand,
        }
    }
}

impl GameContext {
    pub fn new() -> Self {
        GameContext::default()
    }

    pub fn add_particle(&mut self, point: Point) -> bool {
        match self.placing_particle {
            ParticleType::Sand => self.add_sand_particle(point),
            ParticleType::Air => self.add_air_particle(point),
        }
    }

    fn add_sand_particle(&mut self, point: Point) -> bool {
        if !self.is_air(&point) {
            return false;
        }

        self.grid[point.1 as usize][point.0 as usize].particle_type = ParticleType::Sand;
        self.to_update.push(point);
        self.to_update_set.insert(point);
        true
    }

    fn add_air_particle(&mut self, point: Point) -> bool {
        self.grid[point.1 as usize][point.0 as usize].particle_type = ParticleType::Air;
        self.to_update_set.remove(&point);
        self.propogate_updates(&point);
        true
    }

    pub fn next_tick(&mut self) {
        let mut to_update: BinaryHeap<Point> = BinaryHeap::new();
        let mut to_update_set: HashSet<Point> = HashSet::new();
        while let Some(point) = self.to_update.pop() {
            if !self.to_update_set.contains(&point) {
                continue;
            }

            if let Some(below) = point + Point(0, 1)
                && self.is_air(&below)
            {
                self.move_particle(&point, &below);
                self.add_sand_updates(&point, &below, &mut to_update, &mut to_update_set);
                continue;
            }

            let down_left = if let Some(down_left) = point + Point(-1, 1)
                && self.is_air(&down_left)
            {
                Some(down_left)
            } else {
                None
            };

            let down_right = if let Some(down_right) = point + Point(1, 1)
                && self.is_air(&down_right)
            {
                Some(down_right)
            } else {
                None
            };

            if let Some(down_left) = down_left
                && let Some(down_right) = down_right
            {
                let move_left = fastrand::bool();
                if move_left {
                    self.move_particle(&point, &down_left);
                    self.add_sand_updates(&point, &down_left, &mut to_update, &mut to_update_set);
                } else {
                    self.move_particle(&point, &down_right);
                    self.add_sand_updates(&point, &down_right, &mut to_update, &mut to_update_set);
                }
            } else if let Some(down_left) = down_left {
                self.move_particle(&point, &down_left);
                self.add_sand_updates(&point, &down_left, &mut to_update, &mut to_update_set);
            } else if let Some(down_right) = down_right {
                self.move_particle(&point, &down_right);
                self.add_sand_updates(&point, &down_right, &mut to_update, &mut to_update_set);
            }
        }

        self.to_update = to_update;
        self.to_update_set = to_update_set;
    }

    fn move_particle(&mut self, orig_point: &Point, new_point: &Point) {
        self.grid[orig_point.1 as usize][orig_point.0 as usize].particle_type = ParticleType::Air;
        self.grid[new_point.1 as usize][new_point.0 as usize].particle_type = ParticleType::Sand;
    }

    fn add_sand_updates(
        &mut self,
        origin: &Point,
        new_point: &Point,
        to_update: &mut BinaryHeap<Point>,
        to_update_set: &mut HashSet<Point>,
    ) {
        to_update.push(*new_point);
        to_update_set.insert(*new_point);
        self.propogate_updates(origin);
    }

    fn propogate_updates(&mut self, point: &Point) {
        for d in -1..2 {
            if let Some(p) = *point + Point(d, -1)
                && self.grid[p.1 as usize][p.0 as usize].particle_type == ParticleType::Sand
            {
                if self.to_update_set.contains(&p) {
                    continue;
                }

                self.to_update.push(p);
                self.to_update_set.insert(p);
            }
        }
    }

    fn is_air(&self, point: &Point) -> bool {
        self.grid[point.1 as usize][point.0 as usize].particle_type == ParticleType::Air
    }
}
