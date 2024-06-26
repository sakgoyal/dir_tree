# dir_tree
Generate a JSON tree based on the provided path


## Example

File structure:
```sh
/
L src/
    L lib.rs
    L dir_trees.rs
```
```rust
let res = generate_directory_tree("./src");
assert_eq!(serde_json::to_string(&res), r#"{"/":["dir_tree.rs","lib.rs"]}");
```

File structure:
```sh
/
L src/
    L baz/
        L page.rs
    L foo/
        L bar/
            L page.rs
        L page.rs
    L 404.rs
    L layout.rs
    L page.rs
```
```rust
let res = generate_directory_tree("./src");
assert_eq!(serde_json::to_pretty_string(&res), r#"
{
  "/": [
    "404.rs",
    "layout.rs",
    "page.rs"
  ],
  "/baz": [
    "page.rs"
  ],
  "/foo": [
    "page.rs"
  ],
  "/foo/bar": [
    "page.rs"
  ]
}"#);
```
