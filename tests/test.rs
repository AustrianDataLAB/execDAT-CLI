#[cfg(test)]
mod tests {
    //use execd::cli::TemplateCommandArgs;
    use std::fs;
    use std::process::Command;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_output_file_exists() {
        let output_file = PathBuf::from("existing_output.yaml");
        let force_overwrite = false;

        // Remove the existing output file if it exists
        if output_file.exists() {
            fs::remove_file(&output_file).unwrap();
        }

        // Create the existing output file
        fs::write(&output_file, "Existing content").unwrap();

        // Execute the code
        let result = execute_template_command(&output_file, force_overwrite);

        // Check the result
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Output file already exists. Use --force to overwrite.");
    }

    #[test]
    fn test_output_file_not_exists() {
        let output_file = PathBuf::from("new_output.yaml");
        let force_overwrite = true;

        // Remove the output file if it exists
        if output_file.exists() {
            fs::remove_file(&output_file).unwrap();
        }

        // Execute the code
        let result = execute_template_command(&output_file, force_overwrite);

        // Check the result
        assert!(result.is_ok());

        // Check if the file is copied
        let copied_content = fs::read_to_string(&output_file).unwrap();
        let expected_content = fs::read_to_string("src/config/template-config-original.yaml").unwrap();
        assert_eq!(copied_content, expected_content);
    }

    fn execute_template_command(output_file: &Path, force_overwrite: bool) -> Result<(), String> {
        // Check if the output file already exists and handle the overwrite flag
        if output_file.exists() && !force_overwrite {
            return Err(String::from("Output file already exists. Use --force to overwrite."));
        }

        // Execute the cargo template command
        let template_file = Path::new("src/config/template-config-original.yaml");
        let output_file_arg = output_file.to_str().unwrap();
        let force_arg = if force_overwrite { "--force" } else { "" };
        let command_output = Command::new("cargo")
            .arg("template")
            .arg("--file")
            .arg(template_file)
            .arg("--output")
            .arg(output_file_arg)
            .arg(force_arg)
            .output()
            .map_err(|err| err.to_string())?;

        // Check the command exit status
        if !command_output.status.success() {
            let stderr = String::from_utf8_lossy(&command_output.stderr);
            return Err(format!("Failed to execute cargo template command: {}", stderr));
        }

        Ok(())
    }
}
