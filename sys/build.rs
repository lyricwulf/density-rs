use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let density_src = "density/src";

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", density_src);

    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // link the compiled static library
    cc::Build::new()
        .file("density/src/globals.c")
        .file("density/src/structure/header.c")
        .file("density/src/buffers/buffer.c")
        .file("density/src/algorithms/algorithms.c")
        .file("density/src/algorithms/dictionaries.c")
        .file("density/src/algorithms/chameleon/core/chameleon_decode.c")
        .file("density/src/algorithms/chameleon/core/chameleon_encode.c")
        .file("density/src/algorithms/cheetah/core/cheetah_decode.c")
        .file("density/src/algorithms/cheetah/core/cheetah_encode.c")
        .file("density/src/algorithms/lion/core/lion_decode.c")
        .file("density/src/algorithms/lion/core/lion_encode.c")
        .file("density/src/algorithms/lion/forms/lion_form_model.c")
        .compile("density");
}
