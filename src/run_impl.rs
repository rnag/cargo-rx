use crate::*;

use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use cargo_options::CommonOptions;
use colored::Colorize;

pub(crate) use inner_impl::*;

/// Call `cargo run --example` on an example `name`
pub fn cargo_run_example<'a, T: IntoIterator>(
    root_path: &'a Path,
    name: &'a str,
    opts: &'a CommonOptions,
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

    if opts.quiet {
        run.arg("--quiet");
    }

    if let Some(jobs) = opts.jobs {
        run.arg("--jobs").arg(jobs.to_string());
    }

    if opts.release {
        run.arg("--release");
    }

    if let Some(ref profile) = opts.profile {
        run.arg("--profile").arg(profile);
    }

    if opts.all_features {
        run.arg("--all-features");
    } else if let Some(feats) = required_features {
        run.arg("--features");
        if opts.features.is_empty() {
            run.arg(feats);
        } else {
            let other_feats = &opts.features.join(" ");

            let mut enabled_feats = String::with_capacity(feats.len() + other_feats.len() + 1);
            enabled_feats.push_str(feats);
            enabled_feats.push(' ');
            enabled_feats.push_str(other_feats);

            run.arg(enabled_feats);
        };
    } else if !opts.features.is_empty() {
        let feats = opts.features.join(" ");
        run.arg("--features").arg(feats);
    }

    if opts.no_default_features {
        run.arg("--no-default-features");
    }

    if !opts.target.is_empty() {
        run.arg("--target").arg(opts.target.join(" "));
    }

    if let Some(ref target_dir) = opts.target_dir {
        run.arg("--target-dir").arg(target_dir);
    }

    if let Some(ref manifest_path) = opts.manifest_path {
        run.arg("--manifest-path").arg(manifest_path);
    }

    if !opts.message_format.is_empty() {
        run.arg("--message-format")
            .arg(opts.message_format.join(" "));
    }

    if opts.unit_graph {
        run.arg("--unit-graph");
    }

    if opts.ignore_rust_version {
        run.arg("--ignore-rust-version");
    }

    if opts.verbose != 0 {
        let mut verbose = String::with_capacity(opts.verbose + 1);
        verbose.push('-');
        verbose.push_str(&"v".repeat(opts.verbose));

        run.arg(verbose);
    }

    if let Some(ref when) = opts.color {
        run.arg("--color").arg(when);
    }

    if opts.frozen {
        run.arg("--frozen");
    }

    if opts.locked {
        run.arg("--locked");
    }

    if opts.offline {
        run.arg("--offline");
    }

    if !opts.config.is_empty() {
        run.arg("--config").arg(opts.config.join(" "));
    }

    if !opts.unstable_flags.is_empty() {
        run.arg("-Z").arg(opts.unstable_flags.join(" "));
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

#[cfg(target_family = "windows")]
mod inner_impl {
    use super::*;

    use std::borrow::Cow;
    use std::collections::{HashMap, HashSet};
    use std::io::Write;
    use std::process::{Output, Stdio};

    //noinspection DuplicatedCode
    pub(crate) fn process_input_inner(
        example_files: HashSet<ExampleFile>,
        dir: Paths,
        args: Args,
        name_to_required_features: HashMap<String, String>,
    ) -> Result<()> {
        let script_args = args.args;
        let mut cfg: ReplayConfig = Default::default();
        let output: Output;

        let examples = if args.replay {
            cfg = get_last_replay()?;
            vec![Cow::Owned(cfg.last_run.name)]
        } else if let Some(example) = args.name {
            vec![Cow::Owned(example)]
        } else {
            let mut example_names: Vec<_> = example_files.iter().map(|f| f.name.as_str()).collect();

            // Sort A -> Z, using the names of example files
            example_names.sort_unstable();

            let example_names = example_names.join("\n");

            // I was previously testing with the `echo` command -- i.e. the
            // equivalent of `echo "one\ntwo\nthree" | fzf` -- however this is
            // not needed anymore, as I realized we can pipe stdin directly;
            // see below.

            let mut child = Command::new(FZF_CMD)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                // .stderr(Stdio::piped())
                .arg("-m")
                .spawn()
                .expect("Failed to spawn child process");

            // pipe stdin in to the `fzf` command
            let mut stdin = child.stdin.take().expect("Failed to open stdin");
            std::thread::spawn(move || {
                stdin
                    .write_all(example_names.as_bytes())
                    .expect("Failed to write to stdin");
            });

            // get the output from running `fzf`
            output = child.wait_with_output().expect("Failed to read stdout");

            std::str::from_utf8(&output.stdout)?
                .split_terminator('\n')
                .map(Cow::Borrowed)
                .collect()
        };

        let words: Vec<String>;

        let example_args = if args.replay {
            cfg.last_run.arguments.iter().map(String::as_str).collect()
        } else if !script_args.is_empty() {
            // Build and return extra arguments to pass to the script
            let mut extra_args = Vec::with_capacity(script_args.len() + 1);
            extra_args.push("--");
            for arg in script_args.iter() {
                extra_args.push(arg);
            }
            extra_args
        } else if args.input_args {
            // Print label for input
            print!("{} ", "Arguments:".bright_cyan());
            std::io::stdout().flush()?;
            // Read user input
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;
            // Split user-entered arguments, respecting quotes and other characters
            words = shellwords::split(line.trim_end())?;
            // Build and return extra arguments to pass to the script
            let mut extra_args = Vec::with_capacity(words.len() + 1);
            extra_args.push("--");
            for arg in words.iter() {
                extra_args.push(arg);
            }
            extra_args
        } else {
            Vec::default()
        };

        let example_args_ref = &example_args;
        let opts_ref = &args.cargo;
        let root_ref = &dir.root_path;

        // Save info on the example we're running, so we can `--replay` it if needed
        match examples.first() {
            Some(name) if !args.replay => {
                save_last_replay(name, example_args_ref)?;
            }
            _ => {}
        };

        for example in examples {
            let name = example.as_ref();
            let req_features: Option<&String> = name_to_required_features.get(name);

            // Run the Cargo example script
            cargo_run_example(root_ref, name, opts_ref, example_args_ref, req_features)?;
        }

        Ok(())
    }
}

#[cfg(not(target_family = "windows"))]
mod inner_impl {
    use super::*;

    use std::borrow::Cow;
    use std::collections::{HashMap, HashSet};
    use std::io::Write;
    use std::sync::Arc;

    use skim::prelude::*;

    //noinspection DuplicatedCode
    pub(crate) fn process_input_inner(
        example_files: HashSet<ExampleFile>,
        dir: Paths,
        args: Args,
        name_to_required_features: HashMap<String, String>,
    ) -> Result<()> {
        let script_args = args.args;
        let selected_items: Vec<Arc<dyn SkimItem>>;
        let mut cfg: ReplayConfig = Default::default();

        let examples = if args.replay {
            cfg = get_last_replay()?;
            vec![Cow::Owned(cfg.last_run.name)]
        } else if let Some(example) = args.name {
            vec![Cow::Owned(example)]
        } else {
            let mut example_files: Vec<_> = Vec::from_iter(example_files);

            // Sort A -> Z, using the names of example files
            example_files.sort_unstable();

            let options = SkimOptionsBuilder::default()
                // .height(Some("50%"))
                .preview_window(Some("right:70%"))
                .multi(true)
                .preview(Some("")) // preview should be specified to enable preview window
                .build()
                .unwrap();

            let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

            for ex_file in example_files.into_iter() {
                let _ = tx_item.send(Arc::new(ExampleFileItem {
                    file_stem: ex_file.name,
                    file_path: ex_file.path,
                }));
            }
            drop(tx_item); // so that skim could know when to stop waiting for more items.

            selected_items = Skim::run_with(&options, Some(rx_item))
                .map(|out| {
                    // Quit, if user presses a key such as Esc, Backspace, or Delete
                    if out.is_abort {
                        std::process::exit(0);
                    }
                    out.selected_items
                })
                .unwrap_or_else(Vec::new);

            selected_items
                .iter()
                .map(|item| item.text())
                .collect::<Vec<_>>()
        };

        let words: Vec<String>;

        let example_args = if args.replay {
            cfg.last_run.arguments.iter().map(String::as_str).collect()
        } else if !script_args.is_empty() {
            // Build and return extra arguments to pass to the script
            let mut extra_args = Vec::with_capacity(script_args.len() + 1);
            extra_args.push("--");
            for arg in script_args.iter() {
                extra_args.push(arg);
            }
            extra_args
        } else if args.input_args {
            // Print label for input
            print!("{} ", "Arguments:".bright_cyan());
            std::io::stdout().flush()?;
            // Read user input
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;
            // Split user-entered arguments, respecting quotes and other characters
            words = shellwords::split(line.trim_end())?;
            // Build and return extra arguments to pass to the script
            let mut extra_args = Vec::with_capacity(words.len() + 1);
            extra_args.push("--");
            for arg in words.iter() {
                extra_args.push(arg);
            }
            extra_args
        } else {
            Vec::default()
        };

        let example_args_ref = &example_args;
        let opts_ref = &args.cargo;
        let root_ref = &dir.root_path;

        // Save info on the example we're running, so we can `--replay` it if needed
        match examples.first() {
            Some(name) if !args.replay => {
                save_last_replay(name, example_args_ref)?;
            }
            _ => {}
        };

        for example in examples {
            let name = example.as_ref();
            let req_features: Option<&String> = name_to_required_features.get(name);

            // Run the Cargo example script
            cargo_run_example(root_ref, name, opts_ref, example_args_ref, req_features)?;
        }

        Ok(())
    }
}
