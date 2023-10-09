use std::io;
use std::process;
use clap::{Parser, Subcommand};

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

#[derive(Subcommand)]
enum Commands {

    #[command(about="Hard indent org content as per the headlines")]
    Indent {
        #[arg(long, help="Read text from std input")]
        stdin: bool,
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {

    #[command(subcommand)]
    command: Option<Commands>,
}

fn indent(stdin: bool) -> Result<(), String> {
    if stdin {
        let input_lines = stdin_to_vec();
        let output_lines = indent::hard_indent_org(&input_lines);
        vec_to_stdout(output_lines);
        Ok(())
    } else {
        let errmsg = String::from("File support not implemented. Please use --stdin for now");
        Err(errmsg)
    }
}

impl Cli {

    fn execute(&self) -> Result<(), String> {
        match self.command {
            Some(Commands::Indent { stdin }) => indent(stdin),
            None => {
                let errmsg = String::from("Please specify the subcommand");
                Err(errmsg)
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let result = cli.execute();

    match result {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("Error {}", e);
            process::exit(1)
        }
    }
}
