use clap::Parser;

mod cli;
use cli::*;
mod devfile_parser;
use devfile_parser::parse_devfile;

fn main() {
    let args: Arguments = Arguments::parse();

    match &args.subcommand {
        SubCommands::Run(run_args) => {
            dbg!(run_args);
            if let Some(yaml_path) = &run_args.input_file {
                if let Some(path_str) = yaml_path.to_str() {
                    parse_devfile(path_str);
                } else {
                    println!("Invalid YAML file path");
                }
            } else {
                println!("YAML file path is missing");
            }
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
