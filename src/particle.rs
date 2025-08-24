#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Particle {
    #[default]
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

impl Particle {
    pub fn physics(&self) -> ParticlePhysics {
        match self {
            Particle::Air | Particle::Concrete => ParticlePhysics::None,
            Particle::Sand => ParticlePhysics::Sand,
            Particle::Wall => ParticlePhysics::Wall,
            Particle::Water => ParticlePhysics::Water,
        }
    }
}
