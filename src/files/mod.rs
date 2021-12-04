use std::fs;
use std::path::{Path, PathBuf};

pub fn find_classes(generated_dir: &Path) -> Vec<PathBuf> {
    walk(generated_dir, &is_class)
}

fn is_class(path: &Path) -> Option<&Path> {
    let files_to_skip = vec![
        "package-info.java",
        "ObjectFactory.java",
    ];

    if path.extension().map_or(false, |e| e == "java")
        && !files_to_skip.contains(&path.file_name()?.to_str()?)
    {
        Some(path)
    } else {
        None
    }
}

fn walk(path: &Path, is_class: &dyn Fn(&Path) -> Option<&Path>) -> Vec<PathBuf> {
    let mut classes: Vec<PathBuf> = vec![];
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                classes.extend(walk(&path, is_class));
            } else {
                if is_class(&path).is_some() {
                    classes.push(path);
                }
            }
        }
    } else {
        if is_class(path).is_some() {
            classes.push(path.to_path_buf())
        }
    }
    classes
}

//TODO skip hidden folders?
fn _in_package_block_list(path: &str, package_block_list: Vec<String>) -> bool {
    let package = path.replace("/", ".");
    package_block_list.iter().any(|blocked| &package == blocked)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn has_dup<T: PartialEq>(slice: &[T]) -> bool {
        for i in 1..slice.len() {
            if slice[i..].contains(&slice[i - 1]) {
                return true;
            }
        }
        false
    }

    // TODO mock filesystem
    // #[test]
    // fn test_find_classes() {
    //     
    //     let generated_dir =
    //         Path::new("");
    //
    //     let classes = find_classes(generated_dir);
    //
    //     assert!(!has_dup(&classes));
    //     assert!(classes.contains(&Path::new("").to_path_buf()));
    //     assert!(!classes.contains(&Path::new("").to_path_buf()))
    // }
}
