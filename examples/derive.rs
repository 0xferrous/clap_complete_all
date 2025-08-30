use std::io;
use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use clap_complete::aot::{Generator, generate};
use clap_complete_all::Shell;

#[derive(Parser, Debug)]
#[command(
    name = "example-derive",
    about = "Demo app using clap derive with clap_complete_all"
)]
struct Cli {
    /// Some input file
    #[arg(value_hint = clap::ValueHint::AnyPath)]
    file: Option<PathBuf>,

    /// Generate shell completions for the given shell
    #[arg(long, value_enum, value_name = "SHELL")]
    generate: Option<Shell>,
}

fn print_completions<G: Generator>(generator: G, cmd: &mut clap::Command) {
    let bin_name = cmd.get_name().to_string();
    generate(generator, cmd, bin_name, &mut io::stdout());
}

fn main() {
    let cli = Cli::parse();

    if let Some(shell) = cli.generate {
        let mut cmd = Cli::command();
        print_completions(shell, &mut cmd);
        return;
    }

    // Normal program logic here
    if let Some(path) = cli.file {
        println!("Received file: {}", path.display());
    } else {
        println!("Run with --generate <SHELL> to print completions");
    }
}
