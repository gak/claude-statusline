use colored::*;

pub struct StatusLine {
    pub directory: String,
    pub jj_info: Option<String>,
    pub model_name: String,
    pub output_style: Option<String>,
}

impl StatusLine {
    pub fn format(&self) -> String {
        // Force colors to be enabled for statusline
        colored::control::set_override(true);
        
        let mut parts = Vec::new();

        // Directory with folder emoji - subtle inline style
        // RGB(64, 224, 208) - Turquoise with punch
        parts.push(format!("📂{}", self.directory.truecolor(64, 224, 208)));

        // JJ info with dynamic emoji based on changes
        // RGB(255, 20, 147) - Deep pink that pops
        if let Some(jj_info) = &self.jj_info {
            // Check if there are uncommitted changes (indicated by * at the end)
            let emoji = if jj_info.ends_with('*') {
                "⚡" // Lightning for uncommitted changes
            } else {
                "🔀" // Twisted arrows for clean state
            };
            parts.push(format!(" ({}{})", emoji, jj_info.truecolor(255, 20, 147)));
        }

        // Model name with brain emoji - subtle inline style
        // RGB(255, 140, 0) - Vibrant orange
        parts.push(format!(" 🧠{}", self.model_name.truecolor(255, 140, 0)));

        // Output style with theater masks emoji - subtle inline style
        // RGB(50, 205, 50) - Lime green
        if let Some(style) = &self.output_style {
            if style != "default" && style != "null" {
                parts.push(format!(" [🎭{}]", style.truecolor(50, 205, 50)));
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
        assert!(formatted.contains("📂"));
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("🧠"));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
        assert!(!formatted.contains("🎭")); // Should not show default style
    }

    #[test]
    fn test_format_with_jj_info() {
        let status = StatusLine {
            directory: "~/src/grabby".to_string(),
            jj_info: Some("abc123 main*".to_string()),
            model_name: "Claude 3.5 Sonnet".to_string(),
            output_style: Some("default".to_string()),
        };
        
        let formatted = status.format();
        println!("Formatted output: '{}'", formatted);
        // With colors and emojis, check for expected elements
        assert!(formatted.contains("📂"));
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("⚡")); // Should show lightning for changes (ends with *)
        assert!(formatted.contains("abc123 main*"));
        assert!(formatted.contains("🧠"));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
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
        assert!(formatted.contains("📂"));
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("🧠"));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
        assert!(formatted.contains("🎭")); // Should show theater masks for output style
        assert!(formatted.contains("Learning"));
    }

    #[test]
    fn test_format_full_statusline() {
        let status = StatusLine {
            directory: "~/src/grabby".to_string(),
            jj_info: Some("abc123 main conflict*".to_string()),
            model_name: "Claude 3.5 Sonnet".to_string(),
            output_style: Some("Explanatory".to_string()),
        };
        
        let formatted = status.format();
        assert!(formatted.contains("📂"));
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("⚡")); // Should show lightning for changes (ends with *)
        assert!(formatted.contains("abc123 main conflict*"));
        assert!(formatted.contains("🧠"));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
        assert!(formatted.contains("🎭"));
        assert!(formatted.contains("Explanatory"));
    }

    #[test]
    fn test_format_with_clean_jj_status() {
        let status = StatusLine {
            directory: "~/src/grabby".to_string(),
            jj_info: Some("abc123 main".to_string()), // No * = clean
            model_name: "Claude 3.5 Sonnet".to_string(),
            output_style: Some("default".to_string()),
        };
        
        let formatted = status.format();
        assert!(formatted.contains("📂"));
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("🔀")); // Should show twisted arrows for clean state
        assert!(formatted.contains("abc123 main"));
        assert!(formatted.contains("🧠"));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
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
}