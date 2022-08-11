use coco_legacy::Matrix;
use ordered_float::OrderedFloat;
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

pub enum Params {
    Basic {
        fopt: f64,
        xopt: Vec<f64>,
    },
    Rotated {
        fopt: f64,
        xopt: Vec<f64>,
        R: Matrix,
    },
    FixedRotated {
        fopt: f64,
        R: Matrix,
    },
    DoubleRotated {
        fopt: f64,
        xopt: Vec<f64>,
        R: Matrix,
        Q: Matrix,
    },
    Gallagher {
        fopt: f64,
        peaks: usize,
        y: Vec<f64>,
        a: Vec<f64>,
        R: Matrix,
    },
}

impl Params {
    pub fn from(function: Function, dimension: usize, instance: usize) -> Self {
        let rseed: usize = function as usize + 10000 * instance;
        let rseed_3: usize = 3 + 10000 * instance;
        let rseed_17: usize = 17 + 10000 * instance;

        // These (as well as bent_cigar) use a different rseed.
        let rseed = match function {
            Function::BuecheRastrigin => rseed_3,
            Function::Schaffers2 => rseed_17,
            _ => rseed,
        };

        let mut xopt = coco_legacy::compute_xopt(rseed, dimension);
        let fopt = coco_legacy::compute_fopt(function as usize, instance);

        // Special cases for some functions.
        match function {
            Function::BuecheRastrigin => {
                // OME: This step is in the legacy C code but _not_ in the function description.
                for xi in xopt.iter_mut().step_by(2) {
                    *xi = xi.abs();
                }
            }
            Function::LinearSlope => {
                for xi in &mut xopt {
                    *xi = if *xi >= 0.0 { 5.0 } else { -5.0 };
                }
            }
            Function::Rosenbrock => {
                // According to the documentation, xopt should be within [-3, 3],
                // but this is not enough to satisfy that condition...
                for xi in &mut xopt {
                    *xi *= 0.75;
                }
            }
            Function::BentCigar => {
                // No clue why they did this, probably it was a typo?
                xopt = coco_legacy::compute_xopt(rseed + 1000000, dimension);
            }
            Function::Schwefel => {
                xopt = coco_legacy::compute_unif(rseed, dimension);
                for xi in &mut xopt {
                    *xi = if *xi >= 0.5 { 1.0 } else { -1.0 };
                }
            }
            _ => {}
        }

        match function {
            Function::Sphere
            | Function::Ellipsoid
            | Function::Rastrigin
            | Function::BuecheRastrigin
            | Function::LinearSlope
            | Function::Rosenbrock
            | Function::Schwefel => Params::Basic { fopt, xopt },

            Function::EllipsoidRotated
            | Function::Discus
            | Function::BentCigar
            | Function::DifferentPowers => {
                let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
                Params::Rotated { fopt, xopt, R }
            }

            Function::AttractiveSector
            | Function::StepEllipsoid
            | Function::SharpRidge
            | Function::RastriginRotated
            | Function::Weierstrass
            | Function::Schaffers1
            | Function::Schaffers2 => {
                let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
                let Q = coco_legacy::compute_rotation(rseed, dimension);
                Params::DoubleRotated { fopt, xopt, R, Q }
            }

            Function::RosenbrockRotated | Function::GriewankRosenbrock => {
                let Q = coco_legacy::compute_rotation(rseed, dimension);
                Params::FixedRotated { fopt, R: Q }
            }

            Function::Gallagher1 => {
                let R = coco_legacy::compute_rotation(rseed, dimension);

                let mut aperm = coco_legacy::compute_unif(rseed, 100)
                    .into_iter()
                    .enumerate()
                    .collect::<Vec<_>>();
                aperm.sort_unstable_by_key(|&(_, x)| OrderedFloat(x));
                let mut a = aperm
                    .into_iter()
                    .map(|(i, _)| (i + 1) as f64)
                    .map(|j| 1000f64.powf(2.0 * j / 99.0))
                    .collect::<Vec<_>>();
                a.insert(0, 1000.0);

                let mut y = coco_legacy::compute_unif(rseed, dimension * 101);
                for i in 0..dimension {
                    y[i] = y[i] * 8.0 - 4.0;
                }
                for i in dimension..(dimension * 101) {
                    y[i] = y[i] * 10.0 - 5.0;
                }

                Params::Gallagher {
                    fopt,
                    peaks: 101,
                    y,
                    a,
                    R,
                }
            }
            Function::Gallagher2 => todo!(),
            Function::Katsuura => todo!(),
            Function::LunacekBiRastrigin => todo!(),
        }
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
