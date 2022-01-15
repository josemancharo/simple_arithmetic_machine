#[derive(Parser)]
#[grammar = "./parser/grammar.pest"] // relative to src
pub struct SamParser;
pub type SamRule = Rule;