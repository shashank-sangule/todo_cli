use crate::todo::item::{Priority, SortBy, TodoItem};
use chrono::Local;

pub struct ListQuery {
    pub sort_by: SortBy,
    pub asc: bool,
    pub desc: bool,
    pub only_complete: bool,
    pub only_pending: bool,
    pub priority: Option<Priority>,
    pub overdue: bool,
    pub due_today: bool,
    pub due_tomorrow: bool,
    pub due_within: Option<i64>,
}

impl ListQuery {
    // Check if any filters are actually enabled
    pub fn has_any_filters(&self) -> bool {
        self.only_complete
            || self.only_pending
            || self.priority.is_some()
            || self.overdue
            || self.due_today
            || self.due_tomorrow
            || self.due_within.is_some()
    }

    // Check if item passes ALL active filters (AND logic)
    pub fn item_passes_filters(&self, item: &TodoItem) -> bool {
        // If no filters are set, include all items
        if !self.has_any_filters() {
            return true;
        }

        // Check each filter type - item must pass ALL active filters
        self.passes_status_filter(item)
            && self.passes_priority_filter(item)
            && self.passes_time_filter(item)
    }

    pub fn passes_status_filter(&self, item: &TodoItem) -> bool {
        match (self.only_complete, self.only_pending) {
            (true, false) => item.completed(),
            (false, true) => !item.completed(),
            _ => true,
        }
    }

    pub fn passes_priority_filter(&self, item: &TodoItem) -> bool {
        match self.priority {
            Some(required_priority) => item.priority() == Some(required_priority),
            None => true, // No priority filter
        }
    }

    pub fn passes_time_filter(&self, item: &TodoItem) -> bool {
        // If no time filters are set, pass
        if !self.overdue && !self.due_today && !self.due_tomorrow && self.due_within.is_none() {
            return true;
        }

        // Check if item matches ANY of the active time filters (OR logic within time category)
        self.is_overdue(item)
            || self.is_due_today(item)
            || self.is_due_tomorrow(item)
            || self.is_due_within(item)
    }

    // Individual check methods - only check the condition, don't check if filter is active
    pub fn is_due_today(&self, item: &TodoItem) -> bool {
        if !self.due_today {
            return false;
        }
        item.due_date().map(|d| d.date()) == Some(Local::now().naive_local().date())
    }

    pub fn is_due_tomorrow(&self, item: &TodoItem) -> bool {
        if !self.due_tomorrow {
            return false;
        }
        item.due_date().map(|d| d.date())
            == Some(Local::now().naive_local().date() + chrono::Duration::days(1))
    }

    pub fn is_overdue(&self, item: &TodoItem) -> bool {
        if !self.overdue {
            return false;
        }
        item.due_date()
            .is_some_and(|d| d < Local::now().naive_local())
    }

    pub fn is_due_within(&self, item: &TodoItem) -> bool {
        if let Some(days) = self.due_within {
            item.due_date().is_some_and(|d| {
                let now = Local::now().naive_local();
                let date = d.date();
                date >= now.date() && date <= now.date() + chrono::Duration::days(days)
            })
        } else {
            false
        }
    }
}
