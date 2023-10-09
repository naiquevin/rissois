use std::io;

pub fn stdin_to_vec() -> Vec<String> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lines() {
        let s = line.unwrap();
        result.push(s);
    }
    result
}

pub fn vec_to_stdout(lines: Vec<String>) {
    for line in lines.iter() {
        println!("{}", line);
    }
}
