use crate::todo::{TodoError, TodoResult};

pub fn validate_text(text: &str, len: usize) -> TodoResult<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(TodoError::EmptyTodo);
    }
    if trimmed.len() > len {
        return Err(TodoError::TodoTooLong {
            actual: trimmed.len(),
            max: len,
        });
    }
    Ok(trimmed.to_string())
}

pub fn validate_id(id_str: &str) -> TodoResult<u32> {
    if id_str.trim().is_empty() {
        return Err(TodoError::InvalidId {
            id: "empty".to_string(),
        });
    }
    if id_str == "0" {
        return Err(TodoError::InvalidId {
            id: "zero".to_string(),
        });
    }
    id_str
        .trim()
        .parse::<u32>()
        .map_err(|_| TodoError::InvalidId {
            id: id_str.to_string(),
        })
}
