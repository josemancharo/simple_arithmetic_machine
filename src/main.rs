use parser::evaluator::eval;

mod parser;
mod ast;
mod interpreter;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use dialoguer::Input;

use crate::interpreter::virtual_machine::SamVM;

fn main() {

    loop {
        let mut vm = SamVM::new();
        let equation: String = Input::new()
            .with_prompt("math")
            .interact_text()
            .unwrap();

        if equation.as_str() == "exit" {
            break;
        }

        let output = eval(equation.as_str());
        println!("{:?}", output.stack);
        println!("{:?}", vm.interpret(output.stack));
    }


    
}
