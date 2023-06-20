use clap::Parser;

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
    api::{Api, ListParams, PostParams},
    runtime::{conditions, wait::await_condition},
    Client,
};

use execd::Arguments;
use execd::SubCommands;

mod run_parser;
use crate::run_parser::Run;
use run_parser::parse_run;
use std::env;
use std::fs;

static DEFAULT_EXECD_NAMESPACE: &str = "execdev";

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
            handle_status(status_args, &client).await?;
        }
        SubCommands::List(list_args) => {
            handle_list(list_args, &client).await?;
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

            let namespace =
                env::var("EXECD_NAMESPACE").unwrap_or(DEFAULT_EXECD_NAMESPACE.to_string());
            let namespace = namespace.as_str();

            let run_api: Api<Run> = Api::namespaced(client.clone(), namespace);
            let post_params: PostParams = PostParams::default();

            print!("Applying run... ");
            let run_response = run_api.create(&post_params, &run).await?;
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

async fn handle_status(
    status_args: &execd::StatusCommandArgs,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    print!("Connecting to cluster and checking for required CRD definition... ");
    let crd_api: Api<CustomResourceDefinition> = Api::all(client.clone());
    let establish = await_condition(
        crd_api,
        "runs.task.execd.at",
        conditions::is_crd_established(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish).await;
    println!("Success!");

    let namespace = env::var("EXECD_NAMESPACE").unwrap_or(DEFAULT_EXECD_NAMESPACE.to_string());
    let namespace = namespace.as_str();

    let run_api: Api<Run> = Api::namespaced(client.clone(), namespace);

    let run = run_api.get(status_args.request_id.as_str()).await?;

    dbg!(run);

    Ok(())
}

async fn handle_list(
    _list_args: &execd::ListCommandArgs,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    print!("Connecting to cluster and checking for required CRD definition... ");
    let crd_api: Api<CustomResourceDefinition> = Api::all(client.clone());
    let establish = await_condition(
        crd_api,
        "runs.task.execd.at",
        conditions::is_crd_established(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish).await;
    println!("Success!");

    let namespace = env::var("EXECD_NAMESPACE").unwrap_or(DEFAULT_EXECD_NAMESPACE.to_string());
    let namespace = namespace.as_str();

    let run_api: Api<Run> = Api::namespaced(client.clone(), namespace);
    let list_params: ListParams = ListParams::default();

    let runs = run_api.list(&list_params).await?;

    let runs_items = runs.items.len();

    if runs_items == 0 {
        println!("No runs found.");
    } else {
        const NAME_PRINT_WIDTH: usize = 15;
        const CREATED_PRINT_WIDTH: usize = 25;
        const DESCRIPTION_PRINT_WIDTH: usize = 40;

        println!(
            "{:<NAME_PRINT_WIDTH$} {:<CREATED_PRINT_WIDTH$} {:<DESCRIPTION_PRINT_WIDTH$}",
            "NAME", "CREATED", "DESCRIPTION"
        );
        runs.items.iter().for_each(|run| {
            // the format macro hack is used to guarantee consistent spacing because some types don't convert using just the println macro
            println!(
                "{:<NAME_PRINT_WIDTH$} {:<CREATED_PRINT_WIDTH$} {:<DESCRIPTION_PRINT_WIDTH$}",
                run.metadata.name.as_ref().unwrap(),
                format!("{}", run.metadata.creation_timestamp.as_ref().unwrap().0),
                run.spec
                    .description
                    .as_ref()
                    .unwrap()
                    .chars()
                    .take(DESCRIPTION_PRINT_WIDTH)
                    .collect::<String>(),
            );
        });
    }

    Ok(())
}
