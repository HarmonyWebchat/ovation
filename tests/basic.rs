use clap::{Parser, Subcommand};
use ovation::err::OvationError;
use ovation::{CommandContext, CommandDelegate, CommandSet};

#[derive(Subcommand)]
enum TestSet {
    Ping,
    PassthruA { value: u8 },
    PassthruB { value: f32 },
    IntentionalError,
}

#[derive(Parser)]
#[command(version, about)]
struct TestArgs {
    #[command(subcommand)]
    subcommands: TestSet,
}

fn ping(_: &TestArgs, _: &TestSet) -> Result<String, ()> {
    Ok("pong".into())
}

fn passthru(_: &TestArgs, set: &TestSet) -> Result<String, ()> {
    match set {
        TestSet::PassthruA { value } => Ok(value.to_string()),
        TestSet::PassthruB { value } => Ok(value.to_string()),
        _ => unreachable!(),
    }
}

impl CommandSet<TestArgs> for TestSet {
    type ReturnType = String;
    type ErrorType = ();

    fn dispatch<'a>(&self) -> &'a dyn CommandDelegate<TestArgs> {
        match self {
            TestSet::Ping => &ping,
            TestSet::PassthruA { .. } | TestSet::PassthruB { .. } => &passthru,
            TestSet::IntentionalError => &|_, _| Err(()),
        }
    }
}

impl CommandContext for TestArgs {
    type Commands = TestSet;

    fn commands(&self) -> &Self::Commands {
        &self.subcommands
    }
}

// === HELPERS ===

macro_rules! assert_ok {
    ( $ctx:ident [ $( $args:literal ),+ ] if $bind:ident @ ( $guard:expr ) ) => {
        assert!(matches!( $ctx::execute_from([ $( $args ),* ]), Ok($bind) if $guard ))
    };

    ( $ctx:ident [ $( $args:literal ),+ ] ) => {
        assert!( $ctx::execute_from([ $( $args ),* ]).is_ok())
    };
}

macro_rules! assert_clap_err {
    ( $ctx:ident [ $( $args:literal ),+ ] ) => {
        assert!(
            matches!(
                $ctx::execute_from([ $( $args ),* ]),
                Err(OvationError::ClapError(_))
            )
        )
    };
}

macro_rules! assert_terminal {
    ( $ctx:ident [ $( $args:literal ),+ ] ) => {
        assert!(
            matches!(
                $ctx::execute_from([ $( $args ),* ]),
                Err(OvationError::ClapTerminal(_))
            )
        )
    };
}

macro_rules! assert_command_err {
    ( $ctx:ident [ $( $args:literal ),+ ] ) => {
        assert!(
            matches!(
                $ctx::execute_from([ $( $args ),* ]),
                Err(OvationError::CommandError(_))
            )
        )
    };
}

// === INTERNAL COHERENCE TESTS ===

#[test]
fn test_terminals() {
    assert_terminal!(TestArgs["cargo-test-basic", "--help"]);
    assert_terminal!(TestArgs["cargo-test-basic", "--version"]);
}

// === ACTUAL TESTS ===

#[test]
fn test_ping() {
    assert_ok!(TestArgs["cargo-test-basic", "ping"] if s@(s == "pong"));
}

#[test]
fn test_passthru_a() {
    assert_clap_err!(TestArgs["cargo-test-basic", "passthru-a", "--", "3.14"]);
    assert_clap_err!(TestArgs["cargo-test-basic", "passthru-a", "--", "25565"]);
    assert_clap_err!(TestArgs["cargo-test-basic", "passthru-a", "--", "-1"]);

    assert_ok!(TestArgs["cargo-test-basic", "passthru-a", "--", "42"] if s @(s == "42"));
}

#[test]
fn test_passthru_b() {
    assert_ok!(TestArgs["cargo-test-basic", "passthru-b", "--", "3.14" ] if s@(s == "3.14"));
    assert_ok!(TestArgs["cargo-test-basic", "passthru-b", "--", "25565"] if s@(s == "25565"));
    assert_ok!(TestArgs["cargo-test-basic", "passthru-b", "--", "-1"   ] if s@(s == "-1"));
    assert_ok!(TestArgs["cargo-test-basic", "passthru-b", "--", "42"   ] if s@(s == "42"));
}

#[test]
fn test_intentional_err() {
    assert_command_err!(TestArgs["cargo-test-basic", "intentional-error"]);
}
