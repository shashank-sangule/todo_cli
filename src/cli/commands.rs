use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(short, long, default_value = "todo_list.json")]
    pub file: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        title: String,
        due_date: Option<String>,
        priority: Option<String>,
        description: Option<String>,
        tags: Option<Vec<String>>,
    },
    Edit {
        id: u32,
        title: Option<String>,
        description: Option<String>,
        due_date: Option<String>,
        priority: Option<String>,
        tags: Option<Vec<String>>,
    },
    Toggle {
        id: u32,
    },
    Delete {
        id: u32,
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
