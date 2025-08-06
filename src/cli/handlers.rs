use crate::cli::Commands;
use crate::display::display_todos;
use crate::todo::{ListQuery, SortBy, TodoItem, TodoManager, TodoResult};
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

fn apply_sorting(todos: &mut [TodoItem], sort_by: SortBy, ascending: bool) -> TodoResult<()> {
    let comparator = match (sort_by, ascending) {
        (SortBy::Due, true) => |a: &TodoItem, b: &TodoItem| a.due_date().cmp(&b.due_date()),
        (SortBy::Due, false) => |a: &TodoItem, b: &TodoItem| b.due_date().cmp(&a.due_date()),
        (SortBy::Priority, true) => |a: &TodoItem, b: &TodoItem| a.priority().cmp(&b.priority()),
        (SortBy::Priority, false) => |a: &TodoItem, b: &TodoItem| b.priority().cmp(&a.priority()),
        (SortBy::DueThenPriority, true) => |a: &TodoItem, b: &TodoItem| {
            a.due_date()
                .cmp(&b.due_date())
                .then(a.priority().cmp(&b.priority()))
        },
        (SortBy::DueThenPriority, false) => |a: &TodoItem, b: &TodoItem| {
            b.due_date()
                .cmp(&a.due_date())
                .then(b.priority().cmp(&a.priority()))
        },
    };

    todos.sort_by(comparator);
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
    use crate::todo::{ListQuery, Priority, TodoItem};
    use chrono::NaiveDate;
    use std::fs;
    use tempfile::NamedTempFile;

    fn create_test_manager() -> TodoManager {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();
        fs::remove_file(file_path).ok();
        TodoManager::new(file_path.to_string()).unwrap()
    }

    fn create_test_todos() -> Vec<TodoItem> {
        vec![
            TodoItem::new(
                1,
                "High priority task".to_string(),
                None,
                false,
                Some(
                    NaiveDate::from_ymd_opt(2025, 8, 10)
                        .unwrap()
                        .and_hms_opt(9, 0, 0)
                        .unwrap(),
                ),
                Some(Priority::High),
                None,
            ),
            TodoItem::new(
                2,
                "Medium priority task".to_string(),
                None,
                true,
                Some(
                    NaiveDate::from_ymd_opt(2025, 8, 8)
                        .unwrap()
                        .and_hms_opt(9, 0, 0)
                        .unwrap(),
                ),
                Some(Priority::Medium),
                None,
            ),
            TodoItem::new(
                3,
                "Low priority task".to_string(),
                None,
                false,
                Some(
                    NaiveDate::from_ymd_opt(2025, 8, 9)
                        .unwrap()
                        .and_hms_opt(9, 0, 0)
                        .unwrap(),
                ),
                Some(Priority::Low),
                None,
            ),
        ]
    }

    #[test]
    fn test_handle_add_command() {
        let mut manager = create_test_manager();

        let command = Commands::Add {
            title: "New Task".to_string(),
            description: Some("Task description".to_string()),
            due_date: None,
            priority: Some("high".to_string()),
            tags: Some(vec!["work".to_string()]),
        };

        let result = handle_commands(command, &mut manager);

        assert!(result.is_ok());
        assert_eq!(manager.todos.len(), 1);
        assert_eq!(manager.todos[0].title(), "New Task");
        assert_eq!(manager.todos[0].priority(), Some(Priority::High));
    }

    #[test]
    fn test_handle_edit_command() {
        let mut manager = create_test_manager();
        manager
            .add_todo("Original Title".to_string(), None, None, Some("low"), None)
            .unwrap();

        let command = Commands::Edit {
            id: 1,
            title: Some("Updated Title".to_string()),
            description: Some("Updated description".to_string()),
            due_date: None,
            priority: Some("high".to_string()),
            tags: None,
        };

        let result = handle_commands(command, &mut manager);

        assert!(result.is_ok());
        assert_eq!(manager.todos[0].title(), "Updated Title");
        assert_eq!(manager.todos[0].priority(), Some(Priority::High));
        assert_eq!(manager.todos[0].description(), Some("Updated description"));
    }

    #[test]
    fn test_handle_toggle_command() {
        let mut manager = create_test_manager();
        manager
            .add_todo("Task".to_string(), None, None, None, None)
            .unwrap();

        assert!(!manager.todos[0].completed());

        let command = Commands::Toggle { id: 1 };
        let result = handle_commands(command, &mut manager);

        assert!(result.is_ok());
        assert!(manager.todos[0].completed());
    }

    #[test]
    fn test_handle_delete_command() {
        let mut manager = create_test_manager();
        manager
            .add_todo("Task".to_string(), None, None, None, None)
            .unwrap();

        assert_eq!(manager.todos.len(), 1);

        let command = Commands::Delete { id: 1 };
        let result = handle_commands(command, &mut manager);

        assert!(result.is_ok());
        assert_eq!(manager.todos.len(), 0);
    }

    #[test]
    fn test_handle_clear_command() {
        let mut manager = create_test_manager();
        manager
            .add_todo("Task 1".to_string(), None, None, None, None)
            .unwrap();
        manager
            .add_todo("Task 2".to_string(), None, None, None, None)
            .unwrap();

        assert_eq!(manager.todos.len(), 2);

        let command = Commands::ClearList;
        let result = handle_commands(command, &mut manager);

        assert!(result.is_ok());
        assert_eq!(manager.todos.len(), 0);
    }

    #[test]
    fn test_handle_list_command_basic() {
        let mut manager = create_test_manager();

        let command = Commands::List {
            asc: false,
            desc: false,
            sort_by: Some("due".to_string()),
            only_complete: false,
            only_pending: false,
            priority: None,
            overdue: false,
            due_today: false,
            due_tomorrow: false,
            due_within: None,
        };

        let result = handle_commands(command, &mut manager);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_sorting_by_due_date() {
        let mut todos = create_test_todos();

        apply_sorting(&mut todos, SortBy::Due, true).unwrap();

        assert_eq!(todos[0].id(), 2); // 2025-08-08
        assert_eq!(todos[1].id(), 3); // 2025-08-09
        assert_eq!(todos[2].id(), 1); // 2025-08-10

        apply_sorting(&mut todos, SortBy::Due, false).unwrap();

        assert_eq!(todos[0].id(), 1); // 2025-08-10
        assert_eq!(todos[1].id(), 3); // 2025-08-09
        assert_eq!(todos[2].id(), 2); // 2025-08-08
    }

    #[test]
    fn test_apply_sorting_by_priority() {
        let mut todos = create_test_todos();

        apply_sorting(&mut todos, SortBy::Priority, true).unwrap();

        assert_eq!(todos[0].priority(), Some(Priority::Low));
        assert_eq!(todos[1].priority(), Some(Priority::Medium));
        assert_eq!(todos[2].priority(), Some(Priority::High));

        apply_sorting(&mut todos, SortBy::Priority, false).unwrap();

        assert_eq!(todos[0].priority(), Some(Priority::High));
        assert_eq!(todos[1].priority(), Some(Priority::Medium));
        assert_eq!(todos[2].priority(), Some(Priority::Low));
    }

    #[test]
    fn test_apply_sorting_due_then_priority() {
        let mut todos = create_test_todos();

        apply_sorting(&mut todos, SortBy::DueThenPriority, true).unwrap();

        assert_eq!(todos[0].id(), 2); // 2025-08-08
        assert_eq!(todos[1].id(), 3); // 2025-08-09
        assert_eq!(todos[2].id(), 1); // 2025-08-10
    }

    #[test]
    fn test_apply_filter_basic() {
        let todos = create_test_todos();

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

        let filtered = apply_filter(&todos, &query).unwrap();

        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_apply_filter_by_completion() {
        let todos = create_test_todos();

        let query_complete = ListQuery {
            sort_by: SortBy::Due,
            asc: true,
            desc: false,
            only_complete: true,
            only_pending: false,
            priority: None,
            overdue: false,
            due_today: false,
            due_tomorrow: false,
            due_within: None,
        };

        let filtered_complete = apply_filter(&todos, &query_complete).unwrap();

        assert_eq!(filtered_complete.len(), 1);
        assert_eq!(filtered_complete[0].id(), 2);
        assert!(filtered_complete[0].completed());

        let query_pending = ListQuery {
            sort_by: SortBy::Due,
            asc: true,
            desc: false,
            only_complete: false,
            only_pending: true,
            priority: None,
            overdue: false,
            due_today: false,
            due_tomorrow: false,
            due_within: None,
        };

        let filtered_pending = apply_filter(&todos, &query_pending).unwrap();

        assert_eq!(filtered_pending.len(), 2);
        assert!(!filtered_pending[0].completed());
        assert!(!filtered_pending[1].completed());
    }

    #[test]
    fn test_error_handling_invalid_command() {
        let mut manager = create_test_manager();

        let command = Commands::Edit {
            id: 999,
            title: Some("Title".to_string()),
            description: None,
            due_date: None,
            priority: None,
            tags: None,
        };

        let result = handle_commands(command, &mut manager);
        assert!(result.is_err());
    }
}
