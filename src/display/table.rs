use crate::display::formatter::{format_due_date, format_status, truncate_text};
use crate::todo::TodoItem;

pub fn display_todos(todos: &[TodoItem]) {
    if todos.is_empty() {
        println!("📭 No todos found.");
        return;
    }

    println!("{:<3} ✓ {:<35} {:<25} Priority", "ID", "Todo", "Due Date");
    println!("{}", "─".repeat(75));

    for item in todos {
        let truncated_todo = truncate_text(&item.todo, 35);
        let due_date = format_due_date(item.due);
        let priority = item
            .priority
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_else(|| "-".to_string());

        println!(
            "{:<3} {} {:<35} {:<25} {}",
            item.id,
            format_status(item.status),
            truncated_todo,
            due_date,
            priority
        );
    }
}
