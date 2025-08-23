use crate::{GRID_X_SIZE, GRID_Y_SIZE, particle::*, point::*};

use std::collections::{BinaryHeap, HashSet};

pub struct GameContext {
    to_update: BinaryHeap<Point>,
    to_update_set: HashSet<Point>,
    next_update: BinaryHeap<Point>,
    next_update_set: HashSet<Point>,
    pub grid: [[Particle; GRID_X_SIZE]; GRID_Y_SIZE],
    pub placing_particle: Particle,
    pub running: bool,
}

impl Default for GameContext {
    fn default() -> Self {
        GameContext {
            to_update: BinaryHeap::new(),
            to_update_set: HashSet::new(),
            next_update: BinaryHeap::new(),
            next_update_set: HashSet::new(),
            grid: [[Particle::air(); GRID_X_SIZE]; GRID_Y_SIZE],
            placing_particle: Particle::sand(),
            running: true,
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
            ParticleType::Wall => self.grid[point.y()][point.x()] = Particle::wall(),
            ParticleType::Sand => self.grid[point.y()][point.x()] = Particle::sand(),
            ParticleType::Water => self.grid[point.y()][point.x()] = Particle::water(),
            ParticleType::Concrete => {
                self.grid[point.y()][point.x()] = Particle::concrete();
                return true;
            }
            ParticleType::Air => (),
        }

        self.next_update.push(point);
        self.next_update_set.insert(point);
        true
    }

    fn delete_particle(&mut self, point: Point) -> bool {
        match self.grid[point.y()][point.x()].particle_type {
            ParticleType::Air => (),
            ParticleType::Sand
            | ParticleType::Wall
            | ParticleType::Concrete
            | ParticleType::Water => {
                self.grid[point.y()][point.x()] = Particle::air();
                self.next_update_set.remove(&point);
                self.propagate_updates(&point);
            }
        }

        true
    }

    pub fn next_tick(&mut self) {
        self.move_particles();
    }

    fn move_particles(&mut self) {
        self.to_update.append(&mut self.next_update);
        self.to_update_set.clear();
        self.to_update_set.extend(self.next_update_set.drain());

        while let Some(point) = self.to_update.pop() {
            if !self.to_update_set.contains(&point) {
                continue;
            }

            let particle = self.grid[point.y()][point.x()];
            match particle.particle_physics {
                ParticlePhysics::Sand | ParticlePhysics::Wall | ParticlePhysics::Water => (),
                _ => continue,
            }

            if let Some(below) = point.below() {
                match particle.particle_type {
                    ParticleType::Water => {
                        if self.is_air(&below) {
                            self.swap_particle(&point, &below);
                            self.add_updates(&point, &below);
                            continue;
                        }
                    }
                    ParticleType::Sand | ParticleType::Wall => {
                        if self.can_fill(&below) {
                            self.swap_particle(&point, &below);
                            self.add_updates(&point, &below);
                            continue;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            match self.grid[point.y()][point.x()].particle_physics {
                ParticlePhysics::Sand | ParticlePhysics::Water => (),
                _ => continue,
            }

            let mut choices: Vec<Point> = Vec::new();
            if let Some(down_left) = point.down_right()
                && let Some(left) = point.left()
            {
                match particle.particle_type {
                    ParticleType::Water => {
                        if self.is_air(&down_left) && self.is_air(&left) {
                            choices.push(down_left);
                        }
                    }
                    _ => {
                        if self.can_fill(&down_left) && self.can_fill(&left) {
                            choices.push(down_left);
                        }
                    }
                }
            }

            if let Some(down_right) = point.down_right()
                && let Some(right) = point.right()
            {
                match particle.particle_type {
                    ParticleType::Water => {
                        if self.is_air(&down_right) && self.is_air(&right) {
                            choices.push(down_right);
                        }
                    }
                    _ => {
                        if self.can_fill(&down_right) && self.can_fill(&right) {
                            choices.push(down_right);
                        }
                    }
                }
            }

            // Randomly choose between going downleft or downright
            if let Some(choice) = fastrand::choice(&choices) {
                self.swap_particle(&point, choice);
                self.add_updates(&point, choice);
                continue;
            }

            match self.grid[point.y()][point.x()].particle_physics {
                ParticlePhysics::Water => (),
                _ => continue,
            }

            if !self.hits_max_pressure(&point, 1) {
                continue;
            }

            choices.clear();
            if let Some(left) = self.get_next_free_space(&point, Point(-1, 0)) {
                choices.push(left);
            }

            if let Some(right) = self.get_next_free_space(&point, Point(1, 0)) {
                choices.push(right);
            }

            if let Some(new_point) = fastrand::choice(choices) {
                self.swap_particle(&point, &new_point);
                self.add_updates(&point, &new_point);
                continue;
            }
        }
    }

    fn hits_max_pressure(&self, point: &Point, threshold: usize) -> bool {
        let mut pressure = 0;
        let mut current_point = *point;
        while let Some(next_point) = current_point.above()
            && self.grid[next_point.y()][next_point.x()].particle_type == ParticleType::Water
        {
            pressure += 1;
            if pressure >= threshold {
                return true;
            }

            current_point.1 -= 1;
        }

        false
    }

    fn get_next_free_space(&self, point: &Point, direction: Point) -> Option<Point> {
        let mut current_point = *point;
        while let Some(next_point) = current_point + direction {
            if self.grid[next_point.y()][next_point.x()].particle_type == ParticleType::Water {
                current_point = (current_point + direction).unwrap();
                continue;
            }

            return match self.grid[next_point.y()][next_point.x()].particle_type {
                ParticleType::Air => Some(next_point),
                _ => None,
            };
        }

        None
    }

    fn swap_particle(&mut self, orig_point: &Point, new_point: &Point) {
        let particle = self.grid[orig_point.y()][orig_point.x()];
        match self.grid[new_point.y()][new_point.x()].particle_type {
            ParticleType::Air => {
                self.grid[orig_point.y()][orig_point.x()] = Particle::air();
                self.grid[new_point.y()][new_point.x()] = particle;
            }
            ParticleType::Water => {
                self.grid[orig_point.y()][orig_point.x()] = Particle::water();
                self.grid[new_point.y()][new_point.x()] = particle;
            }
            _ => unreachable!(),
        }
    }

    fn add_updates(&mut self, origin: &Point, new_point: &Point) {
        self.next_update.push(*new_point);
        self.next_update_set.insert(*new_point);
        self.propagate_updates(origin);
    }

    fn propagate_updates(&mut self, point: &Point) {
        for y in 0..=2 {
            for x in -1..=1 {
                if let Some(p) = *point + Point(x, y) {
                    if self.next_update_set.contains(&p) {
                        continue;
                    }

                    match self.grid[p.y()][p.x()].particle_physics {
                        ParticlePhysics::Sand | ParticlePhysics::Wall | ParticlePhysics::Water => {}
                        ParticlePhysics::None => continue,
                    }

                    self.next_update.push(p);
                    self.next_update_set.insert(p);
                }
            }
        }
    }

    fn is_air(&self, point: &Point) -> bool {
        self.grid[point.y()][point.x()].particle_type == ParticleType::Air
    }

    fn can_fill(&self, point: &Point) -> bool {
        match self.grid[point.y()][point.x()].particle_type {
            ParticleType::Air | ParticleType::Water => true,
            _ => false,
        }
    }
}
