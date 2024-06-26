use std::path::Path;
use serde_json::{Map, Value};
use walkdir::WalkDir;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_tree() {
        let res = generate_directory_tree("./src");
        let string = serde_json::to_string(&res).unwrap();
        print!("{}", string);
        assert_eq!(string, r#"{"/":["lib.rs"]}"#);
    }
}

/// Generate a JSON tree based on the provided path
///
/// ```
/// let res = generate_directory_tree(r#"./src"#);
/// assert_eq!(serde_json::to_string(&res), r#"{"/":["lib.rs"]}"#);
/// ```
fn generate_directory_tree(path: &str) -> Map<String, Value> {
    let mut dir_map = Map::new();

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let entry_path = entry.path();
        let relative_path = entry_path.strip_prefix(path).unwrap();

        if entry_path.is_file() {
            let parent_dir = relative_path.parent().unwrap_or_else(|| Path::new("/"));
            let file_name = entry_path.file_name().unwrap().to_string_lossy().into_owned();
            let parent_dir_str = format!("/{}", parent_dir.to_string_lossy().replace("\\", "/"));

            dir_map
                .entry(parent_dir_str.clone())
                .or_insert_with(|| Value::Array(vec![]))
                .as_array_mut()
                .unwrap()
                .push(Value::String(file_name));
        }
    }

    dir_map
}
