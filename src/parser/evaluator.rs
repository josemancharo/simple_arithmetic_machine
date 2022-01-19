use super::grammar::{SamParser, SamRule};
use crate::ast::user_functions::UserFunctionDefinition;
use crate::ast::{block::AstBlock, operations::Operation};
use crate::util::hash_str::hash_str;
use pest::iterators::Pairs;
use pest::{iterators::Pair, Parser};

pub fn eval(text: &str) -> AstBlock {
    let mut pairs = SamParser::parse(SamRule::Calculation, text).unwrap();
    let mut block = AstBlock { operations: vec![] };
    let mut add_next: Option<Operation> = None;
    let mut next_operation: Option<Operation> = None;
    while let Some(pair) = pairs.next() {
        match_expression(pair, &mut block, &mut add_next, &mut pairs);
    }
    return block;
}

fn match_expression(
    pair: Pair<SamRule>,
    block: &mut AstBlock,
    add_next: &mut Option<Operation>,
    pairs: &mut Pairs<SamRule>,
) {
    let original_add_next = add_next.clone();
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
        SamRule::Float => {
            let number = pair.as_str().parse::<f64>().unwrap();
            block.operations.push(Operation::Const(number));
        }
        SamRule::Add => *add_next = Some(Operation::Add),
        SamRule::Subtract => *add_next = Some(Operation::Sub),
        SamRule::Multiply => *add_next = Some(Operation::Mul),
        SamRule::Divide => *add_next = Some(Operation::Div),
        SamRule::Power => *add_next = Some(Operation::Pow),
        SamRule::Modulus => *add_next = Some(Operation::Mod),
        SamRule::Gt => *add_next = Some(Operation::Gt),
        SamRule::Lt => *add_next = Some(Operation::Lt),
        SamRule::Gte => *add_next = Some(Operation::Gte),
        SamRule::Lte => *add_next = Some(Operation::Lte),
        SamRule::Eq => *add_next = Some(Operation::Eq),
        SamRule::Neq => *add_next = Some(Operation::Neq),
        SamRule::Xor => *add_next = Some(Operation::BitXor),
        SamRule::And => *add_next = Some(Operation::BitAnd),
        SamRule::Or => *add_next = Some(Operation::BitOr),
        SamRule::RightShift => *add_next = Some(Operation::RightShift),
        SamRule::LeftShift => *add_next = Some(Operation::LeftShift),
        SamRule::Not => block.operations.push(Operation::Not),
        SamRule::Neg => block.operations.push(Operation::Neg),

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

            block
                .operations
                .push(Operation::CallFunc(hash_str(&func_name)))
        }
        SamRule::Pipe => {
            let func_name = pairs.next().unwrap().as_str().trim();
            block
                .operations
                .push(Operation::CallFunc(hash_str(&func_name)));
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
        SamRule::FunctionDeclaration => {
            let mut inner = pair.into_inner();
            let mut func_def = UserFunctionDefinition { parameters: vec![], operations: vec![] };
            let name = inner.next().unwrap().as_str().trim();
            let params = inner.next().unwrap().into_inner();
            for param in params {
                func_def.parameters.push(hash_str(param.as_str()));
            }
            let mut func_block = AstBlock{ operations: vec![] };
            let mut inner_add_next: Option<Operation> = None;
            let mut body = inner.next().unwrap().into_inner();
            while let Some(pair) = body.next() {
                match_expression(pair, &mut func_block, &mut inner_add_next, &mut body);
            }
            func_def.operations = func_block.operations;
            block.operations.push(Operation::StoreFunc(hash_str(name), func_def));
        }
        SamRule::UnaryOperation => {

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
