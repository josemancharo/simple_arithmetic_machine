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

#[cfg(test)]
mod test {
    use crate::ast::operations::Operation;

    #[test]
    fn test_shunting_yard(){
        let output = super::parse_input("1 + 2 * 2").unwrap();
        println!("{:?}", &output);
        assert!(output == vec![Operation::Float(1.0), Operation::Float(2.0), Operation::Float(2.0), Operation::Mul, Operation::Add]);

        let output = super::parse_input("2 ** 3 * 4 + 8 % 3 + (8 * 8)").unwrap();
        println!("{:?}", &output);
        assert!(output == vec![
            Operation::Float(2.0), Operation::Float(3.0), Operation::Pow, 
            Operation::Float(4.0), Operation::Mul, 
            Operation::Float(8.0), Operation::Float(3.0), Operation::Mod, Operation::Add, 
            Operation::Float(8.0), Operation::Float(8.0), Operation::Mul, Operation::Add]);
    }
}