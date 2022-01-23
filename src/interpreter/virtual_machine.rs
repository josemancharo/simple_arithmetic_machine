use std::collections::HashMap;

use crate::{
    algorithms::logarithms::pow,
    ast::{operations::Operation, user_functions::UserFunctionDefinition},
    errors::{SamError, ErrorWithMessage},
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
        return self.pop_stack();
    }

    fn match_command(&mut self, command: Operation) -> Result<(), SamError> {
        match command {
            Operation::Const(x) => {
                if let Ok(x) = x.to_string().parse::<i64>() {
                    self.push_stack(Real::Int(x));
                } else {
                    self.push_stack(Real::Float(x))
                }
            }
            Operation::Add => self.diadic_op(|a, b| a + b),
            Operation::Sub => self.diadic_op(|a, b| a - b),
            Operation::Mul => self.diadic_op(|a, b| a * b),
            Operation::Div => self.diadic_op(|a, b| a / b),
            Operation::Pow => self.diadic_op(|a, b| pow(a, b)),
            Operation::Mod => self.diadic_op(|a, b| a % b),
            Operation::Gt => self.diadic_op(|a, b| Real::Int((a > b) as i64)),
            Operation::Lt => self.diadic_op(|a, b| Real::Int((a < b) as i64)),
            Operation::Lte => self.diadic_op(|a, b| Real::Int((a <= b) as i64)),
            Operation::Gte => self.diadic_op(|a, b| Real::Int((a >= b) as i64)),
            Operation::Eq => self.diadic_op(|a, b| Real::Int((a == b) as i64)),
            Operation::Neq => self.diadic_op(|a, b| Real::Int((a != b) as i64)),
            Operation::BitAnd => self.diadic_op(|a, b| a & b),
            Operation::BitOr => self.diadic_op(|a, b| a | b),
            Operation::BitXor => self.diadic_op(|a, b| a ^ b),
            Operation::RightShift => self.diadic_op(|a, b| a >> b),
            Operation::LeftShift => self.diadic_op(|a, b| a << b),
            Operation::Neg => self.monadic_op(|x| -x)?,
            Operation::BitCompliment => self.monadic_op(|x| !x)?,

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
                            let (b, a) = self.pop_two();
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
                        while let Some(op) = ops.next(){
                            let op = op.clone();
                            self.match_command(op)?;
                        }
                        self.current_scope -= 1;
                        self.user_vars.pop();
                    }
                }
            }
            Operation::StoreFunc(key, func) => {
                self.user_functions.insert(key, func);
                self.push_stack(Real::Int(0));
            }
            _ => { return Err(ErrorWithMessage::new_box("stack empty!")) }
        }
        Ok({})
    }

    fn set_var(&mut self, key: u64, value: Real) {
        self.user_vars[self.current_scope].insert(key, value);
    }

    fn pop_stack(&mut self) -> Result<Real, SamError> {
        return self.stacks[self.current_stack].pop()
            .ok_or(ErrorWithMessage::new_box("stack empty!"));
    } 

    fn pop_two(&mut self) -> (Real, Real) {
        let b = self.stacks[self.current_stack].pop().unwrap();
        let a = self.stacks[self.current_stack].pop().unwrap();
        return (b, a);
    }

    fn diadic_op(&mut self, op: fn(Real, Real) -> Real) {
        let (b, a) = self.pop_two();
        self.push_stack(op(a, b));
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
            while scope != 0 {
                let local = self.user_vars[scope].get(&key);
                if let Some(val) = local {
                    return val.clone();
                }
                scope -= 1;
            }
            return Real::Int(0);
        }
    }
}
