use pest::iterators::{Pair, Pairs};

use crate::{
    ast::{ast_block::AstBlock, operations::Operation, user_functions::UserFunctionDefinition},
    errors::{SamError, ErrorWithMessage},
    util::hash_str::hash_str,
};

use super::grammar::SamRule;

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

    pub(crate) fn push_output(&mut self, op: Operation) {
        self.output.push(op);
    }

    pub(crate) fn pop_op(&mut self) -> Option<Operation> {
        self.operator_stack.pop()
    }

    pub(crate) fn push_op(&mut self, op: Operation) {
        self.operator_stack.push(op)
    }

    pub(crate) fn end_block(&mut self) {
        while let Some(op) = self.pop_op() {
            if op == Operation::StartBlock {
                break;
            }
            self.push_output(op)
        }
    }

    pub(crate) fn end_of_input(&mut self) {
        while let Some(op) = self.pop_op() {
            if op == Operation::StartBlock {
                continue;
            }
            self.push_output(op)
        }
    }

    pub(crate) fn declare_function(&mut self, pair: Pair<SamRule>) -> Result<(), SamError> {
        let mut inner = pair.into_inner();
        let mut func_def = UserFunctionDefinition::new();
        let name = inner.next().ok_or(ErrorWithMessage::default())?.as_str().trim();
        let params = inner.next().ok_or(ErrorWithMessage::default())?.into_inner();
        for param in params {
            func_def.parameters.push(hash_str(param.as_str()));
        }
        let mut body = inner.next().ok_or(ErrorWithMessage::default())?.into_inner();
        let mut engine = SamEvaluator::new();
        let output = engine.eval(&mut body)?;
        func_def.operations = output;
        self.push_op(Operation::StoreFunc(hash_str(name), func_def));
        Ok({})
    }

    pub(crate) fn output_superior_ops(&mut self, op1: &Operation) -> Result<(), SamError> {
        while let Some(op2) = self.operator_stack.last() {
            if op1 == &Operation::StartBlock || op1 > op2 {
                break;
            }
            let op = self.pop_op().ok_or(ErrorWithMessage::default())?;
            if op == Operation::StartBlock {
                break;
            }
            self.push_output(op);
        }
        Ok({})
    }

    pub(crate) fn match_inner_pairs(&mut self, pair: Pair<SamRule>) -> Result<(), SamError> {
        let mut inner = pair.into_inner();
        while let Some(pair) = inner.next() {
            self.match_pair(pair)?;
        }
        Ok({})
    }

    pub fn match_pairs_static(pairs: &mut Pairs<SamRule>) -> Result<Vec<Operation>, SamError> {
        let mut evaluator = SamEvaluator::new();
        evaluator.eval(pairs)?;
        return Ok(evaluator.output);
    }
}
