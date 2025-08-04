pub mod cli;
pub mod display;
pub mod todo;
pub mod utils;

pub use cli::Cli;
pub use display::{display_todos, format_due_date, format_status, truncate_text};
pub use todo::{TodoItem, TodoManager};
