use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn scan_package(
    src_dir: &PathBuf,
) -> Result<HashSet<String>> {
    let mut used = HashSet::new();

    let patterns = vec![
        Regex::new(r"use\s+([a-zA-Z0-9_]+)")?,
        Regex::new(r"extern\s+crate\s+([a-zA-Z0-9_]+)")?,
        Regex::new(r"([a-zA-Z0-9_]+)::")?,
    ];

    for entry in WalkDir::new(src_dir) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        scan_file(path, &patterns, &mut used)?;
    }

    Ok(used)
}

fn scan_file(
    path: &Path,
    patterns: &[Regex],
    used: &mut HashSet<String>,
) -> Result<()> {
    let content = fs::read_to_string(path)?;

    for pattern in patterns {
        for cap in pattern.captures_iter(&content) {
            if let Some(name) = cap.get(1) {
                used.insert(name.as_str().to_string());
            }
        }
    }

    Ok(())
}