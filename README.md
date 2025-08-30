# `clap-complete-all`

## Why?

The default `clap_complete` package doesn't ship with Nushell support. This leaves
almost every completion command implementation to not include nushell completions.
This crate aims to provide a simple wrapper glue to enable nushell by default as well.

However, there is another official crate `clap_complete_nushell` that provides the `Generator` trait for nushell.

This crate wraps the `clap_complete::aot::Shell` and provides the same API so that usage is just
replacing `clap_complete::aot::Shell` with `clap_complete_all::Shell`.

## Minimal Example

```rust
use std::io;

use clap::{value_parser, Arg, ArgAction, Command, ValueHint};
use clap_complete::aot::{generate, Generator};
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
```

You can also run a complete working example from this repo:

```sh
cargo run --example basic -- --generate bash       # Bash
cargo run --example basic -- --generate zsh        # Zsh
cargo run --example basic -- --generate fish       # Fish
cargo run --example basic -- --generate powershell # PowerShell
cargo run --example basic -- --generate elvish     # Elvish
cargo run --example basic -- --generate nushell    # Nushell
```

Redirect output to your completion directory as needed. For example, for Zsh:

```sh
cargo run --example basic -- --generate zsh > _example
```

## Derive Example

A derive-based variant is also provided in `examples/derive.rs`:

```sh
cargo run --example derive -- --generate bash
```

Core differences:
- Uses `#[derive(clap::Parser)]` and struct fields instead of builder API.
- Adds `#[arg(long, value_enum)]` on an `Option<clap_complete_all::Shell>` field to select the target shell.
- Builds a `clap::Command` via `Cli::command()` for AOT completion generation.
