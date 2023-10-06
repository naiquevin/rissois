use std::io;
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Indent { stdin }) => {
            if *stdin {
                let input_lines = stdin_to_vec();
                let output_lines = indent::hard_indent_org(&input_lines);
                vec_to_stdout(output_lines);
            } else {
                println!("File support not implemented. Please use --stdin for now")
            }
        },
        None => {
            println!("Please specify the subcommand")
        }
    }
}
