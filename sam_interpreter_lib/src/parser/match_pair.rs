use pest::iterators::Pair;

use crate::{ast::operations::Operation, util::hash_str::hash_str, SamError};

use super::{evaluator::SamEvaluator, grammar::SamRule, match_diad_op::match_diad_op};

impl SamEvaluator {
    pub(crate) fn match_pair(&mut self, pair: Pair<SamRule>) -> Result<(), SamError> {
        match pair.as_rule() {
            SamRule::Expression => {
                self.push_op(Operation::StartBlock);
                self.match_inner_pairs(pair)?;
                self.end_block();
            }
            SamRule::Float | SamRule::Integer => {
                if let Ok(x) = pair.as_str().replace("_", "").parse::<i64>(){
                    self.push_output(Operation::Int(x));
                }
                else {
                    let x = pair.as_str().parse::<f64>()?;
                    self.push_output(Operation::Float(x));
                }
            }
            SamRule::Hexadecimal => {
                let x = i64::from_str_radix(
                    pair.as_str()
                        .trim_start_matches("0x")
                        .replace("_", "")
                        .as_str(),
                    16,
                )?;
                self.push_output(Operation::Int(x));
            }
            SamRule::Octal => {
                let x = i64::from_str_radix(
                    pair.as_str()
                        .trim_start_matches("0o")
                        .replace("_", "")
                        .as_str(),
                    8,
                )?;
                self.push_output(Operation::Int(x));
            }
            SamRule::Binary => {
                let x = i64::from_str_radix(
                    pair.as_str()
                        .trim_start_matches("0b")
                        .replace("_", "")
                        .as_str(),
                    2,
                )?;
                self.push_output(Operation::Int(x));
            }
            SamRule::PeekStack => self.push_output(Operation::PeekStack),
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
            SamRule::ConditionalOperator => {
                self.end_of_input();
                self.match_inner_pairs(pair)?;
                self.push_op(Operation::Conditional)
            },
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