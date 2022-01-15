#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operation {
    Const(f64),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Start,
    End,
}