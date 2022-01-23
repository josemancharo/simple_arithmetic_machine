
use errors::SamError;
use repl::run_repl;

mod parser;
mod ast;
mod algorithms;
mod interpreter;
mod cli;
mod util;
mod repl;
mod errors;

extern crate pest;
#[macro_use]
extern crate pest_derive;


fn main() -> Result<(), SamError> {
   run_repl()?; 
   Ok({})
}

