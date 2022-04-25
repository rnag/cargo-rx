use crate::BINARY_NAME;

use clap::Parser;

/// A simple, modern Example Runner - and fuzzy finder - for Cargo projects.
#[derive(Parser, Clone, Debug, Default)]
#[clap(bin_name = BINARY_NAME)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder, after_help = "Run `cargo help run` for more detailed information.")]
#[clap(version)]
pub struct Args {
    /// Base options for `cargo run --example`
    #[clap(flatten)]
    pub cargo: cargo_options::CommonOptions,

    /// Example script name
    pub name: Option<String>,

    /// Do not prompt for parameters and only use information entered previously
    #[clap(short = 'R', long)]
    pub replay: bool,

    /// True to prompt for arguments to the selected example script
    #[clap(short, long, short_alias = 'p', alias = "prompt-args")]
    pub input_args: bool,

    /// Extra arguments to pass to the Cargo example script
    #[clap(raw = true)]
    pub args: Vec<String>,
}
