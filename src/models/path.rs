use crate::*;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::io::{Error, ErrorKind};
use std::path::{Component, Path, PathBuf};
use std::result::Result as StdResult;
use std::{env, fs};

use cargo_toml::Manifest;
use path_absolutize::*;

/// Represents *path info* on a Cargo project.
#[derive(Debug)]
pub struct Paths {
    /// *Base path* to a Cargo project directory
    pub root_path: PathBuf,
    /// Path to the *examples* in a Cargo project
    pub examples_path: PathBuf,
    /// Path to the *Cargo.toml* file
    pub cargo_toml_path: PathBuf,
    /// Parsed contents of the *Cargo.toml* manifest file
    pub manifest: Manifest,
}

/// Represents the *type* of an example file.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ExampleType {
    /// This represents a "simple" example file, for ex. a `hello_world.rs`
    /// file in an *examples/* folder.
    Simple,
    /// This represents a "main.rs" file nested within a sub-folder in an
    /// *examples/* folder; the Rust book also calls this a **multi-file**
    /// example.
    MultiFile,
    /// Custom name
    Custom(String),
}

/// Represents an *example file* in a Cargo project.
#[derive(Debug, Eq)]
pub struct ExampleFile {
    /// Path to example file
    pub path: PathBuf,
    /// Type of example file
    pub path_type: ExampleType,
}

impl PartialEq<Self> for ExampleFile {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Hash for ExampleFile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state)
    }
}

impl PartialEq<PathBuf> for ExampleFile {
    fn eq(&self, other: &PathBuf) -> bool {
        &self.path == other
    }
}

impl TryFrom<PathBuf> for ExampleFile {
    type Error = Error;

    fn try_from(mut path: PathBuf) -> StdResult<Self, Self::Error> {
        let file_type = path.metadata()?.file_type();

        if file_type.is_dir() {
            path.push("main.rs");
            if path.is_file() {
                return Ok(Self {
                    path,
                    path_type: ExampleType::MultiFile,
                });
            }
        } else if file_type.is_file() && matches!(path.extension(), Some(e) if e == RUST_FILE_EXT) {
            return Ok(Self {
                path,
                path_type: ExampleType::Simple,
            });
        }

        // TODO
        Err(Error::new(ErrorKind::InvalidData, "not an example file"))
    }
}

impl ExampleFile {
    /// Create an `ExampleFile` from a example `name` and a (possibly relative)
    /// `path`.
    ///
    /// # Arguments
    /// * `root` - The current working directory, used in case the path is
    ///            relative.
    /// * `name` - The name of the example file, without the extension
    /// * `path` - The path to the example file. This can be a relative path
    ///            and contain characters such as `.` and `..` for instance.
    pub fn from_name_and_path<P: AsRef<Path>>(root: &Path, name: String, path: P) -> Self {
        let abs_path = path.as_ref().absolutize_from(root).unwrap();

        Self {
            path: PathBuf::from(abs_path),
            path_type: ExampleType::Custom(name),
        }
    }

    /// Returns the *file stem* (i.e. filename without the extension) for an
    /// example file, or the *folder name* in the case of a `main.rs` example
    /// within a sub-folder.
    pub fn name(&self) -> &str {
        match self.path_type {
            ExampleType::MultiFile => {
                let mut comps = self.path.components();
                // discard the filename (main.rs)
                let _ = comps.next_back();
                // return the parent folder name
                comps.next_back().unwrap().as_os_str().to_str().unwrap()
            }
            ExampleType::Simple => self.path.file_stem().unwrap().to_str().unwrap(),
            ExampleType::Custom(ref name) => name,
        }
    }
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
            let manifest_contents = fs::read(&cargo_toml_path)?;
            let manifest = Manifest::from_slice(&manifest_contents)?;

            return Ok(Self {
                root_path: current_dir,
                examples_path,
                cargo_toml_path,
                manifest,
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
                        let manifest_contents = fs::read(&cargo_toml_path)?;
                        let manifest = Manifest::from_slice(&manifest_contents)?;

                        return Ok(Self {
                            root_path,
                            examples_path,
                            cargo_toml_path,
                            manifest,
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
        let mut name_to_required_features: HashMap<String, String> =
            HashMap::with_capacity(self.manifest.example.len());
        let manifest = self.manifest.clone();

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
    pub fn example_file_paths(&self) -> Result<HashSet<ExampleFile>> {
        let mut files = HashSet::new();

        let manifest = &self.manifest;
        let root = &self.root_path;

        for example in &manifest.example {
            if let Some(path) = &example.path {
                if let Some(name) = &example.name {
                    let f = ExampleFile::from_name_and_path(root, name.to_owned(), path);
                    files.insert(f);
                }
            }
        }

        for entry in fs::read_dir(&self.examples_path)?.filter_map(StdResult::ok) {
            let path: PathBuf = entry.path();

            if let Ok(f) = ExampleFile::try_from(path) {
                files.insert(f);
            }
        }

        Ok(files)
    }
}
