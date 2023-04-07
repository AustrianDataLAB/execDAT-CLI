use clap::Parser;

mod cli;
use cli::*;

fn main() {
    let args: Arguments = Arguments::parse();

    match &args.subcommand {
        SubCommands::Run(run_args) => {
            dbg!(run_args);
        }
        SubCommands::Template(template_args) => {
            dbg!(template_args);
        }
        SubCommands::Status(status_args) => {
            dbg!(status_args);
        }
        SubCommands::List(list_args) => {
            dbg!(list_args);
        }
    }
}
