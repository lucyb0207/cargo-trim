mod reconciler;
mod scanner;
mod workspace;

use anyhow::Result;
use clap::Parser;
use colored::*;
use workspace::*;

#[derive(Parser, Debug)]
#[command(name = "cargo-trim")]
#[command(about = "Detect unused workspace dependencies")]
struct Args {
    #[arg(long)]
    check: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!(
        "{} {:?}",
        "Parsed CLI args:".blue().bold(),
        args
    );

    let metadata = load_workspace()?;

    for package in metadata.workspace_packages() {
        println!(
            "\n{} {}",
            "Checking".cyan().bold(),
            package.name.bold()
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

        let used = scanner::scan_package(src_dir.as_std_path())?;

        let unused = reconciler::find_unused(
            deps,
            used,
        );

        if unused.is_empty() {
            println!("{}", "No unused deps".green());
        } else {
            println!("{}", "Unused dependencies:".red().bold());

            for dep in unused {
                println!("  - {}", dep);
            }
        }
    }

    Ok(())
}