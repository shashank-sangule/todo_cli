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
        #[arg(long)]
        due_date: Option<String>,
        #[arg(long, value_parser=["high", "medium", "low"])]
        priority: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    Edit {
        id: u32,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        due_date: Option<String>,
        #[arg(long, value_parser=["high", "medium", "low"])]
        priority: Option<String>,
        #[arg(long, value_delimiter = ',')]
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
        #[arg(long, group = "order", conflicts_with = "desc")]
        asc: bool,
        #[arg(long, group = "order", conflicts_with = "asc")]
        desc: bool,
        #[arg(long, value_parser=["due", "priority", "due+priority"])]
        sort_by: Option<String>,
        #[arg(long, group = "filter-status")]
        only_complete: bool,
        #[arg(long, group = "filter-status")]
        only_pending: bool,
        #[arg(long, value_parser=["high", "medium", "low"])]
        priority: Option<String>,
        #[arg(long, group = "filter-time")]
        overdue: bool,
        #[arg(long, group = "filter-time")]
        due_today: bool,
        #[arg(long, group = "filter-time")]
        due_tomorrow: bool,
        #[arg(long, group = "filter-time")]
        due_within: Option<i64>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_default_file() {
        let args = vec!["todo", "list"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.file, "todo_list.json");
        assert!(matches!(cli.command, Commands::List { .. }));
    }

    #[test]
    fn test_cli_custom_file() {
        let args = vec!["todo", "--file", "custom.json", "list"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.file, "custom.json");
    }

    #[test]
    fn test_add_command_required_only() {
        let args = vec!["todo", "add", "Buy groceries"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Add {
                title,
                due_date,
                priority,
                description,
                tags,
            } => {
                assert_eq!(title, "Buy groceries");
                assert_eq!(due_date, None);
                assert_eq!(priority, None);
                assert_eq!(description, None);
                assert_eq!(tags, None);
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_add_command_with_all_options() {
        let args = vec![
            "todo",
            "add",
            "Complete project",
            "--due-date",
            "2025-12-25",
            "--priority",
            "high",
            "--description",
            "Finish the todo app",
            "--tags",
            "work,urgent,deadline",
        ];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Add {
                title,
                due_date,
                priority,
                description,
                tags,
            } => {
                assert_eq!(title, "Complete project");
                assert_eq!(due_date, Some("2025-12-25".to_string()));
                assert_eq!(priority, Some("high".to_string()));
                assert_eq!(description, Some("Finish the todo app".to_string()));
                assert_eq!(
                    tags,
                    Some(vec![
                        "work".to_string(),
                        "urgent".to_string(),
                        "deadline".to_string()
                    ])
                );
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_edit_command() {
        let args = vec![
            "todo",
            "edit",
            "1",
            "--title",
            "New title",
            "--priority",
            "low",
            "--description",
            "Updated description",
        ];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Edit {
                id,
                title,
                description,
                due_date,
                priority,
                tags,
            } => {
                assert_eq!(id, 1);
                assert_eq!(title, Some("New title".to_string()));
                assert_eq!(description, Some("Updated description".to_string()));
                assert_eq!(due_date, None);
                assert_eq!(priority, Some("low".to_string()));
                assert_eq!(tags, None);
            }
            _ => panic!("Expected Edit command"),
        }
    }

    #[test]
    fn test_toggle_command() {
        let args = vec!["todo", "toggle", "42"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Toggle { id } => {
                assert_eq!(id, 42);
            }
            _ => panic!("Expected Toggle command"),
        }
    }

    #[test]
    fn test_delete_command() {
        let args = vec!["todo", "delete", "5"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Delete { id } => {
                assert_eq!(id, 5);
            }
            _ => panic!("Expected Delete command"),
        }
    }

    #[test]
    fn test_clear_list_command() {
        let args = vec!["todo", "clear-list"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert!(matches!(cli.command, Commands::ClearList));
    }

    #[test]
    fn test_list_command_basic() {
        let args = vec!["todo", "list"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
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
                assert!(!asc);
                assert!(!desc);
                assert_eq!(sort_by, None);
                assert!(!only_complete);
                assert!(!only_pending);
                assert_eq!(priority, None);
                assert!(!overdue);
                assert!(!due_today);
                assert!(!due_tomorrow);
                assert_eq!(due_within, None);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_list_command_with_options() {
        let args = vec![
            "todo",
            "list",
            "--asc",
            "--sort-by",
            "priority",
            "--only-pending",
            "--priority",
            "high",
            "--overdue",
        ];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
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
                assert!(asc);
                assert!(!desc);
                assert_eq!(sort_by, Some("priority".to_string()));
                assert!(!only_complete);
                assert!(only_pending);
                assert_eq!(priority, Some("high".to_string()));
                assert!(overdue);
                assert!(!due_today);
                assert!(!due_tomorrow);
                assert_eq!(due_within, None);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_invalid_priority_value() {
        let args = vec!["todo", "add", "Task", "--priority", "invalid"];
        let result = Cli::try_parse_from(args);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_sort_by_value() {
        let args = vec!["todo", "list", "--sort-by", "invalid"];
        let result = Cli::try_parse_from(args);

        assert!(result.is_err());
    }

    #[test]
    fn test_conflicting_asc_desc() {
        let args = vec!["todo", "list", "--asc", "--desc"];
        let result = Cli::try_parse_from(args);

        assert!(result.is_err());
    }

    #[test]
    fn test_missing_required_title_for_add() {
        let args = vec!["todo", "add"];
        let result = Cli::try_parse_from(args);

        assert!(result.is_err());
    }

    #[test]
    fn test_missing_required_id_for_toggle() {
        let args = vec!["todo", "toggle"];
        let result = Cli::try_parse_from(args);

        assert!(result.is_err());
    }

    #[test]
    fn test_tags_comma_delimiter() {
        let args = vec![
            "todo",
            "add",
            "Task with tags",
            "--tags",
            "work,personal,urgent",
        ];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Add { tags, .. } => {
                assert_eq!(
                    tags,
                    Some(vec![
                        "work".to_string(),
                        "personal".to_string(),
                        "urgent".to_string()
                    ])
                );
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_due_within_numeric_value() {
        let args = vec!["todo", "list", "--due-within", "7"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::List { due_within, .. } => {
                assert_eq!(due_within, Some(7));
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_all_sort_by_values() {
        let sort_values = vec!["due", "priority", "due+priority"];

        for sort_value in sort_values {
            let args = vec!["todo", "list", "--sort-by", sort_value];
            let cli = Cli::try_parse_from(args).unwrap();

            match cli.command {
                Commands::List { sort_by, .. } => {
                    assert_eq!(sort_by, Some(sort_value.to_string()));
                }
                _ => panic!("Expected List command"),
            }
        }
    }

    #[test]
    fn test_all_priority_values() {
        let priority_values = vec!["high", "medium", "low"];

        for priority_value in priority_values {
            let args = vec!["todo", "add", "Task", "--priority", priority_value];
            let cli = Cli::try_parse_from(args).unwrap();

            match cli.command {
                Commands::Add { priority, .. } => {
                    assert_eq!(priority, Some(priority_value.to_string()));
                }
                _ => panic!("Expected Add command"),
            }
        }
    }
}
