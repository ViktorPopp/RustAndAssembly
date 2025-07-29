use std::fs;
use std::path::PathBuf;

fn main() {
    let mut build = cc::Build::new();

    build.flag("-c");

    for entry in fs::read_dir("asm").unwrap() {
        let path: PathBuf = entry.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext == "s" || ext == "asm" {
                build.file(path);
            }
        }
    }

    build.compile("assembly");
}
