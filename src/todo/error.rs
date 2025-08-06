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

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io;

    #[test]
    fn test_todo_not_found_error() {
        let error = TodoError::TodoNotFound { id: 42 };
        assert_eq!(
            error.to_string(),
            "❌ Todo with ID 42 not found. Use 'todo list' to see available todos"
        );
    }

    #[test]
    fn test_invalid_id_error() {
        let error = TodoError::InvalidId {
            id: "abc".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "❌ Invalid ID: 'abc'. Please provide a valid number"
        );
    }

    #[test]
    fn test_empty_todo_error() {
        let error = TodoError::EmptyTodo;
        assert_eq!(error.to_string(), "❌ Todo cannot be empty");
    }

    #[test]
    fn test_file_error() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let error = TodoError::FileError {
            operation: "read".to_string(),
            path: "/path/to/todos.json".to_string(),
            source: io_error,
        };
        assert_eq!(
            error.to_string(),
            "❌ Failed to read file '/path/to/todos.json'"
        );
        assert!(error.source().is_some());
    }

    #[test]
    fn test_serialization_error() {
        let json_error = serde_json::from_str::<serde_json::Value>("{invalid json}").unwrap_err();
        let error = TodoError::SerializationError(json_error);
        assert_eq!(error.to_string(), "❌ Failed to save/load todos");
        assert!(error.source().is_some());
    }

    #[test]
    fn test_invalid_date_format_error() {
        let error = TodoError::InvalidDateFormat {
            input: "32-13-2023".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "❌ Invalid date format: '32-13-2023'. Use: dd-mm-YYYY HH:MM or natural language like 'tomorrow'"
        );
    }

    #[test]
    fn test_todo_too_long_error() {
        let error = TodoError::TodoTooLong {
            actual: 150,
            max: 100,
        };
        assert_eq!(
            error.to_string(),
            "❌ Todo too long: 150 characters (max: 100)"
        );
    }

    #[test]
    fn test_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
        let todo_error: TodoError = io_error.into();

        match todo_error {
            TodoError::FileError {
                operation,
                path,
                source,
            } => {
                assert_eq!(operation, "access");
                assert_eq!(path, "unknown");
                assert_eq!(source.to_string(), "Access denied");
            }
            _ => panic!("Expected FileError variant"),
        }
    }

    #[test]
    fn test_from_serde_json_error() {
        let json_error = serde_json::from_str::<serde_json::Value>("[1,2,").unwrap_err();
        let todo_error: TodoError = json_error.into();

        match todo_error {
            TodoError::SerializationError(_) => {
                // Test passes if we get the right variant
            }
            _ => panic!("Expected SerializationError variant"),
        }
    }

    #[test]
    fn test_todo_result_type() {
        let ok_result: TodoResult<String> = Ok("success".to_string());
        let err_result: TodoResult<String> = Err(TodoError::EmptyTodo);

        assert!(ok_result.is_ok());
        assert!(err_result.is_err());
    }
}
