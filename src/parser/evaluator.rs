use crate::ast::{block::AstBlock, operations::Operation};
use pest::{
    iterators::{Pair},
    Parser,
};

use super::grammar::{SamParser, SamRule};

pub fn eval(text: &str) -> AstBlock {
    let pairs = SamParser::parse(SamRule::Calculation, text).unwrap();
    let mut block = AstBlock {
        stack: vec![],
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
            block.stack.push(Operation::Start);
            let inner = pair.into_inner();
            let mut add_next_internal: Option<Operation> = None;
            for p in inner {
                match_expression(p, block, &mut add_next_internal);
            }
            block.stack.push(Operation::End);
        }
        SamRule::Add => {
            *add_next = Some(Operation::Add);
        }
        SamRule::Number => {
            let number = pair.as_str().parse::<f64>().unwrap();
            block.stack.push(Operation::Const(number));
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
        SamRule::EOI => {}
        _ => {
            panic!("invalid rule! {:?}", pair.as_rule());
        }
    }
    
    if let Some(op) = add_next {
        if let Some(original) = original_add_next {
            if *op == original {
                block.stack.push(*op);
                *add_next = None; 
            }
        }
    }
    
}
