use pest::iterators::{Pair, Pairs};

use crate::{
    ast::{ast_block::AstBlock, operations::Operation, user_functions::UserFunctionDefinition},
    errors::SamError,
    util::hash_str::hash_str,
};

use super::{grammar::SamRule, match_diad_op::match_diad_op};

pub struct SamEvaluator {
    output: AstBlock,
    operator_stack: AstBlock,
}

impl SamEvaluator {
    pub fn new() -> SamEvaluator {
        SamEvaluator {
            output: vec![],
            operator_stack: vec![],
        }
    }

    pub fn eval(&mut self, pairs: &mut Pairs<SamRule>) -> Result<AstBlock, SamError> {
        while let Some(pair) = pairs.next() {
            self.match_pair(pair)?;
        }
        self.end_of_input();
        return Ok(self.output.clone());
    }

    fn push_output(&mut self, op: Operation) {
        self.output.push(op)
    }

    fn pop_op(&mut self) -> Option<Operation> {
        self.operator_stack.pop()
    }

    fn push_op(&mut self, op: Operation) {
        self.operator_stack.push(op)
    }

    fn end_block(&mut self) {
        while let Some(op) = self.pop_op() {
            if op == Operation::StartBlock {
                break;
            }
            self.push_output(op)
        }
    }

    fn end_of_input(&mut self){
        while let Some(op) = self.pop_op() {
            self.push_output(op)
        }
    }

    fn declare_function(&mut self, pair: Pair<SamRule>) -> Result<(), SamError> {
        let mut inner = pair.into_inner();
        let mut func_def = UserFunctionDefinition::new();
        let name = inner.next().unwrap().as_str().trim();
        let params = inner.next().unwrap().into_inner();
        for param in params {
            func_def.parameters.push(hash_str(param.as_str()));
        }
        let mut body = inner.next().unwrap().into_inner();
        let mut engine = SamEvaluator::new();
        let output = engine.eval(&mut body)?;
        func_def.operations = output;
        self.push_op(Operation::StoreFunc(hash_str(name), func_def));
        Ok({})
    }

    fn output_superior_ops(&mut self, op1: &Operation) {
        while let Some(op2) = self.operator_stack.last() {
            if op1 == &Operation::StartBlock || op1 > op2 {
                break;
            }
            let op = self.pop_op().unwrap();
            self.push_output(op);
        }
    }

    fn match_inner_pairs(&mut self, pair: Pair<SamRule>) -> Result<(), SamError> {
        let mut inner = pair.into_inner();
        while let Some(pair) = inner.next() {
            self.match_pair(pair)?;
        }
        Ok({})
    }

    fn match_pair(&mut self, pair: Pair<SamRule>) -> Result<(), SamError> {
        match pair.as_rule() {
            SamRule::Expression => {
                self.push_op(Operation::StartBlock);
                self.match_inner_pairs(pair)?;
                self.end_block();
            }
            SamRule::Float => {
                let x = pair.as_str().parse::<f64>()?;
                self.push_output(Operation::Float(x));
            }
            SamRule::Integer => {
                let x = pair.as_str().parse::<i64>()?;
                self.push_output(Operation::Int(x));
            }
            SamRule::Hexadecimal => {
                let x = i64::from_str_radix(pair.as_str().trim_start_matches("0x"), 16)?;
                self.push_output(Operation::Int(x));
            }
            SamRule::Octal => {
                let x = i64::from_str_radix(pair.as_str().trim_start_matches("0o"), 8)?;
                self.push_output(Operation::Int(x));
            }
            SamRule::Binary => {
                let x = i64::from_str_radix(pair.as_str().trim_start_matches("0b"), 2)?;
                self.push_output(Operation::Int(x));
            }
            SamRule::Variable => {
                let key = hash_str(pair.as_str().trim());
                self.push_output(Operation::LoadVar(key));
            }
            SamRule::Assignment => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().trim();
                while let Some(pair) = inner.next() {
                    self.match_pair(pair)?;
                }
                self.push_op(Operation::StoreVar(hash_str(name)));
            }

            SamRule::FunctionInvocation => {
                let mut inner = pair.into_inner();
                let key = hash_str(inner.next().unwrap().as_str());
                while let Some(pair) = inner.next() {
                    self.match_pair(pair)?;
                }
                self.push_op(Operation::CallFunc(key));
            }
            SamRule::FunctionDeclaration => {
                self.declare_function(pair)?;
            }
            SamRule::Not => self.push_op(Operation::Not),
            SamRule::Neg => self.push_op(Operation::Neg),
            SamRule::BitCompliment => self.push_op(Operation::BitCompliment),
            _ => {
                if let Some(op) = match_diad_op(pair.as_rule()) {
                    self.output_superior_ops(&op);
                    self.push_op(op);
                }
            }
        }
        Ok({})
    }
}
