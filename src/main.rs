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

mod build_parser;
use crate::build_parser::Build;
use build_parser::parse_build;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;

    let args: Arguments = Arguments::parse();

    match &args.subcommand {
        SubCommands::Run(run_args) => {
            dbg!(run_args);
            if let Some(yaml_path) = &run_args.input_file {
                if let Some(path_str) = yaml_path.to_str() {
                    let d: build_parser::BuildSpec = parse_build(path_str);

                    let ssapply = PatchParams::apply("execdat-cli").force();

                    // see https://github.com/kube-rs/kube/blob/main/examples/crd_apply.rs

                    // 0. Ensure the CRD is installed (you probably just want to do this on CI)
                    // (crd file can be created by piping `Foo::crd`'s yaml ser to kubectl apply)
                    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
                    info!("Creating crd: {}", serde_yaml::to_string(&Build::crd())?);
                    crds.patch(
                        "builds.task.execd.at",
                        &ssapply,
                        &Patch::Apply(Build::crd()),
                    )
                    .await?;

                    info!("Waiting for the api-server to accept the CRD");
                    let establish = await_condition(
                        crds,
                        "builds.task.execd.at",
                        conditions::is_crd_established(),
                    );
                    let _ =
                        tokio::time::timeout(std::time::Duration::from_secs(10), establish).await?;

                    let builds: Api<Build> = Api::default_namespaced(client.clone());

                    let b = Build::new("test", d);
                    let o = builds.patch("test", &ssapply, &Patch::Apply(&b)).await?;

                    dbg!(o);
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
