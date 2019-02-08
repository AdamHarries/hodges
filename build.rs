extern crate bindgen;
extern crate cmake;

use cmake::Config;

use std::env;
use std::path::PathBuf;

fn main() {
    // tell cargo to build our taglib branch
    let dst = Config::new("sys")
        .static_crt(true)
        .very_verbose(true)
        .define("CMAKE_C_FLAGS", "-fPIC -Wall -O3")
        .define("CMAKE_CXX_FLAGS", "-fPIC -Wall -O3")
        .build();
    // tell cargo to look for talamel, taglib, and zlib
    // Specifically, specify the paths.
    // println!("cargo:rustc-link-search={}", dst.display());
    // println!(
    //     "cargo:rustc-link-search={}/build/taglib/install/lib",
    //     dst.display()
    // );
    // println!(
    //     "cargo:rustc-link-search={}/build/zlib/install/lib",
    //     dst.display()
    // );

    // // Link talamel, tag_c, tag, zlib, and the c++ runtime.
    // println!("cargo:rustc-link-lib=static=talamel");
    // println!("cargo:rustc-link-lib=static=tag_c");
    // println!("cargo:rustc-link-lib=static=tag");

    // if cfg!(target_os = "windows") {
    //     // If we're linking for windows, we don't need to link
    //     // the C++ runtime, as it's done for us above
    //     // zlib is also named differently.
    //     println!("cargo:rustc-link-lib=static=zlib");
    // } else if cfg!(target_os = "macos") {
    //     // On OSX we do need to, and it's called 'C++'
    //     // link zlib as standard on unix.
    //     println!("cargo:rustc-link-lib=static=z");
    //     println!("cargo:rustc-link-lib=c++");
    // } else {
    //     // We also need to link it on Linux - it's called 'stdc++'
    //     println!("cargo:rustc-link-lib=static=z");
    //     println!("cargo:rustc-link-lib=stdc++");
    // }

    // create bindings for the static c library
    let header = dst.join("libhodges.h");
    let bindings = bindgen::Builder::default()
        // use the header from the dst, where cmake has writen the headers
        .header(header.to_str().unwrap())
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
