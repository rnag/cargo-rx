use crate::*;

use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::io::{Error, ErrorKind};
use std::path::{Component, Path, PathBuf};
use std::result::Result as StdResult;
use std::{env, fs};

use cargo_toml::{Manifest, Product};
use path_absolutize::*;

/// Allows creating a Path with a Builder Pattern
///
/// Credits: https://internals.rust-lang.org/t/add-zero-cost-builder-methods-to-pathbuf/15318/16?u=rnag
macro_rules! path {
    ( $($segment:expr),+ ) => {{
        let mut path = ::std::path::PathBuf::new();
        $(path.push($segment);)*
        path
    }};
    ( $($segment:expr),+; capacity = $n:expr ) => {{
        let mut path = ::std::path::PathBuf::with_capacity($n);
        $(path.push($segment);)*
        path
    }};
}

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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExampleType {
    /// This represents a *simple* example file, for ex. a `hello_world.rs`
    /// file in an `examples/` folder.
    Simple,

    /// This represents a `main.rs` file nested within a sub-folder in an
    /// `examples/` folder; the Rust book also calls this a **multi-file**
    /// example.
    MultiFile,

    /// This represents a binary crate, i.e. a sub-folder containing a
    /// `Cargo.toml` file in an `examples/` folder; Note that Cargo (and
    /// notably `cargo run --example`) does not support this particular
    /// use-case, as of yet.
    Crate(PathBuf, Option<String>),

    /// This represents an example file with a *custom path* defined in the
    /// `Cargo.toml` file of a Cargo project.
    Custom,
}

/// Represents an *example file* in a Cargo project.
#[derive(Clone, Debug, Eq)]
pub struct ExampleFile {
    /// The *file stem* (i.e. filename without the extension) for an
    /// example file, or the *folder name* in the case of a `main.rs`
    /// example within a sub-folder.
    pub name: String,

    /// Path to example file
    pub path: PathBuf,

    /// Type of example file
    pub path_type: ExampleType,

    /// A space-separated list of required features for the example (or binary) to run.
    ///
    /// The required-features field specifies which features the product needs in order to be built.
    ///
    /// If any of the required features are not selected, the product will be skipped.
    /// This is only relevant for the `[[bin]]`, `[[bench]]`, `[[test]]`, and `[[example]]` sections,
    /// it has no effect on `[lib]`.
    pub required_features: Option<String>,
}

/// *order* a sequence of `ExampleFile`s by the `name` field.

impl Ord for ExampleFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for ExampleFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Determine *equality* and *hash* using the `path` field.

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

    /// Try to create an `ExampleFile` from a `PathBuf` object.
    fn try_from(path: PathBuf) -> StdResult<Self, Self::Error> {
        let file_type = path.metadata()?.file_type();

        if file_type.is_dir() {
            let main_rs = path.join(MAIN_RS);

            if main_rs.is_file() {
                return Ok(Self {
                    // return the parent folder name
                    name: path.last(),
                    path: main_rs,
                    path_type: ExampleType::MultiFile,
                    required_features: None,
                });
            }

            let cargo_toml = path.join(CARGO_TOML);

            if cargo_toml.is_file() {
                // the parent folder name
                let name = path.last();

                let main_rs = path!(
                    path,
                    "src",
                    MAIN_RS;
                    // "srcmain.rs".len() == 10
                    //   adding +2 because of slashes (/) between components
                    capacity = path.as_os_str().len() + 12
                );

                let path = if main_rs.is_file() {
                    main_rs
                } else {
                    cargo_toml.clone()
                };

                return Ok(Self {
                    name,
                    path,
                    path_type: ExampleType::Crate(cargo_toml, None),
                    required_features: None,
                });
            }
        } else if file_type.is_file() && matches!(path.extension(), Some(e) if e == RUST_FILE_EXT) {
            let name = path.file_stem().unwrap().to_str().unwrap().to_owned();

            return Ok(Self {
                name,
                path,
                path_type: ExampleType::Simple,
                required_features: None,
            });
        }

        // TODO
        Err(Error::new(ErrorKind::InvalidData, "not an example file"))
    }
}

