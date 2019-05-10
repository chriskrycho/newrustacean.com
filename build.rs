use std::process::Command;

use cc::Build;

fn main() {
    // Build a C file to link into Rust code.
    Build::new().file("src/e029/ffi-demo.c").compile("ffi-demo");

    // Build a C file which links *against* Rust code.
    Command::new("clang")
        .args(&[
            "src/e031/e031.c",
            "-Ltarget/debug",
            "-lshow_notes",
            "-o",
            "link-against-rust",
        ])
        .spawn()
        .expect("executed the build for linking in Rust from C");
}
