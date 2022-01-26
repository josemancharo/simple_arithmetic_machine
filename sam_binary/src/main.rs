
use sam_library::errors::SamError;
use repl::run_repl;

mod cli;
mod repl;


fn main() -> Result<(), SamError> {
   run_repl()?; 
   Ok({})
}

