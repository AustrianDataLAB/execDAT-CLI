#[cfg(test)]
mod tests {
    use execd::cli::CONFIG_YAML;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    #[test]
    fn test_basic_template_command_generates_default_output() {
        let force_overwrite = false;
        // Define the output file path
        let default_output_file = PathBuf::from("specs-template.yaml");
        let empty_output_file = PathBuf::from("");

        // Remove the existing output file if it exists
        if default_output_file.exists() {
            fs::remove_file(&default_output_file).unwrap();
        }

        let result = execute_template_command(&empty_output_file, force_overwrite);
        assert!(result.is_ok());

        // Check if the file is copied
        let copied_content = fs::read_to_string(&default_output_file).unwrap();
        let expected_content = CONFIG_YAML;
        assert_eq!(copied_content, expected_content);

        // Remove the output file if it exists
        if default_output_file.exists() {
            fs::remove_file(&default_output_file).unwrap();
        }
    }

    #[test]
    fn test_output_file_exists_fails_without_force_overwrite() {
        let empty_output_file = PathBuf::from("existing_output.yaml");
        let force_overwrite = false;

        // Remove the existing output file if it exists
        if empty_output_file.exists() {
            fs::remove_file(&empty_output_file).unwrap();
        }

        // Create the existing output file
        fs::write(&empty_output_file, "Existing content").unwrap();
        let result = execute_template_command(&empty_output_file, force_overwrite);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Output file already exists. Use --force to overwrite."
        );

        // Remove the existing output file if it exists
        if empty_output_file.exists() {
            fs::remove_file(&empty_output_file).unwrap();
        }
    }

    #[test]
    fn test_output_file_succeeds_with_force_overwrite() {
        let empty_output_file = PathBuf::from("new_output.yaml");
        let force_overwrite = true;

        // Remove the output file if it exists
        if empty_output_file.exists() {
            fs::remove_file(&empty_output_file).unwrap();
        }

        // Create the existing output file
        fs::write(&empty_output_file, "Existing content").unwrap();

        let result = execute_template_command(&empty_output_file, force_overwrite);
        assert!(result.is_ok());

        // Check if the file is copied
        let copied_content = fs::read_to_string(&empty_output_file).unwrap();
        let expected_content = CONFIG_YAML;
        assert_eq!(copied_content, expected_content);

        // Remove the output file if it exists
        if empty_output_file.exists() {
            fs::remove_file(&empty_output_file).unwrap();
        }
    }

    fn execute_template_command(
        empty_output_file: &Path,
        force_overwrite: bool,
    ) -> Result<(), String> {
        // Check if the output file already exists and handle the overwrite flag
        if empty_output_file.exists() && !force_overwrite {
            return Err(String::from(
                "Output file already exists. Use --force to overwrite.",
            ));
        }

        // Execute the cargo template command
        let mut cargo_command = Command::new("cargo");
        cargo_command.arg("run").arg("--").arg("template");

        // Append the output file argument if it's not empty
        if !empty_output_file.to_string_lossy().is_empty() {
            let output_file_arg = empty_output_file.to_str().unwrap();
            cargo_command.arg("--output").arg(output_file_arg);
        }

        // Append the force flag if necessary
        if force_overwrite {
            cargo_command.arg("--force");
        }

        let command_output = cargo_command.output().map_err(|err| err.to_string())?;

        // Check the command exit status
        if !command_output.status.success() {
            let stderr = String::from_utf8_lossy(&command_output.stderr);
            return Err(format!(
                "Failed to execute cargo template command: {}",
                stderr
            ));
        }

        Ok(())
    }
}
