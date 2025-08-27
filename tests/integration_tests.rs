use claude_statusline::{
    input::ClaudeInput,
    directory::format_directory,
    jj_status::get_jj_status,
    output::StatusLine,
};

#[test]
fn test_integration_with_sample_json() {
    let json = r#"{
        "session_id": "test-session-123",
        "transcript_path": "/tmp/transcript.json",
        "cwd": "/Users/gak/src/grabby",
        "model": {
            "id": "claude-3-5-sonnet-20241022",
            "display_name": "Claude 3.5 Sonnet"
        },
        "workspace": {
            "current_dir": "/Users/gak/src/grabby",
            "project_dir": "/Users/gak/src/grabby"
        },
        "version": "1.0.71",
        "output_style": {
            "name": "default"
        }
    }"#;
    
    let input: ClaudeInput = serde_json::from_str(json).unwrap();
    
    // Test directory formatting
    let home_dir = Some("/Users/gak");
    let directory = format_directory(&input.workspace.current_dir, home_dir);
    assert_eq!(directory, "~/src/grabby");
    
    // Test jj status (will be empty since /Users/gak/src/grabby is not a jj repo in test environment)
    let jj_info = get_jj_status(&input.workspace.current_dir);
    let jj_formatted = jj_info.format();
    
    // Test output formatting
    let status_line = StatusLine {
        directory,
        jj_info: jj_formatted,
        model_name: input.model.display_name,
        output_style: None, // default style should be None
    };
    
    let formatted = status_line.format();
    
    // Should contain directory and model name
    assert!(formatted.contains("~/src/grabby"));
    assert!(formatted.contains("Claude 3.5 Sonnet"));
    // Should not contain [default] style indicator
    assert!(!formatted.contains("[default]"));
}

#[test]
fn test_integration_with_learning_style() {
    let json = r#"{
        "session_id": "test-session-123",
        "transcript_path": "/tmp/transcript.json",
        "cwd": "/Users/gak/src/claude-statusline",
        "model": {
            "id": "claude-3-5-sonnet-20241022",
            "display_name": "Claude 3.5 Sonnet"
        },
        "workspace": {
            "current_dir": "/Users/gak/src/claude-statusline",
            "project_dir": "/Users/gak/src/claude-statusline"
        },
        "version": "1.0.71",
        "output_style": {
            "name": "Learning"
        }
    }"#;
    
    let input: ClaudeInput = serde_json::from_str(json).unwrap();
    
    let home_dir = Some("/Users/gak");
    let directory = format_directory(&input.workspace.current_dir, home_dir);
    assert_eq!(directory, "~/src/claude-statusline");
    
    let jj_info = get_jj_status(&input.workspace.current_dir);
    let jj_formatted = jj_info.format();
    
    let output_style = if input.output_style.name != "default" && input.output_style.name != "null" {
        Some(input.output_style.name)
    } else {
        None
    };
    
    let status_line = StatusLine {
        directory,
        jj_info: jj_formatted,
        model_name: input.model.display_name,
        output_style,
    };
    
    let formatted = status_line.format();
    
    // Should contain directory, model name, and Learning style
    assert!(formatted.contains("~/src/claude-statusline"));
    assert!(formatted.contains("Claude 3.5 Sonnet"));
    assert!(formatted.contains("[Learning]"));
}

#[test]
fn test_integration_with_long_path() {
    let json = r#"{
        "session_id": "test-session-123",
        "transcript_path": "/tmp/transcript.json",
        "cwd": "/very/long/path/to/some/deeply/nested/directory/with/many/components",
        "model": {
            "id": "claude-3-5-sonnet-20241022",
            "display_name": "Claude 3.5 Sonnet"
        },
        "workspace": {
            "current_dir": "/very/long/path/to/some/deeply/nested/directory/with/many/components",
            "project_dir": "/very/long/path/to/some/deeply/nested/directory"
        },
        "version": "1.0.71",
        "output_style": {
            "name": "default"
        }
    }"#;
    
    let input: ClaudeInput = serde_json::from_str(json).unwrap();
    
    let home_dir = Some("/Users/gak");
    let directory = format_directory(&input.workspace.current_dir, home_dir);
    // Should be truncated to just the basename since it's outside home and very long
    assert_eq!(directory, "components");
    
    let status_line = StatusLine {
        directory,
        jj_info: None,
        model_name: input.model.display_name,
        output_style: None,
    };
    
    let formatted = status_line.format();
    assert!(formatted.contains("components"));
    assert!(formatted.contains("Claude 3.5 Sonnet"));
}

#[test]
fn test_integration_end_to_end_flow() {
    // Test the same flow as main() but with controlled input
    let json = r#"{
        "session_id": "test-session-456",
        "transcript_path": "/tmp/transcript.json",
        "cwd": "/Users/gak/src/test-project",
        "model": {
            "id": "claude-3-5-sonnet-20241022",
            "display_name": "Claude 3.5 Sonnet"
        },
        "workspace": {
            "current_dir": "/Users/gak/src/test-project",
            "project_dir": "/Users/gak/src/test-project"
        },
        "version": "1.0.71",
        "output_style": {
            "name": "Explanatory"
        }
    }"#;
    
    // Parse JSON input
    let input: ClaudeInput = serde_json::from_str(json).unwrap();
    
    // Format directory path
    let home_dir = std::env::var("HOME").ok();
    let directory = format_directory(&input.workspace.current_dir, home_dir.as_deref());
    
    // Get jj status
    let jj_info = get_jj_status(&input.workspace.current_dir);
    let jj_formatted = jj_info.format();
    
    // Determine output style
    let output_style = if input.output_style.name != "default" && input.output_style.name != "null" {
        Some(input.output_style.name)
    } else {
        None
    };
    
    // Create and format status line
    let status_line = StatusLine {
        directory,
        jj_info: jj_formatted,
        model_name: input.model.display_name,
        output_style,
    };
    
    let formatted = status_line.format();
    
    // Verify the complete output
    assert!(formatted.contains("~/src/test-project"));
    assert!(formatted.contains("Claude 3.5 Sonnet"));
    assert!(formatted.contains("[Explanatory]"));
    
    // Should not crash and should produce some output
    assert!(!formatted.is_empty());
    println!("Integration test output: {}", formatted);
}