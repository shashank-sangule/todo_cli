use crate::cli::Commands;
use crate::display::display_todos;
use crate::todo::{ListQuery, SortBy, TodoItem, TodoManager, TodoResult};
use std::cmp::Ordering;
use std::str::FromStr;

pub fn handle_commands(command: Commands, manager: &mut TodoManager) -> TodoResult<()> {
    match command {
        Commands::Add {
            title,
            description,
            due_date,
            priority,
            tags,
        } => {
            manager.add_todo(title, description, due_date, priority.as_deref(), tags)?;
        }
        Commands::Edit {
            id,
            title,
            description,
            due_date,
            priority,
            tags,
        } => {
            manager.edit_todo(
                id,
                title,
                description,
                due_date.as_deref(),
                priority.as_deref(),
                tags,
            )?;
        }
        Commands::Toggle { id } => {
            manager.toggle_todo(id)?;
        }
        Commands::Delete { id } => {
            manager.delete_todo(id)?;
        }
        Commands::ClearList => {
            manager.clear_all();
        }
        Commands::List {
            asc,
            desc,
            sort_by,
            only_complete,
            only_pending,
            priority,
            overdue,
            due_today,
            due_tomorrow,
            due_within,
        } => {
            let sort_by = match sort_by {
                Some(s) => SortBy::from_str(&s)?,
                None => SortBy::Due,
            };

            let priority = TodoManager::parse_priority(priority.as_deref())?;

            let query = ListQuery {
                sort_by,
                asc,
                desc,
                only_complete,
                only_pending,
                priority,
                overdue,
                due_today,
                due_tomorrow,
                due_within,
            };

            handle_list_command(&mut manager.todos, query)?;
        }
    }
    Ok(())
}

fn handle_list_command(todos: &mut [TodoItem], query: ListQuery) -> TodoResult<()> {
    let ascending = query.asc || !query.desc;

    apply_sorting(todos, query.sort_by, ascending)?;

    let filtered_todos = apply_filter(todos, &query)?;

    display_todos(&filtered_todos);
    Ok(())
}

