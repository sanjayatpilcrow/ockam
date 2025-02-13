use clap::{Args, Subcommand};

pub use create::CreateCommand;
pub use delete::DeleteCommand;
pub use list::ListCommand;
pub use show::ShowCommand;

use crate::{CommandGlobalOpts, HELP_TEMPLATE};

mod create;
mod delete;
mod list;
mod show;

#[derive(Clone, Debug, Args)]
pub struct ProjectCommand {
    #[clap(subcommand)]
    subcommand: ProjectSubcommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ProjectSubcommand {
    /// Create projects
    #[clap(display_order = 900, help_template = HELP_TEMPLATE)]
    Create(CreateCommand),

    /// Delete projects
    #[clap(display_order = 900, help_template = HELP_TEMPLATE)]
    Delete(DeleteCommand),

    /// List projects
    #[clap(display_order = 900, help_template = HELP_TEMPLATE)]
    List(ListCommand),

    /// Show projects
    #[clap(display_order = 900, help_template = HELP_TEMPLATE)]
    Show(ShowCommand),
}

impl ProjectCommand {
    pub fn run(opts: CommandGlobalOpts, cmd: ProjectCommand) {
        match cmd.subcommand {
            ProjectSubcommand::Create(scmd) => CreateCommand::run(opts, scmd),
            ProjectSubcommand::Delete(scmd) => DeleteCommand::run(opts, scmd),
            ProjectSubcommand::List(scmd) => ListCommand::run(opts, scmd),
            ProjectSubcommand::Show(scmd) => ShowCommand::run(opts, scmd),
        }
    }
}
