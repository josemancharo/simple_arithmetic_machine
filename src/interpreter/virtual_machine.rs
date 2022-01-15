use crate::ast::operations::Operation;

pub struct SamVM {
    stacks: Vec<Vec<f64>>,
    current_stack: usize
}

impl SamVM {
    pub fn new() -> SamVM {
        return SamVM {
            stacks: vec![vec![]],
            current_stack: 0
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
            Operation::Start => {
                if self.current_stack != 0 {
                    self.stacks.push(vec![]);
                    self.current_stack += 1;
                }
            }
            Operation::End => {
                if self.current_stack != 0 {
                    let result = self.stacks[self.current_stack].pop().unwrap();
                    self.stacks.pop();
                    self.current_stack -= 1;
                    self.stacks[self.current_stack].push(result);
                }
            }
        }
    }
}

