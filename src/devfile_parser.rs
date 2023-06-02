use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct Devfile {
    baseimage: String,
    description: String,
    sourcecode: SourceCode,
}

#[derive(Debug, Deserialize, Serialize)]
struct SourceCode {
    url: String,
    branch: String,
    dependencies: Dependencies,
    entrypoint: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Dependencies {
    os: Vec<Dependency>,
    pip: Vec<Dependency>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Dependency {
    name: String,
    version: String,
}

pub fn parse_devfile(file_path: &str) {
    // Open the YAML file
    let path = Path::new(file_path);
    let mut file = File::open(path).expect("Failed to open file");

    // Read the file contents into a string
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Failed to read file");

    // Parse the YAML contents into the Devfile structure
    let devfile: Devfile = serde_yaml::from_str(&file_contents)
        .expect("Failed to parse devfile");

    // Access the parsed devfile data
    println!("Base Image: {}", devfile.baseimage);
    println!("Description: {}", devfile.description);
    println!("Source Code URL: {}", devfile.sourcecode.url);
    println!("Source Code Branch: {}", devfile.sourcecode.branch);
    println!("Dependencies:");
    for os_dep in &devfile.sourcecode.dependencies.os {
        println!("- OS: {} ({})", os_dep.name, os_dep.version);
    }
    for pip_dep in &devfile.sourcecode.dependencies.pip {
        println!("- Pip: {} ({})", pip_dep.name, pip_dep.version);
    }
    println!("Entrypoint: {}", devfile.sourcecode.entrypoint);

    // Implement your logic for working with the parsed devfile
    // ...
}