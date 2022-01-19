#[derive(Parser)]
#[grammar = "./parser/grammar.pest"]
pub struct SamParser;
pub type SamRule = Rule;

