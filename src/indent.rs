use std::convert::TryInto;

/// Returns the level of org heading for a line
///
/// If the line is not found to be an org heading, None is
/// returned
fn heading_level(line: &String) -> Option<u32> {
    if line.starts_with("*") {
        let mut level: u32 = 0;
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
        Some(num_asterisk) => num_asterisk + 1,
        None => 0,
    }
}

/// Returns an indented line given the line and indent (no. of spaces)
fn indent_line(line: &String, indent: u32) -> String {
    let prefix = " ".repeat(indent.try_into().unwrap());
    format!("{}{}", prefix, line)
}

/// Returns lines hard indented (to the headings in an org file)
pub fn hard_indent_org(lines: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut curr_level: Option<u32> = None;
    for line in lines.iter() {
        match heading_level(&line) {
            Some(level) => {
                curr_level = Some(level);
                result.push(format!("{}", line));
            }
            None => {
                if line == "" {
                    result.push(format!("{}", line));
                } else {
                    let indent = level_to_indent(&curr_level);
                    result.push(indent_line(line, indent));
                }
            }
        }
    }
    result
}

pub mod cli {

    use crate::ioutil;

    pub fn execute(stdin: bool) -> Result<(), String> {
        if stdin {
            let input_lines = ioutil::stdin_to_vec();
            let output_lines = super::hard_indent_org(&input_lines);
            ioutil::vec_to_stdout(output_lines);
            Ok(())
        } else {
            let errmsg = String::from("File support not implemented. Please use --stdin for now");
            Err(errmsg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_level() {
        let line1 = String::from("** this is a level 2 heading");
        assert_eq!(heading_level(&line1), Some(2));

        let line2 = String::from("this is a normal sentence and not a headline");
        assert_eq!(heading_level(&line2), None);

        let line3 = String::from("*this will be considered a heading but is not");
        // @TODO: Fix the behaviour of this function for lines that
        // start with asterisk but without a space following it.
        // assert_eq!(heading_level(&line3), None);
        assert_eq!(heading_level(&line3), Some(1));
    }

    #[test]
    fn test_level_to_indent() {
        let lev1 = None;
        assert_eq!(level_to_indent(&lev1), 0);

        let lev2 = Some(1);
        assert_eq!(level_to_indent(&lev2), 2);

        let lev3 = Some(4);
        assert_eq!(level_to_indent(&lev3), 5);
    }

    #[test]
    fn test_indent_line() {
        let line = String::from("  this is originally indented with only 2 spaces");
        assert_eq!(
            indent_line(&line, 2),
            String::from("    this is originally indented with only 2 spaces")
        );
    }

    #[test]
    fn test_hard_indent_org() {
        let input = "* Heading 1\n\
                     This should be indented by 2 spaces\n\
                     \n\
                     ** Heading 2\n\
                     \n\
                     This should be indented by 3 spaces\n\
                     \n\
                     * Heading 1 again\n\
                     This should again be indented by 2 spaces";
        let lines: Vec<String> = input.lines().map(String::from).collect();
        let result = hard_indent_org(&lines);
        let output = result.join("\n");
        let expected_output = "* Heading 1\n  This should be indented by 2 spaces\n\n** Heading 2\n\n   This should be indented by 3 spaces\n\n* Heading 1 again\n  This should again be indented by 2 spaces";
        assert_eq!(output, expected_output);
    }
}
