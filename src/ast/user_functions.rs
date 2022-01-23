use crate::ast::operations::Operation;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct UserFunctionDefinition {
    pub parameters: Vec<u64>,
    pub operations: Vec<Operation>
}

impl UserFunctionDefinition {
    pub fn new() -> UserFunctionDefinition {
        UserFunctionDefinition {
            parameters: vec![],
            operations: vec![],
        }
    }
}