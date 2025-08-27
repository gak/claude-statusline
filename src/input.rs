use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaudeInput {
    pub session_id: String,
    pub transcript_path: String,
    pub cwd: String,
    pub model: ModelInfo,
    pub workspace: WorkspaceInfo,
    pub version: String,
    pub output_style: OutputStyle,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ModelInfo {
    pub id: String,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WorkspaceInfo {
    pub current_dir: String,
    pub project_dir: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OutputStyle {
    pub name: String,
}

impl ClaudeInput {
    pub fn from_stdin() -> Result<Self, Box<dyn std::error::Error>> {
        let mut input = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;
        let parsed: ClaudeInput = serde_json::from_str(&input)?;
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> ClaudeInput {
        ClaudeInput {
            session_id: "session-123".to_string(),
            transcript_path: "/tmp/transcript.json".to_string(),
            cwd: "/Users/gak/src/grabby".to_string(),
            model: ModelInfo {
                id: "claude-3-5-sonnet-20241022".to_string(),
                display_name: "Claude 3.5 Sonnet".to_string(),
            },
            workspace: WorkspaceInfo {
                current_dir: "/Users/gak/src/grabby".to_string(),
                project_dir: "/Users/gak/src/grabby".to_string(),
            },
            version: "1.0.71".to_string(),
            output_style: OutputStyle {
                name: "default".to_string(),
            },
        }
    }

    #[test]
    fn test_deserialize_sample_input() {
        let json = r#"{
            "session_id": "session-123",
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

        let parsed: ClaudeInput = serde_json::from_str(json).unwrap();
        assert_eq!(parsed, sample_input());
    }

    #[test]
    fn test_serialize_roundtrip() {
        let input = sample_input();
        let json = serde_json::to_string(&input).unwrap();
        let parsed: ClaudeInput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, input);
    }

    #[test]
    fn test_non_default_output_style() {
        let json = r#"{
            "session_id": "session-123",
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
                "name": "Learning"
            }
        }"#;

        let parsed: ClaudeInput = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.output_style.name, "Learning");
    }

    #[test]
    fn test_different_current_and_project_dirs() {
        let json = r#"{
            "session_id": "session-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/Users/gak/src/grabby/subdir",
            "model": {
                "id": "claude-3-5-sonnet-20241022",
                "display_name": "Claude 3.5 Sonnet"
            },
            "workspace": {
                "current_dir": "/Users/gak/src/grabby/subdir",
                "project_dir": "/Users/gak/src/grabby"
            },
            "version": "1.0.71",
            "output_style": {
                "name": "default"
            }
        }"#;

        let parsed: ClaudeInput = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.workspace.current_dir, "/Users/gak/src/grabby/subdir");
        assert_eq!(parsed.workspace.project_dir, "/Users/gak/src/grabby");
    }
}