use std::io;

use clap::{Arg, ArgAction, Command, ValueHint, value_parser};
use clap_complete::aot::{Generator, generate};
use clap_complete_all::Shell;

fn build_cli() -> Command {
    Command::new("example")
        .about("Demo app showing clap_complete_all usage")
        .arg(
            Arg::new("file")
                .help("Some input file")
                .value_hint(ValueHint::AnyPath),
        )
        .arg(
            Arg::new("generate")
                .long("generate")
                .help("Generate shell completions for the given shell")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell))
                .value_name("SHELL"),
        )
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    let bin_name = cmd.get_name().to_string();
    generate(generator, cmd, bin_name, &mut io::stdout());
}

fn main() {
    let mut cmd = build_cli();
    let matches = cmd.clone().get_matches();

    if let Some(shell) = matches.get_one::<Shell>("generate").copied() {
        print_completions(shell, &mut cmd);
        return;
    }

    // Normal program logic here
    if let Some(path) = matches.get_one::<String>("file") {
        println!("Received file: {}", path);
    } else {
        println!("Run with --generate <SHELL> to print completions");
    }
}
