use chrono::{Local, NaiveDateTime};
use clap::{Parser, Subcommand};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;

#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long, default_value = "todo_list.json")]
    file: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        todo: String,
        due: Option<String>,
        priority: Option<String>,
    },
    Edit {
        id: i32,
        todo: String,
        due: Option<String>,
        priority: Option<String>,
    },
    Toggle {
        id: i32,
    },
    Delete {
        id: i32,
    },
    ClearList,
    List {
        #[arg(short, long, group = "order", conflicts_with = "desc")]
        asc: bool,
        #[arg(short, long, group = "order", conflicts_with = "asc")]
        desc: bool,
        #[arg(long, value_parser=["due", "priority", "due+priority"])]
        sort_by: Option<String>,
        #[arg(short, long, group = "filter-status")]
        only_complete: bool,
        #[arg(short, long, group = "filter-status")]
        only_pending: bool,
        #[arg(long, value_parser=["high", "medium", "low"])]
        priority: Option<String>,
        #[arg(short, long, group = "filter-time")]
        overdue: bool,
        #[arg(short, long, group = "filter-time")]
        due_today: bool,
        #[arg(short, long, group = "filter-time")]
        due_tomorrow: bool,
        #[arg(short, long, group = "filter-time")]
        due_within: Option<i64>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TodoItem {
    id: i32,
    todo: String,
    status: bool,
    due: Option<NaiveDateTime>,
    priority: Option<Priority>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Priority {
    Low,
    Medium,
    High,
}

impl FromStr for Priority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "high" => Ok(Priority::High),
            "medium" => Ok(Priority::Medium),
            "low" => Ok(Priority::Low),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::High => write!(f, "🔴 High"),
            Priority::Medium => write!(f, "🟡 Medium"),
            Priority::Low => write!(f, "🟢 Low"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum SortBy {
    Due,
    Priority,
    DueThenPriority,
}

impl FromStr for SortBy {
    type Err = TodoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "due" => Ok(SortBy::Due),
            "priority" => Ok(SortBy::Priority),
            "due+priority" => Ok(SortBy::DueThenPriority),
            _ => Err(TodoError::InvalidSortField),
        }
    }
}

type TodoResult<T> = Result<T, TodoError>;

#[derive(Debug)]
enum TodoError {
    InvalidDateFormat,
    InvalidPriority,
    TodoNotFound(i32),
    FileError(String),
    EmptyTodo,
    SerializationError,
    InvalidSortField,
    TodoTooLong,
}

impl std::fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TodoError::InvalidDateFormat => {
                write!(f, "❌ Invalid date format. Use: dd-mm-YYYY HH:MM")
            }
            TodoError::InvalidPriority => write!(f, "❌ Invalid priority. Use: high, medium, low"),
            TodoError::FileError(msg) => write!(f, "❌ File error: {msg}"),
            TodoError::TodoNotFound(id) => write!(f, "❌ Todo with ID {id} not found"),
            TodoError::EmptyTodo => write!(f, "❌ Todo cannot be empty"),
            TodoError::SerializationError => write!(f, "❌ Failed to save/load todos"),
            TodoError::InvalidSortField => {
                write!(f, "❌ Invalid sort field: Use: due, priority, due+priority")
            }

            TodoError::TodoTooLong => write!(f, "❌ Todo cannot be more than 500 characters"),
        }
    }
}

struct TodoManager {
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
                let formats = [
                    "%d-%m-%Y %H:%M",
                    "%d-%m-%y %H:%M",
                    "%d-%m-%Y",
                    "%Y-%m-%d %H:%M",
                ];

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

struct ListOptions {
    sort_by: SortBy,
    asc: bool,
    desc: bool,
    only_complete: bool,
    only_pending: bool,
    priority: Option<Priority>,
    overdue: bool,
    due_today: bool,
    due_tomorrow: bool,
    due_within: Option<i64>,
}

impl ListOptions {
    // Check if any filters are actually enabled
    fn has_any_filters(&self) -> bool {
        self.only_complete
            || self.only_pending
            || self.priority.is_some()
            || self.overdue
            || self.due_today
            || self.due_tomorrow
            || self.due_within.is_some()
    }

    // Check if item passes ALL active filters (AND logic)
    fn item_passes_filters(&self, item: &TodoItem) -> bool {
        // If no filters are set, include all items
        if !self.has_any_filters() {
            return true;
        }

        // Check each filter type - item must pass ALL active filters
        self.passes_status_filter(item)
            && self.passes_priority_filter(item)
            && self.passes_time_filter(item)
    }

    fn passes_status_filter(&self, item: &TodoItem) -> bool {
        match (self.only_complete, self.only_pending) {
            (true, false) => item.status,  // Only completed items
            (false, true) => !item.status, // Only pending items
            _ => true,                     // No status filter or conflicting filters
        }
    }

    fn passes_priority_filter(&self, item: &TodoItem) -> bool {
        match self.priority {
            Some(required_priority) => item.priority == Some(required_priority),
            None => true, // No priority filter
        }
    }

    fn passes_time_filter(&self, item: &TodoItem) -> bool {
        // If no time filters are set, pass
        if !self.overdue && !self.due_today && !self.due_tomorrow && self.due_within.is_none() {
            return true;
        }

        // Check if item matches ANY of the active time filters (OR logic within time category)
        self.is_overdue(item)
            || self.is_due_today(item)
            || self.is_due_tomorrow(item)
            || self.is_due_within(item)
    }

