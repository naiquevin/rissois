use std::io;

mod indent;


fn stdin_to_vec() -> Vec<String> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lines() {
        let s = line.unwrap();
        result.push(s);
    }
    result
}

fn main() {
    let lines = stdin_to_vec();
    indent::hard_indent_org(lines);
}
