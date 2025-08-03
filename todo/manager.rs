pub struct TodoManager {
    todos: Vec<TodoItem>,
    file_path: String,
    next_id: i32,
}

impl TodoManager {
    fn new(file_path: String) -> TodoResult<Self> {
        let todos = Self::load_todos(&file_path)?;
        let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        Ok(TodoManager {
            todos,
            file_path,
            next_id,
        })
    }

    fn load_todos(file_path: &str) -> TodoResult<Vec<TodoItem>> {
        if !Path::new(file_path).exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(file_path)
            .map_err(|e| TodoError::FileError(format!("Failed to read {file_path}: {e}")))?;

        serde_json::from_str(&content)
            .map_err(|e| TodoError::FileError(format!("Invalid JSON in {file_path}: {e}")))
    }

    fn save(&self) -> TodoResult<()> {
        let content =
            serde_json::to_string(&self.todos).map_err(|_| TodoError::SerializationError)?;
        std::fs::write(&self.file_path, content)
            .map_err(|e| TodoError::FileError(e.to_string()))?;
        Ok(())
    }

    fn validate_todo_text(text: &str) -> TodoResult<String> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Err(TodoError::EmptyTodo);
        }
        if trimmed.len() > 500 {
            return Err(TodoError::TodoTooLong);
        }
        Ok(trimmed.to_string())
    }

    fn add_todo(
        &mut self,
        text: &str,
        due: Option<&str>,
        priority: Option<&str>,
    ) -> TodoResult<()> {
        let text = Self::validate_todo_text(text)?;
        let parsed_due = Self::parse_due_date(due)?;
        let parsed_priority = Self::parse_priority(priority)?;
        let next_id = self.next_id;
        self.next_id = next_id + 1;

        let todo = TodoItem {
            id: next_id,
            todo: text.to_string(),
            status: false,
            due: parsed_due,
            priority: parsed_priority,
        };

        self.todos.push(todo);

        Ok(())
    }

    fn parse_due_date(due_str: Option<&str>) -> TodoResult<Option<NaiveDateTime>> {
        match due_str {
            Some(date_str) if !date_str.trim().is_empty() => {
                let formats = DATE_FORMAT;

                for format in &formats {
                    if let Ok(dt) = NaiveDateTime::parse_from_str(date_str, format) {
                        return Ok(Some(dt));
                    }
                }
                Err(TodoError::InvalidDateFormat)
            }
            _ => Ok(None),
        }
    }

    fn parse_priority(priority_str: Option<&str>) -> TodoResult<Option<Priority>> {
        match priority_str {
            Some(p) => Ok(Some(p.parse().map_err(|_| TodoError::InvalidPriority)?)),
            None => Ok(None),
        }
    }

    fn edit_todo(
        &mut self,
        id: i32,
        new_text: &str,
        due: Option<&str>,
        priority: Option<&str>,
    ) -> TodoResult<()> {
        let new_text = Self::validate_todo_text(new_text)?;

        let todo = self.find_todo_mut(id)?;
        todo.todo = new_text.to_string();

        if let Some(d) = due {
            todo.due = Self::parse_due_date(Some(d))?;
        }
        if let Some(p) = priority {
            todo.priority = Self::parse_priority(Some(p))?;
        }
        Ok(())
    }

    fn find_todo_mut(&mut self, id: i32) -> TodoResult<&mut TodoItem> {
        self.todos
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TodoError::TodoNotFound(id))
    }

    fn toggle_todo(&mut self, id: i32) -> TodoResult<()> {
        let todo = self.find_todo_mut(id)?;
        todo.status = !todo.status;
        Ok(())
    }

    fn delete_todo(&mut self, id: i32) -> TodoResult<()> {
        let original_len = self.todos.len();
        self.todos.retain(|t| t.id != id);

        if self.todos.len() < original_len {
            Ok(())
        } else {
            Err(TodoError::TodoNotFound(id))
        }
    }

    fn clear_all(&mut self) -> usize {
        let count = self.todos.len();
        self.todos.clear();
        count
    }
}
