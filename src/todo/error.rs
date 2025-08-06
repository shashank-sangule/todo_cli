use thiserror::Error;

pub type TodoResult<T> = Result<T, TodoError>;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("❌ Invalid date format: '{input}'. Use: dd-mm-YYYY HH:MM or natural language like 'tomorrow'")]
    InvalidDateFormat { input: String },
    #[error("❌ Invalid priority: '{input}'. Use: high, medium, low (or h, m, l)")]
    InvalidPriority { input: String },
    #[error("❌ Todo with ID {id} not found. Use 'todo list' to see available todos")]
    TodoNotFound { id: u32 },
    #[error("❌ Failed to {operation} file '{path}'")]
    FileError {
        operation: String,
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("❌ Todo cannot be empty")]
    EmptyTodo,
    #[error("❌ Failed to save/load todos")]
    SerializationError(#[from] serde_json::Error),
    #[error("❌ Invalid sort field: '{field}'. Available: due, priority, due+priority")]
    InvalidSortField { field: String },
    #[error("❌ Todo too long: {actual} characters (max: {max})")]
    TodoTooLong { actual: usize, max: usize },
    #[error("❌ Invalid ID: '{id}'. Please provide a valid number")]
    InvalidId { id: String },
    #[error("❌ Date is in the past: {date}")]
    PastDate { date: String },
    #[error("❌ Invalid date: {input}. Reason: {reason}")]
    InvalidDate { input: String, reason: String },
}

impl From<std::io::Error> for TodoError {
    fn from(err: std::io::Error) -> Self {
        TodoError::FileError {
            operation: "access".to_string(),
            path: "unknown".to_string(),
            source: err,
        }
    }
}
