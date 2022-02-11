
use std::cmp::Ordering;

use super::{user_functions::UserFunctionDefinition, ast_block::MatrixDefinition};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Float(f64),
    Int(i64),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    StartBlock,
    BitAnd,
    BitOr,
    BitXor,
    RightShift,
    LeftShift,
    Neg,
    Not,
    BitCompliment,
    Lt,
    Gt,
    Lte,
    Gte,
    Neq,
    Eq,
    BoolAnd,
    BoolOr,
    PeekStack,
    Conditional,
    Ratio,
    LoadVar(u64),
    StoreVar(u64),
    CallFunc(u64),
    StoreFunc(u64, UserFunctionDefinition),
    DefineMatrix(MatrixDefinition)
}

impl Operation {
    fn get_precedence(&self) -> u8 {
        match self {
            Self::Add => 2,
            Self::Sub => 2,
            Self::Mul => 3,
            Self::Div => 3,
            Self::Ratio => 3,
            Self::Pow => 4,
            Self::Mod => 3,
            Self::BoolOr => 0,
            Self::BoolAnd => 0,
            Self::BitAnd => 4,
            Self::BitOr => 2,
            Self::BitXor => 3,
            Self::RightShift => 5,
            Self::LeftShift => 5,
            Self::BitCompliment => 6,
            Self::Lt => 1,
            Self::Gt => 1,
            Self::Lte => 1,
            Self::Gte => 1,
            Self::Eq => 1,
            Self::Neq => 1,
            Self::Not => 16,
            Self::Neg => 2,
            Self::Conditional => 1,
            Self::CallFunc(_) => 255,
            Self::StoreVar(_) => 255,
            Self::StoreFunc(_, _) => 255,
            Self::LoadVar(_) => 255,
            Self::Float(_) => 255,
            Self::Int(_) => 255,
            _ => 0
        }
    }
}

impl PartialOrd for Operation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_precedence().cmp(&other.get_precedence()))
    }
}

impl Eq for Operation {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Operation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_precedence().cmp(&other.get_precedence())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}