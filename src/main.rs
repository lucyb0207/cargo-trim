mod reconciler;
mod scanner;
mod workspace;

use anyhow::Result;
use colored::*;
use workspace::*;

fn main() -> Result<()> {
    let metadata = load_workspace()?;

    for package in metadata.packages {
        println!(
            "\n{} {}",
            "Checking".blue().bold(),
            package.name
        );

        let deps = package_dependencies(&package);

        let package_dir = package
            .manifest_path
            .parent()
            .unwrap();

        let src_dir = package_dir.join("src");

        if !src_dir.exists() {
            continue;
        }

        let used = scanner::scan_package(&src_dir)?;

        let unused = reconciler::find_unused(
            deps,
            used,
        );

        if unused.is_empty() {
            println!("{}", "No unused deps".green());
        } else {
            println!("{}", "Unused dependencies:".red());

            for dep in unused {
                println!("  - {}", dep);
            }
        }
    }

    Ok(())
}