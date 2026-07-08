use crate::{CommandContext, CommandSet};

/// The [`Ok`] type returned by a [`CommandSet`]'s
/// [`CommandDelegate`](crate::CommandDelegate)s, using a [`CommandContext`].
pub type CommandReturn<Ctx: CommandContext> = <Ctx::Commands as CommandSet<Ctx>>::ReturnType;

/// The [`Err`] type returned by a [`CommandSet`]'s
/// [`CommandDelegate`](crate::CommandDelegate)s, using a [`CommandContext`].
pub type CommandError<Ctx: CommandContext> = <Ctx::Commands as CommandSet<Ctx>>::ErrorType;

/// The [`Result`] returned by a [`CommandSet`]'s
/// [`CommandDelegate`](crate::CommandDelegate)s, using a [`CommandContext`].
pub type CommandResult<Ctx: CommandContext> = Result<CommandReturn<Ctx>, CommandError<Ctx>>;

pub enum OvationError<Ctx: crate::CommandContext> {
    ClapTerminal(clap::Error),
    ClapError(clap::Error),
    CommandError(<Ctx::Commands as CommandSet<Ctx>>::ErrorType),
}

pub type OvationResult<Ctx: crate::CommandContext> = Result<CommandReturn<Ctx>, OvationError<Ctx>>;

impl<Ctx: CommandContext> From<clap::Error> for OvationError<Ctx> {
    fn from(err: clap::Error) -> Self {
        use clap::error::ErrorKind;

        match err.kind() {
            ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => Self::ClapTerminal(err),
            _ => Self::ClapError(err),
        }
    }
}

impl<Ctx: CommandContext> std::fmt::Display for OvationError<Ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OvationError::*;

        match self {
            ClapTerminal(err) | ClapError(err) => err.fmt(f),
            CommandError(err) => write!(f, "{err:?}"),
        }
    }
}

impl<Ctx: CommandContext> std::fmt::Debug for OvationError<Ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OvationError::*;

        match self {
            ClapTerminal(err) | ClapError(err) => std::fmt::Debug::fmt(err, f),
            CommandError(err) => std::fmt::Debug::fmt(err, f),
        }
    }
}
