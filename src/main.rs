mod cli_args;

use clap::Parser;

use cli_args::CliArgs;

fn main() {
    let cli_args = CliArgs::parse();

    println!("{:?}", cli_args);
}
