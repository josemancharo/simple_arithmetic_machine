use std::collections::HashMap;

use num_rational::Rational64;

use crate::{SamVM, ast::operations::Operation, errors::ErrorWithMessage, SamError};

use super::{data_types::{Real, SamValue}, builtin_functions::Func};

impl SamVM {
    pub fn interpret(&mut self, commands: Vec<Operation>) -> Result<SamValue, SamError> {
        for command in commands {
            self.match_command(command)?;
        }
        let last_value = self.stacks[self.current_stack]
            .last()
            .ok_or(ErrorWithMessage::new_box("stack empty!"))?;
        return Ok(last_value.clone());
    }

    fn match_command(&mut self, command: Operation) -> Result<(), SamError> {
        match command {
            Operation::Float(x) => self.push_stack(SamValue::float(x)),
            Operation::Int(x) => self.push_stack(SamValue::int(x)),
            Operation::Add => self.diadic_op(|a, b| a + b)?,
            Operation::Sub => self.diadic_op(|a, b| a - b)?,
            Operation::Mul => self.diadic_op(|a, b| a * b)?,
            Operation::Div => self.diadic_op(|a, b| a / b)?,
            Operation::Pow => self.diadic_op(|a, b| a.pow(b))?,
            Operation::Mod => self.diadic_op(|a, b| a % b)?,
            Operation::Gt => self.diadic_op(|a, b| Real::Int((a > b) as i64))?,
            Operation::Lt => self.diadic_op(|a, b| Real::Int((a < b) as i64))?,
            Operation::Lte => self.diadic_op(|a, b| Real::Int((a <= b) as i64))?,
            Operation::Gte => self.diadic_op(|a, b| Real::Int((a >= b) as i64))?,
            Operation::Eq => self.diadic_op(|a, b| Real::Int((a == b) as i64))?,
            Operation::Neq => self.diadic_op(|a, b| Real::Int((a != b) as i64))?,
            Operation::BitAnd => self.diadic_op(|a, b| a & b)?,
            Operation::Ratio => self.diadic_op(|a, b| { 
                match (a, b) {
                    (Real::Int(a), Real::Int(b)) => Real::Rational(Rational64::new(a, b)),
                    _ => a / b
                }
            })?,
            Operation::BoolAnd => self.diadic_op(|a, b| {
                if a != Real::Int(0) && b != Real::Int(0) {
                    Real::Int(1)
                } else {
                    Real::Int(0)
                }
            })?,
            Operation::BoolOr => self.diadic_op(|a, b| {
                if a != Real::Int(0) || b != Real::Int(0) {
                    Real::Int(1)
                } else {
                    Real::Int(0)
                }
            })?,
            Operation::BitOr => self.diadic_op(|a, b| a | b)?,
            Operation::BitXor => self.diadic_op(|a, b| a ^ b)?,
            Operation::RightShift => self.diadic_op(|a, b| a >> b)?,
            Operation::LeftShift => self.diadic_op(|a, b| a << b)?,
            Operation::Neg => self.monadic_op(|x| -x)?,
            Operation::BitCompliment => self.monadic_op(|x| !x)?,
            Operation::Not => self.monadic_op(|x| if x == Real::Int(0) { Real::Int(1) } else { Real::Int(0) })?,
            Operation::PeekStack => {
                let value = self.pop_stack()?;
                self.push_stack(value);
            }
            Operation::LoadVar(key) => {
                let val = self.get_var(key);
                self.push_stack(val);
            }
            Operation::StoreVar(key) => {
                let value = self.pop_stack()?;
                self.set_var(key, value);
                self.push_stack(value);
            }
            Operation::CallFunc(key) => {
                if let Some(func) = self.builtin_functions.get(&key) {
                    match func {
                        &Func::Monad(f) => {
                            let x = self.pop_stack()?;
                            if let SamValue::Real(x) = x {
                                self.push_stack(SamValue::Real(f(x)));
                            }
                        }
                        &Func::Diad(f) => {
                            let (b, a) = self.pop_two()?;
                            if let (SamValue::Real(b), SamValue::Real(a)) = (b, a) {
                                self.push_stack(SamValue::Real(f(a, b)))
                            }
                        }
                    }
                } else {
                    if let Some(func) = self.user_functions.get(&key) {
                        let func = func.clone();
                        self.user_vars.push(HashMap::new());
                        self.current_scope += 1;
                        let params = &func.parameters.clone();
                        for param in params {
                            let val = self.pop_stack()?;
                            self.set_var(param.clone(), val);
                        }

                        let mut ops = func.operations.clone().into_iter();
                        while let Some(op) = ops.next() {
                            let op = op.clone();
                            self.match_command(op)?;
                        }
                        self.current_scope -= 1;
                        self.user_vars.pop();
                    }
                }
            }
            Operation::Conditional => {
                let (c, b, a) = self.pop_three()?;
                if a != SamValue::float(0.0) && c != SamValue::int(0) {
                    self.push_stack(b);
                } else {
                    self.push_stack(c);
                }
            }
            Operation::StoreFunc(key, func) => {
                self.user_functions.insert(key, func);
                self.push_stack(SamValue::default());
            }
            _ => return Err(ErrorWithMessage::new_box("stack empty!")),
        }
        Ok({})
    }
}