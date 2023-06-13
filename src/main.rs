use tracing::log::info;

use clap::Parser;

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
    api::{Api, Patch, PatchParams},
    runtime::{conditions, wait::await_condition},
    Client, CustomResourceExt,
};

mod cli;
use cli::{Arguments, SubCommands};

mod run_parser;
use crate::run_parser::Run;
use run_parser::parse_run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let args: Arguments = Arguments::parse();

    match &args.subcommand {
        SubCommands::Run(run_args) => {
            dbg!(run_args);

            if let Some(yaml_path) = &run_args.input_file {
                if let Some(path_str) = yaml_path.to_str() {
                    let run_spec: run_parser::RunSpec = parse_run(path_str);
                    let patch_params = PatchParams::apply("execdat-cli").force();

                    // see https://github.com/kube-rs/kube/blob/main/examples/crd_apply.rs

                    // 0. Ensure the CRD is installed (you probably just want to do this on CI)
                    // (crd file can be created by piping `Foo::crd`'s yaml ser to kubectl apply)
                    // is this needed?
                    let crd_api: Api<CustomResourceDefinition> = Api::all(client.clone());
                    info!("Creating crd: {}", serde_yaml::to_string(&Run::crd())?);
                    crd_api
                        .patch(
                            "runs.task.execd.at",
                            &patch_params,
                            &Patch::Apply(Run::crd()),
                        )
                        .await?;

                    info!("Waiting for the api-server to accept the CRD");
                    let establish = await_condition(
                        crd_api,
                        "runs.task.execd.at",
                        conditions::is_crd_established(),
                    );
                    let _ =
                        tokio::time::timeout(std::time::Duration::from_secs(10), establish).await?;

                    let runs: Api<Run> = Api::default_namespaced(client.clone());

                    let run = Run::new("test", run_spec);
                    let run_response = runs
                        .patch("test", &patch_params, &Patch::Apply(&run))
                        .await?;

                    dbg!(run_response);
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

    Ok(())
}
