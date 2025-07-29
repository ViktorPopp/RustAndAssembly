fn main() {
    cc::Build::new()
        .file("src/my_assembly.s") // Path to your assembly file
        .flag("-c") // Compile to object file
        .compile("my_assembly"); // Output object file name
}
