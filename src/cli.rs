use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// CLI arguments for the execDAT execution environment.
#[derive(Parser, Debug)]
#[command(version, name = "execd", author = "DAT Team")]
#[command(about = "CLI for the execDAT execution environment.")]
#[command(long_about = "CLI for the execDAT execution environment. \
        Wraps the kubectl command and requires the Kubernetes cluster to be correctly configured. \
        Also, make sure you have the proper permissions to patch Custom Resource Definitions in your namespace.")]
#[command(propagate_version = true)]
pub struct Arguments {
    /// Subcommand to be executed by CLI.
    #[command(subcommand)]
    pub subcommand: SubCommands,
}

/// Sends a request to the server.
#[derive(Args, Debug)]
pub struct RunCommandArgs {
    /// Name of or path to the request specification yaml.
    pub input_file: PathBuf,
}

/// Generates a template yaml file for the request specification.
#[derive(Args, Debug)]
pub struct TemplateCommandArgs {
    /// Name of or path to the output file.
    /// Existing files will not be overwritten, except when the --force flag is set.
    #[arg(short = 'o', long = "output", default_value = "specs-template.yaml")]
    pub output_file: PathBuf,

    /// If set to true, an existing file is overwritten in case of a name confict.
    #[arg(short = 'f', long = "force")]
    pub force_overwrite: bool,
}

/// Shows the status of a specific request.
#[derive(Args, Debug)]
pub struct StatusCommandArgs {
    /// String identifying the request.
    pub request_name: String,
}

/// List all currently ongoing requests.
#[derive(Args, Debug)]
pub struct ListCommandArgs {}

/// All possible subcommands.
#[derive(Subcommand, Debug)]
pub enum SubCommands {
    Run(RunCommandArgs),
    Template(TemplateCommandArgs),
    Status(StatusCommandArgs),
    List(ListCommandArgs),
}

pub static CONFIG_YAML: &str = r#"build:
  baseimage: "python:latest"
  description: "default image for demos"
  sourcecode:
    url: "https://github.com/AustrianDataLAB/execDAT"
    branch: "main"
    dependencies:
      os:
      - name: curl
        version: latest
      pip:
      - name: pandas
        version: latest
    entrypoint: python
description: "default run"
inputdata:
  datapath: "/data"
  transformcmd: "echo 'transform'"
  type: "https"
  url: "https://github.com/AustrianDataLAB/execDAT"
outputdata:
  datapath: "/data/output"
  url: "https://github.com/AustrianDataLAB/execDAT""#;