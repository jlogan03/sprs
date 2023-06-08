fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if !cfg!(feature = "static") {
        // Dynamically link
        println!("cargo:rustc-link-lib=umfpack");
    }
    else {
        // Add the built libraries to path
        let path_to_src = std::env::var("DEP_SUITESPARSE_SRC_ROOT").unwrap();
        println!("cargo:rustc-link-search=native={path_to_src}");
        // Statically link umfpack
        println!("cargo:rustc-link-lib=static=umfpack");
    }

}
