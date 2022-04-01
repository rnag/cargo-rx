use crate::*;

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::{Component, Path, PathBuf};
use std::result::Result as StdResult;
use std::{env, fs};

use cargo_toml::Manifest;

/// Represents *path info* on a Cargo project.
#[derive(Debug)]
pub struct Paths {
    /// *Base path* to a Cargo project directory
    pub root_path: PathBuf,
    /// Path to the *examples* in a Cargo project
    pub examples_path: PathBuf,
    /// Path to the *Cargo.toml* file
    pub cargo_toml_path: PathBuf,
}

impl Paths {
    /// Iterates backward from the current directory, and locates the base
    /// Cargo directory for the project -- which contains at the minimum a
    /// `Cargo.toml` file and an `examples` folder.
    ///
    /// # Returns
    /// Returns the path details on the first such directory path which matches
    /// the criteria.
    ///
    /// # Errors
    /// Raises an error if it cannot locate the base Cargo directory for the
    /// project.
    pub fn resolve() -> Result<Self> {
        let current_dir = env::current_dir()?;

        let examples_folder = Path::new(EXAMPLES_FOLDER);
        let cargo_toml_file = Path::new(CARGO_TOML);

        let examples_path = current_dir.join(&examples_folder);
        let cargo_toml_path = current_dir.join(&cargo_toml_file);

        if examples_path.is_dir() && cargo_toml_path.is_file() {
            return Ok(Self {
                root_path: current_dir,
                examples_path,
                cargo_toml_path,
            });
        }

        let mut comps = current_dir.components();

        while let Some(p) = comps.next_back() {
            match p {
                Component::Normal(_) | Component::CurDir | Component::ParentDir => {
                    let root_path = comps.as_path().to_path_buf();

                    let examples_path = root_path.join(&examples_folder);
                    let cargo_toml_path = root_path.join(&cargo_toml_file);

                    if examples_path.is_dir() && cargo_toml_path.is_file() {
                        return Ok(Self {
                            root_path,
                            examples_path,
                            cargo_toml_path,
                        });
                    }
                }
                _ => {}
            };
        }

        Err(Box::new(Error::new(
            ErrorKind::NotFound,
            format!(
                "could not find `{CARGO_TOML}` with an `{EXAMPLES_FOLDER}` folder \
                in `{cwd}` or any parent directory",
                cwd = current_dir.to_str().unwrap()
            ),
        )))
    }

    /// Return a mapping of *example name* to a list of *required features* for an example.
    pub fn example_to_required_features(&self) -> Result<HashMap<String, String>> {
        let manifest_contents = fs::read(&self.cargo_toml_path)?;
        let manifest = Manifest::from_slice(&manifest_contents)?;

        let mut name_to_required_features: HashMap<String, String> =
            HashMap::with_capacity(manifest.example.len());
        for example in manifest.example {
            match example.name {
                Some(name) if !example.required_features.is_empty() => {
                    name_to_required_features.insert(name, example.required_features.join(" "));
                }
                _ => (),
            };
        }

        Ok(name_to_required_features)
    }

    /// Returns file paths (`PathBuf` objects) of each *example* file in the Cargo project.
    pub fn example_file_paths(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in fs::read_dir(&self.examples_path)?.filter_map(StdResult::ok) {
            let path: PathBuf = entry.path();

            if path.is_file() && matches!(path.extension(), Some(e) if e == RUST_FILE_EXT) {
                files.push(path);
            }
        }

        Ok(files)
    }
}
