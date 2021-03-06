use std::collections::HashMap;

use crate::{
    algorithms::logarithms::pow,
    ast::{operations::Operation, user_functions::UserFunctionDefinition},
    errors::{ErrorWithMessage, SamError},
};

use super::{
    builtin_functions::{setup_builtins, Func},
    constants::generate_constants,
    data_types::Real,
};

pub struct SamVM {
    stacks: Vec<Vec<Real>>,
    current_stack: usize,
    current_scope: usize,
    constants: HashMap<u64, Real>,
    user_vars: Vec<HashMap<u64, Real>>,
    builtin_functions: HashMap<u64, Func>,
    user_functions: HashMap<u64, UserFunctionDefinition>,
}

impl SamVM {
    pub fn new() -> SamVM {
        return SamVM {
            stacks: vec![vec![]],
            current_stack: 0,
            current_scope: 0,
            constants: generate_constants(),
            user_functions: HashMap::new(),
            user_vars: vec![HashMap::new()],
            builtin_functions: setup_builtins(),
        };
    }

    pub fn interpret(&mut self, commands: Vec<Operation>) -> Result<Real, SamError> {
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
            Operation::Float(x) => self.push_stack(Real::Float(x)),
            Operation::Int(x) => self.push_stack(Real::Int(x)),
            Operation::Add => self.diadic_op(|a, b| a + b)?,
            Operation::Sub => self.diadic_op(|a, b| a - b)?,
            Operation::Mul => self.diadic_op(|a, b| a * b)?,
            Operation::Div => self.diadic_op(|a, b| a / b)?,
            Operation::Pow => self.diadic_op(|a, b| pow(a, b))?,
            Operation::Mod => self.diadic_op(|a, b| a % b)?,
            Operation::Gt => self.diadic_op(|a, b| Real::Int((a > b) as i64))?,
            Operation::Lt => self.diadic_op(|a, b| Real::Int((a < b) as i64))?,
            Operation::Lte => self.diadic_op(|a, b| Real::Int((a <= b) as i64))?,
            Operation::Gte => self.diadic_op(|a, b| Real::Int((a >= b) as i64))?,
            Operation::Eq => self.diadic_op(|a, b| Real::Int((a == b) as i64))?,
            Operation::Neq => self.diadic_op(|a, b| Real::Int((a != b) as i64))?,
            Operation::BitAnd => self.diadic_op(|a, b| a & b)?,
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
                            self.push_stack(f(x));
                        }
                        &Func::Diad(f) => {
                            let (b, a) = self.pop_two()?;
                            self.push_stack(f(a, b))
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
                if a != Real::Float(0.0) && c != Real::Int(0) {
                    self.push_stack(b);
                } else {
                    self.push_stack(c);
                }
            }
            Operation::StoreFunc(key, func) => {
                self.user_functions.insert(key, func);
                self.push_stack(Real::Int(0));
            }
            _ => return Err(ErrorWithMessage::new_box("stack empty!")),
        }
        Ok({})
    }

    fn set_var(&mut self, key: u64, value: Real) {
        self.user_vars[self.current_scope].insert(key, value);
    }

    fn pop_stack(&mut self) -> Result<Real, SamError> {
        return self.stacks[self.current_stack]
            .pop()
            .ok_or(ErrorWithMessage::new_box("stack empty!"));
    }

    fn pop_two(&mut self) -> Result<(Real, Real), SamError> {
        let b = self.pop_stack()?;
        let a = self.pop_stack()?;
        return Ok((b, a));
    }

    fn pop_three(&mut self) -> Result<(Real, Real, Real), SamError> {
        let (c, b) = self.pop_two()?;
        let a = self.pop_stack()?;
        return Ok((c, b, a));
    }

    fn diadic_op(&mut self, op: fn(Real, Real) -> Real) -> Result<(), SamError> {
        let (b, a) = self.pop_two()?;
        self.push_stack(op(a, b));
        Ok({})
    }

    fn monadic_op(&mut self, op: fn(Real) -> Real) -> Result<(), SamError> {
        let a = self.pop_stack()?;
        self.push_stack(op(a));
        Ok({})
    }

    fn push_stack(&mut self, val: Real) {
        self.stacks[self.current_stack].push(val);
    }

    fn get_var(&mut self, key: u64) -> Real {
        let global = self.constants.get(&key);
        if let Some(val) = global {
            return val.clone();
        } else {
            let mut scope = self.current_scope;
            loop {
                let local = self.user_vars[scope].get(&key);
                if let Some(val) = local {
                    return val.clone();
                }
                if scope != 0 {
                    scope -= 1;
                }
                if scope == 0 {
                    break;
                }
            }
            return Real::Int(0);
        }
    }
}
