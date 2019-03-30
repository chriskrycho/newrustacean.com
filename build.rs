use cc::Build;

fn main() {
    Build::new()
        .file("src/e029/ffi-demo.c")
        .compile("ffi-demo");
}
