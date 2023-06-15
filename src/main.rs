use clap::Parser;

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
    api::{Api, PostParams},
    runtime::{conditions, wait::await_condition},
    Client,
};

use execd::Arguments;
use execd::SubCommands;

mod run_parser;
use crate::run_parser::Run;
use run_parser::parse_run;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let args: Arguments = Arguments::parse();

    match &args.subcommand {
        SubCommands::Run(run_args) => {
            handle_run(run_args, &client).await?;
        }
        SubCommands::Template(template_args) => {
            handle_template(template_args).await?;
        }
        SubCommands::Status(status_args) => {
            dbg!(status_args);
        }
        SubCommands::List(list_args) => {
            dbg!(list_args);
        }
    }

    Ok(())
}

async fn handle_run(
    run_args: &execd::RunCommandArgs,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(yaml_path) = &run_args.input_file {
        if let Some(path_str) = yaml_path.to_str() {

            // see https://github.com/kube-rs/kube/blob/main/examples/crd_apply.rs

            print!("Parsing YAML file at {}... ", path_str);
            let run_spec: run_parser::RunSpec = parse_run(path_str);
            println!("Success!");
            
            print!("Connecting to cluster and checking for required CRD definition... ");
            let crd_api: Api<CustomResourceDefinition> = Api::all(client.clone());
            let establish = await_condition(
                crd_api,
                "runs.task.execd.at",
                conditions::is_crd_established(),
            );
            let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish).await;
            println!("Success!");

            // generate a run from the spec and replace the metadata to let the server generate a unique name for us
            let mut run = Run::new("", run_spec);
            run.metadata.name = None;
            run.metadata.generate_name = Some("run-".to_string());

            let run_api: Api<Run> = Api::namespaced(client.clone(), "execdev");
            let post_params: PostParams = PostParams::default();

            print!("Applying run... ");
            let run_response = run_api
                .create( &post_params, &run)
                .await?;
            println!("Success!");

            println!("\nThe ID of you run is '{}'. You can get its status by running the 'execd status <REQUEST_ID>' command.", run_response.metadata.name.unwrap());
            println!("See 'execd status --help' for more information.");
        } else {
            println!("Invalid YAML file path!");
        }
    } else {
        println!("YAML file path is missing!");
    }
    Ok(())
}

async fn handle_template(
    template_args: &execd::TemplateCommandArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_file = &template_args.output_file;
    let force_overwrite = template_args.force_overwrite;

    // Check if the output file already exists and handle the overwrite flag
    if output_file.exists() && !force_overwrite {
        println!("Output file already exists. Use --force to overwrite.");
    } else {
        // Copy the template file to the output path
        let template_file = "src/config/template-config-original.yaml";
        match fs::copy(template_file, output_file) {
            Ok(_) => println!("Template file copied to: {:?}", output_file),
            Err(err) => eprintln!("Failed to copy template file: {}", err),
        }
    }
    Ok(())
}
