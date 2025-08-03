pub fn validate_todo_text(text: &str) -> TodoResult<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(TodoError::EmptyTodo);
    }
    if trimmed.len() > MAX_TODO_LENGTH {
        return Err(TodoError::TodoTooLong);
    }
    Ok(trimmed.to_string())
}
