pub mod fileinfo {

    use std::collections::HashMap;
    use std::path::Path;
    use std::process::Command;

    use chrono::{NaiveDateTime, ParseError};
    use regex::Regex;

    fn str_to_dt(s: &str) -> Result<NaiveDateTime, ParseError> {
        NaiveDateTime::parse_from_str(s, "%m/%d/%Y %H:%M:%S")
    }

    fn run_cmd(path: &Path) -> Vec<String> {
        let fp_str = path.to_str().unwrap();
        let output = Command::new("GetFileInfo")
            .arg(fp_str)
            .output()
            .expect("Error getting file info");
        let mut result = Vec::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            result.push(String::from(line));
        }
        result
    }

    fn parse_output(lines: Vec<String>) -> HashMap<String, String> {
        let re = Regex::new(r": ").unwrap();
        let mut m: HashMap<String, String> = HashMap::new();
        for line in lines.iter() {
            let res: Vec<&str> = re.split(line).collect();
            let key = res[0];
            let val = res[1];
            if key == "file" || key == "created" || key == "modified" {
                m.insert(key.to_string(), val.to_string());
            }
        }
        m
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct FileInfo {
        pub path: String,
        pub created: NaiveDateTime,
        pub modified: NaiveDateTime,
    }

    impl FileInfo {
        pub fn get(path: &Path) -> Self {
            let output = parse_output(run_cmd(path));
            Self {
                path: path.to_str().unwrap().to_string(),
                created: str_to_dt(&output["created"]).unwrap(),
                modified: str_to_dt(&output["modified"]).unwrap(),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::path::Path;

        use super::FileInfo;

        #[test]
        fn test_with_actual_file() {
            let path = Path::new("/Users/vineet/code/rissois/Cargo.toml");
            let info = FileInfo::get(&path);
            assert_eq!(info.path, "/Users/vineet/code/rissois/Cargo.toml");
            assert_eq!(
                info.created.format("%m/%d/%Y %H:%M:%S").to_string(),
                "10/01/2023 17:56:59"
            );
        }
    }
}
