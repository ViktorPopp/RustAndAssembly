use std::fs;
use std::path::PathBuf;

fn main() {
    let mut build = cc::Build::new();

    // Enable PIC for assembly compilation
    build.flag("-fPIC");

    // Keep the -c flag to compile without linking
    build.flag("-c");

    // Iterate over assembly files in src/asm
    for entry in fs::read_dir("src/asm").unwrap() {
        let path: PathBuf = entry.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext == "s" || ext == "asm" {
                build.file(&path);
            }
        }
    }

    // Compile the assembly files into a static library
    build.compile("assembly");

    // Optional: Rerun build script if assembly files change
    println!("cargo:rerun-if-changed=src/asm");
}
