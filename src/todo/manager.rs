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
        let next_id = todos.iter().map(|t| t.id()).max().unwrap_or(0) + 1;
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

        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

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
        title: String,
        description: Option<String>,
        due_date: Option<String>,
        priority: Option<&str>,
        tags: Option<Vec<String>>,
    ) -> TodoResult<()> {
        let title = validate_text(title.as_str(), 140)?;
        let description = description
            .map(|d| validate_text(d.as_str(), 1000))
            .transpose()?;
        let parsed_due = parse_due_date(due_date.as_deref())?;
        let parsed_priority = Self::parse_priority(priority)?;
        let next_id = self.next_id;
        self.next_id = next_id + 1;

        let todo = TodoItem::new(
            next_id,
            title,
            description,
            false,
            parsed_due,
            parsed_priority,
            tags,
        );

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
        new_title: Option<String>,
        new_description: Option<String>,
        due: Option<&str>,
        priority: Option<&str>,
        tags: Option<Vec<String>>,
    ) -> TodoResult<()> {
        let todo = self.find_todo_mut(id)?;

        if let Some(text) = new_title {
            todo.set_title(validate_text(&text, 140)?);
        }
        if let Some(description) = new_description {
            todo.set_description(Some(validate_text(&description, 1000)?));
        }
        if let Some(due_date) = due {
            todo.set_due_date(parse_due_date(Some(due_date))?);
        }
        if let Some(priority_str) = priority {
            todo.set_priority(Self::parse_priority(Some(priority_str))?);
        }
        if let Some(tags) = tags {
            todo.set_tags(Some(tags));
        }

        self.save()?;
        println!("✏️ Todo {id} edited!");
        Ok(())
    }

    pub fn find_todo_mut(&mut self, id: u32) -> TodoResult<&mut TodoItem> {
        validate_id(&id.to_string())?;
        self.todos
            .iter_mut()
            .find(|t| t.id() == id)
            .ok_or(TodoError::TodoNotFound { id })
    }

    pub fn toggle_todo(&mut self, id: u32) -> TodoResult<()> {
        let todo = self.find_todo_mut(id)?;
        todo.set_completed(!todo.completed());

        if todo.completed() {
            println!("✅ Todo {id} marked as completed!");
        } else {
            println!("✅ Todo {id} marked as pending!");
        }

        self.save()?;
        Ok(())
    }

    pub fn delete_todo(&mut self, id: u32) -> TodoResult<()> {
        let original_len = self.todos.len();
        self.todos.retain(|t| t.id() != id);

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    fn create_test_file() -> NamedTempFile {
        NamedTempFile::new().expect("Failed to create temp file")
    }

    fn create_test_todos_json() -> String {
        r#"[
            {
                "id": 1,
                "title": "Test Todo",
                "description": "Test description",
                "completed": false,
                "due_date": "2024-12-25T14:30:00",
                "priority": "High",
                "tags": ["work"],
                "created_at": "2024-01-01T10:00:00",
                "updated_at": "2024-01-01T10:00:00"
            }
        ]"#
        .to_string()
    }

    #[test]
    fn test_new_with_nonexistent_file() {
        let temp_file = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();

        // Delete the file so it doesn't exist
        fs::remove_file(file_path).ok();

        let manager = TodoManager::new(file_path.to_string()).unwrap();

        assert_eq!(manager.todos.len(), 0);
        assert_eq!(manager.next_id, 1);
        assert_eq!(manager.file_path, file_path);
    }

    #[test]
    fn test_new_with_existing_file() {
        let temp_file = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();

        fs::write(file_path, create_test_todos_json()).unwrap();

        let manager = TodoManager::new(file_path.to_string()).unwrap();

        assert_eq!(manager.todos.len(), 1);
        assert_eq!(manager.next_id, 2); // max id + 1
        assert_eq!(manager.todos[0].id(), 1);
        assert_eq!(manager.todos[0].title(), "Test Todo");
    }

    #[test]
    fn test_load_todos_empty_file() {
        let temp_file = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();

        fs::write(file_path, "").unwrap();

        let todos = TodoManager::load_todos(file_path).unwrap();
        assert_eq!(todos.len(), 0);
    }

    #[test]
    fn test_add_todo() {
        let temp_file = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        fs::remove_file(file_path).ok();

        let mut manager = TodoManager::new(file_path.to_string()).unwrap();

        manager
            .add_todo(
                "New Task".to_string(),
                Some("Task description".to_string()),
                None,
                Some("high"),
                Some(vec!["work".to_string()]),
            )
            .unwrap();

        assert_eq!(manager.todos.len(), 1);
        assert_eq!(manager.next_id, 2);
        assert_eq!(manager.todos[0].title(), "New Task");
        assert_eq!(manager.todos[0].description(), Some("Task description"));
        assert_eq!(manager.todos[0].priority(), Some(Priority::High));
        assert_eq!(
            manager.todos[0].tags(),
            Some(["work".to_string()].as_slice())
        );

        // Verify file was saved
        assert!(Path::new(file_path).exists());
    }

    #[test]
    fn test_add_todo_validation_error() {
        let temp_file = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        fs::remove_file(file_path).ok();

        let mut manager = TodoManager::new(file_path.to_string()).unwrap();

        // Test title too long
        let long_title = "a".repeat(150);
        let result = manager.add_todo(long_title, None, None, None, None);

        assert!(result.is_err());
        assert_eq!(manager.todos.len(), 0); // Should not add invalid todo
    }

    #[test]
    fn test_edit_todo() {
        let temp_file = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        fs::write(file_path, create_test_todos_json()).unwrap();

        let mut manager = TodoManager::new(file_path.to_string()).unwrap();

        manager
            .edit_todo(
                1,
                Some("Updated Title".to_string()),
                Some("Updated description".to_string()),
                None,
                Some("low"),
                Some(vec!["personal".to_string()]),
            )
            .unwrap();

        let todo = &manager.todos[0];
        assert_eq!(todo.title(), "Updated Title");
        assert_eq!(todo.description(), Some("Updated description"));
        assert_eq!(todo.priority(), Some(Priority::Low));
        assert_eq!(todo.tags(), Some(["personal".to_string()].as_slice()));
    }

    #[test]
    fn test_edit_nonexistent_todo() {
        let temp_file = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        fs::remove_file(file_path).ok();

        let mut manager = TodoManager::new(file_path.to_string()).unwrap();

        let result = manager.edit_todo(999, Some("Title".to_string()), None, None, None, None);

        assert!(result.is_err());
        if let Err(TodoError::TodoNotFound { id }) = result {
            assert_eq!(id, 999);
        } else {
            panic!("Expected TodoNotFound error");
        }
    }

    #[test]
    fn test_toggle_todo() {
        let temp_file = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        fs::write(file_path, create_test_todos_json()).unwrap();

        let mut manager = TodoManager::new(file_path.to_string()).unwrap();

        // Initially not completed
        assert!(!manager.todos[0].completed());

        manager.toggle_todo(1).unwrap();
        assert!(manager.todos[0].completed());

        manager.toggle_todo(1).unwrap();
        assert!(!manager.todos[0].completed());
    }

    #[test]
    fn test_delete_todo() {
        let temp_file = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        fs::write(file_path, create_test_todos_json()).unwrap();

        let mut manager = TodoManager::new(file_path.to_string()).unwrap();

        assert_eq!(manager.todos.len(), 1);

        manager.delete_todo(1).unwrap();

        assert_eq!(manager.todos.len(), 0);
    }

    #[test]
    fn test_delete_nonexistent_todo() {
        let temp_file = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        fs::remove_file(file_path).ok();

        let mut manager = TodoManager::new(file_path.to_string()).unwrap();

        let result = manager.delete_todo(999);

        assert!(result.is_err());
        if let Err(TodoError::TodoNotFound { id }) = result {
            assert_eq!(id, 999);
        } else {
            panic!("Expected TodoNotFound error");
        }
    }

    #[test]
    fn test_clear_all() {
        let temp_file = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        fs::write(file_path, create_test_todos_json()).unwrap();

        let mut manager = TodoManager::new(file_path.to_string()).unwrap();

        assert_eq!(manager.todos.len(), 1);

        let count = manager.clear_all();

        assert_eq!(count, 1);
        assert_eq!(manager.todos.len(), 0);
    }

    #[test]
    fn test_parse_priority() {
        assert_eq!(
            TodoManager::parse_priority(Some("high")).unwrap(),
            Some(Priority::High)
        );
        assert_eq!(
            TodoManager::parse_priority(Some("medium")).unwrap(),
            Some(Priority::Medium)
        );
        assert_eq!(
            TodoManager::parse_priority(Some("low")).unwrap(),
            Some(Priority::Low)
        );
        assert_eq!(TodoManager::parse_priority(None).unwrap(), None);

        let result = TodoManager::parse_priority(Some("invalid"));
        assert!(result.is_err());
    }
}
