use cargo_toml::{Dependency, Manifest};
use crates_index::GitIndex;
use rayon::iter::ParallelIterator;
use std::fs::File;
use std::io::prelude::Write;

fn main() {
    let mut manifest = Manifest::from_path(format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR")))
        .expect("Cargo.toml not present.");

    let index = GitIndex::new_cargo_default().expect("Failed to get crates.io registry index.");

    let crates = index
        .crates_parallel()
        .filter_map(|maybe_crate| {
            if let Ok(c) = maybe_crate {
                let name = c.name().to_string();
                let version = c.highest_normal_version();

                match (name, version) {
                    (name, Some(_)) if name != String::from("all") => Some(name),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    manifest.dependencies.clear();

    // let mut use_statements: Vec<String> = Vec::new();

    for name in crates {
        if name.starts_with("zz") {
            manifest
                .dependencies
                .insert(name.clone(), Dependency::Simple(String::from("*")));

            //   use_statements.push(format!("pub use {};", name.replace('-', "_")));
        }
    }

    let mut manifest_file = File::options()
        .write(true)
        .truncate(true)
        .open(format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR")))
        .expect("Cargo.toml not present.");

    // let mut lib_file = File::options()
    //     .write(true)
    //     .truncate(true)
    //     .open(format!("{}/src/lib.rs", env!("CARGO_MANIFEST_DIR")))
    //     .expect("lib.rs not present.");

    write!(
        manifest_file,
        "{}",
        toml::to_string(&manifest).expect("Failed to generate new manifest data.")
    )
    .expect("Failed to write new Cargo.toml file.");

    // write!(lib_file, "{}", use_statements.join("\n"))
    //     .expect("Failed to write new Cargo.toml file.");
}
