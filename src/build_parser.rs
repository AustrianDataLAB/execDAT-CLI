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
    baseimage: String,
    description: String,
    sourcecode: SourceCode,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct SourceCode {
    url: String,
    branch: String,
    dependencies: Dependencies,
    entrypoint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Dependencies {
    os: Vec<Dependency>,
    pip: Vec<Dependency>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Dependency {
    name: String,
    version: String,
}

pub fn parse_build(file_path: &str) -> BuildSpec {
    // Open the YAML file
    let path = Path::new(file_path);
    let mut file = File::open(path).expect("Failed to open file");

    // Read the file contents into a string
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Failed to read file");

    // Parse the YAML contents into the build spec structure
    let build_spec: BuildSpec =
        serde_yaml::from_str(&file_contents).expect("Failed to parse devfile");

    build_spec
}
