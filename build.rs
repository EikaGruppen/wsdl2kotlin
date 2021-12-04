use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["..", "tree-sitter-java"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        // .file(dir.join("scanner.c"))
        .compile("tree-sitter-java");
}
