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
