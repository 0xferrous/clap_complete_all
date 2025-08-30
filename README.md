# `clap-complete-all`

## Why?

The default `clap_complete` package doesn't ship with Nushell support. This leaves
almost every completion command implementation to not include nushell completions.
This crate aims to provide a simple wrapper glue to enable nushell by default as well.

However, there is another official crate `clap_complete_nushell` that provides the `Generator` trait for nushell.

This crate wraps the `clap_complete::aot::Shell` and provides the same API so that usage is just
replacing `clap_complete::aot::Shell` with `clap_complete_all::Shell`.

```rust
use clap_complete::aot::{generate, Generator};
use clap_complete_all::Shell;
use clap::{Command, Arg, ValueHint, value_parser, ArgAction};

fn build_cli() -> Command {
    Command::new("example")
        .arg(Arg::new("file")
            .help("some input file")
            .value_hint(ValueHint::AnyPath))
        .arg(Arg::new("generator")
            .long("generate")
            .action(ArgAction::Set)
            .value_parser(value_parser!(Shell)))
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(generator, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
```
