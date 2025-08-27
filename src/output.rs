use colored::*;
use crate::jj_status::JjInfo;

pub struct StatusLine {
    pub directory: String,
    pub jj_info: Option<JjInfo>,
    pub model_name: String,
    pub output_style: Option<String>,
}

impl StatusLine {
    pub fn format(&self) -> String {
        // Force colors to be enabled for statusline
        colored::control::set_override(true);
        
        let mut parts = Vec::new();

        // Dark grey separator dot - RGB(96, 96, 96)
        let separator = " ‧ ".truecolor(96, 96, 96);

        // Directory with folder emoji and space
        // RGB(64, 224, 208) - Turquoise with punch
        parts.push(format!("📂 {}", self.directory.truecolor(64, 224, 208)));

        // JJ info with dynamic emoji based on changes (no parentheses)
        if let Some(jj_info) = &self.jj_info {
            if jj_info.change_id.is_some() {
                // Check if there are uncommitted changes
                let emoji = if jj_info.has_changes {
                    "⚡" // Lightning for uncommitted changes
                } else {
                    "🔀" // Twisted arrows for clean state
                };
                
                let mut jj_parts = Vec::new();
                
                // Add change ID in duller hot pink (greyer)
                if let Some(change_id) = &jj_info.change_id {
                    jj_parts.push(change_id.truecolor(200, 80, 140).to_string()); // Duller hot pink with more grey
                }
                
                // Add bookmarks in full hot pink (brightest)
                if !jj_info.bookmarks.is_empty() {
                    let bright_bookmarks = jj_info.bookmarks
                        .iter()
                        .map(|bookmark| bookmark.truecolor(255, 20, 147).to_string()) // Full hot pink for branch names
                        .collect::<Vec<_>>()
                        .join(" ");
                    jj_parts.push(bright_bookmarks);
                }
                
                // Add conflict indicator in duller hot pink (greyer)
                if jj_info.has_conflict {
                    jj_parts.push("conflict".truecolor(200, 80, 140).to_string()); // Same as change ID
                }
                
                let mut jj_display = jj_parts.join(" ");
                
                // Add asterisk for changes
                if jj_info.has_changes {
                    jj_display.push('*');
                }
                
                parts.push(format!("{}{} {}", separator, emoji, jj_display));
            }
        }

        // Model name with brain emoji and space
        // RGB(255, 140, 0) - Vibrant orange
        parts.push(format!("{}🧠 {}", separator, self.model_name.truecolor(255, 140, 0)));

        // Output style with theater masks emoji and space (no brackets)
        // RGB(50, 205, 50) - Lime green
        if let Some(style) = &self.output_style {
            if style != "default" && style != "null" {
                parts.push(format!("{}🎭 {}", separator, style.truecolor(50, 205, 50)));
            }
        }

        parts.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_minimal_statusline() {
        let status = StatusLine {
            directory: "~/src/grabby".to_string(),
            jj_info: None,
            model_name: "Claude 3.5 Sonnet".to_string(),
            output_style: Some("default".to_string()),
        };
        
        let formatted = status.format();
        // With colors and emojis, check for expected elements
        assert!(formatted.contains("📂 "));
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("🧠 "));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
        assert!(formatted.contains("‧")); // Should contain separator dots
        assert!(!formatted.contains("🎭")); // Should not show default style
        assert!(!formatted.contains("(")); // Should not contain parentheses
    }

