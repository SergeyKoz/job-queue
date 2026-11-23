use std::collections::HashMap;

struct TaskRow {
    kind: String,
    #[allow(dead_code)]
    priority: u32,
    #[allow(dead_code)]
    max_attempts: u32,
}

#[derive(Debug)]
pub struct RowError {
    pub row: usize,
    pub message: String,
}

fn parse_row(index: usize, row: &str) -> Result<TaskRow, RowError> {
    let fields: Vec<&str> = row.split(',').collect();

    if row.trim().is_empty() {
        return Err(RowError {
            row: index,
            message: "Empty row".to_string(),
        });
    }

    if fields.len() < 3 {
        return Err(RowError {
            row: index,
            message: format!("Expected 3 fields, found {}", fields.len()),
        });
    }

    let kind = fields[0].trim();

    if kind.is_empty() {
        return Err(RowError {
            row: index,
            message: "Kind cannot be empty".to_string(),
        });
    }

    Ok(TaskRow {
        kind: kind.to_string(),
        priority: fields[1].trim().parse().map_err(|_| RowError {
            row: index,
            message: format!("Priority '{}' is not a number", fields[1]),
        })?,
        max_attempts: fields[2].trim().parse().map_err(|_| RowError {
            row: index,
            message: format!("Max_attempts '{}' is not a number", fields[2]),
        })?,
    })
}

pub fn parse_tasks(content: &str) -> (HashMap<String, u32>, Vec<RowError>) {
    let mut tasks = Vec::new();
    let mut parse_errors = Vec::new();

    for (index, row) in content.lines().enumerate() {
        match parse_row(index + 1, row) {
            Ok(task) => tasks.push(task),
            Err(error) => parse_errors.push(error),
        }
    }

    (count_tasks_stat(&tasks), parse_errors)
}

fn count_tasks_stat(tasks: &[TaskRow]) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for task in tasks {
        *counts.entry(task.kind.clone()).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_tasks() {
        let (parsed, _) = parse_tasks("kind1,1,1\nkind1,1,1\nkind2,1,1\nkind3,1,1");
        assert_eq!(parsed["kind1"], 2);
        assert_eq!(parsed["kind2"], 1);
        assert_eq!(parsed["kind3"], 1);
        assert_eq!(parsed.len(), 3);
    }

    #[test]
    fn parse_invalid_rows() {
        let (parsed, errors) = parse_tasks(
            "kind1,1,1\n\nkind1\nkind1,1\n,1,1\nkind1,_,1\nkind1,1,_\nkind1,-1,1\nkind1,1,-1",
        );

        assert_eq!(parsed["kind1"], 1);

        assert_eq!(errors[0].row, 2);
        assert_eq!(errors[0].message, "Empty row");
        assert_eq!(errors[1].row, 3);
        assert_eq!(errors[1].message, "Expected 3 fields, found 1");
        assert_eq!(errors[2].row, 4);
        assert_eq!(errors[2].message, "Expected 3 fields, found 2");
        assert_eq!(errors[3].row, 5);
        assert_eq!(errors[3].message, "Kind cannot be empty");
        assert_eq!(errors[4].row, 6);
        assert_eq!(errors[4].message, "Priority '_' is not a number");
        assert_eq!(errors[5].row, 7);
        assert_eq!(errors[5].message, "Max_attempts '_' is not a number");
        assert_eq!(errors[6].row, 8);
        assert_eq!(errors[6].message, "Priority '-1' is not a number");
        assert_eq!(errors[7].row, 9);
        assert_eq!(errors[7].message, "Max_attempts '-1' is not a number");
    }
}
