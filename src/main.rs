mod cli;
mod tasks;

use anyhow::anyhow;
use cli::{CommandLineArgs, Action};
use tasks::{Task};
use structopt::StructOpt;
use std::{path::PathBuf};

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    cli::CommandLineArgs::from_args();

    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    match action {
        Action::Add { text } => tasks::add_task(journal_file, Task::new(text)),
        Action::List => tasks::list_tasks(journal_file),
        Action::Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())
}
