use clap::Parser;
use todo_app::cli::{handle_commands, Cli};
use todo_app::todo::TodoResult;
use todo_app::TodoManager;

fn main() -> TodoResult<()> {
    let cli = Cli::parse();
    let mut manager = TodoManager::new(cli.file)?;

    handle_commands(cli.command, &mut manager)?;

    manager.save()?;
    Ok(())
}
