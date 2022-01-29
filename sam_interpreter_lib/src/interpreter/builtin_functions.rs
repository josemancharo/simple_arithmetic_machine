use std::{collections::HashMap};

use crate::{util::hash_str::hash_str, algorithms::{trigonometry::*, logarithms::*}};

use super::data_types::Real;


pub enum Func {
    Monad(fn (Real) -> Real),
    Diad(fn (Real, Real) -> Real),
}


pub fn setup_builtins() -> HashMap<u64, Func> {
    let mut map: HashMap<u64, Func> = HashMap::new();
    map.insert(hash_str("sin"), Func::Monad(|x| { sin(x) }));
    map.insert(hash_str("asin"), Func::Monad(|x| { asin(x) }));
    map.insert(hash_str("cos"), Func::Monad(|x| { cos(x) }));
    map.insert(hash_str("acos"), Func::Monad(|x| { acos(x) }));
    map.insert(hash_str("tan"), Func::Monad(|x| { tan(x) }));
    map.insert(hash_str("atan"), Func::Monad(|x| { atan(x) }));
    map.insert(hash_str("log"), Func::Monad(|x| { log(x, Real::Float(10_f64)) }));
    map.insert(hash_str("ln"), Func::Monad(|x| { ln(x) }));
    map.insert(hash_str("log_base"), Func::Diad(|x, y| { log(x, y) }));
    map.insert(hash_str("sqrt"), Func::Monad(|x| { pow(x, Real::Float(0.5)) }));
    return map;
}
