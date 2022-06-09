pub static DIMENSIONS: &[usize] = &[2, 3, 5, 10, 20, 40];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use crate::bbob::Function;

    #[test]
    fn numbers_match() {
        assert_eq!(Function::Sphere as usize, 1);
        assert_eq!(Function::LunacekBiRastrigin as usize, 24);
    }
}
