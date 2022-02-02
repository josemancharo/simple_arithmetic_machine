mod grammar;
mod match_diad_op;
mod evaluator;
mod match_pair;

use pest::Parser;

use crate::ast::ast_block::AstBlock;

pub fn parse_input(input: &str) -> Result<AstBlock, crate::errors::SamError> {
    let mut evaluator = evaluator::SamEvaluator::new();
    let mut pairs = grammar::SamParser::parse(grammar::SamRule::Calculation, input)?;
    let output = evaluator.eval(&mut pairs)?;
    return Ok(output);
}

#[cfg(test)]
mod test {
    use crate::ast::operations::Operation;

    #[test]
    fn test_shunting_yard(){
        let output = super::parse_input("1 + 2 * 2").unwrap();
        println!("{:?}", &output);
        assert!(output == vec![Operation::Int(1), Operation::Int(2), Operation::Int(2), Operation::Mul, Operation::Add]);

        let output = super::parse_input("2 ** 3 * 4 + 8 % 3 + (8 * 8)").unwrap();
        println!("{:?}", &output);
        assert!(output == vec![
            Operation::Int(2), Operation::Int(3), Operation::Pow, 
            Operation::Int(4), Operation::Mul, 
            Operation::Int(8), Operation::Int(3), Operation::Mod, Operation::Add, 
            Operation::Int(8), Operation::Int(8), Operation::Mul, Operation::Add]);
    }
}