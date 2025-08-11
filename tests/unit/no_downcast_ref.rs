#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_no_downcast_ref_in_core() {
        // AI agents regularly attempt to misuse Any.
        // This test ensures that downcast_ref is only used in the allowed helper method
        // and not anywhere else in the core implementation.

        fn check_file_for_downcast_ref(path: &Path) -> anyhow::Result<Vec<(String, usize)>> {
            let content = fs::read_to_string(path)?;
            let mut violations = Vec::new();
            let mut allowed_count = 0;

            for (line_num, line) in content.lines().enumerate() {
                // TODO: this should also enforce downcast_mut
                if line.contains("downcast_ref") {
                    // Check if this is in the allowed method
                    // The only allowed use is in the downcast_rust_value method
                    let is_allowed = path.ends_with("core/value.rs")
                        && line.contains("b.as_any().downcast_ref::<T>().unwrap()");

                    if !is_allowed && allowed_count == 0 {
                        violations.push((path.display().to_string(), line_num + 1));
                        allowed_count += 1;
                    }
                }
            }

            Ok(violations)
        }

        fn check_directory(dir: &Path) -> anyhow::Result<Vec<(String, usize)>> {
            let mut all_violations = Vec::new();

            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    all_violations.extend(check_directory(&path)?);
                } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    all_violations.extend(check_file_for_downcast_ref(&path)?);
                }
            }

            Ok(all_violations)
        }

        let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let violations = check_directory(&src_dir).expect("Failed to check source directory");

        if !violations.is_empty() {
            let mut message =
                String::from("Found unauthorized use of downcast_ref in core implementation:\n");
            for (file, line) in violations {
                message.push_str(&format!("  {file}:{line}\n"));
            }
            message.push_str("\ndowncast_ref should only be used in user-facing library code, not in the core implementation.");
            panic!("{}", message);
        }
    }
}
