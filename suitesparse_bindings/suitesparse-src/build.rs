use std::path::{PathBuf, Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let root = std::env::var_os("OUT_DIR").unwrap();

    let mut suitesparse_config = false;
    if std::env::var_os("CARGO_FEATURE_CAMD").is_some() {
        suitesparse_config = true;
        cc::Build::new()
            .define("DLONG", None)
            .include("SuiteSparse/SuiteSparse_config")
            .include("SuiteSparse/CAMD/Include")
            .file("SuiteSparse/CAMD/Source/camd_1.c")
            .file("SuiteSparse/CAMD/Source/camd_2.c")
            .file("SuiteSparse/CAMD/Source/camd_aat.c")
            .file("SuiteSparse/CAMD/Source/camd_control.c")
            .file("SuiteSparse/CAMD/Source/camd_defaults.c")
            .file("SuiteSparse/CAMD/Source/camd_dump.c")
            .file("SuiteSparse/CAMD/Source/camd_global.c")
            .file("SuiteSparse/CAMD/Source/camd_info.c")
            .file("SuiteSparse/CAMD/Source/camd_order.c")
            .file("SuiteSparse/CAMD/Source/camd_postorder.c")
            .file("SuiteSparse/CAMD/Source/camd_preprocess.c")
            .file("SuiteSparse/CAMD/Source/camd_valid.c")
            .cargo_metadata(false)
            .compile("camdl");
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .include("SuiteSparse/CAMD/Include")
            .file("SuiteSparse/CAMD/Source/camd_1.c")
            .file("SuiteSparse/CAMD/Source/camd_2.c")
            .file("SuiteSparse/CAMD/Source/camd_aat.c")
            .file("SuiteSparse/CAMD/Source/camd_control.c")
            .file("SuiteSparse/CAMD/Source/camd_defaults.c")
            .file("SuiteSparse/CAMD/Source/camd_dump.c")
            .file("SuiteSparse/CAMD/Source/camd_global.c")
            .file("SuiteSparse/CAMD/Source/camd_info.c")
            .file("SuiteSparse/CAMD/Source/camd_order.c")
            .file("SuiteSparse/CAMD/Source/camd_postorder.c")
            .file("SuiteSparse/CAMD/Source/camd_preprocess.c")
            .file("SuiteSparse/CAMD/Source/camd_valid.c")
            .cargo_metadata(false)
            .compile("camd");
    }
    if std::env::var_os("CARGO_FEATURE_LDL").is_some() {
        // We first build ldl with LDL_LONG to make the bindings to
        // the long bits of the library
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .include("SuiteSparse/LDL/Include")
            .file("SuiteSparse/LDL/Source/ldl.c")
            .cargo_metadata(false)
            .define("LDL_LONG", None)
            .compile("ldll");
        // We must then copy this to another location since the next
        // invocation is just a compile definition
        let mut ldl_path = std::path::PathBuf::from(root.clone());
        ldl_path.push("SuiteSparse/LDL/Source/ldl.o");
        let mut ldll_path = ldl_path.clone();
        ldll_path.set_file_name("ldll.o");
        std::fs::copy(&ldl_path, &ldll_path).unwrap();
        // And now we build ldl again (in int form), and link with the long bits
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .include("SuiteSparse/LDL/Include")
            .file("SuiteSparse/LDL/Source/ldl.c")
            .object(&ldll_path)
            .cargo_metadata(false)
            .compile("ldl");
    }
    if cfg!(feature = "umfpack") {
        suitesparse_config = true;

        let amd_files = get_source_files(PathBuf::from("SuiteSparse/AMD/Source"));

        let mut cholmod_files = get_source_files(PathBuf::from("SuiteSparse/CHOLMOD/Core"));
        cholmod_files.append(&mut get_source_files(PathBuf::from("SuiteSparse/CHOLMOD/Check")));
        cholmod_files.append(&mut get_source_files(PathBuf::from("SuiteSparse/CHOLMOD/Cholesky")));
        cholmod_files.append(&mut get_source_files(PathBuf::from("SuiteSparse/CHOLMOD/Partition")));
        cholmod_files.append(&mut get_source_files(PathBuf::from("SuiteSparse/CHOLMOD/Modify")));
        cholmod_files.append(&mut get_source_files(PathBuf::from("SuiteSparse/CHOLMOD/MatrixOps")));
        cholmod_files.append(&mut get_source_files(PathBuf::from("SuiteSparse/CHOLMOD/Supernodal")));
        cholmod_files.append(&mut get_source_files(PathBuf::from("SuiteSparse/CHOLMOD/GPU")));

        let umfpack_files = get_source_files(PathBuf::from("SuiteSparse/UMFPACK/Source"));

        // Build AMD
        //    Double-Int version
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .include("SuiteSparse/AMD/Include")
            .files(&amd_files)
            // .define("DINT", None)
            .compile("amd");

        let objs = get_files_of_kind(PathBuf::from(&root).join("SuiteSparse/AMD/Source"), "o");
        println!("{objs:?}");
        add_object_file_prefixes(PathBuf::from(&root).join("SuiteSparse/AMD/Source").to_str().unwrap(), "di_");

        //    Double-Long version & combine with double-int
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .include("SuiteSparse/AMD/Include")
            .files(&amd_files)
            .define("DLONG", None)
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_1.o"))
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_2.o"))
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_aat.o"))
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_control.o"))
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_defaults.o"))
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_dump.o"))
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_global.o"))
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_info.o"))
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_order.o"))
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_post_tree.o"))
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_postorder.o"))
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_preprocess.o"))
            .object(PathBuf::from(&root).join("SuiteSparse/AMD/Source/di_amd_valid.o"))
            .compile("amd");

        // Build COLAMD
        //    Double-Int version
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .include("SuiteSparse/COlAMD/Include")
            .file("SuiteSparse/COLAMD/Source/colamd.c")
            .compile("colamd");
        add_object_file_prefixes(PathBuf::from(&root).join("SuiteSparse/COLAMD/Source/").to_str().unwrap(), "di_");

        //    Double-Long version & combine with double-int
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .include("SuiteSparse/COlAMD/Include")
            .file("SuiteSparse/COLAMD/Source/colamd.c")
            .define("DLONG", None)
            .object(PathBuf::from(&root).join("SuiteSparse/COLAMD/Source/di_colamd.o"))
            .compile("colamd");

        // Build CHOLMOD
        //    Double-Int version
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .include("SuiteSparse/CHOLMOD/Include")
            .include("SuiteSparse/AMD/Include")
            .include("SuiteSparse/COLAMD/Include")
            .include("SuiteSparse/include")
            .include("SuiteSparse/AMD/Source")

            .files(&cholmod_files)
            .compile("cholmod");

        //    Double-Long version & combine with double-int
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .include("SuiteSparse/CHOLMOD/Include")
            .files(&cholmod_files)
            .define("DLONG", None)
            .object("SuiteSparse/CHOLMOD/Source/cholmod_di.o")
            .compile("cholmod");

        // Build UMFPACK
        //    Double-Int version
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .include("SuiteSparse/UMFPACK/Include")
            .files(&umfpack_files)
            .object("SuiteSparse/AMD/Source/amd.o")
            .object("SuiteSparse/CHOLMOD/Source/cholmod.o")
            .define("DINT", None)
            .compile("umfpacki");
        // copy_file("SuiteSparse/UMFPACK/Source/umfpack.o", "umfpack_di.o");

        //    Double-Long version & combine with double-int
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .include("SuiteSparse/UMFPACK/Include")
            .files(&umfpack_files)
            .object("SuiteSparse/AMD/Source/amd.o")
            .object("SuiteSparse/CHOLMOD/Source/cholmod.o")
            .object("SuiteSparse/UMFPACK/Source/umfpack_di.o")
            .define("DLONG", None)
            .compile("umfpack");

    }
    if suitesparse_config {
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .file("SuiteSparse/SuiteSparse_config/SuiteSparse_config.c")
            .cargo_metadata(false)
            .compile("suitesparseconfig");
    }
    println!("cargo:root={}", root.to_string_lossy());
}

