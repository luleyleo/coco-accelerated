fn main() {
    cc::Build::new()
        .file("src/legacy_code.c")
        .compile("bbob_legacy");
}
