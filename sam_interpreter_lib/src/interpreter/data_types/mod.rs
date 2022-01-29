pub mod real;

use nalgebra::DMatrix;
pub use real::Real;

use num_rational::{Ratio};

#[derive(Debug,Clone)]
pub enum SamValue {
    Real(Real),
    Matrix(DMatrix<f64>)
}