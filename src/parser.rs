use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use kube::CustomResource;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "task.execd.at",
    version = "v1alpha1",
    kind = "Build",
    namespaced
)]
pub struct BuildSpec {
    pub baseimage: String,
    pub description: Option<String>,
    pub sourcecode: SourceCode,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct SourceCode {
    pub branch: Option<String>,
    pub buildcmd: Option<String>,
    pub commit: Option<String>,
    pub dependencies: Option<Dependencies>,
    pub depencencycmd: Option<String>,
    pub entrypoint: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Dependencies {
    pub asdf: Option<Vec<Dependency>>,
    pub cargo: Option<Vec<Dependency>>,
    pub go: Option<Vec<Dependency>>,
    pub gradle: Option<Vec<Dependency>>,
    pub maven: Option<Vec<Dependency>>,
    pub npm: Option<Vec<Dependency>>,
    pub os: Option<Vec<Dependency>>,
    pub pip: Option<Vec<Dependency>>,
    pub yarn: Option<Vec<Dependency>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "task.execd.at",
    version = "v1alpha1",
    kind = "Run",
    namespaced
)]
pub struct RunSpec {
    pub build: BuildSpec,
    pub outputdata: OutputDataSpec,
    pub inputdata: Option<InputDataSpec>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct OutputDataSpec {
    pub datapath: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct InputDataSpec {
    pub datapath: String,
    pub transformcmd: Option<String>,
    pub url: String,

    #[serde(rename = "type")]
    pub input_data_type: String,
}

pub fn parse_run(file_path: &str) -> RunSpec {
    // Open the YAML file
    let path = Path::new(file_path);
    let mut file = File::open(path).expect("Failed to open file");

    // Read the file contents into a string
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Failed to read file");

    // Parse the YAML contents into the run spec structure
    let run_spec: RunSpec = serde_yaml::from_str(&file_contents).expect("Failed to parse YAML file");

    run_spec
}
