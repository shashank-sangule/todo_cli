pub type TodoResult<T> = Result<T, TodoError>;

#[derive(Debug)]
pub enum TodoError {
    InvalidDateFormat,
    InvalidPriority,
    TodoNotFound(i32),
    FileError(String),
    EmptyTodo,
    SerializationError,
    InvalidSortField,
    TodoTooLong,
}

impl std::fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TodoError::InvalidDateFormat => {
                write!(f, "❌ Invalid date format. Use: dd-mm-YYYY HH:MM")
            }
            TodoError::InvalidPriority => write!(f, "❌ Invalid priority. Use: high, medium, low"),
            TodoError::FileError(msg) => write!(f, "❌ File error: {msg}"),
            TodoError::TodoNotFound(id) => write!(f, "❌ Todo with ID {id} not found"),
            TodoError::EmptyTodo => write!(f, "❌ Todo cannot be empty"),
            TodoError::SerializationError => write!(f, "❌ Failed to save/load todos"),
            TodoError::InvalidSortField => {
                write!(f, "❌ Invalid sort field: Use: due, priority, due+priority")
            }

            TodoError::TodoTooLong => write!(f, "❌ Todo cannot be more than 500 characters"),
        }
    }
}

impl std::error::Error for TodoError {}
