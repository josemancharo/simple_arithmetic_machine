use std::collections::HashMap;

use crate::util::hash_str::hash_str;

use super::data_types::Real;


pub fn generate_constants() -> HashMap<u64, Real> {
    let mut map = HashMap::new();
    map.insert(hash_str("pi"), Real::Float(std::f64::consts::PI));
    map.insert(hash_str("e"), Real::Float(std::f64::consts::E));
    map.insert(hash_str("tau"), Real::Float(std::f64::consts::TAU));
    map.insert(hash_str("G"), Real::Float(6.67428_f64.powi(-11)));
    map.insert(hash_str("true"), Real::Int(1));
    map.insert(hash_str("false"), Real::Int(0));
    return map;
}