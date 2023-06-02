use clap::Parser;
use kube::{Client, api::{Api, ResourceExt, ListParams, PostParams}};
use k8s_openapi::api::core::v1::Pod;
use serde_json::json;
use tracing::*;

mod cli;
use cli::*;
mod devfile_parser;
use devfile_parser::parse_devfile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;

    let pods: Api<Pod> = Api::default_namespaced(client);

    println!("before:");

    for p in pods.list(&ListParams::default()).await? {
        println!("found pod {}", p.name_any());
    }

    let p: Pod = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": { "name": "test" },
        "spec": {
            "containers": [{
              "name": "blog",
              "image": "alpine"
            }],
        }
    }))?;

    let pp = PostParams::default();
    match pods.create(&pp, &p).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(p.name_any(), name);
            info!("Created {}", name);
        }
        Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => return Err(e.into()),                        // any other case is probably bad
    }

    println!("after:");

    for p in pods.list(&ListParams::default()).await? {
        println!("found pod {}", p.name_any());
    }

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

    Ok(())
}
