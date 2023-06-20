use clap::Parser;

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
    api::{Api, ListParams, PostParams},
    runtime::{conditions, wait::await_condition},
    Client,
};

mod cli;
use cli::*;

mod parser;
use parser::*;

use std::env;
use std::fs;

static DEFAULT_EXECD_NAMESPACE: &str = "execdev";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // see https://github.com/kube-rs/kube/blob/main/examples/crd_apply.rs

    // Creating the client
    let client = Client::try_default().await?;

    // Connecting to cluster and checking for required CRD definition
    let crd_api: Api<CustomResourceDefinition> = Api::all(client.clone());
    let establish = await_condition(
        crd_api,
        "runs.task.execd.at",
        conditions::is_crd_established(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish).await;

    // Getting namespace from environment variable
    let namespace = env::var("EXECD_NAMESPACE").unwrap_or(DEFAULT_EXECD_NAMESPACE.to_string());
    let namespace = namespace.as_str();

    // Creating the run API
    let run_api: Api<Run> = Api::namespaced(client.clone(), namespace);

    // Argument Parsing
    let args: Arguments = Arguments::parse();
    match &args.subcommand {
        SubCommands::Run(run_args) => {
            handle_run(run_args, run_api.clone()).await?;
        }
        SubCommands::Template(template_args) => {
            handle_template(template_args).await?;
        }
        SubCommands::Status(status_args) => {
            handle_status(status_args, run_api.clone()).await?;
        }
        SubCommands::List(list_args) => {
            handle_list(list_args, run_api.clone()).await?;
        }
    }

    Ok(())
}

async fn handle_run(
    run_args: &RunCommandArgs,
    run_api: Api<Run>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path_str) = &run_args.input_file.to_str() {
        // Parse the YAML file
        let run_spec: parser::RunSpec = parse_run(path_str);

        // Generate a run from the spec and replace the metadata to let the server generate a unique name for us
        let mut run = Run::new("", run_spec);
        run.metadata.name = None;
        run.metadata.generate_name = Some("run-".to_string());

        // Create the run
        let post_params: PostParams = PostParams::default();
        let run_response = run_api.create(&post_params, &run).await?;

        println!("Successfully created run from YAML file at {}!", path_str);
        println!("The name of you run is '{}' and you can find it in the namespace '{}'. You can get its status by running the 'execd status <REQUEST_NAME>' command.", 
                run_response.metadata.name.unwrap(),
                run_response.metadata.namespace.unwrap());
        println!("See 'execd status --help' for more information.");
    } else {
        println!("Invalid YAML file path!");
    }

    Ok(())
}

async fn handle_template(
    template_args: &TemplateCommandArgs,
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

async fn handle_status(
    status_args: &StatusCommandArgs,
    run_api: Api<Run>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the run with the given request ID
    let run = run_api.get(status_args.request_name.as_str()).await?;
    dbg!(run.spec);
    dbg!(run.status);

    Ok(())
}

async fn handle_list(
    _list_args: &ListCommandArgs,
    run_api: Api<Run>,
) -> Result<(), Box<dyn std::error::Error>> {
    let list_params: ListParams = ListParams::default();
    let runs = run_api.list(&list_params).await?;

    // Check number of items in the returned ObjectList
    if runs.items.len() <= 0 {
        println!("No runs found.");
    } else {
        // Constants for print formatting
        const NAME_PRINT_WIDTH: usize = 15;
        const CREATED_PRINT_WIDTH: usize = 25;
        const DESCRIPTION_PRINT_WIDTH: usize = 40;

        // Printing all runs in the list in a table-like style
        println!(
            "{:<NAME_PRINT_WIDTH$} {:<CREATED_PRINT_WIDTH$} {:<DESCRIPTION_PRINT_WIDTH$}",
            "NAME", "CREATED", "DESCRIPTION"
        );

        runs.items.iter().for_each(|run| {
            let print_name = run.metadata.name.as_ref().unwrap();

            // the format macro hack is used to guarantee consistent spacing because the Time type doesn't convert well just the println macro
            let print_creation_timestamp =
                format!("{}", run.metadata.creation_timestamp.as_ref().unwrap().0);

            let print_description = run
                .spec
                .description
                .as_ref()
                .unwrap()
                .chars()
                .take(DESCRIPTION_PRINT_WIDTH)
                .collect::<String>();

            println!(
                "{:<NAME_PRINT_WIDTH$} {:<CREATED_PRINT_WIDTH$} {:<DESCRIPTION_PRINT_WIDTH$}",
                print_name, print_creation_timestamp, print_description
            );
        });
    }

    Ok(())
}