impl ExampleFile {
    /// Create an `ExampleFile` from a example `name`, a (possibly relative)
    /// `path`, and a list of *required features* for the example.
    ///
    /// # Arguments
    /// * `root` - The current working directory, used in case the path is
    ///            relative.
    /// * `name` - The name of the example file, without the extension
    /// * `path` - The path to the example file. This can be a relative path
    ///            and contain characters such as `.` and `..` for instance.
    pub fn new<P: AsRef<Path>>(
        root: &Path,
        name: String,
        path: P,
        required_features: Option<String>,
    ) -> Self {
        let abs_path = path.as_ref().absolutize_from(root).unwrap();
        let path = PathBuf::from(abs_path);

        Self {
            name,
            path,
            path_type: ExampleType::Custom,
            required_features,
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

        let examples_path = current_dir.join(examples_folder);
        let cargo_toml_path = current_dir.join(cargo_toml_file);

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

                    let examples_path = root_path.join(examples_folder);
                    let cargo_toml_path = root_path.join(cargo_toml_file);

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

    /// Returns an ordered (A -> Z) mapping of file name to resolved file
    /// (`ExampleFile` objects) of each *example* file in the Cargo project.
    pub fn example_files(&self) -> Result<BTreeMap<Cow<'_, str>, ExampleFile>> {
        let mut files: BTreeMap<Cow<'_, str>, _> = BTreeMap::new();
        let mut file_paths: HashSet<PathBuf> = HashSet::new();

        #[inline]
        fn required_features(example: &Product) -> Option<String> {
            if example.required_features.is_empty() {
                None
            } else {
                Some(example.required_features.join(" "))
            }
        }

        let manifest = &self.manifest;
        let root = &self.root_path;

        for example in manifest.example.iter() {
            // only if `name` and `path` are both provided
            if let Some(ref path) = example.path {
                if let Some(ref name) = example.name {
                    // I debated whether to add this for Mac/Linux, however it
                    // seems like `cargo run --example` doesn't support
                    // backslashes (\) in `example.path` in Cargo.toml either,
                    // so it's likely not worth the effort in this case.

                    // #[cfg(not(target_family = "windows"))]
                    // let path = path.replace('\\', "/");
                    let f =
                        ExampleFile::new(root, name.to_owned(), path, required_features(example));

                    file_paths.insert(f.path.clone());
                    files.insert(Cow::Borrowed(name), f);
                }
            }
            // only if `name` and `required-features` are both provided
            else if let Some(ref name) = example.name {
                let required_features = required_features(example);

                if required_features.is_some() {
                    let f = ExampleFile::new(root, name.to_owned(), "N/A", required_features);
                    files.insert(Cow::Borrowed(name), f);
                }
            }
        }

        for entry in fs::read_dir(&self.examples_path)?.filter_map(StdResult::ok) {
            let path: PathBuf = entry.path();

            // if a file path is already specified in the `Cargo.toml`, then this
            // is an `ExampleType::Custom`, so we don't need to check the file.
            if file_paths.contains(&path) {
                continue;
            }

            if let Ok(f) = ExampleFile::try_from(path) {
                // if we have a Cargo crate (with its own `Cargo.toml`) in
                // the `examples/` folder, we'll need to check that here.
                if let ExampleType::Crate(ref cargo_toml, _) = f.path_type {
                    let manifest_contents = fs::read_to_string(cargo_toml)?;
                    let num_bins = manifest_contents.matches("[[bin]]").count();
                    // if we have multiple binary (or `[[bin]]`) targets
                    // listed in the `Cargo.toml`, then we'll need to
                    // separately add each target as an example.
                    if num_bins > 1 {
                        let crate_dir = cargo_toml.parent().unwrap();
                        let manifest = Manifest::from_str(&manifest_contents)?;
                        // iterate over each binary target
                        for ref bin in manifest.bin {
                            if let Some(ref name) = bin.name {
                                let key = Cow::Owned(name.to_owned());
                                let path_type =
                                    ExampleType::Crate(cargo_toml.clone(), Some(name.to_owned()));
                                let path = if let Some(ref p) = bin.path {
                                    Path::new(p).absolutize_from(crate_dir).unwrap().into()
                                } else {
                                    f.path.clone()
                                };
                                // add the binary target to the list of (runnable) files
                                let bin_f = ExampleFile {
                                    name: name.to_owned(),
                                    path,
                                    path_type,
                                    required_features: required_features(bin),
                                };
                                files.insert(key, bin_f);
                            }
                        }
                        continue;
                    }
                }

                let key = Cow::Owned(f.name.to_owned());

                // if the example name already exists in `Cargo.toml`, then
                // just update values as needed.
                if let Some(example) = files.get_mut(&key) {
                    example.path = f.path;
                    example.path_type = f.path_type;
                }
                // else, we record and add a new example file that can be run.
                else {
                    files.insert(key, f);
                }
            }
        }

        Ok(files)
    }
}
