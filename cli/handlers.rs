pub fn handle_command(command: Commands, manager: &mut TodoManager) -> TodoResult<()> {
    match command {
        Commands::Add {
            todo,
            due,
            priority,
        } => {
            handle_add_command(manager, &todo, due.as_deref(), priority.as_deref())?;
        }
        Commands::Edit {
            id,
            todo,
            due,
            priority,
        } => {
            handle_edit_command(manager, id, &todo, due.as_deref(), priority.as_deref())?;
        }
        Commands::Toggle { id } => {
            handle_toggle_command(manager, id)?;
        }
        Commands::Delete { id } => {
            handle_delete_command(manager, id)?;
        }
        Commands::ClearList => {
            handle_clear_command(manager)?;
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
            handle_list_command(
                manager,
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
            )?;
        }
    }
    Ok(())
}

fn handle_add_command(
    manager: &mut TodoManager,
    todo: &str,
    due: Option<&str>,
    priority: Option<&str>,
) -> TodoResult<()> {
    manager.add_todo(todo, due, priority)?;
    println!("✅ Todo added!");
    Ok(())
}

fn handle_edit_command(
    manager: &mut TodoManager,
    id: i32,
    todo: &str,
    due: Option<&str>,
    priority: Option<&str>,
) -> TodoResult<()> {
    manager.edit_todo(id, todo, due, priority)?;
    println!("✏️ Todo edited!");
    Ok(())
}

fn handle_toggle_command(manager: &mut TodoManager, id: i32) -> TodoResult<()> {
    manager.toggle_todo(id)?;
    println!("🔄 Status toggled!");
    Ok(())
}

fn handle_delete_command(manager: &mut TodoManager, id: i32) -> TodoResult<()> {
    manager.delete_todo(id)?;
    println!("🗑️ Todo deleted!");
    Ok(())
}

fn handle_clear_command(manager: &mut TodoManager) -> TodoResult<()> {
    let count = manager.clear_all();
    println!("🗑️ Cleared {count} todo(s)!");
    Ok(())
}

fn handle_list_command(
    manager: &mut TodoManager,
    sort_by: Option<String>,
    asc: bool,
    desc: bool,
    only_complete: bool,
    only_pending: bool,
    priority: Option<String>,
    overdue: bool,
    due_today: bool,
    due_tomorrow: bool,
    due_within: Option<i64>,
) -> TodoResult<()> {
    let sort_by = match sort_by {
        Some(s) => SortBy::from_str(&s)?,
        None => SortBy::Due,
    };

    let priority = parse_priority_option(priority.as_deref())?;

    let list_options = crate::todo::manager::ListOptions {
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

    let todos = manager.get_filtered_todos(list_options)?;
    display_todos(&todos);
    Ok(())
}

fn parse_priority_option(priority_str: Option<&str>) -> TodoResult<Option<Priority>> {
    match priority_str {
        Some(p) => {
            Ok(Some(p.parse().map_err(|_| {
                crate::todo::error::TodoError::InvalidPriority
            })?))
        }
        None => Ok(None),
    }
}
