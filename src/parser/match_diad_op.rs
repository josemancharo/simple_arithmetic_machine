use crate::ast::operations::Operation;

use super::grammar::SamRule;


pub fn match_diad_op(op: SamRule) -> Option<Operation> {
    match op {
        SamRule::Add => Some(Operation::Add),
        SamRule::Subtract => Some(Operation::Sub),
        SamRule::Multiply => Some(Operation::Mul),
        SamRule::Divide => Some(Operation::Div),
        SamRule::Power => Some(Operation::Pow),
        SamRule::Modulus => Some(Operation::Mod),
        SamRule::Gt => Some(Operation::Gt),
        SamRule::Lt => Some(Operation::Lt),
        SamRule::Gte => Some(Operation::Gte),
        SamRule::Lte => Some(Operation::Lte),
        SamRule::Eq => Some(Operation::Eq),
        SamRule::Neq => Some(Operation::Neq),
        SamRule::Xor => Some(Operation::BitXor),
        SamRule::And => Some(Operation::BitAnd),
        SamRule::Or => Some(Operation::BitOr),
        SamRule::RightShift => Some(Operation::RightShift),
        SamRule::LeftShift => Some(Operation::LeftShift),
        _ => None
    }
}