use clap::{Parser, Subcommand};
use std::process;

mod indent;
mod ioutil;
mod macos;

#[derive(Subcommand)]
enum Command {
    #[command(about = "Hard indent org content as per the headlines")]
    Indent {
        #[arg(long, help = "Read text from std input")]
        stdin: bool,
    },
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    fn execute(&self) -> Result<(), String> {
        match self.command {
            Some(Command::Indent { stdin }) => indent::cli::execute(stdin),
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
