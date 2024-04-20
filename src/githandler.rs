use regex::Regex;
use std::io::Error;
use std::process::Command;

#[derive(Debug)]
pub struct BlameEntry {
    pub user: String,
    pub date: String,
}

pub struct DiffedFileChangedLines {
    pub file_path: String,
    pub changed_lines: Vec<String>,
}

pub fn get_current_user() -> Result<String, Error> {
    let output = Command::new("git").args(["config", "user.name"]).output();

    let output = match output {
        Ok(output) => output,
        Err(e) => {
            eprintln!("COMMAND Error: {}", e); // Use eprintln for errors
            return Err(e);
        }
    };

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        eprintln!("Git command failed: {}", err_msg);
        return Err(Error::new(std::io::ErrorKind::Other, "Git command failed"));
    }

    let user = String::from_utf8(output.stdout).map_err(|e| {
        eprintln!("FORMATTING Error: {}", e);
        Error::new(std::io::ErrorKind::InvalidData, e)
    })?;

    Ok(user.trim().to_string()) // Trim the output to remove any newline characters
}

pub fn blame_user_from_line(file_path: &str, line_number: usize) -> Result<BlameEntry, Error> {
    let location = format!("{},{}", line_number, line_number);
    println!("location: {:?}", location);
    let output = Command::new("git")
        .args([
            "blame",
            "-L",
            &format!("{},{}", line_number, line_number),
            "--",
            file_path,
        ])
        .output();

    let output = match output {
        Ok(output) => output,
        Err(e) => {
            eprintln!("COMMAND Error: {}", e); // Use eprintln for errors
            return Err(e);
        }
    };
    println!("output: {:?}", output);

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        eprintln!("Git command failed: {}", err_msg);
        return Err(Error::new(std::io::ErrorKind::Other, "Git command failed"));
    }

    let user = String::from_utf8(output.stdout).map_err(|e| {
        eprintln!("FORMATTING Error: {}", e);
        Error::new(std::io::ErrorKind::InvalidData, e)
    })?;
    println!("user: {:?}", user);
    // possible regex /\((.*?)\s+\d{4}/gm
    let regex_pattern = r"\((.*?)\s+(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} (\+|-)\d{4})";
    let re = Regex::new(regex_pattern).unwrap();
    let groups = re.captures(&user).unwrap();
    let user = groups.get(1).unwrap().as_str();
    let date = groups.get(2).unwrap().as_str();
    let blame_entry = BlameEntry {
        user: user.to_string(),
        date: date.to_string(),
    };
    println!("blame_entry: {:?}", blame_entry);
    Ok(blame_entry)
}

pub fn changed_lines_per_diffed_file(diff: &str) -> Vec<DiffedFileChangedLines> {
    let lines = diff.lines();
    println!("lines: {:?}", lines);
    let mut lines_from_changed_files: Vec<DiffedFileChangedLines> = Vec::new();
    for line in lines {
        let mut idx = 0;
        println!("line: {:?}", line);
        if line.starts_with("diff") {
            let file_path = line.split(' ').collect::<Vec<&str>>()[2];
            println!("file_path: {:?}", file_path);
            lines_from_changed_files.push(DiffedFileChangedLines {
                file_path: file_path.to_string(),
                changed_lines: Vec::new(),
            });
            idx += 1;
        }
        lines_from_changed_files[idx - 1]
            .changed_lines
            .push(line.to_string());
    }
    lines_from_changed_files
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_get_current_user() {
        let user = match get_current_user() {
            Ok(user) => user,
            Err(e) => panic!("Error: {}", e),
        };
        assert_eq!(user, "OthelloEngineer");
    }

    #[test]
    fn test_blame_user_from_line() {
        let user = match blame_user_from_line("examples/example.rs", 1) {
            Ok(user) => user,
            Err(e) => panic!("Error: {}", e),
        };
        assert_eq!(user.user, "jolee18");
    }

    #[test]
    fn test_changed_lines_per_diffed_file() {
        let diff_lines = fs::read_to_string("examples/git_diff.txt").unwrap();
        println!("diff_lines: {:?}", diff_lines);
        let changed_lines = changed_lines_per_diffed_file(&diff_lines);
        assert_eq!(changed_lines.len(), 2);
    }
}
