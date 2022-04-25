pub use skim_impl::*;

#[cfg(target_family = "windows")]
mod skim_impl {}

#[cfg(not(target_family = "windows"))]
mod skim_impl {
    use std::borrow::Cow;
    use std::fmt::Display;
    use std::fs;
    use std::path::PathBuf;

    use colored::Colorize;
    use skim::prelude::*;

    /// Represents an *example file* item
    pub struct ExampleFileItem {
        /// Absolute file path to the *example*
        pub file_path: PathBuf,

        /// Filename of the *example*, excluding the file extension
        pub file_stem: String,
    }

    impl SkimItem for ExampleFileItem {
        fn text(&self) -> Cow<'_, str> {
            Cow::Borrowed(&self.file_stem)
        }

        fn preview(&self, _context: PreviewContext<'_>) -> ItemPreview {
            let file_contents = match fs::read_to_string(&self.file_path) {
                Ok(contents) => contents,
                // We ran into an error reading the file; usually, this
                // happens when the file doesn't exist.
                Err(e) => return ItemPreview::AnsiText(format_err(e)),
            };

            let lines = file_contents
                .lines()
                .map(|line| {
                    let trimmed_line = line.trim_start();
                    if trimmed_line.starts_with("//!") {
                        line.bright_white().on_black().bold().to_string()
                    } else if trimmed_line.starts_with("//") {
                        line.white().bold().to_string()
                    } else if trimmed_line.starts_with("#[") {
                        line.white().italic().to_string()
                    } else {
                        line.to_owned()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            ItemPreview::AnsiText(format!("\n{}", lines))
        }
    }

    fn format_err<E: Display>(e: E) -> String {
        format!("\n{}: {}", "error".bold(), e)
            .as_str()
            .red()
            .on_white()
            .to_string()
    }
}
