
use std::cmp::Ordering;

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
    BitAnd,
    BitOr,
    BitXor,
    RightShift,
    LeftShift,
    Neg,
    BitCompliment,
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

impl Operation {
    fn get_precedence(&self) -> u8 {
        match self {
            Self::Add => 1,
            Self::Sub => 1,
            Self::Mul => 2,
            Self::Div => 2,
            Self::Pow => 4,
            Self::Mod => 2,
            Self::BitAnd => 3,
            Self::BitOr => 1,
            Self::BitXor => 2,
            Self::RightShift => 4,
            Self::LeftShift => 4,
            Self::Lt => 8,
            Self::Gt => 8,
            Self::Lte => 8,
            Self::Gte => 8,
            Self::Eq => 8,
            Self::Neq => 16,
            Self::BitCompliment => 16,
            Self::CallFunc(_) => 255,
            Self::StoreVar(_) => 255,
            Self::StoreFunc(_, _) => 255,
            Self::LoadVar(_) => 255,
            Self::Const(_) => 255,
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