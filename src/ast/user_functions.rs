use crate::ast::operations::Operation;

#[derive(Clone,Debug,PartialEq)]
pub struct UserFunctionDefinition {
    pub parameters: Vec<u64>,
    pub operations: Vec<Operation>
}