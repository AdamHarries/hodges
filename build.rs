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
        .cflag("-fPIC")
        .cflag("-O3")
        .cflag("-Ofast")
        .cflag("-march=native")
        .cflag("-funroll-loops")
        // .cxxflag("-fPIC")
        // .cxxflag("-Wall")
        // .cxxflag("-O3")
        .build();
    // Link hodges, and the various ffmpeg libraries, once we've told it where to search foor hodges
    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-lib=static=hodges");
    println!("cargo:rustc-link-lib=avdevice");
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avfilter");
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=swresample");
    println!("cargo:rustc-link-lib=swscale");
    println!("cargo:rustc-link-lib=avutil");

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
