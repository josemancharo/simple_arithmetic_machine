use super::user_functions::UserFunctionDefinition;

#[derive(Debug, Clone, PartialEq)]
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
    BitAnd,
    BitOr,
    BitXor,
    RightShift,
    LeftShift,
    Neg,
    Not,
    Lt,
    Gt,
    Lte,
    Gte,
    Neq,
    Eq,
    LoadVar(u64),
    StoreVar(u64),
    CallFunc(u64),
    StoreFunc(u64, UserFunctionDefinition)
}