use std::collections::{HashMap, HashSet};

pub fn find_unused(
    declared: HashMap<String, String>,
    used: HashSet<String>,
) -> Vec<String> {
    declared
        .into_iter()
        .filter(|(canonical, _)| !used.contains(canonical))
        .map(|(_, original)| original)
        .collect()
}