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
        #[arg(short, long, group = "order")]
        asc: bool,
        #[arg(short, long, group = "order")]
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

            TodoError::TodoTooLong => write!(f, "❌ Todo cannot be more than 500 character"),
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

    fn add_todo(
        &mut self,
        text: &str,
        due: Option<&str>,
        priority: Option<&str>,
    ) -> TodoResult<()> {
        let trimmed_text = text.trim();
        if trimmed_text.is_empty() {
            return Err(TodoError::EmptyTodo);
        }
        if trimmed_text.len() > 500 {
            // Add max length check
            return Err(TodoError::TodoTooLong);
        }
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
        let trimmed_text = new_text.trim();
        if trimmed_text.is_empty() {
            return Err(TodoError::EmptyTodo);
        }
        if trimmed_text.len() > 500 {
            // Add max length check
            return Err(TodoError::TodoTooLong);
        }

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

            handle_list_command(
                &mut manager.todos,
                sort_by,
                asc,
                desc,
                only_complete,
                only_pending,
                priority.as_deref(),
                overdue,
                due_today,
                due_tomorrow,
                due_within,
            )?;
        }
    }

    manager.save()?;
    Ok(())
}

fn handle_list_command(
    todos: &mut [TodoItem],
    sort_by: SortBy,
    asc: bool,
    desc: bool,
    only_complete: bool,
    only_pending: bool,
    priority: Option<&str>,
    overdue: bool,
    due_today: bool,
    due_tomorrow: bool,
    due_within: Option<i64>,
) -> TodoResult<()> {
    let ascending = asc || !desc;

    apply_sorting(todos, sort_by, ascending)?;

    let status = if only_complete {
        Some(true)
    } else if only_pending {
        Some(false)
    } else {
        None
    };

    let todos = &apply_filter(
        todos,
        status,
        priority,
        overdue,
        due_today,
        due_tomorrow,
        due_within,
    )?;

    display_todos(todos);
    Ok(())
}

fn apply_sorting(todos: &mut [TodoItem], sort_by: SortBy, ascending: bool) -> TodoResult<()> {
    match sort_by {
        SortBy::Due => {
            if ascending {
                todos.sort_by(|a, b| a.due.cmp(&b.due));
            } else {
                todos.sort_by(|a, b| b.due.cmp(&a.due));
            }
        }
        SortBy::Priority => {
            if ascending {
                todos.sort_by(|a, b| a.priority.cmp(&b.priority));
            } else {
                todos.sort_by(|a, b| b.priority.cmp(&a.priority));
            }
        }
        SortBy::DueThenPriority => {
            if ascending {
                todos.sort_by(|a, b| a.due.cmp(&b.due).then(a.priority.cmp(&b.priority)));
            } else {
                todos.sort_by(|a, b| b.due.cmp(&a.due).then(b.priority.cmp(&a.priority)));
            }
        }
    }
    Ok(())
}

fn apply_filter(
    todos: &[TodoItem],
    status: Option<bool>,
    priority: Option<&str>,
    overdue: bool,
    due_today: bool,
    due_tomorrow: bool,
    due_within: Option<i64>,
) -> TodoResult<Vec<TodoItem>> {
    let filtered = todos.iter().filter(|todo| {
        let status_match = status.is_none_or(|s| todo.status == s);
        let priority_match = match priority {
            Some("high") => todo.priority == Some(Priority::High),
            Some("medium") => todo.priority == Some(Priority::Medium),
            Some("low") => todo.priority == Some(Priority::Low),
            Some(_) => return false,
            None => true,
        };
        let now = Local::now().naive_local();
        let overdue_match = !overdue || todo.due.is_some_and(|t| t < now);
        let due_today_match = !due_today || todo.due.is_some_and(|t| t.date() == now.date());
        let due_tomorrow_match = !due_tomorrow
            || todo
                .due
                .is_some_and(|t| t.date() == now.date() + chrono::Duration::days(1));
        let due_within_match = due_within.is_none_or(|n| {
            todo.due.is_some_and(|t| {
                let date = t.date();
                date >= now.date() && date <= now.date() + chrono::Duration::days(n)
            })
        });
        status_match
            && priority_match
            && overdue_match
            && due_today_match
            && due_tomorrow_match
            && due_within_match
    });

    Ok(filtered.cloned().collect())
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
