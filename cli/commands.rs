#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(short, long, default_value = "todo_list.json")]
    file: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
