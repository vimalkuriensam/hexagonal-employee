use std::{collections::HashMap, fmt::Display, hash::Hash};

pub fn load_env_var<'a, K: Display + Eq + Hash, V: Display + Eq + Hash>(
    var: &K,
    map: &'a HashMap<K, V>,
) -> Result<&'a V, String> {
    let val = map.get(var);
    val.ok_or_else(|| format!("{} not found", var))
}
