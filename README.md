
# Ovation

Ovation is a dispatch layer for clap that associates
[`Subcommand`](https://docs.rs/clap/4.6.1/clap/trait.Subcommand.html) variants
to typed handler functions.

For example, given an enum
```rust
#[derive(clap::Subcommand)]
enum ExampleSet {
    Ping,
    Hi {
        name: String,
    }
}
```

We can associate functions with each variant to execute their subcommand:
```rust
# #[derive(clap::Subcommand)]
# enum ExampleSet { Ping, Hi { name: String } }

fn say_hi<Ctx: ovation::CommandContext<Commands = ExampleSet>>(ctx: &Ctx, set: &ExampleSet) -> Result<String, ()> {
    match set {
        ExampleSet::Hi{ name } => Ok(format!("Hi, {name}!")),
        _ => unreachable!(),
    }
}

impl<Ctx> ovation::CommandSet<Ctx> for ExampleSet
where
    Ctx: ovation::CommandContext<Commands = Self>,
{
    type ReturnType = String;
    type ErrorType = ();

    fn dispatch<'a>(&self) -> &'a dyn ovation::CommandDelegate<Ctx> {
        match self {
            // Inline closures work for simple commands...
            ExampleSet::Ping => &|_, _| Ok("pong".into()),

            // ...and more complex commands can use named functions.
            ExampleSet::Hi { .. } => &say_hi,
        }
    }
}
```

## Quickstart

`Cargo.toml`:
```toml
[dependencies]
    ovation = "0.1.0"
```

or run `cargo add ovation`.

### Example

```rust,no_run
#[macro_use]
extern crate clap;

use clap::{Parser, Subcommand};
use ovation::{CommandContext, CommandDelegate, CommandSet};

#[derive(Subcommand)]
enum ExampleSet {
    A,
    B { inner: u8, },
    C { inner: String, },
}

#[derive(Parser)]
#[command(about, version)]
struct ExampleArgs {
    #[command(subcommand)]
    subcommands: ExampleSet,
}

fn do_a(ctx: &ExampleArgs, set: &ExampleSet) -> Result<String, ()> {
    Ok("hello".into())
}

fn do_b_or_c(ctx: &ExampleArgs, set: &ExampleSet) -> Result<String, ()> {
    // matching again is the only way to receive nested fields until (if ever)
    // clap supports deriving subcommand on variants with unnamed fields.
    match set {
        ExampleSet::B{ inner } => Ok(inner.to_string()),
        ExampleSet::C{ inner } => Ok(inner.to_string()),
        _ => unreachable!()
    }
}

impl CommandSet<ExampleArgs> for ExampleSet {
    type ReturnType = String;
    type ErrorType = ();

    fn dispatch<'a>(&self) -> &'a dyn CommandDelegate<ExampleArgs> {
        match self {
            ExampleSet::A => &do_a,
            ExampleSet::B{ .. } | ExampleSet::C{ .. } => &do_b_or_c,
        }
    }
}

impl CommandContext for ExampleArgs {
    // for "slash-commands" outside of command-line contexts.
    const PREFIX: &'static str = "/";

    type Commands = ExampleSet;

    fn commands(&self) -> &ExampleSet {
        &self.subcommands
    }
}

fn main() -> Result<(), ovation::MixedError<ExampleArgs>> {
    let result: String = ExampleArgs::execute()?;
    // or
    let result: String = ExampleArgs::execute_from([crate_name!(), "a"])?;

    println!("{result}");
    Ok(())
}
```

