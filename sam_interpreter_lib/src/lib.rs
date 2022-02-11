pub mod parser;
pub mod interpreter;
pub mod errors;
mod ast;
mod algorithms;
mod util;

extern crate nalgebra as na;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub use parser::parse_input;
pub use interpreter::virtual_machine::SamVM;
pub use errors::SamError;