//! Text utility functions - Pure Rust String operations

/// Remove comments from input (lines starting with prefix)
pub fn uncomment_string(input: &str, prefix: &str) -> String {
    let mut result = String::new();

    for line in input.lines() {
        if let Some(pos) = line.find(prefix) {
            // Keep part before comment
            result.push_str(&line[..pos]);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }

    result
}

/// Strip lines that end with prefix (line continuation markers)
pub fn strip_lines_string(input: &str, prefix: &str) -> String {
    let mut result = String::new();
    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        if let Some(pos) = line.find(prefix) {
            // Line has continuation marker
            let before_prefix = &line[..pos];
            let trimmed = before_prefix.trim_end();

            // If line is empty or ends with newline after trimming, drop it
            if trimmed.is_empty() || trimmed.ends_with('\n') {
                // Skip the newline - continue on same line
                continue;
            } else {
                // Keep content before prefix, continue next line
                result.push_str(trimmed);
                continue;
            }
        } else {
            // No continuation marker
            result.push_str(line);
            if lines.peek().is_some() {
                result.push('\n');
            }
        }
    }

    result
}
