pub mod real;
pub mod reference_type;

use num_rational::Ratio;
pub use real::Real;
pub use reference_type::SamObject;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SamValue {
    Real(Real),
    Reference(Uuid),
}

impl ToString for SamValue {
    fn to_string(&self) -> String {
        match self {
            Self::Real(x) => x.to_string(),
            Self::Reference(y) => y.to_string(),
        }
    }
}

impl Default for SamValue {
    fn default() -> Self {
        Self::int(0)
    }
}

impl SamValue {
    pub fn int(int: i64) -> Self {
        Self::Real(Real::Int(int))
    }

    pub fn float(float: f64) -> Self {
        Self::Real(Real::Float(float))
    }

    pub fn rational(ratio: Ratio<i64>) -> Self {
        Self::Real(Real::Rational(ratio))
    }
}
