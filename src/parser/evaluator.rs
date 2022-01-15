use crate::ast::{block::AstBlock, operations::Operation};
use pest::{
    iterators::Pair,
    Parser,
};
use crate::util::hash_str::hash_str;

use super::grammar::{SamParser, SamRule};

pub fn eval(text: &str) -> AstBlock {
    let pairs = SamParser::parse(SamRule::Calculation, text).unwrap();
    let mut block = AstBlock {
        operations: vec![],
    };
    let mut add_next:Option<Operation> = None;
    for pair in pairs {
        match_expression(pair, &mut block, &mut add_next);
    }
    return block;
}

fn match_expression(pair: Pair<SamRule>, block: &mut AstBlock, add_next: &mut Option<Operation>) {
    let original_add_next = *add_next;
    match pair.as_rule() {
        SamRule::Expression => {
            block.operations.push(Operation::StartBlock);
            let inner = pair.into_inner();
            let mut add_next_internal: Option<Operation> = None;
            for pair in inner {
                match_expression(pair, block, &mut add_next_internal);
            }
            block.operations.push(Operation::EndBlock);
        }
        SamRule::Number => {
            let number = pair.as_str().parse::<f64>().unwrap();
            block.operations.push(Operation::Const(number));
        }
        SamRule::Add => {
            *add_next = Some(Operation::Add);
        }
        SamRule::Subtract => {
            *add_next = Some(Operation::Sub);
        }
        SamRule::Multiply => {
            *add_next = Some(Operation::Mul);
        }
        SamRule::Divide => {
            *add_next = Some(Operation::Div);
        }
        SamRule::Power => {
            *add_next = Some(Operation::Pow);
        }
        SamRule::Modulus => {
            *add_next = Some(Operation::Mod);
        }
        SamRule::Variable => {
            let key = hash_str(pair.as_str());
            block.operations.push(Operation::LoadVar(key));
        }
        SamRule::EOI => {}
        _ => {
            panic!("invalid rule! {:?}", pair.as_rule());
        }
    }
    
    if let Some(op) = add_next {
        if let Some(original) = original_add_next {
            if *op == original {
                block.operations.push(op.clone());
                *add_next = None; 
            }
        }
    }
    
}
