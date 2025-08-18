use crate::{GRID_X_SIZE, GRID_Y_SIZE, particle::*, point::*};

use std::collections::{BinaryHeap, HashSet};

pub struct GameContext {
    to_update: BinaryHeap<Point>,
    to_update_set: HashSet<Point>,
    next_update: BinaryHeap<Point>,
    next_update_set: HashSet<Point>,
    water_particles: BinaryHeap<Point>,
    water_particles_set: HashSet<Point>,
    pub grid: [[Particle; GRID_X_SIZE]; GRID_Y_SIZE],
    pub placing_particle: Particle,
}

impl Default for GameContext {
    fn default() -> Self {
        GameContext {
            to_update: BinaryHeap::new(),
            to_update_set: HashSet::new(),
            next_update: BinaryHeap::new(),
            next_update_set: HashSet::new(),
            water_particles: BinaryHeap::new(),
            water_particles_set: HashSet::new(),
            grid: [[Particle::air(); GRID_X_SIZE]; GRID_Y_SIZE],
            placing_particle: Particle::sand(),
        }
    }
}

impl GameContext {
    pub fn new() -> Self {
        GameContext::default()
    }

    pub fn add_particle(&mut self, point: Point) -> bool {
        match self.placing_particle.particle_type {
            ParticleType::Sand
            | ParticleType::Wall
            | ParticleType::Concrete
            | ParticleType::Water => self.place_particle(point),
            ParticleType::Air => self.delete_particle(point),
        }
    }

    fn place_particle(&mut self, point: Point) -> bool {
        if !self.is_air(&point) {
            return false;
        }

        match self.placing_particle.particle_type {
            ParticleType::Wall => self.grid[point.1 as usize][point.0 as usize] = Particle::wall(),
            ParticleType::Sand => self.grid[point.1 as usize][point.0 as usize] = Particle::sand(),
            ParticleType::Water => {
                self.grid[point.1 as usize][point.0 as usize] = Particle::water();
                self.water_particles.push(point);
                self.water_particles_set.insert(point);
            }
            ParticleType::Concrete => {
                self.grid[point.1 as usize][point.0 as usize] = Particle::concrete();
                return true;
            }
            ParticleType::Air => (),
        }

        self.next_update.push(point);
        self.next_update_set.insert(point);
        true
    }

    fn delete_particle(&mut self, point: Point) -> bool {
        match self.grid[point.1 as usize][point.0 as usize].particle_type {
            ParticleType::Air => (),
            ParticleType::Sand | ParticleType::Wall | ParticleType::Concrete => {
                self.grid[point.1 as usize][point.0 as usize] = Particle::air();
                self.next_update_set.remove(&point);
                self.propogate_updates(&point);
            }
            ParticleType::Water => unimplemented!(),
        }

        true
    }

    pub fn next_tick(&mut self) {
        self.update_sand_physics_particles();
        self.update_water_physics_particles();
    }

    fn update_sand_physics_particles(&mut self) {
        self.to_update = self.next_update.clone();
        self.to_update_set = self.next_update_set.clone();
        self.next_update.clear();
        self.next_update_set.clear();
        while let Some(point) = self.to_update.pop() {
            if !self.to_update_set.contains(&point) {
                continue;
            }

            match self.grid[point.1 as usize][point.0 as usize].particle_physics {
                ParticlePhysics::Sand | ParticlePhysics::Wall => (),
                _ => continue,
            }

            if let Some(below) = point + Point(0, 1)
                && self.can_fill(&below)
            {
                self.swap_particle(&point, &below);
                self.add_updates(&point, &below);
                continue;
            }

            match self.grid[point.1 as usize][point.0 as usize].particle_physics {
                ParticlePhysics::Sand => (),
                _ => continue,
            }

            let mut choices: Vec<Point> = Vec::new();
            if let Some(down_left) = point + Point(-1, 1)
                && self.can_fill(&down_left)
                && let Some(left) = point + Point(-1, 0)
                && self.can_fill(&left)
            {
                choices.push(down_left);
            }

            if let Some(down_right) = point + Point(1, 1)
                && self.can_fill(&down_right)
                && let Some(right) = point + Point(1, 0)
                && self.can_fill(&right)
            {
                choices.push(down_right);
            }

            // Randomly choose between going downleft or downright
            if let Some(choice) = fastrand::choice(choices) {
                self.swap_particle(&point, &choice);
                self.add_updates(&point, &choice);
                continue;
            }
        }
    }

