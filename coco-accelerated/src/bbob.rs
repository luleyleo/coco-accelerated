use strum::{AsRefStr, EnumCount, EnumIter};

pub static DIMENSIONS: &[usize] = &[2, 3, 5, 10, 20, 40];

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter, AsRefStr)]
#[repr(usize)]
pub enum Function {
    Sphere = 1,
    Ellipsoid,
    Rastrigin,
    BuecheRastrigin,
    LinearSlope,
    AttractiveSector,
    StepEllipsoid,
    Rosenbrock,
    RosenbrockRotated,
    EllipsoidRotated,
    Discus,
    BentCigar,
    SharpRidge,
    DifferentPowers,
    RastriginRotated,
    Weierstrass,
    Schaffers1,
    Schaffers2,
    GriewankRosenbrock,
    Schwefel,
    Gallagher1,
    Gallagher2,
    Katsuura,
    LunacekBiRastrigin,
}

impl Function {
    pub fn name(&self) -> &str {
        self.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use strum::EnumCount;

    use crate::bbob::Function;

    #[test]
    fn numbers_match() {
        assert_eq!(Function::COUNT, 24);
        assert_eq!(Function::Sphere as usize, 1);
        assert_eq!(Function::LunacekBiRastrigin as usize, Function::COUNT);
    }
}
