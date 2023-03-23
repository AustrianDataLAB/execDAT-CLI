use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// CLI for the execDAT execution environment.
#[derive(Parser, Debug)]
#[command(version, name = "execd", author = "DAT Team")]
#[command(about = "CLI for the execDAT execution environment.")]
#[command(long_about = "CLI for the execDAT execution environment. \
        Wraps the kubectl command and requires the Kubernetes cluster to be correctly configured. \
        Also, make sure you have the proper permissions to patch Custom Resource Definitions in your namespace.")]
#[command(propagate_version = true)]
struct Cli {
    /// Subcommand to be executed by CLI.
    #[command(subcommand)]
    subcommand: SubCommands,
}

/// Sends a request to the server.
#[derive(Args, Debug)]
struct RunCommandArgs {
    /// Name of or path to the request specification yaml.
    input_file: PathBuf,
}

/// Generates a template yaml file for the request specification.
#[derive(Args, Debug)]
struct TemplateCommandArgs {
    /// Name of or path to the output file.
    /// Existing files will not be overwritten, except when the --force flag is set.
    #[arg(short = 'o', long = "output", default_value = "specs-template.yaml")]
    output_file: PathBuf,

    /// If set to true, an existing file is overwritten in case of a name confict.
    #[arg(short = 'f', long = "force")]
    force_overwrite: bool,
}

/// Shows the status of a specific request.
#[derive(Args, Debug)]
struct StatusCommandArgs {
    /// String identifying the request.
    request_id: String,
}

/// List all currently ongoing requests.
#[derive(Args, Debug)]
struct ListCommandArgs {}

/// All possible subcommands.
#[derive(Subcommand, Debug)]
enum SubCommands {
    Run(RunCommandArgs),
    Template(TemplateCommandArgs),
    Status(StatusCommandArgs),
    List(ListCommandArgs),
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.subcommand {
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
