use std::{collections::hash_map::DefaultHasher, hash::{Hasher, Hash}};

pub fn hash_str(string: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    let key = hasher.finish();
    return key;
}