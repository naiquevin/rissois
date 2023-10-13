use crate::macos::fileinfo::FileInfo;
use chrono::NaiveDateTime;
use convert_case::{Case, Casing};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

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

fn make_title(filepath: &Path) -> String {
    filename_slug(filepath).to_case(Case::Title)
}

fn ts_to_prefix(dt: &NaiveDateTime) -> String {
    dt.format("%Y%m%d%H%M%S").to_string()
}

fn target_filepath(target_dir: &Path, src: &Path, ts_prefix: &TsPrefix) -> PathBuf {
    let slug = filename_slug(src);
    let ts: NaiveDateTime = match ts_prefix {
        TsPrefix::Created => {
            let info = FileInfo::get(src);
            info.created
        }
        TsPrefix::Now => chrono::offset::Local::now().naive_local(),
    };
    let prefix = ts_to_prefix(&ts);
    let mut target = PathBuf::new();
    // format!("{}/{}-{}.org", target_dir.to_str().unwrap(), prefix, slug)
    target.push(target_dir);
    target.push(format!("{}-{}.org", prefix, slug));
    target
}

#[allow(dead_code)]
fn note_skeleton(id: &Uuid, orig_filepath: &Path, title: &String) -> String {
    let orig_path_str = orig_filepath.to_str().unwrap();
    let org_dt_fmt = "[%Y-%m-%d %a %H:%M]";
    let orig_created_at = FileInfo::get(orig_filepath)
        .created
        .format(org_dt_fmt)
        .to_string();
    let now = chrono::offset::Local::now()
        .naive_local()
        .format(org_dt_fmt)
        .to_string();
    format!(
        r#":PROPERTIES:
:ID:               {id}
:ORIG_NOTE:        {orig_path_str}
:ORIG_IMPORTED_AT: {now}
:ORIG_CREATED_AT:  {orig_created_at}
:END:
#+title: {title}

"#
    )
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
    let target = target_filepath(target_dir, filepath, ts_prefix);
    match target.try_exists() {
        Ok(false) => {
            let note_title = match title {
                Some(s) => s.clone(),
                None => make_title(&filepath),
            };
            let note_id = Uuid::new_v4();
            let content = note_skeleton(&note_id, &filepath, &note_title);
            if *dry_run {
                println!("Running in dry-run mode");
                println!("Target file: {}", target.to_str().unwrap());
                println!("File contents:");
                println!("{}", content);
            } else {
                println!("Writing to file {}", target.to_str().unwrap());
                fs::write(target, content);
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
    fn test_make_title() {
        assert_eq!("My File", make_title(Path::new("/path/to/MyFile.txt")));
        assert_eq!("My File", make_title(Path::new("/path/to/My_File.txt")));
    }

    #[test]
    fn test_target_filepath() {
        let target_dir = Path::new("/tmp");
        let src = Path::new("/Users/vineet/Dropbox/notes/JVM.org");
        let ts_from = TsPrefix::Now;
        let target = target_filepath(&target_dir, &src, &ts_from);
        let re = Regex::new(r"\d{14}-jvm.org").unwrap();
        assert!(re.is_match(target.as_path().to_str().unwrap()));

        // The following file needs to exist
        let src = Path::new("/Users/vineet/Dropbox/notes/RabbitMQ.rst");
        let ts_from = TsPrefix::Created;
        let target = target_filepath(&target_dir, &src, &ts_from);
        let re = Regex::new(r"\d{14}-rabbit_mq.org").unwrap();
        assert!(re.is_match(target.as_path().to_str().unwrap()));
    }
}
