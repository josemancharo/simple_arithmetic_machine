use crate::ast::operations::Operation;

pub struct UserFunctionDefinition {
    parameters: Vec<u64>,
    operations: Vec<Operation>
}