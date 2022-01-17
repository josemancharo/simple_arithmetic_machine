use std::collections::HashMap;

use crate::{ast::operations::Operation, algorithms::logarithms::pow};

use super::{constants::generate_constants, user_functions::UserFunctionDefinition, builtin_functions::{Func, setup_builtins}, data_types::Real};

pub struct SamVM {
    stacks: Vec<Vec<Real>>,
    current_stack: usize,
    previous_command: Option<Operation>,
    constants: HashMap<u64, Real>,
    user_vars: HashMap<u64, Real>,
    builtin_functions: HashMap<u64, Func>,
    user_functions: Vec<UserFunctionDefinition>,
}

impl SamVM {
    pub fn new() -> SamVM {
        return SamVM {
            stacks: vec![vec![]],
            current_stack: 0,
            previous_command: None,
            constants: generate_constants(),
            user_functions: vec![],
            user_vars: HashMap::new(),
            builtin_functions: setup_builtins()
        }
    }

    pub fn interpret(&mut self, commands: Vec<Operation>) -> Real {
        for command in commands {
            self.match_command(command);
        }
        return self.pop_stack();
    }

    fn match_command(&mut self, command: Operation){
        match command {
            Operation::Const(x) => {
                if let Ok(x) = x.to_string().parse::<i64>() {
                    self.push_stack(Real::Int(x));
                }
                else {
                    self.push_stack(Real::Float(x))
                }
            }
            Operation::Add => {
                self.diadic_op(|a, b| { a + b })
            }
            Operation::Sub => {
                self.diadic_op(|a, b| { a - b })
            }
            Operation::Mul => {
                self.diadic_op(|a, b| { a * b })
            }
            Operation::Div => {
                self.diadic_op(|a, b| { a / b })
            }
            Operation::Pow => {
                self.diadic_op(|a, b| { pow(a, b) })
            }
            Operation::Mod => {
                self.diadic_op(|a, b| { a % b })
            }
            Operation::LoadVar(key) => {
                let val = self.get_var(key);
                self.push_stack(val);
            } 
            Operation::StartBlock => {
                if self.current_stack != 0 {
                    self.stacks.push(vec![]);
                    self.current_stack += 1;
                }
            }
            Operation::EndBlock => {
                if self.current_stack != 0 {
                    let result = self.pop_stack();
                    self.stacks.pop();
                    self.current_stack -= 1;
                    self.push_stack(result);
                }
            }
            Operation::StoreVar(x) => {
                let value = self.pop_stack();
                self.user_vars.insert(x, value);
                self.push_stack(value);
                println!("{:?}", self.user_vars)
            }
            Operation::CallFunc(key) => {
                let func = self.builtin_functions.get(&key).unwrap();
                match func {
                    &Func::Monad(f) => {
                        let x = self.pop_stack();
                        self.push_stack(f(x));
                    }
                    &Func::Diad(f) => {
                        let (b, a) = self.pop_two();
                        self.push_stack(f(a, b))
                    }
                }
            }
        }
        self.previous_command = Some(command);
    }

    fn pop_stack(&mut self) -> Real {
        return self.stacks[self.current_stack].pop().unwrap();
    }

    fn pop_two(&mut self) -> (Real, Real) {
        let b = self.stacks[self.current_stack].pop().unwrap();
        let a = self.stacks[self.current_stack].pop().unwrap();
        return (b, a);
    }

    fn diadic_op(&mut self, op: fn(Real, Real) -> Real){
        let (b, a) = self.pop_two();
        self.push_stack(op(a, b));
    }

    fn push_stack(&mut self, val: Real){
        self.stacks[self.current_stack].push(val);
    }

    fn get_var(&mut self, key: u64) -> Real {
         let global = self.constants.get(&key);
         if let Some(val) = global {
             return val.clone();
         }
         else {
             let local = self.user_vars.get(&key);
             if let Some(val) = local {
                 return val.clone();
             }
             else {
                 return Real::Int(0);
             }
         }
    }

}

