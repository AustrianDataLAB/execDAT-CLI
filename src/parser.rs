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
    #[serde(rename = "baseimage")]
    pub base_image: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "sourcecode")]
    pub source_code: SourceCode,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct SourceCode {
    #[serde(rename = "branch")]
    pub branch: Option<String>,
    #[serde(rename = "buildcmd")]
    pub build_command: Option<String>,
    #[serde(rename = "commit")]
    pub commit: Option<String>,
    #[serde(rename = "dependencies")]
    pub dependencies: Option<Dependencies>,
    #[serde(rename = "depencencycmd")]
    pub depencency_command: Option<String>,
    #[serde(rename = "entrypoint")]
    pub entrypoint: String,
    #[serde(rename = "url")]
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
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "task.execd.at",
    version = "v1alpha1",
    kind = "Run",
    status = "RunStatus",
    namespaced
)]
pub struct RunSpec {
    #[serde(rename = "build")]
    pub build: BuildSpec,
    #[serde(rename = "outputdata")]
    pub output_data: OutputDataSpec,
    #[serde(rename = "inputdata")]
    pub input_data: Option<InputDataSpec>,
    #[serde(rename = "description")]
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct RunStatus {
    #[serde(rename = "currentPhase")]
    pub current_phase: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct OutputDataSpec {
    #[serde(rename = "datapath")]
    pub data_path: String,
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct InputDataSpec {
    #[serde(rename = "datapath")]
    pub datapath: String,
    #[serde(rename = "transformcmd")]
    pub transform_command: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "type")]
    pub data_type: Option<String>,
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
    let run_spec: RunSpec =
        serde_yaml::from_str(&file_contents).expect("Failed to parse YAML file");

    run_spec
}
