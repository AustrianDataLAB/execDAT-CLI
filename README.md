# execDAT-CLI

This is CLI tool used on the client-side to communicate with a server that has [execDAT](https://github.com/AustrianDataLAB/execDAT) set up.

This tool requires a Kubernetes cluster to be available and the permissions to change Custom Resource Definitions in the current namespace. This this is not the case, this tool will not work and behaviour is undefined. Note, that this wraps ` kubectl `, which needs to be configured accordingly.

## Development Setup

In order to run the tool without the executable run ` cargo run -- [ARGS] `. This requires _Cargo_, the _Rust_ packadge manager, to be installed. The [Cargo Book](https://doc.rust-lang.org/cargo/index.html) is a great resource with you need any help setting up and configuring _Cargo_ locally on your machine.

Alternativly, a VSC dev container configuration is provided. The container has all tools required for development preinstalled.

## External Crates

We are using the _Serde_ crate and the _serde_derive_ feature to serialize and deserialize data. Additionally, the _serde_yaml_ crate handles the YAML file format. To parse command line arguments, we use the _Clap_ crate.

## Additional Features
- [ ] Status checks through unique ID
- [ ] Status list for all ongoing requests
- [ ] Checks proper cluster configuration beforehand 
- [ ] Subcommand to generate spec file templates
- [ ] Script to properly install on Linux
- [ ] Script to properly install on Mac
- [ ] Script to properly install on Windows