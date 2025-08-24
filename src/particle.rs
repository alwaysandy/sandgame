#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Particle {
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

impl Default for Particle {
    fn default() -> Self {
        Particle::Air
    }
}

impl Particle {
    pub fn physics(&self) -> ParticlePhysics {
        match self {
            Particle::Air | Particle::Concrete => ParticlePhysics::None,
            Particle::Sand => ParticlePhysics::Sand,
            Particle::Wall => ParticlePhysics::Wall,
            Particle::Water => ParticlePhysics::Water
        }
    }
}
