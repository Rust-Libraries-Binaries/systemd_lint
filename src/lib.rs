pub fn lint_service_file(content: &str) -> Vec<String> {
    let mut warnings = Vec::new();

    // Example check: Warn if ExecStart points to /bin/false
    if content.contains("ExecStart=/bin/false") {
        warnings.push("ExecStart points to /bin/false, which may be incorrect.".to_string());
    }

    // Example check: Warn if running as root
    if content.contains("User=root") {
        warnings.push("Running as root is not recommended. Consider using a non-privileged user.".to_string());
    }

    warnings
}

pub fn suggest_optimizations(content: &str) -> Vec<String> {
    let mut suggestions = Vec::new();

    // Example suggestion: Removing unnecessary dependencies
    if content.contains("After=network.target") {
        suggestions.push("Consider removing 'After=network.target' if not needed.".to_string());
    }

    // Example suggestion: Improving security settings
    if !content.contains("ProtectSystem=full") {
        suggestions.push("Add 'ProtectSystem=full' for improved security.".to_string());
    }

    suggestions
}
