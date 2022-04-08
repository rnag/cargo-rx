use crate::*;

use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use cargo_options::CommonOptions;
use colored::Colorize;

/// Defines the logic for running `cargo run --example`
pub trait RunExampleExt {
    /// Call `cargo run --example` on an example `name`
    ///
    /// # Arguments
    ///
    /// * `root_path` - the base path to the Cargo directory with a
    ///                 `Cargo.toml` file.
    /// * `name` - the name of the Cargo example to run.
    /// * `args` - additional arguments to pass to `cargo run --example`.
    /// * `required_features` - any required features to enable when running
    ///                         the example.
    fn run_example<'a, T: IntoIterator>(
        &self,
        root_path: &'a Path,
        name: &'a str,
        args: T,
        required_features: Option<&'a String>,
    ) -> Result<()>
    where
        <T as IntoIterator>::Item: AsRef<OsStr>;
}

impl RunExampleExt for CommonOptions {
    fn run_example<'a, T: IntoIterator>(
        &self,
        root_path: &'a Path,
        name: &'a str,
        args: T,
        required_features: Option<&'a String>,
    ) -> Result<()>
    where
        <T as IntoIterator>::Item: AsRef<OsStr>,
    {
        let mut run = Command::new(CARGO_CMD);

        run.current_dir(root_path)
            .arg("run")
            .arg("--example")
            .arg(name);

        if self.quiet {
            run.arg("--quiet");
        }

        if let Some(jobs) = self.jobs {
            run.arg("--jobs").arg(jobs.to_string());
        }

        if self.release {
            run.arg("--release");
        }

        if let Some(ref profile) = self.profile {
            run.arg("--profile").arg(profile);
        }

        if self.all_features {
            run.arg("--all-features");
        } else if let Some(feats) = required_features {
            run.arg("--features");
            if self.features.is_empty() {
                run.arg(feats);
            } else {
                let other_feats = &self.features.join(" ");

                let mut enabled_feats = String::with_capacity(feats.len() + other_feats.len() + 1);
                enabled_feats.push_str(feats);
                enabled_feats.push(' ');
                enabled_feats.push_str(other_feats);

                run.arg(enabled_feats);
            };
        } else if !self.features.is_empty() {
            let feats = self.features.join(" ");
            run.arg("--features").arg(feats);
        }

        if self.no_default_features {
            run.arg("--no-default-features");
        }

        if !self.target.is_empty() {
            run.arg("--target").arg(self.target.join(" "));
        }

        if let Some(ref target_dir) = self.target_dir {
            run.arg("--target-dir").arg(target_dir);
        }

        if let Some(ref manifest_path) = self.manifest_path {
            run.arg("--manifest-path").arg(manifest_path);
        }

        if !self.message_format.is_empty() {
            run.arg("--message-format")
                .arg(self.message_format.join(" "));
        }

        if self.unit_graph {
            run.arg("--unit-graph");
        }

        if self.ignore_rust_version {
            run.arg("--ignore-rust-version");
        }

        if self.verbose != 0 {
            let mut verbose = String::with_capacity(self.verbose + 1);
            verbose.push('-');
            verbose.push_str(&"v".repeat(self.verbose));

            run.arg(verbose);
        }

        if let Some(ref when) = self.color {
            run.arg("--color").arg(when);
        }

        if self.frozen {
            run.arg("--frozen");
        }

        if self.locked {
            run.arg("--locked");
        }

        if self.offline {
            run.arg("--offline");
        }

        if !self.config.is_empty() {
            run.arg("--config").arg(self.config.join(" "));
        }

        if !self.unstable_flags.is_empty() {
            run.arg("-Z").arg(self.unstable_flags.join(" "));
        }

        run.args(args);

        // TODO: maybe it would be a better idea to use something like `shellwords::join()`
        let cargo_run_args = run
            .get_args()
            .map(OsStr::display_string)
            .collect::<Vec<_>>()
            .join(" ");

        #[cfg(target_family = "windows")]
        println!(
            " {} {} {}",
            ">>".white().bold(),
            CARGO_CMD.bright_blue().italic(),
            cargo_run_args.as_str().bright_blue().italic()
        );

        #[cfg(not(target_family = "windows"))]
        println!(
            " {} {} {}",
            "❯❯".white().bold(),
            CARGO_CMD.blue().italic(),
            cargo_run_args.as_str().blue().italic()
        );

        run.spawn()?.wait()?;

        Ok(())
    }
}