fn cmp_option<T: Ord>(a: &Option<T>, b: &Option<T>, ascending: bool) -> Ordering {
    match (a, b) {
        (Some(a_val), Some(b_val)) => {
            if ascending {
                a_val.cmp(b_val)
            } else {
                b_val.cmp(a_val)
            }
        }
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn apply_sorting(todos: &mut [TodoItem], sort_by: SortBy, ascending: bool) -> TodoResult<()> {
    use std::cmp::Ordering;

    match sort_by {
        SortBy::Due => {
            todos.sort_by(|a, b| cmp_option(&a.due_date(), &b.due_date(), ascending));
        }
        SortBy::Priority => {
            todos.sort_by(|a, b| cmp_option(&a.priority(), &b.priority(), ascending));
        }
        SortBy::DueThenPriority => {
            todos.sort_by(|a, b| {
                let date_cmp = cmp_option(&a.due_date(), &b.due_date(), ascending);
                if date_cmp == Ordering::Equal {
                    cmp_option(&a.priority(), &b.priority(), ascending)
                } else {
                    date_cmp
                }
            });
        }
    }

    Ok(())
}

fn apply_filter(todos: &[TodoItem], list_options: &ListQuery) -> TodoResult<Vec<TodoItem>> {
    let filtered: Vec<TodoItem> = todos
        .iter()
        .filter(|todo| list_options.item_passes_filters(todo))
        .cloned()
        .collect();

    Ok(filtered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Commands;
    use crate::todo::{ListQuery, Priority, SortBy, TodoItem, TodoManager};
    use chrono::NaiveDateTime;
    use std::cmp::Ordering;
    use tempfile::NamedTempFile;

    // Helper to create test dates
    fn test_date(year: i32, month: u32, day: u32, hour: u32, min: u32) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(
            &format!("{year}-{month:02}-{day:02} {hour:02}:{min:02}:00"),
            "%Y-%m-%d %H:%M:%S",
        )
        .unwrap()
    }

    // Helper to create a test TodoManager with temporary file
    fn create_test_manager() -> TodoManager {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_string_lossy().into_owned();
        std::mem::forget(temp_file);
        TodoManager::new(file_path).unwrap()
    }

    #[test]
    fn test_handle_commands_add() {
        let mut manager = create_test_manager();

        let command = Commands::Add {
            title: "Test todo".to_string(),
            description: Some("Test description".to_string()),
            due_date: None,
            priority: Some("high".to_string()),
            tags: Some(vec!["work".to_string()]),
        };

        let result = handle_commands(command, &mut manager);
        assert!(result.is_ok());
        assert_eq!(manager.todos.len(), 1);

        let added_todo = manager.todos.last().unwrap();
        assert_eq!(added_todo.title(), "Test todo");
        assert_eq!(added_todo.priority(), Some(Priority::High));
    }

    #[test]
    fn test_handle_commands_edit() {
        let mut manager = create_test_manager();
        manager
            .add_todo("Original".to_string(), None, None, Some("low"), None)
            .unwrap();
        let todo_id = manager.todos[0].id();

        let command = Commands::Edit {
            id: todo_id,
            title: Some("Updated".to_string()),
            description: None,
            due_date: None,
            priority: Some("high".to_string()),
            tags: None,
        };

        let result = handle_commands(command, &mut manager);
        assert!(result.is_ok());
        assert_eq!(manager.todos[0].title(), "Updated");
        assert_eq!(manager.todos[0].priority(), Some(Priority::High));
    }

    #[test]
    fn test_handle_commands_toggle() {
        let mut manager = create_test_manager();
        manager
            .add_todo("Test".to_string(), None, None, None, None)
            .unwrap();
        let todo_id = manager.todos[0].id();

        let command = Commands::Toggle { id: todo_id };
        let result = handle_commands(command, &mut manager);

        assert!(result.is_ok());
        assert!(manager.todos[0].completed());
    }

    #[test]
    fn test_handle_commands_delete() {
        let mut manager = create_test_manager();
        manager
            .add_todo("Test".to_string(), None, None, None, None)
            .unwrap();
        let todo_id = manager.todos[0].id();

        let command = Commands::Delete { id: todo_id };
        let result = handle_commands(command, &mut manager);

        assert!(result.is_ok());
        assert_eq!(manager.todos.len(), 0);
    }

    #[test]
    fn test_handle_commands_clear_list() {
        let mut manager = create_test_manager();
        manager
            .add_todo("Task 1".to_string(), None, None, None, None)
            .unwrap();
        manager
            .add_todo("Task 2".to_string(), None, None, None, None)
            .unwrap();

        let command = Commands::ClearList;
        let result = handle_commands(command, &mut manager);

        assert!(result.is_ok());
        assert_eq!(manager.todos.len(), 0);
    }

    #[test]
    fn test_handle_commands_invalid_todo_id() {
        let mut manager = create_test_manager();

        let command = Commands::Toggle { id: 999 };
        let result = handle_commands(command, &mut manager);

        assert!(result.is_err());
        match result.unwrap_err() {
            crate::todo::TodoError::TodoNotFound { id } => assert_eq!(id, 999),
            _ => panic!("Expected TodoNotFound error"),
        }
    }

    #[test]
    fn test_apply_sorting_by_due_date() {
        let mut todos = vec![
            TodoItem::new(
                1,
                "Late".to_string(),
                None,
                false,
                Some(test_date(2025, 8, 15, 10, 0)),
                None,
                None,
            ),
            TodoItem::new(
                2,
                "Early".to_string(),
                None,
                false,
                Some(test_date(2025, 8, 10, 10, 0)),
                None,
                None,
            ),
            TodoItem::new(3, "No date".to_string(), None, false, None, None, None),
        ];

        apply_sorting(&mut todos, SortBy::Due, true).unwrap();

        assert_eq!(todos[0].id(), 2); // Early date first
        assert_eq!(todos[1].id(), 1); // Late date second
        assert_eq!(todos[2].id(), 3); // No date last
    }

    #[test]
    fn test_apply_sorting_by_priority() {
        let mut todos = vec![
            TodoItem::new(
                1,
                "High".to_string(),
                None,
                false,
                None,
                Some(Priority::High),
                None,
            ),
            TodoItem::new(
                2,
                "Low".to_string(),
                None,
                false,
                None,
                Some(Priority::Low),
                None,
            ),
            TodoItem::new(
                3,
                "Medium".to_string(),
                None,
                false,
                None,
                Some(Priority::Medium),
                None,
            ),
        ];

        apply_sorting(&mut todos, SortBy::Priority, true).unwrap();

        // Ascending: Low < Medium < High
        assert_eq!(todos[0].id(), 2); // Low
        assert_eq!(todos[1].id(), 3); // Medium
        assert_eq!(todos[2].id(), 1); // High
    }

    #[test]
    fn test_cmp_option_priority_ordering() {
        let high = Some(Priority::High);
        let low = Some(Priority::Low);
        let none: Option<Priority> = None;

        // Test ascending
        assert_eq!(cmp_option(&low, &high, true), Ordering::Less);
        // Test Some vs None (Some comes before None)
        assert_eq!(cmp_option(&high, &none, true), Ordering::Less);
        assert_eq!(cmp_option(&none, &high, true), Ordering::Greater);
    }

    #[test]
    fn test_handle_list_command() {
        let mut todos = vec![
            TodoItem::new(1, "Task 1".to_string(), None, true, None, None, None),
            TodoItem::new(2, "Task 2".to_string(), None, false, None, None, None),
        ];

        let query = ListQuery {
            sort_by: SortBy::Due,
            asc: true,
            desc: false,
            only_complete: false,
            only_pending: false,
            priority: None,
            overdue: false,
            due_today: false,
            due_tomorrow: false,
            due_within: None,
        };

        let result = handle_list_command(&mut todos, query);
        assert!(result.is_ok());
    }

    #[test]
    fn test_full_workflow() {
        let mut manager = create_test_manager();

        // Add -> Edit -> Toggle -> Delete workflow
        let add_cmd = Commands::Add {
            title: "Workflow test".to_string(),
            description: None,
            due_date: None,
            priority: Some("medium".to_string()),
            tags: None,
        };
        handle_commands(add_cmd, &mut manager).unwrap();

        let todo_id = manager.todos[0].id();
        assert_eq!(manager.todos[0].priority(), Some(Priority::Medium));

        // Edit priority
        let edit_cmd = Commands::Edit {
            id: todo_id,
            title: None,
            description: None,
            due_date: None,
            priority: Some("high".to_string()),
            tags: None,
        };
        handle_commands(edit_cmd, &mut manager).unwrap();
        assert_eq!(manager.todos[0].priority(), Some(Priority::High));

        // Toggle completion
        let toggle_cmd = Commands::Toggle { id: todo_id };
        handle_commands(toggle_cmd, &mut manager).unwrap();
        assert!(manager.todos[0].completed());

        // Delete
        let delete_cmd = Commands::Delete { id: todo_id };
        handle_commands(delete_cmd, &mut manager).unwrap();
        assert_eq!(manager.todos.len(), 0);
    }
}
