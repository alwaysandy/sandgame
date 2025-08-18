#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParticleType {
    Air,
    Sand,
    Wall,
    Concrete,
    Water,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParticlePhysics {
    None,
    Sand,
    Wall,
    Water,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub particle_physics: ParticlePhysics,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            particle_type: ParticleType::Air,
            particle_physics: ParticlePhysics::None,
        }
    }
}

impl Particle {
    pub fn air() -> Self {
        Self {
            particle_type: ParticleType::Air,
            particle_physics: ParticlePhysics::None,
        }
    }

    pub fn sand() -> Self {
        Self {
            particle_type: ParticleType::Sand,
            particle_physics: ParticlePhysics::Sand,
        }
    }

    pub fn wall() -> Self {
        Self {
            particle_type: ParticleType::Wall,
            particle_physics: ParticlePhysics::Wall,
        }
    }

    pub fn concrete() -> Self {
        Self {
            particle_type: ParticleType::Concrete,
            particle_physics: ParticlePhysics::None,
        }
    }

    pub fn water() -> Self {
        Self {
            particle_type: ParticleType::Water,
            particle_physics: ParticlePhysics::Water,
        }
    }
}
