#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::fs;
    use execd::TemplateCommandArgs;

    #[test]
    fn test_output_file_exists() {
        let template_args = TemplateCommandArgs {
            output_file: PathBuf::from("existing_output.yaml"),
            force_overwrite: false,
        };
        
        fs::write("existing_output.yaml", "Existing content").unwrap();
        
        let output_file = &template_args.output_file;
        let force_overwrite = template_args.force_overwrite;
        
        // Check if the output file already exists and handle the overwrite flag
        if output_file.exists() && !force_overwrite {
            println!("Output file already exists. Use --force to overwrite.");
        } else {
            // Copy the template file to the output path
            let template_file = "src/config/template-config-original.yaml";
            match fs::copy(template_file, output_file) {
                Ok(_) => println!("Template file copied to: {:?}", output_file),
                Err(err) => eprintln!("Failed to copy template file: {}", err),
            }
        }

        fs::remove_file("existing_output.yaml").unwrap();
    }

    #[test]
    fn test_output_file_not_exists() {
        let template_args = TemplateCommandArgs {
            output_file: PathBuf::from("new_output.yaml"),
            force_overwrite: true,
        };
        
        let output_file = &template_args.output_file;
        let force_overwrite = template_args.force_overwrite;
        
        // Check if the output file already exists and handle the overwrite flag
        if output_file.exists() && !force_overwrite {
            println!("Output file already exists. Use --force to overwrite.");
        } else {
            // Copy the template file to the output path
            let template_file = "src/config/template-config-original.yaml";
            match fs::copy(template_file, output_file) {
                Ok(_) => println!("Template file copied to: {:?}", output_file),
                Err(err) => eprintln!("Failed to copy template file: {}", err),
            }
        }

        fs::remove_file("new_output.yaml").unwrap();
    }
}
