use crate::display::formatter::{format_due_date, format_status, truncate_text};
use crate::todo::Priority;
use crate::todo::TodoItem;
use tabled::Tabled;

#[derive(Tabled)]
pub struct TodoTableRow {
    #[tabled(rename = "ID")]
    pub id: u32,

    #[tabled(rename = "Tags")]
    pub tags: String,

    #[tabled(rename = "Title")]
    pub title: String,

    #[tabled(rename = "Description")]
    pub description: String,

    #[tabled(rename = "Due Date")]
    pub due_date: String,

    #[tabled(rename = "Priority")]
    pub priority: String,

    #[tabled(rename = "Completed")]
    pub completed: String,
}

impl From<&TodoItem> for TodoTableRow {
    fn from(todo: &TodoItem) -> Self {
        let status = if todo.completed() {
            "✅ Completed".to_string()
        } else if todo.is_overdue() {
            "🔴 Overdue".to_string()
        } else {
            "⏳ Pending".to_string()
        };

        let priority = match todo.priority() {
            Some(Priority::High) => "🔴 High".to_string(),
            Some(Priority::Medium) => "🟡 Medium".to_string(),
            Some(Priority::Low) => "🟢 Low".to_string(),
            _ => "⚪ None".to_string(),
        };

        let due_date = format_due_date(todo.due_date());

        let tags = match todo.tags_string() {
            Some(t) => t,
            None => "No tags".to_string(),
        };

        let description = match todo.description() {
            Some(d) => truncate_text(d, 30),
            None => "No Description".to_string(),
        };

        let title = truncate_text(todo.title(), 30);

        Self {
            id: todo.id(),
            title,
            tags,
            description,
            due_date,
            priority,
            completed: status,
        }
    }
}

pub fn display_todos(todos: &[TodoItem]) {
    if todos.is_empty() {
        println!("📭 No todos found.");
        return;
    }

    // let rows: Vec<TodoTableRow> = todos.iter().map(TodoTableRow::from).collect();

    // let mut table = Table::new(rows);

    // println!("\n{table}\n");

    println!("{:<3} ✓ {:<35} {:<25} Priority", "ID", "Todo", "Due Date");
    println!("{}", "─".repeat(75));

    for item in todos {
        let truncated_todo = truncate_text(item.title(), 35);
        let due_date = format_due_date(item.due_date());
        let priority = item
            .priority()
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_else(|| "-".to_string());

        println!(
            "{:<3} {} {:<35} {:<25} {}",
            item.id(),
            format_status(item.completed()),
            truncated_todo,
            due_date,
            priority
        );
    }
}
