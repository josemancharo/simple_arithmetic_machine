use nalgebra::DMatrix;

use crate::{ast::{user_functions::UserFunctionDefinition, operations::Operation}, SamError};

use super::Real;

#[derive(Debug,Clone,PartialEq)]
pub enum SamObject {
    Matrix(DMatrix<f64>),
    Function(UserFunctionDefinition),
}

impl SamObject {
    pub fn call_method(&mut self, method: &Operation, params: &Vec<Real>) -> Result<Real, SamError> {
        match self {
            Self::Matrix(m) => Self::call_matrix_method(method, params, m),
            Self::Function(f) => Self::call_function_method(method, params, f)
        }
    }

    fn call_matrix_method(method: &Operation, params: &Vec<Real>, matrix: &mut DMatrix<f64>) -> Result<Real, SamError> {
        Ok(Real::default())
    }

    fn call_function_method(method: &Operation, params: &Vec<Real>, func: &UserFunctionDefinition) -> Result<Real, SamError> {
        Ok(Real::default())
    }
}