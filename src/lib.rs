pub mod dir_tree;

#[cfg(test)]
mod tests {
    use crate::dir_tree::generate_directory_tree;

    #[test]
    fn test_dir_tree() {
        let res = generate_directory_tree("./src");
        let string = serde_json::to_string(&res).unwrap();
        println!("Generated JSON: {}", string);
        println!("Expected  JSON: {}", "{\"/\":[\"dir_tree.rs\",\"lib.rs\"]}");
        assert_eq!(string, "{\"/\":[\"dir_tree.rs\",\"lib.rs\"]}");
    }
}
