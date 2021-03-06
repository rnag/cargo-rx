use cargo_rx::*;

use clap::Parser;

pub fn parse_args() -> Args {
    Args::parse()
}

fn main() -> Result<()> {
    let args = parse_args();

    if args.cargo.verbose > 0 {
        println!("VERBOSE output is enabled");
        println!("Arguments: {:#?}", args);
    }

    process_input(args)?;

    Ok(())
}
