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

        // Directory in vibrant teal - bold but easier on eyes than electric blue  
        // RGB(64, 224, 208) - Turquoise with punch
        parts.push(self.directory.truecolor(64, 224, 208).to_string());

        // JJ info in hot pink - energetic and fun
        // RGB(255, 20, 147) - Deep pink that pops
        if let Some(jj_info) = &self.jj_info {
            parts.push(format!(" ({})", jj_info.truecolor(255, 20, 147)));
        }

        // Model name in electric orange - warm and attention-getting
        // RGB(255, 140, 0) - Vibrant orange
        parts.push(format!(" {}", self.model_name.truecolor(255, 140, 0)));

        // Output style in neon lime - bright and modern
        // RGB(50, 205, 50) - Lime green
        if let Some(style) = &self.output_style {
            if style != "default" && style != "null" {
                parts.push(format!(" [{}]", style.truecolor(50, 205, 50)));
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
        println!("Formatted output: '{}'", formatted);
        // With colors, we need to check that the text is present (ignoring ANSI codes)
        assert!(formatted.contains("~/src/grabby"));
        assert!(formatted.contains("abc123 main*")); // Content should be present
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
        assert!(formatted.contains("Learning")); // Just check for text content
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
        assert!(formatted.contains("abc123 main conflict*")); // Just check for text content
        assert!(formatted.contains("Claude 3.5 Sonnet"));
        assert!(formatted.contains("Explanatory")); // Just check for text content
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