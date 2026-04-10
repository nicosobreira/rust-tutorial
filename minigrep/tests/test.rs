use minigrep::Config;
use minigrep::CustomError;

#[test]
fn build_invalid_pattern() {
    let args = ["./minigrep".to_string()];

    let error = Config::build(args.into_iter()).unwrap_err();

    // Now compare them directly
    assert!(matches!(error, CustomError::InvalidPattern));
}

#[test]
fn build_assign_single_file_path() {
    let pattern = "pattern".to_string();
    let file_path = "file.txt".to_string();

    let args = ["./minigrep".to_string(), pattern.clone(), file_path.clone()];

    let config = Config::build(args.into_iter()).unwrap();

    assert_eq!(config.pattern, pattern);
    assert_eq!(
        config.files_paths.iter().next().unwrap().to_string_lossy(),
        file_path
    );
}
