use std::io;
use std::convert::TryInto;


// function to return the level of heading. If the line is not a
// heading, None is returned
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

fn indent_line(line: String, indent: u32) -> String {
    let prefix = " ".repeat(indent.try_into().unwrap());
    let parts = vec![prefix, line];
    parts.join("")
}

fn main() {
    let stdin = io::stdin();
    let mut curr_level: Option<u32> = None;
    for line in stdin.lines() {
        let s = line.unwrap();
        match heading_level(&s) {
            Some(level) => {
                curr_level = Some(level);
                println!("{}", s);
            }
            None => {
                if s == "" {
                    println!("{}", s);
                } else {
                    let indent = level_to_indent(&curr_level);
                    println!("{}", indent_line(s, indent));
                }
            }
        }
    }
}
