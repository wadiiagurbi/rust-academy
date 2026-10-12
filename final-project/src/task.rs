use std::fmt;
use std::str::FromStr;

/// Lifecycle status of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Status {
    #[default]
    Todo,
    Doing,
    Done,
}

impl FromStr for Status {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "todo" => Ok(Self::Todo),
            "doing" => Ok(Self::Doing),
            "done" => Ok(Self::Done),
            other => anyhow::bail!("invalid status `{other}`: expected todo|doing|done"),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Todo => "todo",
            Self::Doing => "doing",
            Self::Done => "done",
        };
        write!(f, "{s}")
    }
}

/// Importance of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
}

impl FromStr for Priority {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            other => anyhow::bail!("invalid priority `{other}`: expected low|medium|high"),
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        };
        write!(f, "{s}")
    }
}

/// A single task row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub status: Status,
    pub priority: Priority,
    pub created_at: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} [status: {}, priority: {}] (created: {})",
            self.id, self.title, self.status, self.priority, self.created_at
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_parses_all_variants() {
        assert_eq!("todo".parse::<Status>().unwrap(), Status::Todo);
        assert_eq!("doing".parse::<Status>().unwrap(), Status::Doing);
        assert_eq!("done".parse::<Status>().unwrap(), Status::Done);
    }

    #[test]
    fn status_parsing_is_case_insensitive() {
        assert_eq!("TODO".parse::<Status>().unwrap(), Status::Todo);
        assert_eq!("Doing".parse::<Status>().unwrap(), Status::Doing);
        assert_eq!("DONE".parse::<Status>().unwrap(), Status::Done);
    }

    #[test]
    fn status_rejects_invalid_values() {
        assert!("finished".parse::<Status>().is_err());
        assert!("".parse::<Status>().is_err());
        assert!("to-do".parse::<Status>().is_err());
    }

    #[test]
    fn status_display_roundtrips_through_parse() {
        for status in [Status::Todo, Status::Doing, Status::Done] {
            let text = status.to_string();
            assert_eq!(text.parse::<Status>().unwrap(), status);
        }
    }

    #[test]
    fn status_display_uses_lowercase_words() {
        assert_eq!(Status::Todo.to_string(), "todo");
        assert_eq!(Status::Doing.to_string(), "doing");
        assert_eq!(Status::Done.to_string(), "done");
    }

    #[test]
    fn priority_parses_all_variants() {
        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
    }

    #[test]
    fn priority_parsing_is_case_insensitive() {
        assert_eq!("LOW".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("Medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("HIGH".parse::<Priority>().unwrap(), Priority::High);
    }

    #[test]
    fn priority_rejects_invalid_values() {
        assert!("urgent".parse::<Priority>().is_err());
        assert!("".parse::<Priority>().is_err());
        assert!("med".parse::<Priority>().is_err());
    }

    #[test]
    fn priority_display_roundtrips_through_parse() {
        for priority in [Priority::Low, Priority::Medium, Priority::High] {
            let text = priority.to_string();
            assert_eq!(text.parse::<Priority>().unwrap(), priority);
        }
    }

    #[test]
    fn priority_display_uses_lowercase_words() {
        assert_eq!(Priority::Low.to_string(), "low");
        assert_eq!(Priority::Medium.to_string(), "medium");
        assert_eq!(Priority::High.to_string(), "high");
    }

    #[test]
    fn task_display_contains_all_fields() {
        let task = Task {
            id: 7,
            title: String::from("Write docs"),
            status: Status::Doing,
            priority: Priority::High,
            created_at: String::from("2026-09-18 10:00:00"),
        };
        let text = task.to_string();
        assert!(text.contains("[7]"));
        assert!(text.contains("Write docs"));
        assert!(text.contains("doing"));
        assert!(text.contains("high"));
        assert!(text.contains("2026-09-18 10:00:00"));
    }
}
