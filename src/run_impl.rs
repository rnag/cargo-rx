use crate::*;

use std::path::Path;
use std::process::Command;

use colored::Colorize;

pub(crate) use inner_impl::*;

/// Call `cargo run --example` on an example `name`
pub fn cargo_run_example<'a, 'b, T: IntoIterator>(
    root_path: &'a Path,
    name: &'a str,
    args: T,
    required_features: Option<&'a String>,
) -> Result<()>
where
    Vec<&'a str>: Extend<<T as IntoIterator>::Item>,
{
    let mut base_args = vec!["run", "--example", name];

    if let Some(feat) = required_features {
        base_args.push("--features");
        base_args.push(feat);
    };

    base_args.extend(args);

    #[cfg(target_family = "windows")]
    println!(
        " {} {} {}",
        ">>".white().bold(),
        CARGO_CMD.bright_blue().italic(),
        base_args.join(" ").as_str().bright_blue().italic()
    );

    #[cfg(not(target_family = "windows"))]
    println!(
        " {} {} {}",
        "❯❯".white().bold(),
        CARGO_CMD.blue().italic(),
        base_args.join(" ").as_str().blue().italic()
    );

    let _res = Command::new(CARGO_CMD)
        .args(base_args)
        .current_dir(root_path)
        .spawn()?
        .wait()?;

    Ok(())
}

#[cfg(target_family = "windows")]
mod inner_impl {
    use super::*;

    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::io::Write;
    use std::path::PathBuf;
    use std::process::{Command, Output, Stdio};

    //noinspection DuplicatedCode
    pub(crate) fn process_input_inner(
        my_files: Vec<PathBuf>,
        dir: Paths,
        args: Args,
        name_to_required_features: HashMap<String, String>,
    ) -> Result<()> {
        let script_args = args.args;
        let mut cfg: ReplayConfig = Default::default();
        let fzf: Output;

        let examples = if args.replay {
            cfg = get_last_replay()?;
            vec![Cow::Owned(cfg.last_run.name)]
        } else if let Some(example) = args.name {
            vec![Cow::Owned(example)]
        } else {
            let example_names = my_files
                .iter()
                .map(|f| f.file_stem().unwrap().to_str().unwrap())
                .collect::<Vec<_>>()
                .join("\n");

            let echo = Command::new(ECHO_CMD)
                .arg(format!("'{example_names}'"))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;

            let piped_input = echo.stdout.unwrap();

            fzf = Command::new(FZF_CMD)
                .arg("-m")
                .stdin(piped_input)
                .output()
                .unwrap_or_else(|e| panic!("failed to execute `{}` process: {}", FZF_CMD, e));

            std::str::from_utf8(&fzf.stdout)?
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
        } else if args.prompt_args {
            // Print label for input
            print!("Arguments: ");
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

        // Save info on the example we're running, so we can `--replay` it if needed
        match examples.first() {
            Some(name) if !args.replay => {
                save_last_replay(name, &example_args)?;
            }
            _ => {}
        };

        for example in examples {
            let name = example.as_ref();
            let req_features: Option<&String> = name_to_required_features.get(name);

            // Run the Cargo example script
            cargo_run_example(&dir.root_path, name, &example_args, req_features)?;
        }

        Ok(())
    }
}

#[cfg(not(target_family = "windows"))]
mod inner_impl {
    use super::*;

    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::Arc;

    use skim::prelude::*;

    //noinspection DuplicatedCode
    pub(crate) fn process_input_inner(
        my_files: Vec<PathBuf>,
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
            let options = SkimOptionsBuilder::default()
                // .height(Some("50%"))
                .preview_window(Some("right:70%"))
                .multi(true)
                .preview(Some("")) // preview should be specified to enable preview window
                .build()
                .unwrap();

            let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

            for ex_file in my_files.into_iter() {
                let _ = tx_item.send(Arc::new(ExampleFileItem {
                    file_stem: ex_file.file_stem().unwrap().to_os_string(),
                    file_path: ex_file,
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
        } else if args.prompt_args {
            // Print label for input
            print!("Arguments: ");
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

        // Save info on the example we're running, so we can `--replay` it if needed
        match examples.first() {
            Some(name) if !args.replay => {
                save_last_replay(name, &example_args)?;
            }
            _ => {}
        };

        for example in examples {
            let name = example.as_ref();
            let req_features: Option<&String> = name_to_required_features.get(name);

            // Run the Cargo example script
            cargo_run_example(&dir.root_path, name, &example_args, req_features)?;
        }

        Ok(())
    }
}
