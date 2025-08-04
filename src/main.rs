use clap::Parser;
use todo_app::TodoManager;
use todo_app::cli::Cli;
use todo_app::todo::TodoResult;

fn main() -> TodoResult<()> {
    let cli = Cli::parse();
    let manager = TodoManager::new(cli.file)?;

    manager.save()?;
    Ok(())
}
