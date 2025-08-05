use crate::todo::{Priority, TodoError, TodoItem, TodoResult};
use crate::utils::validation::validate_id;
use crate::utils::{parse_due_date, validate_text};
use std::path::Path;

pub struct TodoManager {
    pub todos: Vec<TodoItem>,
    pub file_path: String,
    pub next_id: u32,
}

impl TodoManager {
    pub fn new(file_path: String) -> TodoResult<Self> {
        let todos = Self::load_todos(&file_path)?;
        let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        Ok(TodoManager {
            todos,
            file_path,
            next_id,
        })
    }

    pub fn load_todos(file_path: &str) -> TodoResult<Vec<TodoItem>> {
        if !Path::new(file_path).exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(file_path).map_err(|e| TodoError::FileError {
            operation: "read".to_string(),
            path: file_path.to_string(),
            source: e,
        })?;

        serde_json::from_str(&content).map_err(TodoError::SerializationError)
    }

    pub fn save(&self) -> TodoResult<()> {
        let content = serde_json::to_string(&self.todos)?;
        std::fs::write(&self.file_path, content).map_err(|e| TodoError::FileError {
            operation: "write".to_string(),
            path: self.file_path.clone(),
            source: e,
        })?;
        Ok(())
    }

    pub fn add_todo(
        &mut self,
        text: &str,
        due: Option<&str>,
        priority: Option<&str>,
    ) -> TodoResult<()> {
        let text = validate_text(text, 500)?;
        let parsed_due = parse_due_date(due)?;
        let parsed_priority = Self::parse_priority(priority)?;
        let next_id = self.next_id;
        self.next_id = next_id + 1;

        let todo = TodoItem {
            id: next_id,
            todo: text,
            status: false,
            due: parsed_due,
            priority: parsed_priority,
        };

        self.todos.push(todo);
        self.save()?;
        println!("✅ Todo added with ID: {next_id}");

        Ok(())
    }

    pub fn parse_priority(priority_str: Option<&str>) -> TodoResult<Option<Priority>> {
        match priority_str {
            Some(p) => Ok(Some(p.parse().map_err(|_| TodoError::InvalidPriority {
                input: p.to_string(),
            })?)),
            None => Ok(None),
        }
    }

    pub fn edit_todo(
        &mut self,
        id: u32,
        new_text: &str,
        due: Option<&str>,
        priority: Option<&str>,
    ) -> TodoResult<()> {
        let new_text = validate_text(new_text, 500)?;

        let todo = self.find_todo_mut(id)?;
        todo.todo = new_text.to_string();

        if let Some(d) = due {
            todo.due = parse_due_date(Some(d))?;
        }
        if let Some(p) = priority {
            todo.priority = Self::parse_priority(Some(p))?;
        }
        self.save()?;
        println!("✏️ Todo {id} edited!");
        Ok(())
    }

    pub fn find_todo_mut(&mut self, id: u32) -> TodoResult<&mut TodoItem> {
        validate_id(&id.to_string())?;
        self.todos
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TodoError::TodoNotFound { id })
    }

    pub fn toggle_todo(&mut self, id: u32) -> TodoResult<()> {
        let todo = self.find_todo_mut(id)?;
        todo.status = !todo.status;
        self.save()?;
        println!("🔄 Status toggled for todo {id}!");
        Ok(())
    }

    pub fn delete_todo(&mut self, id: u32) -> TodoResult<()> {
        let original_len = self.todos.len();
        self.todos.retain(|t| t.id != id);

        if self.todos.len() < original_len {
            self.save()?;
            println!("🗑️ Todo {id} deleted!");
            Ok(())
        } else {
            Err(TodoError::TodoNotFound { id })
        }
    }

    pub fn clear_all(&mut self) -> usize {
        let count = self.todos.len();
        self.todos.clear();
        if let Err(e) = self.save() {
            eprintln!("❌ Failed to clear todos: {e}");
        } else {
            println!("🗑️ Cleared {count} todo(s)!");
        }
        count
    }
}
