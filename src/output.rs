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

        // Directory in cyan (dimmed)
        parts.push(self.directory.bright_cyan().dimmed().to_string());

        // JJ info in green (dimmed)
        if let Some(jj_info) = &self.jj_info {
            parts.push(format!(" ({})", jj_info.bright_green().dimmed()));
        }

        // Model name in magenta (dimmed)
        parts.push(format!(" {}", self.model_name.bright_magenta().dimmed()));

        // Output style in yellow brackets (dimmed)
        if let Some(style) = &self.output_style {
            if style != "default" && style != "null" {
                parts.push(format!(" [{}]", style.bright_yellow().dimmed()));
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
        // Since we can't easily test colored output, just check the basic structure
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
        assert!(!formatted.contains("[default]")); // Should not show default style
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
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("(abc123 main*)"));
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
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
        assert!(formatted.contains("[Learning]"));
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
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("(abc123 main conflict*)"));
        assert!(formatted.contains("Claude 3.5 Sonnet"));
        assert!(formatted.contains("[Explanatory]"));
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