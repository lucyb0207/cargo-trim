use anyhow::Result;
use cargo_metadata::{Metadata, MetadataCommand, Package};
use std::collections::HashMap;

pub fn load_workspace() -> Result<Metadata> {
    Ok(MetadataCommand::new().exec()?)
}

pub fn package_dependencies(
    package: &Package,
) -> HashMap<String, String> {
    let mut deps = HashMap::new();

    for dep in &package.dependencies {
        deps.insert(
            dep.name.replace('-', "_"),
            dep.name.clone(),
        );
    }

    deps
}