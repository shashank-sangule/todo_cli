use chrono::{Local, NaiveDateTime};
use clap::{Parser, Subcommand};
use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;
use std::process;
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
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoItem {
    id: i32,
    todo: String,
    status: bool,
    due: Option<NaiveDateTime>,
    priority: Option<Priority>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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

// Enhanced display functions
impl TodoItem {
    fn format_status(&self) -> &'static str {
        if self.status { "✅" } else { "⬜" }
    }

    fn format_due_date(&self) -> String {
        match self.due {
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
}


fn main() {
    let cli = Cli::parse();
    let file_path = &cli.file;

    let mut todo_list = load_todo_list(file_path);

    match &cli.command {
        Commands::Add {
            todo,
            due,
            priority,
        } => {
            if todo.trim().is_empty() {
                eprintln!("❌ Todo cannot be empty");
                process::exit(1);
            }
            let next_id = todo_list.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            let due_str = match due {
                Some(due) => due,
                None => "",
            };
            let parsed_due = if !due_str.is_empty() {
                match NaiveDateTime::parse_from_str(due_str, "%d-%m-%Y %H:%M") {
                    Ok(dt) => Some(dt),
                    Err(_) => {
                        eprintln!(
                            "❌ Invalid date format. Use: dd-mm-YYYY HH:MM (e.g., 05-08-25 15:30)"
                        );
                        process::exit(1);
                    }
                }
            } else {
                None
            };
            let parsed_priority = match priority {
                Some(p) => Some(p.parse::<Priority>().unwrap_or_else(|_| {
                    eprintln!("❌ Invalid priority. Use: high, medium, low.");
                    process::exit(1);
                })),
                None => None,
            };
            let next_todo = TodoItem {
                id: next_id,
                todo: todo.to_string(),
                status: false,
                due: parsed_due,
                priority: parsed_priority,
            };
            todo_list.push(next_todo);
            println!("✅ Todo added!");
        }

        Commands::Edit { id, todo } => {
            let edit_todo = search_by_id(*id, &mut todo_list);
            edit_todo.todo = todo.to_string();
            println!("✏️ Todo edited!");
        }

        Commands::Toggle { id } => {
            let todo = search_by_id(*id, &mut todo_list);
            todo.status = !todo.status;
            println!("🔄 Status toggled!");
        }

        Commands::Delete { id } => {
            let original_len = todo_list.len();
            todo_list.retain(|t| t.id != *id);

            if original_len > todo_list.len() {
                println!("🗑️ Todo deleted!");
            } else {
                eprintln!("❌ Todo with ID {} not found.", id);
                process::exit(1);
            }
        }

        Commands::ClearList => {
            let count = todo_list.len();
            todo_list = vec![];
            println!("🗑️ Cleared {} todo(s)!", count);
        }

        Commands::List { sort_by, asc, desc } => {
            let ascending = *asc || (!asc && !desc);
            if let Some(sort) = sort_by {
                match sort.as_str() {
                    "due" => {
                        if ascending {
                            todo_list.sort_by(|a, b| a.due.cmp(&b.due));
                        } else {
                            todo_list.sort_by(|a, b| b.due.cmp(&a.due));
                        }
                    }
                    "priority" => {
                        if ascending {
                            todo_list.sort_by(|a, b| a.priority.cmp(&b.priority));
                        } else {
                            todo_list.sort_by(|a, b| b.priority.cmp(&a.priority));
                        }
                    }
                    "due+priority" => {
                        if ascending {
                            todo_list.sort_by(|a, b| {
                                a.due.cmp(&b.due).then(a.priority.cmp(&b.priority))
                            });
                        } else {
                            todo_list.sort_by(|a, b| {
                                b.due.cmp(&a.due).then(b.priority.cmp(&a.priority))
                            });
                        }
                    }
                    _ => {
                        eprintln!("❌ Invalid sort_by value. Use: due, priority, due+priority");
                        process::exit(1);
                    }
                }
            }

            display_todos_simple(&todo_list);
        }
    }

    save_todo_list(file_path, &todo_list);
}

fn load_todo_list(file_path: &str) -> Vec<TodoItem> {
    if Path::new(file_path).exists() {
        match fs::read_to_string(file_path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
            Err(_) => {
                eprintln!("⚠️ Could not read file: {}", file_path);
                vec![]
            }
        }
    } else {
        vec![]
    }
}

fn save_todo_list(file_path: &str, todo_list: &Vec<TodoItem>) {
    let content = match serde_json::to_string(todo_list) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("❌ Failed to serialize todos");
            process::exit(1);
        }
    };

    if let Err(_) = fs::write(file_path, content) {
        eprintln!("❌ Failed to write to file: {}", file_path);
        process::exit(1);
    }
}

fn search_by_id(id: i32, todo_list: &mut Vec<TodoItem>) -> &mut TodoItem {
    match todo_list.iter_mut().find(|t| t.id == id) {
        Some(todo) => {
            return todo;
        }
        None => {
            eprintln!("❌ Todo with ID {} not found.", id);
            process::exit(1);
        }
    }
}

fn display_todos_simple(todo_list: &[TodoItem]) {
    if todo_list.is_empty() {
        println!("📭 No todos found.");
        return;
    }

    println!(
        "{:<3} {} {:<35} {:<25} {}",
        "ID", "✓", "Todo", "Due Date", "Priority"
    );
    println!("{}", "─".repeat(75));

    for item in todo_list {
        let truncated_todo = TodoItem::truncate_text(&item.todo, 35);
        let due_date = item.format_due_date();
        let priority = item
            .priority
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_else(|| "-".to_string());

        println!(
            "{:<3} {} {:<35} {:<25} {}",
            item.id,
            item.format_status(),
            truncated_todo,
            due_date,
            priority
        );
    }
}
