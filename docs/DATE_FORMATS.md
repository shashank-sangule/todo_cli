# Date Format Reference

This guide explains all supported date formats for the `--due-date` option in the Rust Todo CLI application.

## Table of Contents

- [Overview](#overview)
- [Absolute Date Formats](#absolute-date-formats)
- [Natural Language Dates](#natural-language-dates)
- [Time Specification](#time-specification)
- [Examples by Use Case](#examples-by-use-case)
- [Date Validation](#date-validation)
- [Timezone Handling](#timezone-handling)
- [Common Mistakes](#common-mistakes)

## Overview

The Todo CLI supports two main categories of date input:

1. **Absolute Dates**: Specific dates in various standard formats
2. **Natural Language**: Human-readable relative dates like "tomorrow" or "next week"

All dates are parsed and stored in local system timezone.

## Absolute Date Formats

### Standard Formats

#### DD-MM-YYYY Format
```bash
# European/international format
todo add "Doctor appointment" --due-date "25-12-2025"
todo add "Project deadline" --due-date "15-08-2025"
todo add "Meeting" --due-date "01-01-2026"
```

#### DD/MM/YYYY Format
```bash
# Alternative separator
todo add "Conference call" --due-date "25/12/2025"
todo add "Review meeting" --due-date "15/08/2025"
```

#### ISO 8601 Format (YYYY-MM-DD)
```bash
# International standard format
todo add "Product launch" --due-date "2025-12-25"
todo add "Quarterly review" --due-date "2025-08-15"
todo add "Annual planning" --due-date "2026-01-01"
```

#### YYYY/MM/DD Format
```bash
# Alternative ISO format
todo add "Team retreat" --due-date "2025/12/25"
```

### Date with Time

#### DD-MM-YYYY HH:MM Format
```bash
# European format with time
todo add "Client meeting" --due-date "25-12-2025 14:30"
todo add "Standup meeting" --due-date "15-08-2025 09:00"
todo add "Code review" --due-date "01-09-2025 16:45"
```

#### DD/MM/YYYY HH:MM Format
```bash
# Alternative with time
todo add "Presentation" --due-date "25/12/2025 14:30"
```

#### ISO Format with Time
```bash
# ISO 8601 with time
todo add "Database maintenance" --due-date "2025-12-25 14:30"
todo add "Server update" --due-date "2025-08-15 02:00"
```

### Time-Only Formats

When only time is specified, it defaults to today:
```bash
# These formats are NOT directly supported
# Use natural language instead: "today at 14:30"
```

## Natural Language Dates

The Todo CLI supports various natural language date expressions through the `chrono-english` crate.

### Relative Days

#### Basic Relative Dates
```bash
# Tomorrow
todo add "Follow up email" --due-date "tomorrow"

# Yesterday (will be rejected as it's in the past)
todo add "Review notes" --due-date "yesterday"  # ❌ Error: Date is in the past
```

#### Specific Day References
```bash
# Today (explicit)
todo add "Daily report" --due-date "today"

# Day of week (next occurrence)
todo add "Team meeting" --due-date "monday"
todo add "Weekly review" --due-date "friday"
todo add "Weekend planning" --due-date "saturday"
```

### Relative Weeks

```bash
# Next week (7 days from now)
todo add "Sprint planning" --due-date "next week"

# Last week (will be rejected)
todo add "Retrospective notes" --due-date "last week"  # ❌ Error: Date is in the past
```

### Relative Time Expressions

```bash
# Days from now
todo add "Quarterly review" --due-date "in 3 days"
todo add "Monthly report" --due-date "in 7 days"
todo add "Project deadline" --due-date "in 30 days"

# Weeks from now
todo add "Conference preparation" --due-date "in 2 weeks"
todo add "Vacation planning" --due-date "in 4 weeks"
```

### Complex Expressions

```bash
# Specific future dates
todo add "Q4 planning" --due-date "next monday"
todo add "Budget review" --due-date "next friday"

# Month references (if supported by chrono-english)
todo add "Annual review" --due-date "next month"
```

## Time Specification

### Default Time Behavior

When no time is specified:
- **Absolute dates**: Default to 00:00 (midnight)
- **Natural language**: Default to 00:00 (midnight)

### Explicit Time

```bash
# Morning meetings
todo add "Standup" --due-date "tomorrow 09:00"
todo add "Planning meeting" --due-date "25-12-2025 09:30"

# Afternoon tasks
todo add "Client call" --due-date "next monday 14:00"
todo add "Presentation" --due-date "15-08-2025 15:30"

# Evening deadlines
todo add "Report submission" --due-date "friday 17:00"
todo add "Code review" --due-date "2025-08-20 18:30"
```

### 24-Hour Format

All times use 24-hour format:
```bash
# Correct format
todo add "Early meeting" --due-date "tomorrow 06:30"
todo add "Late deadline" --due-date "friday 23:59"

# 12-hour format is NOT supported
todo add "Lunch meeting" --due-date "tomorrow 2:30 PM"  # ❌ Will not parse correctly
```

## Examples by Use Case

### Daily Planning

```bash
# Today's tasks
todo add "Morning standup" --due-date "today 09:00"
todo add "Code review" --due-date "today 14:00"
todo add "End of day summary" --due-date "today 17:30"

# Tomorrow's preparation
todo add "Prepare presentation" --due-date "tomorrow"
todo add "Client call preparation" --due-date "tomorrow 08:30"
```

### Weekly Planning

```bash
# Start of week
todo add "Weekly goals setting" --due-date "monday 09:00"
todo add "Team sync" --due-date "monday 10:00"

# Mid-week check-ins
todo add "Progress review" --due-date "wednesday 15:00"
todo add "Stakeholder update" --due-date "thursday 16:00"

# End of week
todo add "Weekly retrospective" --due-date "friday 16:00"
todo add "Week summary report" --due-date "friday 17:00"
```

### Project Management

```bash
# Project milestones
todo add "Requirements complete" --due-date "2025-08-20"
todo add "Design review" --due-date "2025-08-25 14:00"
todo add "Development complete" --due-date "2025-09-15"
todo add "Testing phase" --due-date "2025-09-25"
todo add "Go-live" --due-date "2025-10-01 09:00"
```

### Recurring Patterns

```bash
# Weekly recurring (manual creation)
todo add "Weekly team meeting" --due-date "monday 10:00"
todo add "Weekly report" --due-date "friday 16:00"

# Monthly patterns
todo add "Monthly review" --due-date "2025-09-01"
todo add "Budget check" --due-date "2025-09-30"
```

## Date Validation

### Validation Rules

1. **No Past Dates**: Dates in the past are rejected
2. **Reasonable Future**: Very far future dates (>10 years) may be warned
3. **Valid Calendar Dates**: Invalid dates like "32-01-2025" are rejected
4. **Time Bounds**: Times must be in 00:00-23:59 format

### Validation Examples

```bash
# ✅ Valid dates
todo add "Future task" --due-date "tomorrow"
todo add "Next week task" --due-date "next monday"
todo add "Specific date" --due-date "25-12-2025 14:30"

# ❌ Invalid dates
todo add "Past task" --due-date "yesterday"           # Past date
todo add "Bad format" --due-date "December 25th"      # Invalid format
todo add "Invalid date" --due-date "32-01-2025"       # Invalid calendar date
todo add "Bad time" --due-date "tomorrow 25:00"       # Invalid time
```

### Error Messages

When date parsing fails, you'll see helpful error messages:

```
❌ Invalid date format: 'December 25th'. Use: dd-mm-YYYY HH:MM or natural language like 'tomorrow'
❌ Date is in the past: yesterday
❌ Invalid date: 32-01-2025. Reason: Invalid calendar date
```

## Timezone Handling

### Current Behavior

- All dates are interpreted in **local system timezone**
- No explicit timezone support currently
- Dates are stored and displayed in local time

### Examples

```bash
# If your system is in EST (UTC-5)
todo add "Morning meeting" --due-date "tomorrow 09:00"
# Stored as: 2025-08-09 09:00:00 EST

# If your system is in PST (UTC-8)
todo add "Morning meeting" --due-date "tomorrow 09:00"  
# Stored as: 2025-08-09 09:00:00 PST
```

### Future Timezone Support

Planned features for future versions:
```bash
# Explicit timezone specification (future feature)
todo add "Global meeting" --due-date "tomorrow 09:00 UTC"
todo add "Conference call" --due-date "2025-08-15 14:00 EST"
```

## Common Mistakes

### Format Confusion

```bash
# ❌ American MM/DD/YYYY format
todo add "Task" --due-date "08/15/2025"  # Interpreted as 8th day of 15th month

# ✅ Use DD/MM/YYYY or ISO format
todo add "Task" --due-date "15/08/2025"  # 15th August 2025
todo add "Task" --due-date "2025-08-15"  # ISO format, unambiguous
```

### Time Format Issues

```bash
# ❌ 12-hour format with AM/PM
todo add "Meeting" --due-date "tomorrow 2:30 PM"

# ✅ 24-hour format
todo add "Meeting" --due-date "tomorrow 14:30"
```

### Natural Language Limitations

```bash
# ❌ These expressions may not work
todo add "Task" --due-date "day after tomorrow"
todo add "Task" --due-date "in a couple of days"
todo add "Task" --due-date "end of month"

# ✅ Use specific expressions
todo add "Task" --due-date "in 2 days"
todo add "Task" --due-date "2025-08-31"
```

### Past Date Attempts

```bash
# ❌ These will be rejected
todo add "Meeting notes" --due-date "yesterday"
todo add "Last week task" --due-date "last friday"

# ✅ Only future dates allowed
todo add "Follow-up" --due-date "tomorrow"
todo add "Next review" --due-date "next friday"
```

## Quick Reference

### Absolute Formats
```bash
"DD-MM-YYYY"           # 25-12-2025
"DD/MM/YYYY"           # 25/12/2025  
"YYYY-MM-DD"           # 2025-12-25 (ISO)
"DD-MM-YYYY HH:MM"     # 25-12-2025 14:30
"YYYY-MM-DD HH:MM"     # 2025-12-25 14:30
```

### Natural Language
```bash
"tomorrow"             # Next day
"today"                # Current day
"monday"               # Next Monday
"next week"            # 7 days from now
"in 3 days"            # 3 days from now
"next monday"          # Next Monday specifically
```

### Time Specification
```bash
"HH:MM"                # 24-hour format only
"09:00"                # 9 AM
"14:30"                # 2:30 PM
"23:59"                # 11:59 PM
```

---

## Testing Date Formats

You can test date parsing without creating todos:

```bash
# Add and immediately delete to test parsing
todo add "Test" --due-date "your-date-here"
todo list  # Check if date appears correctly
todo delete 1  # Clean up
```

**For more examples and usage patterns, see [USAGE.md](USAGE.md).**
