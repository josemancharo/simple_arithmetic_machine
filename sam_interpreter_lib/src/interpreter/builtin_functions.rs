use std::{collections::HashMap};

use crate::{util::hash_str::hash_str};

use super::data_types::Real;


pub enum Func {
    Monad(fn (Real) -> Real),
    Diad(fn (Real, Real) -> Real),
}


pub fn setup_builtins() -> HashMap<u64, Func> {
    let mut map: HashMap<u64, Func> = HashMap::new();
    map.insert(hash_str("sin"), Func::Monad(|x| { x.sin() }));
    map.insert(hash_str("asin"), Func::Monad(|x| { x.asin() }));
    map.insert(hash_str("cos"), Func::Monad(|x| { x.cos() }));
    map.insert(hash_str("acos"), Func::Monad(|x| { x.acos() }));
    map.insert(hash_str("tan"), Func::Monad(|x| { x.tan() }));
    map.insert(hash_str("atan"), Func::Monad(|x| { x.atan() }));
    map.insert(hash_str("log"), Func::Monad(|x| { x.log(Real::Float(10_f64)) }));
    map.insert(hash_str("ln"), Func::Monad(|x| { x.ln() }));
    map.insert(hash_str("log_base"), Func::Diad(|x, y| { x.log(y) }));
    map.insert(hash_str("sqrt"), Func::Monad(|x| { x.pow(Real::Float(0.5)) }));
    return map;
}