    fn update_water_physics_particles(&mut self) {
        let mut water_particles: BinaryHeap<Point> = BinaryHeap::new();
        let mut water_particles_set: HashSet<Point> = HashSet::new();
        while let Some(point) = self.water_particles.pop() {
            if !self.water_particles_set.contains(&point) {
                continue;
            }

            if let Some(below) = point + Point(0, 1)
                && self.is_air(&below)
            {
                self.swap_particle(&point, &below);
                self.update_water(
                    &point,
                    &below,
                    &mut water_particles,
                    &mut water_particles_set,
                );
                continue;
            }

            let mut choices: Vec<Point> = Vec::new();
            if let Some(down_left) = point + Point(-1, 1)
                && self.is_air(&down_left)
                && let Some(left) = point + Point(-1, 0)
                && self.is_air(&left)
            {
                choices.push(down_left);
            }

            if let Some(down_right) = point + Point(1, 1)
                && self.is_air(&down_right)
                && let Some(right) = point + Point(1, 0)
                && self.is_air(&right)
            {
                choices.push(down_right);
            }

            if let Some(choice) = fastrand::choice(choices) {
                self.swap_particle(&point, &choice);
                self.update_water(
                    &point,
                    &choice,
                    &mut water_particles,
                    &mut water_particles_set,
                );
                continue;
            }

            water_particles.push(point);
            water_particles_set.insert(point);
        }

        self.water_particles = water_particles;
        self.water_particles_set = water_particles_set;
    }

    fn swap_particle(&mut self, orig_point: &Point, new_point: &Point) {
        let particle = self.grid[orig_point.1 as usize][orig_point.0 as usize];
        match self.grid[new_point.1 as usize][new_point.0 as usize].particle_type {
            ParticleType::Air => {
                self.grid[orig_point.1 as usize][orig_point.0 as usize] = Particle::air();
                self.grid[new_point.1 as usize][new_point.0 as usize] = particle;
            }
            ParticleType::Water => {
                self.water_particles_set.remove(new_point);
                self.water_particles.push(*orig_point);
                self.water_particles_set.insert(*orig_point);
                self.grid[orig_point.1 as usize][orig_point.0 as usize] = Particle::water();
                self.grid[new_point.1 as usize][new_point.0 as usize] = particle;
            }
            _ => unreachable!(),
        }
    }

    fn add_updates(&mut self, origin: &Point, new_point: &Point) {
        self.next_update.push(*new_point);
        self.next_update_set.insert(*new_point);
        self.propogate_updates(origin);
    }

    fn update_water(
        &mut self,
        origin: &Point,
        new_point: &Point,
        water_particles: &mut BinaryHeap<Point>,
        water_particles_set: &mut HashSet<Point>,
    ) {
        water_particles.push(*new_point);
        water_particles_set.insert(*new_point);
        water_particles_set.remove(origin);
        self.propogate_updates(origin);
    }

    fn propogate_updates(&mut self, point: &Point) {
        for d in -1..2 {
            if let Some(p) = *point + Point(d, -1) {
                if self.next_update_set.contains(&p) {
                    continue;
                }

                match self.grid[p.1 as usize][p.0 as usize].particle_physics {
                    ParticlePhysics::Sand | ParticlePhysics::Wall => (),
                    ParticlePhysics::Water => continue,
                    ParticlePhysics::None => continue,
                }

                self.next_update.push(p);
                self.next_update_set.insert(p);
            }
        }
    }

    fn is_air(&self, point: &Point) -> bool {
        self.grid[point.1 as usize][point.0 as usize].particle_type == ParticleType::Air
    }

    fn can_fill(&self, point: &Point) -> bool {
        match self.grid[point.1 as usize][point.0 as usize].particle_type {
            ParticleType::Air | ParticleType::Water => true,
            _ => false,
        }
    }
}
