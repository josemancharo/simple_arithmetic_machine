use std::collections::HashMap;

use crate::ast::operations::Operation;

use super::constants::generate_constants;

pub struct SamVM {
    stacks: Vec<Vec<f64>>,
    current_stack: usize,
    previous_command: Option<Operation>,
    constants: HashMap<u64, f64>
}

impl SamVM {
    pub fn new() -> SamVM {
        return SamVM {
            stacks: vec![vec![]],
            current_stack: 0,
            previous_command: None,
            constants: generate_constants()
        }
    }
    pub fn interpret(&mut self, commands: Vec<Operation>) -> f64 {
        for command in commands {
            self.match_command(command);
        }
        return self.stacks[self.current_stack].pop().unwrap();
    }

    fn match_command(&mut self, command: Operation){
        match command {
            Operation::Const(x) => {
                self.stacks[self.current_stack].push(x);
            }
            Operation::Add => {
                let b = self.stacks[self.current_stack].pop().unwrap();
                let a = self.stacks[self.current_stack].pop().unwrap();
                self.stacks[self.current_stack].push(a + b);
            }
            Operation::Sub => {
                let b = self.stacks[self.current_stack].pop().unwrap();
                let a = self.stacks[self.current_stack].pop().unwrap();
                self.stacks[self.current_stack].push(a - b);
            }
            Operation::Mul => {
                let b = self.stacks[self.current_stack].pop().unwrap();
                let a = self.stacks[self.current_stack].pop().unwrap();
                self.stacks[self.current_stack].push(a * b);
            }
            Operation::Div => {
                let b = self.stacks[self.current_stack].pop().unwrap();
                let a = self.stacks[self.current_stack].pop().unwrap();
                self.stacks[self.current_stack].push(a / b);
            }
            Operation::Pow => {
                let b = self.stacks[self.current_stack].pop().unwrap();
                let a = self.stacks[self.current_stack].pop().unwrap();
                self.stacks[self.current_stack].push(a.powf(b));
            }
            Operation::Mod => {
                let b = self.stacks[self.current_stack].pop().unwrap();
                let a = self.stacks[self.current_stack].pop().unwrap();
                self.stacks[self.current_stack].push(a % b);
            }
            Operation::LoadVar(key) => {
                let val = self.constants.get(&key).unwrap();
                self.stacks[self.current_stack].push(*val);
            } 
            Operation::StartBlock => {
                if self.current_stack != 0 {
                    self.stacks.push(vec![]);
                    self.current_stack += 1;
                }
            }
            Operation::EndBlock => {
                if self.current_stack != 0 {
                    let result = self.stacks[self.current_stack].pop().unwrap();
                    self.stacks.pop();
                    self.current_stack -= 1;
                    self.stacks[self.current_stack].push(result);
                }
            }
        }
        self.previous_command = Some(command);
    }
}

