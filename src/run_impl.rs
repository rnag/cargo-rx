use crate::*;
pub(crate) use inner_impl::*;

use colored::Colorize;

#[cfg(target_family = "windows")]
mod inner_impl {
    use super::*;

    use std::borrow::Cow;
    use std::collections::{HashMap, HashSet};
    use std::io::Write;
    use std::process::{Command, Output, Stdio};

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
            print!("{} ", "Arguments:".cyan().bold());
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
            args.cargo
                .run_example(root_ref, name, example_args_ref, req_features)?;
        }

        Ok(())
    }
}

#[cfg(not(target_family = "windows"))]
mod inner_impl {
    use super::*;

    use std::borrow::Cow;
    use std::collections::{BTreeMap, HashMap};
    use std::io::Write;
    use std::sync::Arc;

    use skim::prelude::*;

    //noinspection DuplicatedCode
    pub(crate) fn process_input_inner(
        example_file_name_to_path: BTreeMap<Cow<'_, str>, ExampleFile>,
        dir: &Paths,
        args: Args,
        name_to_required_features: HashMap<String, String>,
    ) -> Result<()> {
        let script_args = args.args;
        let selected_items: Vec<Arc<dyn SkimItem>>;
        let mut cfg: ReplayConfig = Default::default();

        let examples_to_run = if args.replay {
            cfg = get_last_replay()?;
            vec![Cow::Owned(cfg.last_run.name)]
        } else if let Some(example) = args.name {
            vec![Cow::Owned(example)]
        } else {
            let options = SkimOptionsBuilder::default()
                // .height(Some("50%"))
                .preview_window(Some("right:70%"))
                .multi(true)
                .preview(Some("")) // preview should be specified to enable preview window
                .build()
                .unwrap();

            let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

            for example in example_file_name_to_path.values() {
                let _ = tx_item.send(Arc::new(ExampleFileItem {
                    file_stem: example.name.clone(),
                    file_path: example.path.clone(),
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
            print!("{} ", "Arguments:".cyan().bold());
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
        let root_ref = &dir.root_path;

        // Save info on the example we're running, so we can `--replay` it if needed
        match examples_to_run.first() {
            Some(name) if !args.replay => {
                save_last_replay(name, example_args_ref)?;
            }
            _ => {}
        };

        for example_name in examples_to_run {
            let name = example_name.as_ref();

            let example = example_file_name_to_path.get(name).unwrap();
            let req_features: Option<&String> = name_to_required_features.get(name);

            // Run the Cargo example script
            args.cargo.run_example(
                &example.path_type,
                root_ref,
                name,
                example_args_ref,
                req_features,
            )?;
        }

        Ok(())
    }
}
