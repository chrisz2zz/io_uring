use core::panic;
use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-lib=dylib=uring");
    println!("cargo:rerun-if-changed=wrapper.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let extern_c_path = env::temp_dir().join("bindgen").join("extern.c");

    let bindgen_output = Command::new("bindgen")
        .arg("--experimental")
        .arg("--wrap-static-fns")
        .arg("wrapper.h")
        .arg("--output")
        .arg(out_path.join("bindings.rs"))
        .output()
        .expect("Failed to generate bindings");

    if !bindgen_output.status.success() {
        panic!(
            "Could not generate bindings:\n{}",
            String::from_utf8_lossy(&bindgen_output.stderr)
        );
    }

    let gcc_output = Command::new("gcc")
        .arg("-c")
        .arg("-fPIC")
        .arg("-I/usr/include")
        .arg("-I.")
        .arg(&extern_c_path)
        .arg("-o")
        .arg(out_path.join("extern.o"))
        .output()
        .expect("Failed to compile C code");

    if !gcc_output.status.success() {
        panic!(
            "Failed to compile C code:\n{}",
            String::from_utf8_lossy(&gcc_output.stderr)
        );
    }

    let ar_output = Command::new("ar")
        .arg("crus")
        .arg(out_path.join("libextern.a"))
        .arg(out_path.join("extern.o"))
        .output()
        .expect("Failed to create static library");

    if !ar_output.status.success() {
        panic!(
            "Failed to create static library:\n{}",
            String::from_utf8_lossy(&ar_output.stderr)
        );
    }

    println!("cargo:rustc-link-search=native={}", out_path.display());
    println!("cargo:rustc-link-lib=static=extern");
}
