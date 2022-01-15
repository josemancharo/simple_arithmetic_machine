use super::operations::Operation;

#[derive(Debug)]
pub struct AstBlock {
    pub operations: Vec<Operation>,
}