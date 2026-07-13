#![allow(type_alias_bounds)]
#![feature(associated_type_defaults)]
#![doc = include_str!("../README.md")]

pub mod err;

use clap::{Parser, Subcommand};
pub use ovation_derive::{CommandContext, CommandSet};

use std::{ffi::OsString, fmt::Debug};

use err::*;

pub trait CommandDelegate<Ctx: CommandContext>:
    Fn(&Ctx, &Ctx::Commands) -> CommandResult<Ctx>
{
}

impl<Ctx, F> CommandDelegate<Ctx> for F
where
    Ctx: CommandContext,
    F: Fn(&Ctx, &Ctx::Commands) -> CommandResult<Ctx>,
{
}

/// A trait that gets applied to enums that implement [`Subcommand`],
/// where [`CommandSet<Ctx>`]'s `Ctx` is a [`CommandContext`] struct
/// (which is a supertrait over [`Parser`]).
///
/// This allows for an implementation of [`CommandSet`] for each/any wrapper
/// struct implementing [`CommandContext`], as long as that struct
/// specifically wraps `Self`.
///
/// For example:
/// ```
/// # use ovation::{CommandContext, CommandDelegate, CommandSet};
/// #[derive(clap::Subcommand)]
/// enum ExampleSubcommands {}
///
/// #[derive(clap::Parser)]
/// struct ExampleCommandArgs {
///     #[command(subcommand)]
///     subcommands: ExampleSubcommands,
/// }
///
/// #[derive(clap::Parser)]
/// struct ExampleCommandArgs2 {
///     #[command(subcommand)]
///     subcommands: ExampleSubcommands,
/// }
///
/// impl CommandSet<ExampleCommandArgs> for ExampleSubcommands {
///     type ReturnType = u8;
///     type ErrorType = ();
/// #    fn dispatch<'a>(&self) -> &'a dyn CommandDelegate<ExampleCommandArgs> { todo!() }
///     // ...
/// }
///
/// impl CommandSet<ExampleCommandArgs2> for ExampleSubcommands {
///     type ReturnType = f32;
///     type ErrorType = ();
/// #    fn dispatch<'a>(&self) -> &'a dyn CommandDelegate<ExampleCommandArgs2> { todo!() }
///     // ...
/// }
///
/// impl CommandContext for ExampleCommandArgs {
///     type Commands = ExampleSubcommands;
///     // ...
/// #    fn commands(&self) -> &ExampleSubcommands { todo!() }
/// }
///
/// impl CommandContext for ExampleCommandArgs2 {
///     type Commands = ExampleSubcommands;
///     // ...
/// #    fn commands(&self) -> &ExampleSubcommands { todo!() }
/// }
/// ```
pub trait CommandSet<Ctx: CommandContext<Commands = Self>>: Subcommand {
    /// The return type shared by any and all delegates produced by
    /// [`Self::dispatch()`](CommandSet::dispatch). This type is specific to
    /// each implementation, and is intended to correspond directly with the
    /// implementation's wrapping [`Ctx`](CommandContext) type. That is, each
    /// wrapping [`CommandContext`] can have its own return type.
    ///
    /// Defaults to `()`.
    type ReturnType = ();

    /// The error type shared by any and all delegates produced by
    /// [`Self::dispatch()`](CommandSet::dispatch). This type is specific to
    /// each implementation, and is intended to correspond directly with the
    /// implementation's wrapping [`Ctx`](CommandContext) type. That is, each
    /// wrapping [`CommandContext`] can have its own error type.
    ///
    /// Defaults to `()`.
    type ErrorType: Debug = ();

    /// [`CommandSet`] is intended to be implemented only on enums, so this
    /// function would be returning a [`CommandDelegate<Ctx>`] corresponding to
    /// the enum's variants.
    fn dispatch<'a>(&self) -> &'a dyn CommandDelegate<Ctx>;

    /// A convenience function over [`Self::dispatch()`](CommandSet::dispatch).
    fn call_delegate(&self, ctx: &Ctx) -> CommandResult<Ctx> {
        self.dispatch()(ctx, self)
    }
}

fn context_from<Ctx: CommandContext>(result: Result<Ctx, clap::Error>) -> OvationResult<Ctx> {
    let ctx = result.map_err(OvationError::from)?;

    ctx.commands().call_delegate(&ctx).map_err(OvationError::CommandError)
}

pub trait CommandContext: Parser {
    type Commands: CommandSet<Self>;

    fn commands(&self) -> &Self::Commands;

    fn execute_from<I, T>(args: I) -> OvationResult<Self>
    where
        T: Into<OsString> + Clone,
        I: IntoIterator<Item = T>,
    {
        context_from(Self::try_parse_from(args))
    }

    fn execute() -> OvationResult<Self> {
        context_from(Self::try_parse())
    }
}
