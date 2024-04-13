use std::process::Command;
use std::io::Error;
use regex::Regex;

#[derive(Debug)]
pub struct BlameEntry {
    pub user: String,
    pub date: String,
}

pub fn get_current_user() -> Result<String, Error> {
    let output = Command::new("git")
                         .args(["config", "user.name"]) 
                         .output();

    let output = match output {
        Ok(output) => output,
        Err(e) => {
            eprintln!("COMMAND Error: {}", e); // Use eprintln for errors
            return Err(e);
        },
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
                         .args(["blame", "-L", &format!("{},{}", line_number, line_number), "--", file_path])
                         .output();

    let output = match output {
        Ok(output) => output,
        Err(e) => {
            eprintln!("COMMAND Error: {}", e); // Use eprintln for errors
            return Err(e);
        },
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

#[cfg(test)]
mod tests {
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
}
