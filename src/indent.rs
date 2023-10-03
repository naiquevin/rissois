use std::convert::TryInto;


/// Returns the level of org heading for a line
///
/// If the line is not found to be an org heading, None is
/// returned
fn heading_level(line: &String) -> Option<u32> {
    if line.starts_with("*") {
        let mut level:u32 = 0;
        for c in line.chars() {
            if c == '*' {
                level = level + 1;
            } else {
                break;
            }
        }
        Some(level)
    } else {
        None
    }
}

/// Returns indent (no. of spaces) given org heading level
fn level_to_indent(level: &Option<u32>) -> u32 {
    match level {
        Some(num_asterisk) => {
            num_asterisk + 1
        }
        None => {
            0
        }
    }
}

/// Returns an indented line given the line and indent (no. of spaces)
fn indent_line(line: &String, indent: u32) -> String {
    let prefix = " ".repeat(indent.try_into().unwrap());
    format!("{}{}", prefix, line)
}

/// Returns lines hard indented (to the headings in an org file)
pub fn hard_indent_org(lines: Vec<String>) {
    let mut curr_level: Option<u32> = None;
    for line in lines.iter() {
        match heading_level(&line) {
            Some(level) => {
                curr_level = Some(level);
                println!("{}", line);
            }
            None => {
                if line == "" {
                    println!("{}", line);
                } else {
                    let indent = level_to_indent(&curr_level);
                    println!("{}", indent_line(line, indent));
                }
            }
        }
    }
}