    // Individual check methods - only check the condition, don't check if filter is active
    fn is_due_today(&self, item: &TodoItem) -> bool {
        if !self.due_today {
            return false;
        }
        item.due.map(|d| d.date()) == Some(Local::now().naive_local().date())
    }

    fn is_due_tomorrow(&self, item: &TodoItem) -> bool {
        if !self.due_tomorrow {
            return false;
        }
        item.due.map(|d| d.date())
            == Some(Local::now().naive_local().date() + chrono::Duration::days(1))
    }

    fn is_overdue(&self, item: &TodoItem) -> bool {
        if !self.overdue {
            return false;
        }
        item.due.is_some_and(|d| d < Local::now().naive_local())
    }

    fn is_due_within(&self, item: &TodoItem) -> bool {
        if let Some(days) = self.due_within {
            item.due.is_some_and(|d| {
                let now = Local::now().naive_local();
                let date = d.date();
                date >= now.date() && date <= now.date() + chrono::Duration::days(days)
            })
        } else {
            false
        }
    }
}

fn main() -> TodoResult<()> {
    let cli = Cli::parse();
    let mut manager = TodoManager::new(cli.file)?;

    match cli.command {
        Commands::Add {
            todo,
            due,
            priority,
        } => {
            manager.add_todo(&todo, due.as_deref(), priority.as_deref())?;
            println!("✅ Todo added!");
        }
        Commands::Edit {
            id,
            todo,
            due,
            priority,
        } => {
            manager.edit_todo(id, &todo, due.as_deref(), priority.as_deref())?;
            println!("✏️ Todo edited!");
        }
        Commands::Toggle { id } => {
            manager.toggle_todo(id)?;
            println!("🔄 Status toggled!");
        }
        Commands::Delete { id } => {
            manager.delete_todo(id)?;
            println!("🗑️ Todo deleted!");
        }
        Commands::ClearList => {
            let count = manager.clear_all();
            println!("🗑️ Cleared {count} todo(s)!");
        }
        Commands::List {
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
        } => {
            let sort_by = match sort_by {
                Some(s) => SortBy::from_str(&s)?,
                None => SortBy::Due,
            };

            let priority = TodoManager::parse_priority(priority.as_deref())?;

            let list_options = ListOptions {
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

            handle_list_command(&mut manager.todos, list_options)?;
        }
    }

    manager.save()?;
    Ok(())
}

fn handle_list_command(todos: &mut [TodoItem], list_options: ListOptions) -> TodoResult<()> {
    let ascending = list_options.asc || !list_options.desc;

    apply_sorting(todos, list_options.sort_by, ascending)?;

    let filtered_todos = apply_filter(todos, &list_options)?;

    display_todos(&filtered_todos);
    Ok(())
}

fn apply_sorting(todos: &mut [TodoItem], sort_by: SortBy, ascending: bool) -> TodoResult<()> {
    let comparator = match (sort_by, ascending) {
        (SortBy::Due, true) => |a: &TodoItem, b: &TodoItem| a.due.cmp(&b.due),
        (SortBy::Due, false) => |a: &TodoItem, b: &TodoItem| b.due.cmp(&a.due),
        (SortBy::Priority, true) => |a: &TodoItem, b: &TodoItem| a.priority.cmp(&b.priority),
        (SortBy::Priority, false) => |a: &TodoItem, b: &TodoItem| b.priority.cmp(&a.priority),
        (SortBy::DueThenPriority, true) => {
            |a: &TodoItem, b: &TodoItem| a.due.cmp(&b.due).then(a.priority.cmp(&b.priority))
        }
        (SortBy::DueThenPriority, false) => {
            |a: &TodoItem, b: &TodoItem| b.due.cmp(&a.due).then(b.priority.cmp(&a.priority))
        }
    };

    todos.sort_by(comparator);
    Ok(())
}

fn apply_filter(todos: &[TodoItem], list_options: &ListOptions) -> TodoResult<Vec<TodoItem>> {
    let filtered: Vec<TodoItem> = todos
        .iter()
        .filter(|todo| list_options.item_passes_filters(todo))
        .cloned()
        .collect();

    Ok(filtered)
}

fn display_todos(todos: &[TodoItem]) {
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

fn format_status(status: bool) -> &'static str {
    if status { "✅" } else { "⬜" }
}

fn format_due_date(due: Option<NaiveDateTime>) -> String {
    match due {
        Some(due) => {
            let now = Local::now().naive_local();
            let diff = due.signed_duration_since(now);

            if diff.num_days() < 0 {
                format!("🔴 {} (overdue)", due.format("%d-%m-%Y %H:%M"))
            } else if diff.num_days() == 0 {
                format!("🟡 {} (today)", due.format("%H:%M"))
            } else if diff.num_days() == 1 {
                format!("🟢 {} (tomorrow)", due.format("%H:%M"))
            } else if diff.num_days() <= 7 {
                format!(
                    "🟢 {} ({} days)",
                    due.format("%d-%m %H:%M"),
                    diff.num_days()
                )
            } else {
                format!("⚪ {}", due.format("%d-%m-%Y"))
            }
        }
        None => "-".to_string(),
    }
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    }
}
