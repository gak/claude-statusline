use std::path::Path;

pub fn format_directory(path: &str, home_dir: Option<&str>) -> String {
    if let Some(home) = home_dir {
        if path.starts_with(home) {
            let relative = path.strip_prefix(home).unwrap_or("");
            if relative.is_empty() {
                "~".to_string()
            } else {
                format!("~{}", relative)
            }
        } else {
            truncate_path(path)
        }
    } else {
        truncate_path(path)
    }
}

fn truncate_path(path: &str) -> String {
    if path.len() > 30 {
        Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(path)
            .to_string()
    } else {
        path.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_directory_home_abbreviation() {
        let path = "/Users/gak/src/grabby";
        let home = "/Users/gak";
        assert_eq!(format_directory(path, Some(home)), "~/src/grabby");
    }

    #[test]
    fn test_format_directory_exact_home() {
        let path = "/Users/gak";
        let home = "/Users/gak";
        assert_eq!(format_directory(path, Some(home)), "~");
    }

    #[test]
    fn test_format_directory_no_home() {
        let path = "/usr/local/bin";
        assert_eq!(format_directory(path, None), "/usr/local/bin");
    }

    #[test]
    fn test_format_directory_outside_home() {
        let path = "/usr/local/bin";
        let home = "/Users/gak";
        assert_eq!(format_directory(path, Some(home)), "/usr/local/bin");
    }

    #[test]
    fn test_truncate_long_path() {
        let long_path = "/very/long/path/to/some/deeply/nested/directory/with/many/components";
        let home = "/Users/gak";
        assert_eq!(format_directory(long_path, Some(home)), "components");
    }

    #[test]
    fn test_truncate_long_path_no_home() {
        let long_path = "/very/long/path/to/some/deeply/nested/directory/with/many/components";
        assert_eq!(format_directory(long_path, None), "components");
    }

    #[test]
    fn test_format_directory_home_subdirectory_long() {
        let path = "/Users/gak/very/long/path/to/some/deeply/nested/directory";
        let home = "/Users/gak";
        assert_eq!(format_directory(path, Some(home)), "~/very/long/path/to/some/deeply/nested/directory");
    }

    #[test]
    fn test_edge_case_empty_path() {
        assert_eq!(format_directory("", None), "");
    }

    #[test]
    fn test_edge_case_root_path() {
        assert_eq!(format_directory("/", None), "/");
    }
}