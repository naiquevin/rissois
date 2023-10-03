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

fn vec_to_stdout(lines: Vec<String>) {
    for line in lines.iter() {
        println!("{}", line);
    }
}

fn main() {
    let input_lines = stdin_to_vec();
    let output_lines = indent::hard_indent_org(&input_lines);
    vec_to_stdout(output_lines);
}