/// Returns a vector of all the .c files in a given directory
fn get_files_of_kind(dir: PathBuf, kind: &str) -> Vec<PathBuf> {
    let mut sources = Vec::new();
    for entry in dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some(kind) {
                sources.push(path);
            }
        }
    }
    sources
}

fn get_source_files(dir: PathBuf) -> Vec<PathBuf> {
    return get_files_of_kind(dir, "c")
}

fn get_object_files(dir: PathBuf) -> Vec<PathBuf> {
    return get_files_of_kind(dir, "o")
}

/// Copy object files in a directory, adding prefixes
/// then return the paths to the new files
fn add_object_file_prefixes(dir: &str, prefix: &str) -> Vec<PathBuf> {
    let dir = PathBuf::from(dir);
    let objects = get_object_files(dir);
    let mut new_paths: Vec<PathBuf> = Vec::new();
    for object in objects {
        let name = object.file_name().unwrap().to_str().unwrap();
        let new_name = format!("{prefix}{name}");
        let mut newpath = object.clone();
        newpath.set_file_name(new_name);
        println!("cargo:warn={object:?},{newpath:?}");
        std::fs::copy(&object, &newpath).unwrap();
        new_paths.push(newpath);
    }
    // println!("cargo:warn={new_paths:?}");
    new_paths
}