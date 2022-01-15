use std::collections::HashMap;

use crate::util::hash_str::hash_str;

pub fn generate_constants() -> HashMap<u64, f64> {
    let mut map = HashMap::new();
    map.insert(hash_str("pi"), std::f64::consts::PI);
    map.insert(hash_str("e"), std::f64::consts::E);
    map.insert(hash_str("tau"), std::f64::consts::TAU);
    map.insert(hash_str("G"), 6.67428_f64.powi(-11));
    return map;
}