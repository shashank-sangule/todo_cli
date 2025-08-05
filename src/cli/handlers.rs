use crate::cli::Commands;
use crate::display::display_todos;
use crate::todo::{ListQuery, SortBy, TodoItem, TodoManager, TodoResult};
use std::str::FromStr;

pub fn handle_commands(command: Commands, manager: &mut TodoManager) -> TodoResult<()> {
    match command {
        Commands::Add {
            title,
            description,
            due_date,
            priority,
            tags,
        } => {
            manager.add_todo(title, description, due_date, priority.as_deref(), tags)?;
        }
        Commands::Edit {
            id,
            title,
            description,
            due_date,
            priority,
            tags,
        } => {
            manager.edit_todo(
                id,
                title,
                description,
                due_date.as_deref(),
                priority.as_deref(),
                tags,
            )?;
        }
        Commands::Toggle { id } => {
            manager.toggle_todo(id)?;
        }
        Commands::Delete { id } => {
            manager.delete_todo(id)?;
        }
        Commands::ClearList => {
            manager.clear_all();
        }
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
            let sort_by = match sort_by {
                Some(s) => SortBy::from_str(&s)?,
                None => SortBy::Due,
            };

            let priority = TodoManager::parse_priority(priority.as_deref())?;

            let query = ListQuery {
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

            handle_list_command(&mut manager.todos, query)?;
        }
    }
    Ok(())
}

fn handle_list_command(todos: &mut [TodoItem], query: ListQuery) -> TodoResult<()> {
    let ascending = query.asc || !query.desc;

    apply_sorting(todos, query.sort_by, ascending)?;

    let filtered_todos = apply_filter(todos, &query)?;

    display_todos(&filtered_todos);
    Ok(())
}

fn apply_sorting(todos: &mut [TodoItem], sort_by: SortBy, ascending: bool) -> TodoResult<()> {
    let comparator = match (sort_by, ascending) {
        (SortBy::Due, true) => |a: &TodoItem, b: &TodoItem| a.due_date().cmp(&b.due_date()),
        (SortBy::Due, false) => |a: &TodoItem, b: &TodoItem| b.due_date().cmp(&a.due_date()),
        (SortBy::Priority, true) => |a: &TodoItem, b: &TodoItem| a.priority().cmp(&b.priority()),
        (SortBy::Priority, false) => |a: &TodoItem, b: &TodoItem| b.priority().cmp(&a.priority()),
        (SortBy::DueThenPriority, true) => |a: &TodoItem, b: &TodoItem| {
            a.due_date()
                .cmp(&b.due_date())
                .then(a.priority().cmp(&b.priority()))
        },
        (SortBy::DueThenPriority, false) => |a: &TodoItem, b: &TodoItem| {
            b.due_date()
                .cmp(&a.due_date())
                .then(b.priority().cmp(&a.priority()))
        },
    };

    todos.sort_by(comparator);
    Ok(())
}

fn apply_filter(todos: &[TodoItem], list_options: &ListQuery) -> TodoResult<Vec<TodoItem>> {
    let filtered: Vec<TodoItem> = todos
        .iter()
        .filter(|todo| list_options.item_passes_filters(todo))
        .cloned()
        .collect();

    Ok(filtered)
}
