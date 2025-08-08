# Usage Guide

This guide provides comprehensive examples and usage patterns for the Rust Todo CLI application.

## Table of Contents

- [Basic Operations](#basic-operations)
- [Advanced Filtering](#advanced-filtering)
- [Date Handling](#date-handling)
- [Priority Management](#priority-management)
- [Tag Organization](#tag-organization)
- [Sorting Options](#sorting-options)
- [File Management](#file-management)
- [Common Workflows](#common-workflows)
- [Troubleshooting](#troubleshooting)

## Basic Operations

### Adding Todos

#### Simple Todo
```bash
todo add "Buy groceries"
```
**Output:**
```
✅ Todo added with ID: 1
```

#### Todo with Description
```bash
todo add "Complete project report" --description "Include charts and analysis sections"
```

#### Todo with All Options
```bash
todo add "Team presentation" \
  --priority high \
  --due-date "2025-08-15 14:00" \
  --description "Present Q3 results to stakeholders" \
  --tags "work,presentation,urgent"
```

### Viewing Todos

#### List All Todos
```bash
todo list
```
**Output:**
```
ID  ✓ Todo                              Due Date                       Priority
─────────────────────────────────────────────────────────────────────────────────
1   ⬜ Buy groceries                     -                              -
2   ⬜ Complete project report           -                              -
3   ⬜ Team presentation                 🔴 15-08-2025 14:00 (overdue)  🔴 High
```

#### List with Custom File
```bash
todo --file work-todos.json list
```

### Managing Todos

#### Toggle Completion Status
```bash
todo toggle 1
```
**Output:**
```
✅ Todo 1 marked as completed!
```

#### Edit Todo
```bash
# Change priority
todo edit 2 --priority high

# Update multiple fields
todo edit 3 --title "Updated title" --description "New description" --priority medium
```

#### Delete Todo
```bash
todo delete 1
```
**Output:**
```
🗑️ Todo 1 deleted!
```

#### Clear All Todos
```bash
todo clear-list
```
**Output:**
```
🗑️ Cleared 3 todo(s)!
```

## Advanced Filtering

### Status Filters

#### Show Only Completed Todos
```bash
todo list --only-complete
```

#### Show Only Pending Todos
```bash
todo list --only-pending
```

### Priority Filters

#### Filter by High Priority
```bash
todo list --priority high
```

#### Combine Status and Priority
```bash
todo list --only-pending --priority high
```

### Time-Based Filters

#### Show Overdue Todos
```bash
todo list --overdue
```
**Output:**
```
ID  ✓ Todo                              Due Date                       Priority
─────────────────────────────────────────────────────────────────────────────────
3   ⬜ Team presentation                 🔴 15-08-2025 14:00 (overdue)  🔴 High
5   ⬜ Submit report                     🔴 10-08-2025 09:00 (overdue)  🟡 Medium
```

#### Show Todos Due Today
```bash
todo list --due-today
```

#### Show Todos Due Tomorrow
```bash
todo list --due-tomorrow
```

#### Show Todos Due Within N Days
```bash
# Show todos due within the next 7 days
todo list --due-within 7

# Show todos due within the next 30 days
todo list --due-within 30
```

### Complex Filter Combinations

#### Urgent Items (High Priority, Due Soon)
```bash
todo list --priority high --due-within 3 --only-pending
```

#### Weekly Review (All Incomplete Items)
```bash
todo list --only-pending --sort-by due+priority --asc
```

## Date Handling

### Supported Date Formats

#### Absolute Dates
```bash
# DD-MM-YYYY format
todo add "Doctor appointment" --due-date "25-12-2025"

# DD-MM-YYYY HH:MM format
todo add "Meeting with client" --due-date "25-12-2025 14:30"

# ISO format (YYYY-MM-DD)
todo add "Project deadline" --due-date "2025-12-25"

# ISO with time
todo add "Conference call" --due-date "2025-12-25 09:15"
```

#### Natural Language Dates
```bash
# Relative dates
todo add "Follow up email" --due-date "tomorrow"
todo add "Weekly report" --due-date "next week"

# English expressions (requires chrono-english)
todo add "Quarterly review" --due-date "in 3 days"
todo add "Annual planning" --due-date "next monday"
```

### Date Examples with Output

```bash
# Adding todos with various date formats
todo add "Task 1" --due-date "tomorrow"
todo add "Task 2" --due-date "2025-08-15 14:00"
todo add "Task 3" --due-date "next week"

todo list
```
**Output:**
```
ID  ✓ Todo                              Due Date                       Priority
─────────────────────────────────────────────────────────────────────────────────
1   ⬜ Task 1                            🟡 09:00 (today)               -
2   ⬜ Task 2                            🟢 15-08 14:00 (5 days)        -
3   ⬜ Task 3                            🟢 15-08-2025                  -
```

### Date Color Coding

- 🔴 **Red**: Overdue items
- 🟡 **Yellow**: Due today
- 🟢 **Green**: Due tomorrow or within a week
- ⚪ **White**: Due more than a week away
- `-`: No due date set

## Priority Management

### Setting Priorities

#### During Creation
```bash
todo add "Critical bug fix" --priority high
todo add "Update documentation" --priority medium
todo add "Organize desk" --priority low
```

#### Updating Existing Todos
```bash
todo edit 1 --priority high
todo edit 2 --priority medium
```

### Priority Display

Priorities are displayed with color-coded emojis:
- 🔴 **High**: Urgent, important items
- 🟡 **Medium**: Standard priority items
- 🟢 **Low**: Nice-to-have items
- `-`: No priority set

### Priority-Based Workflows

#### Daily Planning (High Priority Items)
```bash
todo list --priority high --only-pending
```

#### Weekly Review (All Priorities, Sorted)
```bash
todo list --sort-by priority --desc
```

## Tag Organization

### Adding Tags

#### Single Tag
```bash
todo add "Code review" --tags "work"
```

#### Multiple Tags
```bash
todo add "Grocery shopping" --tags "personal,shopping,weekend"
todo add "Client meeting" --tags "work,meeting,urgent"
```

### Tag Examples by Category

#### Work Tags
```bash
todo add "Sprint planning" --tags "work,agile,meeting"
todo add "Deploy to production" --tags "work,deployment,urgent"
todo add "Write unit tests" --tags "work,testing,development"
```

#### Personal Tags
```bash
todo add "Gym session" --tags "personal,health,fitness"
todo add "Call family" --tags "personal,family,communication"
todo add "Read book" --tags "personal,reading,education"
```

#### Project Tags
```bash
todo add "Database design" --tags "project-alpha,database,architecture"
todo add "UI mockups" --tags "project-alpha,design,frontend"
```

### Tag-Based Organization

While the current version doesn't support tag filtering directly, you can organize todos using consistent tagging patterns:

```bash
# Consistent work tags
todo add "Task 1" --tags "work,backend,api"
todo add "Task 2" --tags "work,frontend,ui"
todo add "Task 3" --tags "work,testing,qa"

# View all todos to see tag organization
todo list
```

## Sorting Options

### Basic Sorting

#### Sort by Due Date
```bash
# Ascending (earliest first)
todo list --sort-by due --asc

# Descending (latest first)  
todo list --sort-by due --desc
```

#### Sort by Priority
```bash
# Ascending (low to high)
todo list --sort-by priority --asc

# Descending (high to low)
todo list --sort-by priority --desc
```

#### Combined Sort (Due Date + Priority)
```bash
# Sort by due date first, then by priority
todo list --sort-by due+priority --asc
```

### Sorting with Filters

#### Most Urgent Tasks
```bash
# High priority items, sorted by due date
todo list --priority high --sort-by due --asc
```

#### Overdue Items by Priority
```bash
# Overdue items, highest priority first
todo list --overdue --sort-by priority --desc
```

#### Weekly Planning View
```bash
# Pending items due within 7 days, sorted by due date then priority
todo list --only-pending --due-within 7 --sort-by due+priority --asc
```

## File Management

### Using Custom Files

#### Separate Work and Personal Todos
```bash
# Work todos
todo --file work.json add "Sprint retrospective" --tags "work,agile"
todo --file work.json list

# Personal todos
todo --file personal.json add "Dentist appointment" --tags "personal,health"
todo --file personal.json list
```

#### Project-Specific Todo Files
```bash
# Project Alpha todos
todo --file project-alpha.json add "API documentation" --priority high

# Project Beta todos  
todo --file project-beta.json add "User testing" --priority medium
```

### File Locations

#### Default Location
```bash
# Uses ./todo_list.json in current directory
todo list
```

#### Absolute Paths
```bash
todo --file /home/user/documents/todos.json add "Important task"
```

#### Relative Paths
```bash
todo --file ../shared/team-todos.json add "Team building event"
```

## Common Workflows

### Daily Planning Workflow

#### 1. Review Overdue Items
```bash
todo list --overdue
```

#### 2. Check Today's Tasks
```bash
todo list --due-today --only-pending
```

#### 3. Plan Tomorrow
```bash
todo list --due-tomorrow --only-pending
```

#### 4. Add New Tasks
```bash
todo add "Daily standup" --due-date "tomorrow 09:00" --tags "work,meeting"
```

### Weekly Review Workflow

#### 1. Review Completed Items
```bash
todo list --only-complete
```

#### 2. Check Pending Items
```bash
todo list --only-pending --sort-by due+priority
```

#### 3. Identify Overdue Items
```bash
todo list --overdue --sort-by priority --desc
```

#### 4. Plan Next Week
```bash
todo list --due-within 7 --sort-by due
```

### Project Management Workflow

#### 1. Add Project Tasks
```bash
todo add "Requirements gathering" --tags "project-x,analysis" --priority high
todo add "Architecture design" --tags "project-x,design" --priority high
todo add "Implementation" --tags "project-x,development" --priority medium
todo add "Testing" --tags "project-x,qa" --priority medium
todo add "Deployment" --tags "project-x,ops" --priority low
```

#### 2. Track Progress
```bash
# View all project tasks
todo list --sort-by priority --desc

# Mark tasks as complete
todo toggle 1  # Requirements complete
todo toggle 2  # Architecture complete
```

#### 3. Adjust Priorities
```bash
# Increase priority as deadline approaches
todo edit 3 --priority high  # Implementation becomes urgent
```

### GTD (Getting Things Done) Workflow

#### 1. Capture Everything
```bash
todo add "Review email" --tags "inbox"
todo add "Call plumber" --tags "inbox"
todo add "Research vacation spots" --tags "inbox"
```

#### 2. Process and Organize
```bash
# Process inbox items
todo edit 1 --tags "work,communication" --priority medium --due-date "today"
todo edit 2 --tags "personal,home" --priority high --due-date "tomorrow"
todo edit 3 --tags "personal,planning" --priority low --due-date "next week"
```

#### 3. Review and Do
```bash
# Daily review
todo list --due-today --sort-by priority --desc

# Weekly review
todo list --due-within 7 --sort-by due+priority
```

## Troubleshooting

### Common Issues

#### Issue: "Todo with ID X not found"
```bash
# Check available todos first
todo list

# Use the correct ID from the list
todo toggle 1  # Use actual ID from list
```

#### Issue: "Invalid date format"
```bash
# ❌ Wrong format
todo add "Task" --due-date "August 15th"

# ✅ Correct formats
todo add "Task" --due-date "15-08-2025"
todo add "Task" --due-date "2025-08-15 14:00"
todo add "Task" --due-date "tomorrow"
```

#### Issue: "Invalid priority"
```bash
# ❌ Wrong priority
todo add "Task" --priority urgent

# ✅ Correct priorities
todo add "Task" --priority high
todo add "Task" --priority medium  
todo add "Task" --priority low
```

#### Issue: "Todo cannot be empty"
```bash
# ❌ Empty title
todo add ""

# ✅ Valid title
todo add "Valid task title"
```

#### Issue: "Todo too long"
```bash
# ❌ Title too long (>140 characters)
todo add "This is an extremely long todo title that exceeds the maximum allowed length and will be rejected by the validation system"

# ✅ Appropriate length title
todo add "Reasonable length title" --description "Longer details go in description"
```

### Error Messages Guide

#### File Permission Issues
```
❌ Failed to write file 'todo_list.json'
```
**Solution:** Check file permissions and disk space.

#### JSON Corruption
```
❌ Failed to save/load todos
```
**Solution:** Check if the JSON file is corrupted. Back up and recreate if needed.

#### Invalid Arguments
```
❌ Invalid sort field: 'invalid'. Available: due, priority, due+priority
```
**Solution:** Use one of the valid sort options.

### Performance Tips

#### Large Todo Lists
- Use filtering to reduce displayed items: `todo list --due-within 7`
- Sort efficiently: `todo list --sort-by due+priority --asc`
- Consider using separate files for different contexts

#### File Management
- Use descriptive file names: `project-alpha-todos.json`
- Keep separate files for different contexts (work, personal, projects)
- Regular cleanup of completed items

#### Search Strategies
Since direct search isn't available yet, use these patterns:
- Consistent tag naming for easy identification
- Descriptive titles that are easy to scan
- Meaningful due dates for time-based organization

### Getting Help

#### Command Help
```bash
# General help
todo --help

# Command-specific help
todo add --help
todo list --help
```

#### Version Information
```bash
todo --version
```

#### Debug Information
```bash
# Enable debug logging (if supported)
RUST_LOG=debug todo list
```

---

## Quick Reference Card

### Essential Commands
```bash
# Basic operations
todo add "title" [--priority high|medium|low] [--due-date "date"] [--description "desc"] [--tags "tag1,tag2"]
todo list [--only-complete|--only-pending] [--priority high|medium|low] [--sort-by due|priority|due+priority] [--asc|--desc]
todo toggle <id>
todo edit <id> [--title "new"] [--priority high|medium|low] [--due-date "date"] [--description "desc"] [--tags "tags"]
todo delete <id>
todo clear-list

# Filtering
--only-complete          # Show completed todos
--only-pending           # Show pending todos  
--priority high|medium|low  # Filter by priority
--overdue               # Show overdue items
--due-today             # Show items due today
--due-tomorrow          # Show items due tomorrow
--due-within <days>     # Show items due within N days

# Sorting
--sort-by due|priority|due+priority  # Sort criteria
--asc                   # Ascending order
--desc                  # Descending order

# File management
--file <path>           # Use custom file
```

### Date Formats
- `25-12-2025` or `25/12/2025`
- `2025-12-25` (ISO format)  
- `25-12-2025 14:30` (with time)
- `tomorrow`, `next week` (natural language)

### Priority Levels
- `high` 🔴 - Urgent items
- `medium` 🟡 - Standard items  
- `low` 🟢 - Nice-to-have items

---

**For more advanced usage and API documentation, see [docs.rs/todo_app](https://docs.rs/todo_app)**
