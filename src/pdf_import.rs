use std::{borrow::Cow, fs, path::Path};

use uuid::Uuid;

use crate::import::{target_filepath, TsPrefix};

pub fn import_pdf(filepath: &Path, target_dir: &Path, title: Option<&str>, dry_run: bool) -> Result<(), String> {
    let bytes = std::fs::read(&filepath).unwrap();
    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    let note_title = title.map_or_else(
        || filepath.file_stem().unwrap().to_string_lossy(),
        |t| Cow::Borrowed(t),
    );
    let note_id = Uuid::new_v4();
    let orig_path_str = filepath.to_str().unwrap();
    let org_dt_fmt = "[%Y-%m-%d %a %H:%M]";
    let now = chrono::offset::Local::now()
        .naive_local()
        .format(org_dt_fmt)
        .to_string();
    let content = format!(
        r#":PROPERTIES:
:ID:               {note_id}
:ORIG_NOTE:        {orig_path_str}
:ORIG_IMPORTED_AT: {now}
:END:
#+title: {note_title}

{out}
"#
    );
    // @TODO: Add support for `TsPrefix::Created` for linux. Currently
    // it's only supported for MacOS.
    let target = target_filepath(&target_dir, &filepath, &TsPrefix::Now);
    if dry_run {
        println!("Running in dry-run mode");
        println!("Target file: {}", target.display());
        println!("File contents:");
        println!("{content}");
        Ok(())
    } else {
        println!("Writing to file: {}", target.display());
        let res = fs::write(target, content);
        res.map_err(|e| e.to_string())
    }
}
