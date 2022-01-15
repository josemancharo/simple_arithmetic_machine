use super::operations::Operation;

#[derive(Debug)]
pub struct AstBlock {
    pub stack: Vec<Operation>,
}