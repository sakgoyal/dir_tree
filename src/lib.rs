#[cfg(test)]
mod tests {
    use crate::generate_directory_tree;


    #[test]
    fn test_dir_tree() {
        let res = generate_directory_tree("./src");
        let string = serde_json::to_string(&res).unwrap();
        println!("Generated JSON: {}", string);
        println!("Expected  JSON: {}", "{\"/\":[\"lib.rs\"]}");
        assert_eq!(string, "{\"/\":[\"lib.rs\"]}");
    }
}

/// Generate a JSON tree based on the provided path
///
/// ## Example
///
/// ### Example 1
/// File structure:
/// ```sh
/// /
/// L src/
///     L lib.rs
/// ```
/// ```
/// let res = crate::dir_tree::generate_directory_tree("./src");
/// assert_eq!(serde_json::to_string(&res).unwrap(), r#"{"/":["lib.rs"]}"#);
pub fn generate_directory_tree(path: &str) -> std::collections::HashMap<String, Vec<String>> {
    let mut dir_map = std::collections::HashMap::new();

    for entry in walkdir::WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let entry_path = entry.path();
        let relative_path = entry_path.strip_prefix(path).unwrap();

        if entry_path.is_file() {
            let parent_dir = relative_path.parent().unwrap_or_else(|| std::path::Path::new("/"));
            let file_name = entry_path.file_name().unwrap().to_string_lossy().into_owned();
            let parent_dir_str = format!("/{}", parent_dir.to_string_lossy().replace("\\", "/"));

            dir_map
                .entry(parent_dir_str.clone())
                .or_insert_with(|| vec![])
                .push(file_name);
        }
    }

    dir_map
}
