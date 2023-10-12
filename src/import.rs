use crate::macos::fileinfo::FileInfo;
use chrono::NaiveDateTime;
use convert_case::{Case, Casing};
use std::path::Path;

use clap;

//
// - Write a org file inside a directory with a derived name
//   - name = "<timestamp>-<underscore_separated_name>.org"
//   - the timestamp can be either
//     1. specified
//     2. current time
//     3. obtained from the created at time of the original file
//   - underscore separated name = name of the provided file except
//     the extension. Make sure that hyphens and spaces are replaced
//     with underscores in this
//   - title can either be specified. If not, the name of the file
//     (except the extension) is to be used
//
//
// User input
//
//   1. path to the input file (note to import) (required)
//   2. target path (required)
//   3. timestamp: Enum(now|creation)
//   4. title: Some(String)
//

#[derive(clap::ValueEnum, Clone)]
pub enum TsPrefix {
    Now,
    Created,
}

fn filename_slug(filepath: &Path) -> String {
    filepath
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_case(Case::Snake)
        .replace(".", "_")
}

fn ts_to_prefix(dt: &NaiveDateTime) -> String {
    dt.format("%Y%m%d%H%M%S").to_string()
}

fn target_filepath(target_dir: &Path, src: &Path, ts_prefix: &TsPrefix) -> String {
    let slug = filename_slug(src);
    let ts: NaiveDateTime = match ts_prefix {
        TsPrefix::Created => {
            let info = FileInfo::get(src);
            info.created
        }
        TsPrefix::Now => chrono::offset::Local::now().naive_local(),
    };
    let prefix = ts_to_prefix(&ts);
    format!("{}/{}-{}.org", target_dir.to_str().unwrap(), prefix, slug)
}

#[allow(dead_code)]
#[allow(unused)]
pub fn cli_import(
    filepath: &Path,
    target_dir: &Path,
    ts_prefix: &TsPrefix,
    title: &Option<String>,
    dry_run: &bool,
) -> Result<(), String> {
    let target_path_str = target_filepath(target_dir, filepath, ts_prefix);
    let target = Path::new(&target_path_str);
    match target.try_exists() {
        Ok(false) => {
            if *dry_run {
                println!("Running in dry-run mode");
                println!("Target file: {}", target.to_str().unwrap());
            }
            Ok(())
        }
        Ok(true) => Err("File already exists. Aborting".to_string()),
        Err(_) => Err("Unable to check existence of target file".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_filename_slug() {
        assert_eq!(filename_slug(Path::new("/path/to/MyFile.txt")), "my_file");
        assert_eq!(filename_slug(Path::new("/path/to/My File.txt")), "my_file");
        assert_eq!(filename_slug(Path::new("/path/to/my_File.txt")), "my_file");
        assert_eq!(filename_slug(Path::new("/path/to/myFILE.txt")), "my_file");
        assert_eq!(
            filename_slug(Path::new("/path/to/myfile.log.txt")),
            "myfile_log"
        );
        assert_eq!(filename_slug(Path::new("/path/to/my-file.txt")), "my_file");
    }

    #[test]
    fn test_target_filepath() {
        let target_dir = Path::new("/tmp");
        let src = Path::new("/Users/vineet/Dropbox/notes/JVM.org");
        let ts_from = TsPrefix::Now;
        let target = target_filepath(&target_dir, &src, &ts_from);
        let re = Regex::new(r"\d{14}-jvm.org").unwrap();
        assert!(re.is_match(target.as_str()));

        // The following file needs to exist
        let src = Path::new("/Users/vineet/Dropbox/notes/RabbitMQ.rst");
        let ts_from = TsPrefix::Created;
        let target = target_filepath(&target_dir, &src, &ts_from);
        let re = Regex::new(r"\d{14}-rabbit_mq.org").unwrap();
        assert!(re.is_match(target.as_str()));
    }
}
