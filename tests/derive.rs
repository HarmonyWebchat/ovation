#[macro_use]
mod helpers;

use clap::{Parser, Subcommand};
use ovation::{CommandContext, CommandDelegate, CommandSet};

fn pong_a(_: &TestArgsA, _: &TestSet) -> Result<String, ()> {
    Ok("pong_a".into())
}

fn pong_b(_: &TestArgsB, _: &TestSet) -> Result<u8, ()> {
    Ok(42)
}

#[derive(CommandSet, Subcommand)]
#[contexts(
    impl(for = TestArgsA, return = String, error = ()),
    impl(for = TestArgsB, return = u8, error = ())
)]
enum TestSet {
    #[delegates(TestArgsA = pong_a, TestArgsB = pong_b)]
    Ping,
}

#[derive(CommandContext, Parser)]
#[set(TestSet)]
struct TestArgsA {
    #[command(subcommand)]
    subcommands: TestSet,
}

#[derive(CommandContext, Parser)]
#[set(TestSet)]
struct TestArgsB {
    #[command(subcommand)]
    subcommands: TestSet,
}

