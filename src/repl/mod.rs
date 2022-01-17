use crate::{parser::evaluator::eval, errors::SamError};
use crate::interpreter::virtual_machine::SamVM;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use nu_ansi_term::Color::{ Red, Green };

pub fn run_repl() -> Result<(), SamError> {
    let mut history: Vec<String> = vec![];
    let mut vm = SamVM::new();
    let mut rl = Editor::<()>::new();
    nu_ansi_term::enable_ansi_support().expect("No ANSI support");
    let current_exe = std::env::current_exe()?;
    let exe_path = current_exe.parent().unwrap();
    let history_path = exe_path.join("repl_history.txt");
    match rl.load_history(&history_path.as_os_str()) {
        _ => {}
    };
    
    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                if line.trim() == "" {
                    continue;
                }
                rl.add_history_entry(line.as_str());

                let output = eval(line.as_str());
                println!("{}", Green.paint(vm.interpret(output.operations)?.to_string()));
                history.push(line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("{}", Red.paint("CTRL-C"));
                break
            },
            Err(ReadlineError::Eof) => {
                println!("{}", Red.paint("CTRL-D"));
                break
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break
            }
        }
    }

    rl.save_history(&history_path.as_os_str())?;
    Ok({})
}
