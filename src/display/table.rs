use crate::display::formatter::{format_due_date, format_status, truncate_text};
use crate::todo::TodoItem;

pub fn display_todos(todos: &[TodoItem]) {
    if todos.is_empty() {
        println!("📭 No todos found.");
        return;
    }

    println!("{:<3} ✓  {:<35} {:<30} Priority", "ID", "Todo", "Due Date");
    println!("{}", "─".repeat(85));

    for item in todos {
        let truncated_todo = truncate_text(item.title(), 35);
        let due_date = format_due_date(item.due_date());
        let priority = item
            .priority()
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_else(|| "-".to_string());

        println!(
            "{:<3} {} {:<35} {:<30} {}",
            item.id(),
            format_status(item.completed()),
            truncated_todo,
            due_date,
            priority.trim()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::Priority;
    use crate::todo::TodoItem;
    use chrono::NaiveDateTime;
    use std::io::{self, Write};

    pub fn display_todos_to_writer<W: Write>(todos: &[TodoItem], writer: &mut W) -> io::Result<()> {
        if todos.is_empty() {
            writeln!(writer, "📭 No todos found.")?;
            return Ok(());
        }

        writeln!(
            writer,
            "{:<3} ✓  {:<35} {:<30} Priority",
            "ID", "Todo", "Due Date"
        )?;
        writeln!(writer, "{}", "─".repeat(85))?;

        for item in todos {
            let truncated_todo = truncate_text(item.title(), 35);
            let due_date = format_due_date(item.due_date());
            let priority = item
                .priority()
                .map(|p| p.to_string())
                .unwrap_or_else(|| "-".to_string());

            writeln!(
                writer,
                "{:<3} {} {:<35} {:<30} {}",
                item.id(),
                format_status(item.completed()),
                truncated_todo,
                due_date,
                priority.trim()
            )?;
        }
        Ok(())
    }

    fn create_test_todo(
        id: u32,
        title: &str,
        completed: bool,
        due_date: Option<NaiveDateTime>,
        priority: Option<Priority>,
    ) -> TodoItem {
        TodoItem::new(
            id,
            title.to_string(),
            None,
            completed,
            due_date,
            priority,
            None,
        )
    }

    fn test_date(year: i32, month: u32, day: u32, hour: u32, min: u32) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(
            &format!("{year}-{month:02}-{day:02} {hour:02}:{min:02}:00"),
            "%Y-%m-%d %H:%M:%S",
        )
        .unwrap()
    }

    #[test]
    fn test_display_todos_empty_list() {
        let mut output = Vec::new();
        let todos: Vec<TodoItem> = vec![];

        display_todos_to_writer(&todos, &mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        assert_eq!(output_str.trim(), "📭 No todos found.");
    }

    #[test]
    fn test_display_todos_single_completed_item() {
        let mut output = Vec::new();
        let due_date = test_date(2025, 8, 15, 10, 30);
        let todo = create_test_todo(
            1,
            "Buy groceries",
            true,
            Some(due_date),
            Some(Priority::High),
        );
        let todos = vec![todo];

        display_todos_to_writer(&todos, &mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output_str.lines().collect();

        assert_eq!(
            lines[0],
            "ID  ✓  Todo                                Due Date                       Priority"
        );

        assert_eq!(lines[1], "─".repeat(85));

        assert!(lines[2].starts_with("1  "));
        assert!(lines[2].contains("Buy groceries"));
        assert!(lines[2].contains("High"));
    }

    #[test]
    fn test_display_todos_single_incomplete_item() {
        let mut output = Vec::new();
        let due_date = test_date(2025, 12, 31, 23, 59);
        let todo = create_test_todo(
            2,
            "Complete project",
            false,
            Some(due_date),
            Some(Priority::Medium),
        );
        let todos = vec![todo];

        display_todos_to_writer(&todos, &mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output_str.lines().collect();

        assert!(lines[2].starts_with("2  "));
        assert!(lines[2].contains("Complete project"));
        assert!(lines[2].contains("Medium"));
    }

    #[test]
    fn test_display_todos_missing_optional_fields() {
        let mut output = Vec::new();
        let todo = create_test_todo(3, "Simple task", false, None, None);
        let todos = vec![todo];

        display_todos_to_writer(&todos, &mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output_str.lines().collect();

        assert!(lines[2].contains("Simple task"));
        assert!(lines[2].contains("-"));
    }

    #[test]
    fn test_display_todos_long_title_truncation() {
        let mut output = Vec::new();
        let long_title =
            "This is a very very very long todo title that should definitely be truncated";
        let due_date = test_date(2025, 9, 1, 12, 0);
        let todo = create_test_todo(4, long_title, false, Some(due_date), Some(Priority::Low));
        let todos = vec![todo];

        display_todos_to_writer(&todos, &mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output_str.lines().collect();

        assert!(lines[2].contains("This is a very very very long"));
        assert!(lines[2].contains("Low"));
    }

    #[test]
    fn test_display_todos_multiple_items() {
        let mut output = Vec::new();
        let todos = vec![
            create_test_todo(
                1,
                "First task",
                true,
                Some(test_date(2025, 8, 10, 9, 0)),
                Some(Priority::High),
            ),
            create_test_todo(2, "Second task", false, None, Some(Priority::Medium)),
            create_test_todo(
                3,
                "Third task",
                false,
                Some(test_date(2025, 8, 20, 14, 30)),
                None,
            ),
        ];

        display_todos_to_writer(&todos, &mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output_str.lines().collect();

        assert_eq!(lines.len(), 5);

        assert!(lines[2].starts_with("1  "));
        assert!(lines[3].starts_with("2  "));
        assert!(lines[4].starts_with("3  "));

        assert!(lines[2].contains("First task") && lines[2].contains("High"));
        assert!(lines[3].contains("Second task") && lines[3].contains("Medium"));
        assert!(lines[4].contains("Third task"));
    }

    #[test]
    fn test_display_todos_formatting_consistency() {
        let mut output = Vec::new();
        let todo = create_test_todo(
            999,
            "Test",
            false,
            Some(test_date(2025, 1, 1, 0, 0)),
            Some(Priority::Low),
        );
        let todos = vec![todo];

        display_todos_to_writer(&todos, &mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output_str.lines().collect();

        let separator = lines[1];
        let todo_line = lines[2];

        assert_eq!(separator.chars().count(), 85);
        assert!(todo_line.len() >= 60);
        assert!(todo_line.starts_with("999"));
    }
}
