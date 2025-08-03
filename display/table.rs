pub fn display_todos(todos: &[TodoItem]) {
    if todos.is_empty() {
        println!("📭 No todos found.");
        return;
    }

    print_table_header();
    print_table_separator();

    for item in todos {
        print_todo_row(item);
    }
}

fn print_table_header() {
    println!("{:<3} ✓ {:<35} {:<25} Priority", "ID", "Todo", "Due Date");
}

fn print_table_separator() {
    println!("{}", "─".repeat(TABLE_WIDTH));
}

fn print_todo_row(item: &TodoItem) {
    let truncated_todo = truncate_text(&item.todo, TODO_DISPLAY_LENGTH);
    let due_date = format_due_date(item.due);
    let priority = format_priority(item.priority);

    println!(
        "{:<3} {} {:<35} {:<25} {}",
        item.id,
        format_status(item.status),
        truncated_todo,
        due_date,
        priority
    );
}

fn format_priority(priority: Option<Priority>) -> String {
    priority
        .as_ref()
        .map(|p| p.to_string())
        .unwrap_or_else(|| "-".to_string())
}
