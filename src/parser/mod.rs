mod grammar;
mod match_diad_op;
mod evaluator;

use pest::Parser;

use crate::ast::ast_block::AstBlock;

pub fn parse_input(input: &str) -> Result<AstBlock, crate::errors::SamError> {
    let mut evaluator = evaluator::SamEvaluator::new();
    let mut pairs = grammar::SamParser::parse(grammar::SamRule::Calculation, input)?;
    let output = evaluator.eval(&mut pairs)?;
    return Ok(output);
}