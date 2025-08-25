use crate::{GRID_X_SIZE, GRID_Y_SIZE, particle::*, point::*};

use std::collections::HashSet;

pub struct GameContext {
    to_update: Vec<Point>,
    next_updates: HashSet<Point>,
    pub grid: [[Particle; GRID_X_SIZE]; GRID_Y_SIZE],
    pub placing_particle: Particle,
    pub running: bool,
}

impl Default for GameContext {
    fn default() -> Self {
        GameContext {
            to_update: Vec::new(),
            next_updates: HashSet::new(),
            grid: [[Particle::Air; GRID_X_SIZE]; GRID_Y_SIZE],
            placing_particle: Particle::Sand,
            running: true,
        }
    }
}

impl GameContext {
    pub fn new() -> Self {
        GameContext::default()
    }

    pub fn add_particle(&mut self, point: Point) {
        match self.placing_particle {
            Particle::Sand | Particle::Wall | Particle::Concrete | Particle::Water => {
                self.place_particle(point)
            }
            Particle::Air => self.delete_particle(point),
        }
    }

    fn place_particle(&mut self, point: Point) {
        if !self.is_air(&point) || matches!(self.placing_particle, Particle::Air) {
            return;
        }

        self.grid[point.y()][point.x()] = self.placing_particle;
        self.next_updates.insert(point);
    }

    fn delete_particle(&mut self, point: Point) {
        if matches!(self.grid[point.y()][point.x()], Particle::Air) {
            return;
        }

        self.grid[point.y()][point.x()] = Particle::Air;
        self.next_updates.remove(&point);
        self.propagate_updates(&point);
    }

    pub fn next_tick(&mut self) {
        self.move_particles();
    }

    fn move_particles(&mut self) {
        self.to_update.extend(self.next_updates.drain());
        while let Some(point) = self.to_update.pop() {
            let particle = self.grid[point.y()][point.x()];
            match particle.physics() {
                ParticlePhysics::Sand | ParticlePhysics::Wall | ParticlePhysics::Water => (),
                _ => continue,
            }

            if let Some(below) = point.below()
                && self.can_move_into(&point, &below)
            {
                self.swap_particle(&point, &below);
                self.add_updates(&point, &below);
                continue;
            }

            match self.grid[point.y()][point.x()].physics() {
                ParticlePhysics::Sand | ParticlePhysics::Water => (),
                _ => continue,
            }

            let mut choices: Vec<Point> = Vec::new();
            if let Some(down_left) = point.down_left()
                && let Some(left) = point.left()
                && self.can_move_into(&point, &left)
                && self.can_move_into(&point, &down_left)
            {
                choices.push(down_left);
            }

            if let Some(down_right) = point.down_right()
                && let Some(right) = point.right()
                && self.can_move_into(&point, &right)
                && self.can_move_into(&point, &down_right)
            {
                choices.push(down_right);
            }

            // Randomly choose between going downleft or downright
            if let Some(choice) = fastrand::choice(&choices) {
                self.swap_particle(&point, choice);
                self.add_updates(&point, choice);
                continue;
            }

            match self.grid[point.y()][point.x()].physics() {
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
            && self.grid[next_point.y()][next_point.x()] == Particle::Water
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
            if self.grid[next_point.y()][next_point.x()] == Particle::Water {
                current_point = next_point;
                continue;
            }

            return match self.grid[next_point.y()][next_point.x()] {
                Particle::Air => Some(next_point),
                _ => None,
            };
        }

        None
    }

    fn swap_particle(&mut self, orig_point: &Point, new_point: &Point) {
        let particle = self.grid[orig_point.y()][orig_point.x()];
        match self.grid[new_point.y()][new_point.x()] {
            Particle::Air => {
                self.grid[orig_point.y()][orig_point.x()] = Particle::Air;
                self.grid[new_point.y()][new_point.x()] = particle;
            }
            Particle::Water => {
                self.grid[orig_point.y()][orig_point.x()] = Particle::Water;
                self.grid[new_point.y()][new_point.x()] = particle;
            }
            _ => unreachable!(),
        }
    }

    fn add_updates(&mut self, origin: &Point, new_point: &Point) {
        self.next_updates.insert(*new_point);
        self.propagate_updates(origin);
        self.propagate_updates(new_point);
    }

    fn propagate_updates(&mut self, point: &Point) {
        for y in -1..=2 {
            for x in -1..=1 {
                if let Some(p) = *point + Point(x, y) {
                    self.next_updates.insert(p);
                }
            }
        }
    }

    fn is_air(&self, point: &Point) -> bool {
        matches!(self.grid[point.y()][point.x()], Particle::Air)
    }

    fn can_move_into(&self, origin: &Point, point: &Point) -> bool {
        match self.grid[origin.y()][origin.x()] {
            Particle::Water => matches!(self.grid[point.y()][point.x()], Particle::Air),
            _ => matches!(
                self.grid[point.y()][point.x()],
                Particle::Air | Particle::Water
            ),
        }
    }
}
