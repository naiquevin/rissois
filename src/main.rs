use clap::{self, Parser, Subcommand};
use expanduser::expanduser;
use std::process;

mod import;
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

    #[command(about = "Import notes from an existing file")]
    Import {
        #[arg(required = true, help = "Path to the source note file")]
        filepath: String,
        #[arg(required = true, help = "Path to the target directory")]
        target_dir: String,
        #[arg(short = 'p', value_enum, default_value_t = import::TsPrefix::Now, help = "Timestamp prefix")]
        ts_prefix: import::TsPrefix,
        #[arg(
            short = 't',
            help = "Title of the imported note. Will be derived from the source filename if not specified"
        )]
        title: Option<String>,
        #[arg(long, help = "Dry run")]
        dry_run: bool,
    },
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    fn execute(self: &Self) -> Result<(), String> {
        match &self.command {
            Some(Command::Indent { stdin }) => indent::cli::execute(*stdin),
            Some(Command::Import {
                filepath,
                target_dir,
                title,
                ts_prefix,
                dry_run,
            }) => {
                let fp = expanduser(filepath).unwrap();
                let td = expanduser(target_dir).unwrap();
                let res = import::cli::execute(&fp, &td, &ts_prefix, &title, dry_run)?;
                Ok(res)
            }
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
            eprintln!("Error: {}", e);
            process::exit(1)
        }
    }
}