    #[test]
    fn test_format_with_jj_info() {
        let jj_info = JjInfo {
            change_id: Some("abc123".to_string()),
            bookmarks: vec!["main".to_string()],
            has_conflict: false,
            has_changes: true,
        };
        
        let status = StatusLine {
            directory: "~/src/grabby".to_string(),
            jj_info: Some(jj_info),
            model_name: "Claude 3.5 Sonnet".to_string(),
            output_style: Some("default".to_string()),
        };
        
        let formatted = status.format();
        println!("Formatted output: '{}'", formatted);
        // With colors and emojis, check for expected elements
        assert!(formatted.contains("📂 "));
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("⚡ ")); // Should show lightning with space for changes
        assert!(formatted.contains("abc123")); // Change ID should be present
        assert!(formatted.contains("main")); // Bookmark should be present
        assert!(formatted.contains("*")); // Changes asterisk should be present
        assert!(formatted.contains("🧠 "));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
        assert!(formatted.contains("‧")); // Should contain separator dots
        assert!(!formatted.contains("(")); // Should not contain parentheses
    }

    #[test]
    fn test_format_with_output_style() {
        let status = StatusLine {
            directory: "~/src/grabby".to_string(),
            jj_info: None,
            model_name: "Claude 3.5 Sonnet".to_string(),
            output_style: Some("Learning".to_string()),
        };
        
        let formatted = status.format();
        assert!(formatted.contains("📂 "));
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("🧠 "));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
        assert!(formatted.contains("🎭 ")); // Should show theater masks with space
        assert!(formatted.contains("Learning"));
        assert!(formatted.contains("‧")); // Should contain separator dots
    }

    #[test]
    fn test_format_full_statusline() {
        let jj_info = JjInfo {
            change_id: Some("abc123".to_string()),
            bookmarks: vec!["main".to_string()],
            has_conflict: true,
            has_changes: true,
        };
        
        let status = StatusLine {
            directory: "~/src/grabby".to_string(),
            jj_info: Some(jj_info),
            model_name: "Claude 3.5 Sonnet".to_string(),
            output_style: Some("Explanatory".to_string()),
        };
        
        let formatted = status.format();
        assert!(formatted.contains("📂 "));
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("⚡ ")); // Should show lightning with space for changes
        assert!(formatted.contains("abc123")); // Change ID should be present
        assert!(formatted.contains("main")); // Bookmark should be present
        assert!(formatted.contains("conflict")); // Conflict indicator should be present
        assert!(formatted.contains("*")); // Changes asterisk should be present
        assert!(formatted.contains("🧠 "));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
        assert!(formatted.contains("🎭 "));
        assert!(formatted.contains("Explanatory"));
        assert!(formatted.contains("‧")); // Should contain separator dots
        assert!(!formatted.contains("(")); // Should not contain parentheses
    }

    #[test]
    fn test_format_with_clean_jj_status() {
        let jj_info = JjInfo {
            change_id: Some("abc123".to_string()),
            bookmarks: vec!["main".to_string()],
            has_conflict: false,
            has_changes: false, // No changes = clean
        };
        
        let status = StatusLine {
            directory: "~/src/grabby".to_string(),
            jj_info: Some(jj_info),
            model_name: "Claude 3.5 Sonnet".to_string(),
            output_style: Some("default".to_string()),
        };
        
        let formatted = status.format();
        assert!(formatted.contains("📂 "));
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("🔀 ")); // Should show twisted arrows with space for clean state
        assert!(formatted.contains("abc123")); // Change ID should be present
        assert!(formatted.contains("main")); // Bookmark should be present
        assert!(!formatted.contains("*")); // Should not have changes asterisk
        assert!(formatted.contains("🧠 "));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
        assert!(formatted.contains("‧")); // Should contain separator dots
        assert!(!formatted.contains("(")); // Should not contain parentheses
    }

    #[test]
    fn test_null_output_style_not_shown() {
        let status = StatusLine {
            directory: "~/src/grabby".to_string(),
            jj_info: None,
            model_name: "Claude 3.5 Sonnet".to_string(),
            output_style: Some("null".to_string()),
        };
        
        let formatted = status.format();
        assert!(!formatted.contains("[null]"));
    }

    #[test]
    fn test_brighter_pink_branch_names() {
        // Test with bookmarks (branch) - branch names should be brighter pink
        let jj_info_with_branch = JjInfo {
            change_id: Some("abc123".to_string()),
            bookmarks: vec!["feature-branch".to_string()],
            has_conflict: false,
            has_changes: false,
        };
        
        let status_with_branch = StatusLine {
            directory: "~/src/grabby".to_string(),
            jj_info: Some(jj_info_with_branch),
            model_name: "Claude 3.5 Sonnet".to_string(),
            output_style: None,
        };
        
        // Test without bookmarks (no branch) - only change ID in regular pink
        let jj_info_no_branch = JjInfo {
            change_id: Some("abc123".to_string()),
            bookmarks: vec![], // No bookmarks = not in named branch
            has_conflict: false,
            has_changes: false,
        };
        
        let status_no_branch = StatusLine {
            directory: "~/src/grabby".to_string(),
            jj_info: Some(jj_info_no_branch),
            model_name: "Claude 3.5 Sonnet".to_string(),
            output_style: None,
        };
        
        let formatted_with_branch = status_with_branch.format();
        let formatted_no_branch = status_no_branch.format();
        
        // Both should contain the change ID
        assert!(formatted_with_branch.contains("abc123"));
        assert!(formatted_no_branch.contains("abc123"));
        
        // Only the branch version should contain the branch name
        assert!(formatted_with_branch.contains("feature-branch"));
        assert!(!formatted_no_branch.contains("feature-branch"));
        
        // The formatted output should contain different color codes for branch names
        // Branch version should have full hot pink for branch names, duller for change ID
        assert!(formatted_with_branch.contains("255;20;147"));  // Full hot pink for branch name
        assert!(formatted_with_branch.contains("200;80;140"));   // Duller hot pink for change ID
        assert!(formatted_no_branch.contains("200;80;140"));     // Only duller hot pink for change ID
        assert!(!formatted_no_branch.contains("255;20;147"));    // No full hot pink without branches
    }
}