use std::process;
use clap::{Parser, Subcommand};

mod indent;
mod ioutil;


#[derive(Subcommand)]
enum Command {

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
    command: Option<Command>,
}

fn indent(stdin: bool) -> Result<(), String> {
    if stdin {
        let input_lines = ioutil::stdin_to_vec();
        let output_lines = indent::hard_indent_org(&input_lines);
        ioutil::vec_to_stdout(output_lines);
        Ok(())
    } else {
        let errmsg = String::from("File support not implemented. Please use --stdin for now");
        Err(errmsg)
    }
}

impl Cli {

    fn execute(&self) -> Result<(), String> {
        match self.command {
            Some(Command::Indent { stdin }) => indent(stdin),
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
