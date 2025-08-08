# Filtering and Sorting Guide

This comprehensive guide explains all filtering and sorting options available in the Rust Todo CLI application.

## Table of Contents

- [Overview](#overview)
- [Status Filters](#status-filters)
- [Priority Filters](#priority-filters)  
- [Time-Based Filters](#time-based-filters)
- [Combining Filters](#combining-filters)
- [Sorting Options](#sorting-options)
- [Advanced Use Cases](#advanced-use-cases)
- [Performance Tips](#performance-tips)
- [Filter Examples](#filter-examples)

## Overview

The Todo CLI provides powerful filtering and sorting capabilities to help you focus on the right tasks at the right time. All filters use **AND logic** - items must match all specified criteria to be displayed.

### Basic Command Structure

```bash
todo list [STATUS_FILTERS] [PRIORITY_FILTERS] [TIME_FILTERS] [SORT_OPTIONS]
```

### Filter Categories

1. **Status Filters**: Filter by completion status
2. **Priority Filters**: Filter by priority level  
3. **Time-Based Filters**: Filter by due dates and time ranges
4. **Sorting**: Order results by different criteria

## Status Filters

### Completion Status

#### Show Only Completed Todos
```bash
todo list --only-complete
```

**Use Cases:**
- Review completed work
- Generate reports of finished tasks  
- Archive or clean up old todos

**Example Output:**
```
ID  ✓ Todo                              Due Date                       Priority
─────────────────────────────────────────────────────────────────────────────────
2   ✅ Project documentation             🟢 10-08-2025 (completed)      🔴 High
4   ✅ Code review                       -                              🟡 Medium
```

#### Show Only Pending Todos
```bash
todo list --only-pending
```

**Use Cases:**
- Focus on remaining work
- Daily planning sessions
- Sprint planning
- Progress tracking

**Example Output:**
```
ID  ✓ Todo                              Due Date                       Priority
─────────────────────────────────────────────────────────────────────────────────
1   ⬜ Team meeting preparation          🟡 09-08-2025 09:00 (today)    🔴 High
3   ⬜ Database optimization             🟢 12-08-2025 (3 days)         🟡 Medium  
5   ⬜ Write unit tests                  🟢 15-08-2025 (6 days)         🟢 Low
```

### Status Filter Logic

- **No status filter**: Shows all todos (completed and pending)
- **--only-complete**: Shows only ✅ completed todos
- **--only-pending**: Shows only ⬜ pending todos
- **Cannot combine**: --only-complete and --only-pending are mutually exclusive

## Priority Filters

### Filter by Priority Level

#### High Priority Only
```bash
todo list --priority high
```

**Use Cases:**
- Focus on urgent tasks
- Crisis management
- Executive reporting
- Critical path identification

#### Medium Priority Only  
```bash
todo list --priority medium
```

**Use Cases:**
- Regular work planning
- Balanced workload management
- Standard sprint items

#### Low Priority Only
```bash
todo list --priority low
```

**Use Cases:**
- Backlog grooming
- Nice-to-have items
- Long-term improvements
- When you have extra time

### Priority Levels

- **high** 🔴: Urgent, critical items
- **medium** 🟡: Standard importance items  
- **low** 🟢: Nice-to-have, low urgency items
- **No filter**: Shows all priorities including items without priority

### Priority Examples

```bash
# Show only high priority pending tasks
todo list --priority high --only-pending

# Show completed medium priority items
todo list --priority medium --only-complete
```

## Time-Based Filters

### Due Date Filters

#### Overdue Items
```bash
todo list --overdue
```

**Shows:** Items with due dates in the past (🔴 red indicator)

**Use Cases:**
- Damage control
- Catching up on missed deadlines
- Priority reassessment
- Stakeholder communication

**Example:**
```
ID  ✓ Todo                              Due Date                       Priority
─────────────────────────────────────────────────────────────────────────────────
1   ⬜ Submit quarterly report           🔴 05-08-2025 17:00 (overdue)  🔴 High
3   ⬜ Client follow-up                  🔴 07-08-2025 14:00 (overdue)  🟡 Medium
```

#### Due Today
```bash
todo list --due-today
```

**Shows:** Items due on the current date (🟡 yellow indicator)

**Use Cases:**
- Daily planning
- Morning standup preparation  
- End-of-day reviews
- Time management

#### Due Tomorrow
```bash
todo list --due-tomorrow
```

**Shows:** Items due the next day (🟢 green indicator)

**Use Cases:**
- Next-day preparation
- Workload balancing
- Resource planning
- Deadline awareness

#### Due Within N Days
```bash
# Due within next 7 days
todo list --due-within 7

# Due within next 30 days  
todo list --due-within 30

# Due within next 3 days
todo list --due-within 3
```

**Shows:** Items due within the specified number of days from today

**Use Cases:**
- Weekly planning (--due-within 7)
- Monthly overview (--due-within 30)
- Short-term focus (--due-within 3)
- Sprint planning (--due-within 14)

### Time Filter Logic

Time filters use **OR logic** within the time category:
- An item appears if it matches ANY active time filter
- Example: `--overdue --due-today` shows items that are either overdue OR due today

### Time Range Examples

```bash
# This week's items (including overdue)
todo list --due-within 7 --overdue

# Today and tomorrow's work
todo list --due-today --due-tomorrow

# Urgent time-sensitive items  
todo list --overdue --due-today --priority high
```

## Combining Filters

### Filter Combination Rules

**Between Categories (AND Logic):**
- Status AND Priority AND Time filters must all match
- Example: `--only-pending --priority high --overdue` shows items that are pending AND high priority AND overdue

**Within Time Category (OR Logic):**
- `--overdue --due-today` shows items that are overdue OR due today

### Practical Combinations

#### Daily Focus (Most Important)
```bash
# Today's high-priority work + overdue items
todo list --priority high --overdue --due-today --only-pending
```

#### Weekly Planning
```bash
# All pending items due within a week, sorted by due date
todo list --only-pending --due-within 7 --sort-by due --asc
```

#### Crisis Management
```bash  
# All overdue high-priority items
todo list --overdue --priority high --only-pending --sort-by due --asc
```

#### End of Sprint Review
```bash
# Recently completed high and medium priority items
todo list --only-complete --priority high --sort-by priority --desc
```

#### Backlog Grooming
```bash
# Low priority items without specific due dates
todo list --priority low --only-pending
```

## Sorting Options

### Sort Criteria

#### Sort by Due Date
```bash
# Earliest first (ascending)
todo list --sort-by due --asc

# Latest first (descending)
todo list --sort-by due --desc
```

**Use Cases:**
- Chronological planning
- Deadline management
- Time-sensitive prioritization

#### Sort by Priority
```bash
# Low to High (ascending: Low, Medium, High)
todo list --sort-by priority --asc

# High to Low (descending: High, Medium, Low)  
todo list --sort-by priority --desc
```

**Use Cases:**
- Importance-based task selection
- Resource allocation
- Executive summaries

#### Combined Sort (Due Date + Priority)
```bash
# Sort by due date first, then by priority
todo list --sort-by due+priority --asc

# Reverse order
todo list --sort-by due+priority --desc
```

**Use Cases:**
- Balanced time and importance prioritization
- Sprint planning
- Comprehensive task organization

### Sort Direction

- **--asc**: Ascending order (default if neither specified)
- **--desc**: Descending order  
- **Cannot combine**: --asc and --desc are mutually exclusive

### Sorting Examples

```bash
# Most urgent first (overdue + high priority, sorted by due date)
todo list --overdue --priority high --sort-by due --asc

# This week's work, importance first
todo list --due-within 7 --sort-by priority --desc --only-pending

# Complete project timeline
todo list --sort-by due+priority --asc
```

## Advanced Use Cases

### Project Management Scenarios

#### Sprint Planning
```bash
# Current sprint items (next 2 weeks)
todo list --due-within 14 --only-pending --sort-by due+priority --asc
```

#### Daily Standup Preparation  
```bash
# Today's work + overdue items
todo list --due-today --overdue --only-pending --sort-by priority --desc
```

#### Weekly Review
```bash
# Completed work this week
todo list --only-complete --sort-by due --desc

# Upcoming week's work
todo list --due-within 7 --only-pending --sort-by due --asc
```

#### Crisis Management
```bash
# Critical overdue items needing immediate attention  
todo list --overdue --priority high --only-pending --sort-by due --asc
```

### Personal Productivity Workflows

#### Getting Things Done (GTD)

**Daily Review:**
```bash
# Today's actions
todo list --due-today --only-pending --sort-by priority --desc
```

**Weekly Review:**
```bash
# Next week's commitments
todo list --due-within 7 --only-pending --sort-by due --asc
```

#### Eisenhower Matrix Simulation

**Urgent & Important (Do First):**
```bash
todo list --priority high --overdue --due-today --only-pending
```

**Important, Not Urgent (Schedule):**
```bash
todo list --priority high --due-within 7 --only-pending --sort-by due --asc
```

**Urgent, Not Important (Delegate/Quick Wins):**
```bash
todo list --priority medium --due-today --due-tomorrow --only-pending  
```

**Neither Urgent nor Important (Eliminate/Later):**
```bash
todo list --priority low --only-pending
```

### Team Collaboration

#### Team Lead Dashboard
```bash
# Team's overdue items requiring attention
todo list --overdue --priority high --only-pending --sort-by due --asc
```

#### Progress Reporting
```bash
# Completed high-priority work  
todo list --priority high --only-complete --sort-by due --desc
```

#### Resource Planning
```bash
# Upcoming workload (next month)
todo list --due-within 30 --only-pending --sort-by due+priority --asc
```

## Performance Tips

### Efficient Filtering

#### Use Specific Filters
```bash
# ✅ Efficient: Specific filter combination
todo list --priority high --due-within 7

# ❌ Less efficient: No filters (shows everything)
todo list
```

#### Combine Complementary Filters
```bash
# ✅ Good: Status + time + priority
todo list --only-pending --due-within 7 --priority high

# ❌ Redundant: Conflicting status filters (impossible combination)
todo list --only-complete --only-pending  # Error: mutually exclusive
```

### Large Dataset Optimization

For large todo lists (100+ items):

```bash
# Use status filters to reduce dataset
todo list --only-pending --priority high

# Use time filters for relevance  
todo list --due-within 7 --overdue

# Sort for quick scanning
todo list --only-pending --sort-by priority --desc
```

### Memory Considerations

- All filtering happens in memory after loading
- Sorting is performed on the filtered subset
- Large files (1000+ todos) load entirely into memory

## Filter Examples

### Scenario-Based Examples

#### Monday Morning Planning
```bash
# What needs attention this week?
todo list --only-pending --overdue --due-within 7 --sort-by due+priority --asc
```

**Expected Output:**
```
ID  ✓ Todo                              Due Date                       Priority  
─────────────────────────────────────────────────────────────────────────────────
3   ⬜ Submit expense report             🔴 05-08-2025 (overdue)        🟡 Medium
1   ⬜ Client presentation prep          🟡 08-08-2025 14:00 (today)    🔴 High
5   ⬜ Code review                       🟢 10-08-2025 (2 days)         🟡 Medium
```

#### Friday Afternoon Cleanup
```bash
# What did I accomplish this week?
todo list --only-complete --sort-by due --desc
```

#### Emergency Mode
```bash
# Critical items needing immediate attention
todo list --overdue --priority high --only-pending --sort-by due --asc
```

#### Capacity Planning
```bash
# What's coming up in the next two weeks?
todo list --due-within 14 --only-pending --sort-by due+priority --asc
```

### Edge Cases and Gotchas

#### Empty Results
```bash
# This might return no results
todo list --priority high --only-complete --overdue
# Reason: Completed items are rarely overdue (contradiction)
```

#### Mutually Exclusive Filters
```bash
# ❌ This will error
todo list --only-complete --only-pending

# ❌ This will error  
todo list --asc --desc
```

#### No Due Date Handling
```bash
# Items without due dates won't appear in time-based filters
todo list --due-within 7  # Won't show items without due dates

# To include all pending items, use:
todo list --only-pending  # Shows all pending, regardless of due date
```

## Quick Reference

### Filter Options
```bash
# Status filters
--only-complete              # Show completed todos only
--only-pending               # Show pending todos only

# Priority filters  
--priority high|medium|low   # Filter by priority level

# Time filters
--overdue                   # Show overdue items
--due-today                 # Show items due today
--due-tomorrow              # Show items due tomorrow  
--due-within <days>         # Show items due within N days

# Sorting
--sort-by due|priority|due+priority  # Sort criteria
--asc                       # Ascending order
--desc                      # Descending order
```

### Common Combinations
```bash
# Daily focus
todo list --overdue --due-today --priority high --only-pending

# Weekly planning  
todo list --due-within 7 --only-pending --sort-by due+priority --asc

# Crisis management
todo list --overdue --priority high --sort-by due --asc

# Progress review
todo list --only-complete --sort-by due --desc

# Backlog review
todo list --priority low --only-pending
```

---

**For more examples and usage patterns, see [USAGE.md](USAGE.md).**
