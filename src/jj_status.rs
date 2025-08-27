use std::process::Command;

pub struct JjInfo {
    pub change_id: Option<String>,
    pub bookmarks: Vec<String>,
    pub has_conflict: bool,
    pub has_changes: bool,
}

impl JjInfo {
    pub fn empty() -> Self {
        Self {
            change_id: None,
            bookmarks: Vec::new(),
            has_conflict: false,
            has_changes: false,
        }
    }

    pub fn format(&self) -> Option<String> {
        if self.change_id.is_none() {
            return None;
        }

        let mut parts = Vec::new();
        
        if let Some(change_id) = &self.change_id {
            parts.push(change_id.clone());
        }

        if !self.bookmarks.is_empty() {
            parts.push(self.bookmarks.join(" "));
        }

        if self.has_conflict {
            parts.push("conflict".to_string());
        }

        let mut result = parts.join(" ");
        
        if self.has_changes {
            result.push('*');
        }

        Some(result)
    }
}

pub fn get_jj_status(path: &str) -> JjInfo {
    // For now, fall back to using jj commands until jj-lib API is clearer
    // This maintains the same interface for future jj-lib integration
    
    // Check if this is a jj repository
    let is_jj_repo = Command::new("jj")
        .arg("root")
        .current_dir(path)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);
    
    if !is_jj_repo {
        return JjInfo::empty();
    }
    
    // Get current revision info
    let revision_output = Command::new("jj")
        .args(&["log", "-r", "@", "--no-graph", "-T", 
               "change_id.short() ++ \" \" ++ bookmarks.join(\" \") ++ if(conflict, \" conflict\", \"\")"])
        .current_dir(path)
        .output();
        
    let revision_info = match revision_output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => return JjInfo::empty(),
    };
    
    if revision_info.is_empty() || revision_info == "@" {
        return JjInfo::empty();
    }
    
    // Parse the output
    let parts: Vec<&str> = revision_info.split_whitespace().collect();
    let change_id = parts.first().map(|s| s.to_string());
    let has_conflict = revision_info.contains(" conflict");
    
    // Get bookmarks (everything between change_id and "conflict" if present)
    let bookmarks: Vec<String> = parts
        .iter()
        .skip(1) // Skip change_id
        .take_while(|&part| *part != "conflict")
        .map(|s| s.to_string())
        .collect();
    
    // Check for changes
    let has_changes = Command::new("jj")
        .args(&["diff", "--summary"])
        .current_dir(path)
        .output()
        .map(|output| output.status.success() && !output.stdout.is_empty())
        .unwrap_or(false);
    
    JjInfo {
        change_id,
        bookmarks,
        has_conflict,
        has_changes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_jj_info_formats_to_none() {
        let info = JjInfo::empty();
        assert_eq!(info.format(), None);
    }

    #[test]
    fn test_jj_info_with_change_id_only() {
        let info = JjInfo {
            change_id: Some("abc123".to_string()),
            bookmarks: Vec::new(),
            has_conflict: false,
            has_changes: false,
        };
        assert_eq!(info.format(), Some("abc123".to_string()));
    }

    #[test]
    fn test_jj_info_with_change_id_and_bookmarks() {
        let info = JjInfo {
            change_id: Some("abc123".to_string()),
            bookmarks: vec!["main".to_string(), "feature".to_string()],
            has_conflict: false,
            has_changes: false,
        };
        assert_eq!(info.format(), Some("abc123 main feature".to_string()));
    }

    #[test]
    fn test_jj_info_with_changes() {
        let info = JjInfo {
            change_id: Some("abc123".to_string()),
            bookmarks: vec!["main".to_string()],
            has_conflict: false,
            has_changes: true,
        };
        assert_eq!(info.format(), Some("abc123 main*".to_string()));
    }

    #[test]
    fn test_jj_info_with_conflict() {
        let info = JjInfo {
            change_id: Some("abc123".to_string()),
            bookmarks: vec!["main".to_string()],
            has_conflict: true,
            has_changes: false,
        };
        assert_eq!(info.format(), Some("abc123 main conflict".to_string()));
    }

    #[test]
    fn test_jj_info_with_everything() {
        let info = JjInfo {
            change_id: Some("abc123".to_string()),
            bookmarks: vec!["main".to_string()],
            has_conflict: true,
            has_changes: true,
        };
        assert_eq!(info.format(), Some("abc123 main conflict*".to_string()));
    }
}