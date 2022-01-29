
use clap::StructOpt;
use cli::SamArgs;
use sam_library::errors::SamError;
use repl::run_repl;

mod cli;
mod repl;


fn main() -> Result<(), SamError> {
   let args = SamArgs::parse();
   run_repl()?; 
   Ok({})
}

