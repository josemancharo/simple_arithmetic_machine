use crate::ast::{block::AstBlock, operations::Operation};
use pest::iterators::Pairs;
use pest::{
    iterators::Pair,
    Parser,
};
use crate::util::hash_str::hash_str;
use super::grammar::{SamParser, SamRule};

pub fn eval(text: &str) -> AstBlock {
    let mut pairs = SamParser::parse(SamRule::Calculation, text).unwrap();
    let mut block = AstBlock {
        operations: vec![],
    };
    let mut add_next:Option<Operation> = None;
    while let Some(pair) = pairs.next() {
        match_expression(pair, &mut block, &mut add_next, &mut pairs);
    }
    return block;
}

fn match_expression(pair: Pair<SamRule>, block: &mut AstBlock, add_next: &mut Option<Operation>, pairs: &mut Pairs<SamRule>) {
    let original_add_next = *add_next;
    match pair.as_rule() {
        SamRule::Expression => {
            block.operations.push(Operation::StartBlock);
            let mut inner = pair.into_inner();
            let mut add_next_internal: Option<Operation> = None;
            while let Some(pair) = inner.next() {
                match_expression(pair, block, &mut add_next_internal, &mut inner);
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
            let key = hash_str(pair.as_str().trim());
            block.operations.push(Operation::LoadVar(key));
        }
        SamRule::FunctionInvocation => {
            let mut inner = pair.into_inner();
            let func_name = inner.next().unwrap().as_str().trim();
            let mut inner_add_next: Option<Operation> = None;
            while let Some(pair) = inner.next() {
                match_expression(pair, block, &mut inner_add_next, &mut inner)
            }

            block.operations.push(Operation::CallFunc(hash_str(&func_name)))
        }
        SamRule::Pipe => {
            let func_name = pairs.next().unwrap().as_str().trim();
            block.operations.push(Operation::CallFunc(hash_str(&func_name)));
        }
        SamRule::Assignment => {
            let mut inner = pair.into_inner();
            let mut inner_add_next: Option<Operation> = None;
            let name = inner.next().unwrap().as_str().trim();
            while let Some(pair) = inner.next() {
                match_expression(pair, block, &mut inner_add_next, &mut inner)
            }
            block.operations.push(Operation::StoreVar(hash_str(name)));
        }
        _ => {}
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
