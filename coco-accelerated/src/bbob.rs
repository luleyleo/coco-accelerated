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

impl Function {
    pub fn name(&self) -> &'static str {
        match self {
            Function::Sphere => "sphere",
            Function::Ellipsoid => "ellipsoid",
            Function::Rastrigin => "rastrigin",
            Function::BuecheRastrigin => "bueche_rastrigin",
            Function::LinearSlope => "linear_slope",
            Function::AttractiveSector => "attractive_sector",
            Function::StepEllipsoid => "step_ellipsoid",
            Function::Rosenbrock => "rosenbrock",
            Function::RosenbrockRotated => "rosenbrock_rotated",
            Function::EllipsoidRotated => "ellipsoid_rotated",
            Function::Discus => "discus",
            Function::BentCigar => "bent_cigar",
            Function::SharpRidge => "sharp_ridge",
            Function::DifferentPowers => "different_powers",
            Function::RastriginRotated => "rastrigin_rotated",
            Function::Weierstrass => "weierstrass",
            Function::Schaffers1 => "schaffers1",
            Function::Schaffers2 => "schaffers2",
            Function::GriewankRosenbrock => "griewank_rosenbrock",
            Function::Schwefel => "schwefel",
            Function::Gallagher1 => "gallagher1",
            Function::Gallagher2 => "gallagher2",
            Function::Katsuura => "katsuura",
            Function::LunacekBiRastrigin => "lunacek_bi_rastrigin",
        }
    }
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
