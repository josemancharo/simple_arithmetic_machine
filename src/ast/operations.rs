#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operation {
    Const(f64),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    StartBlock,
    EndBlock,
    LoadVar(u64),
}