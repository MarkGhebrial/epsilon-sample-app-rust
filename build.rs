use cc::Build;
use std::{
    process::Command,
    env,
    path::PathBuf
};

fn main() {
    // Assemble eadk.s
    println!("cargo:rerun-if-changed=eadk/eadk.s");
    Build::new().file("eadk/eadk.s").compile("asm");
    println!("FINISHED COMPILING eadk.s ~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    // Build bindings for the eadk
    let bindings = bindgen::Builder::default()
        .header("eadk/eadk.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/eadk-bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("eadk-bindings.rs"))
        .expect("Couldn't write bindings!");

    // Turn icon.png into a linker-embeddable file
    println!("cargo:rerun-if-changed=src/icon.png");
    let output = Command::new("./eadk/inliner.py")
        .args(&["src/icon.png", "target/icon.ld"])
        .output().expect("Failure to launch process");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}
