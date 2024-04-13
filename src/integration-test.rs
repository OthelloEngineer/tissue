#[cfg(test)]
mod tests {
    use crate::{
        filereader, githandler,
        todofinder::{self, Submission},
    };

    use super::*;
    use std::fs;
    #[test]
    fn find_all_todos() {
        let files = match filereader::comments_from_file_in_project("examples") {
            Ok(files) => files,
            Err(e) => panic!("Error: {}", e),
        };
        let mut submissions: Vec<Submission> = Vec::new();
        for file in files {
            for line in file.lines {
                let issuer = githandler::blame_user_from_line(&file.file_path, line.0).unwrap();
                let submission = Submission {
                    line_number: line.0,
                    line: line.1,
                    file_path: file.file_path.clone(),
                    issuer: issuer.user,
                    //2024-03-24 02:45:51 +0100
                    //YYYY-MM-DD HH:MM:SS +0000 offset
                    date: issuer.date,
                };
                submissions.push(submission);
            }
        }
        let todos = submissions
            .iter()
            .filter(|submission| todofinder::is_to_do(&submission.line))
            .collect::<Vec<_>>();
        assert_eq!(todos.len(), 16);
    }
}
