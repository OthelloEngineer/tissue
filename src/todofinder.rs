use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum IssueType {
    Bug,
    Feature,
    Improvement,
    Other,
}
impl IssueType {
    pub fn as_str(&self) -> &str {
        match self {
            IssueType::Bug => "bug",
            IssueType::Feature => "feature",
            IssueType::Improvement => "improvement",
            IssueType::Other => "other",
        }
    }
}
pub fn parse_issue_type(issue_type: &str) -> IssueType {
    match issue_type {
        "bug" => IssueType::Bug,
        "feature" => IssueType::Feature,
        "improvement" => IssueType::Improvement,
        _ => IssueType::Other,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ToDo {
    pub submission: Submission,
    pub description: String,
    pub assigned: Option<String>,
    pub issue_type: IssueType,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Submission {
    pub line_number: usize,
    pub line: String,
    pub file_path: String,
    pub issuer: String,
    pub date: String,
}

pub fn is_to_do(line: &str) -> bool {
    line.to_lowercase().contains("todo")
}

// TODO bug: this function is not implemented; Assigned OthelloEngineer.
// TODO feature: implement this function; Assigned OthelloEngineer.
pub fn parse_submission(submission: Submission) -> Result<ToDo, String> {
    let lower_line = submission.line.to_lowercase();
    let original_words: Vec<&str> = submission.line.split_whitespace().collect();
    let words: Vec<&str> = lower_line.split_whitespace().collect();
    let issue_type = find_issue_type(words[1])?;
    let mut assigned = None;
    let mut description = String::from("");

    let mut description_finished = false;
    let next_word = 3;
    for (i, word) in words[2..].iter().enumerate() {
        if description_finished {
            // To allow ; in description. If description is finished, then the rest word must be assigned.
            assigned = Some(original_words[i + next_word..].join(" "));
            break;
        }

        if word.ends_with(";") && words[i + 3].eq("assigned") {
            description_finished = true;
            let no_semicolon_word = word.trim_end_matches(";");
            description.push_str(format!("{}", no_semicolon_word).as_str());
            continue;
        }
        description.push_str(format!("{} ", word).as_str());
    }

    if description.is_empty() {
        return Err(format!(
            "No description found after {} {} at line nr. {} in {} \n comment: '{}'",
            words[0], words[1], submission.line_number, submission.file_path, submission.line
        ));
    }

    Ok(ToDo {
        submission: submission,
        description: description.trim().to_string(),
        assigned: assigned,
        issue_type: issue_type,
    })
}

fn find_issue_type(line: &str) -> Result<IssueType, String> {
    if !line.ends_with(":") {
        return Err(format!(
            "Cannot confirm issue type. Does not end with ':'. Found issue type {}",
            line
        ));
    }
    match line {
        "bug:" => Ok(IssueType::Bug),
        "feature:" => Ok(IssueType::Feature),
        "improvement:" => Ok(IssueType::Improvement),
        _ => Ok(IssueType::Other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_to_do() {
        assert_eq!(is_to_do("TODO: implement this function"), true);
        assert_eq!(is_to_do("todo: implement this function"), true);
        assert_eq!(is_to_do("to do: implement this function"), false);
        assert_eq!(is_to_do("to do implement this function"), false);
    }

    #[test]
    fn test_find_issue_type() {
        assert_eq!(find_issue_type("bug:"), Ok(IssueType::Bug));
        assert_eq!(find_issue_type("feature:"), Ok(IssueType::Feature));
        assert_eq!(find_issue_type("improvement:"), Ok(IssueType::Improvement));
        assert_eq!(find_issue_type("other:"), Ok(IssueType::Other));
        assert_eq!(
            find_issue_type("other"),
            Err(
                "Cannot confirm issue type. Does not end with ':'. Found issue type other"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_parse_to_do_line() {
        let submission = Submission {
            line_number: 1,
            line: String::from("TODO bug: implement this function"),
            file_path: String::from("src/todofinder.rs"),
            issuer: String::from("OthelloEngineer"),
            date: String::from("2021-09-01"),
        };
        let to_do = parse_submission(submission).unwrap();
        assert_eq!(to_do.description, "implement this function");
        assert_eq!(to_do.assigned, None);
        assert_eq!(to_do.issue_type, IssueType::Bug);
    }

    #[test]
    fn test_parse_to_do_line_with_assigned() {
        let submission = Submission {
            line_number: 1,
            line: String::from("TODO feature: implement this function; assigned OthelloEngineer"),
            file_path: String::from("src/todofinder.rs"),
            issuer: String::from("OthelloEngineer"),
            date: String::from("2021-09-01"),
        };
        let to_do = parse_submission(submission).unwrap();
        assert_eq!(to_do.description, "implement this function");
        assert_eq!(to_do.assigned, Some(String::from("OthelloEngineer")));
        assert_eq!(to_do.issue_type, IssueType::Feature);
    }

    #[test]
    fn test_parse_to_do_line_with_assigned_and_semicolon() {
        let submission = Submission {
            line_number: 1,
            line: String::from(
                "TODO feature: implement this function; now; assigned OthelloEngineer",
            ),
            file_path: String::from("src/todofinder.rs"),
            issuer: String::from("OthelloEngineer"),
            date: String::from("2021-09-01"),
        };
        let to_do = parse_submission(submission).unwrap();
        assert_eq!(to_do.description, "implement this function; now");
        assert_eq!(to_do.assigned, Some(String::from("OthelloEngineer")));
        assert_eq!(to_do.issue_type, IssueType::Feature);
    }
}
