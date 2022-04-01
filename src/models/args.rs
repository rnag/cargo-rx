use clap::Parser;

/// Example Runner for Cargo projects
#[derive(Parser, Debug, Default)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Example script name
    pub name: Option<String>,

    /// Do not prompt for parameters and only use information entered previously
    #[clap(short, long)]
    pub replay: bool,

    /// True to prompt for arguments to example script
    #[clap(short, long)]
    pub prompt_args: bool,

    /// True for more verbose console output
    #[clap(short, long)]
    pub verbose: bool,

    /// Extra arguments to pass to the Cargo example script
    #[clap(raw = true)]
    pub args: Vec<String>,
}
