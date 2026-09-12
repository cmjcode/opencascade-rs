/// Minimum compatible version of OpenCASCADE library (major, minor)
///
/// Pre-installed OpenCASCADE library will be checked for compatibility using semver rules.
const OCCT_VERSION: (u8, u8) = (7, 8);

/// The list of used OpenCASCADE libraries which needs to be linked with.
const OCCT_LIBS: &[&str] = &[
    "TKMath",
    "TKernel",
    "TKDE",
    "TKFeat",
    "TKGeomBase",
    "TKG2d",
    "TKG3d",
    "TKTopAlgo",
    "TKGeomAlgo",
    "TKBRep",
    "TKPrim",
    "TKDESTEP",
    "TKDEIGES",
    "TKDESTL",
    "TKMesh",
    "TKHLR",
    "TKShHealing",
    "TKFillet",
    "TKBool",
    "TKBO",
    "TKOffset",
    "TKXSBase",
    "TKCAF",
    "TKLCAF",
    "TKXCAF",
];

fn main() {
    let target = std::env::var("TARGET").expect("No TARGET environment variable defined");
    let is_windows = target.to_lowercase().contains("windows");
    let is_windows_gnu = target.to_lowercase().contains("windows-gnu");

    let occt_config = OcctConfig::detect();

    println!("cargo:rustc-link-search=native={}", occt_config.library_dir.to_str().unwrap());

    let lib_type = if occt_config.is_dynamic { "dylib" } else { "static" };
    let is_apple = target.to_lowercase().contains("apple");

    if !occt_config.is_dynamic && is_apple {
        // Gabungkan semua file .o menggunakan libtool untuk menghindari pemotongan command-line ARG_MAX ar
        if let Some(occt_root) = occt_config.library_dir.parent() {
            let build_src = occt_root.join("build").join("src");
            if build_src.exists() {
                if let Ok(entries) = std::fs::read_dir(&build_src) {
                    for entry in entries.flatten() {
                        let pkg_name = entry.file_name().to_string_lossy().into_owned();
                        if pkg_name.starts_with("TK") {
                            let lib_file = occt_config.library_dir.join(format!("lib{pkg_name}.a"));
                            let obj_dir = entry.path().join("CMakeFiles").join(format!("{pkg_name}.dir"));
                            if obj_dir.exists() && lib_file.exists() {
                                let mut obj_files = Vec::new();
                                collect_object_files(&obj_dir, &mut obj_files);
                                if !obj_files.is_empty() {
                                    let _ = std::process::Command::new("libtool")
                                        .arg("-static")
                                        .arg("-o")
                                        .arg(&lib_file)
                                        .args(&obj_files)
                                        .status();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if !occt_config.is_dynamic {
        if let Ok(entries) = std::fs::read_dir(&occt_config.library_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("libTK") && name_str.ends_with(".a") {
                    let lib_name = &name_str[3..name_str.len() - 2];
                    println!("cargo:rustc-link-lib=static={lib_name}");
                }
            }
        }
    }
    for lib in OCCT_LIBS {
        println!("cargo:rustc-link-lib={lib_type}={lib}");
    }


    if is_apple {
        println!("cargo:rustc-link-lib=c++");
    }

    if is_windows {
        println!("cargo:rustc-link-lib=dylib=user32");
    }

    // TODO(bschwind) - Iterate over the src/ directory to populate this.
    let rust_bridges = [
        "src/b_rep.rs",
        "src/b_rep_adaptor.rs",
        "src/b_rep_algo_api.rs",
        "src/b_rep_bnd_lib.rs",
        "src/b_rep_check.rs",
        "src/b_rep_builder_api.rs",
        "src/b_rep_feat.rs",
        "src/b_rep_fillet_api.rs",
        "src/b_rep_g_prop.rs",
        "src/b_rep_int_curve_surface.rs",
        "src/b_rep_lib.rs",
        "src/b_rep_mesh.rs",
        "src/b_rep_offset_api.rs",
        "src/b_rep_prim_api.rs",
        "src/b_rep_tools.rs",
        "src/bin_tools.rs",
        "src/bnd.rs",
        "src/bop_algo.rs",
        "src/geom.rs",
        "src/geom2d.rs",
        "src/geom_abs.rs",
        "src/geom_api.rs",
        "src/g_prop.rs",
        "src/gc.rs",
        "src/gc_pnts.rs",
        "src/gp.rs",
        "src/hlr_b_rep.rs",
        "src/if_select.rs",
        "src/iges_control.rs",
        "src/law.rs",
        "src/message.rs",
        "src/poly.rs",
        "src/shape_analysis.rs",
        "src/shape_fix.rs",
        "src/shape_upgrade.rs",
        "src/standard.rs",
        "src/step_control.rs",
        "src/stl_api.rs",
        "src/t_col_gp.rs",
        "src/top_abs.rs",
        "src/top_exp.rs",
        "src/top_loc.rs",
        "src/top_tools.rs",
        "src/topo_ds.rs",
    ];

    let mut build = cxx_build::bridges(rust_bridges);

    if is_windows_gnu {
        build.define("OCC_CONVERT_SIGNALS", "TRUE");
    }

    if let "windows" = std::env::consts::OS {
        let current = std::env::current_dir().unwrap();
        build.include(current.parent().unwrap());
    }

    build
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-Wno-deprecated-declarations")
        .define("_USE_MATH_DEFINES", "TRUE")
        .include(occt_config.include_dir)
        .include("include")
        .compile("rust-occt");

    println!("cargo:rerun-if-changed=include");
    for bridge in rust_bridges {
        println!("cargo:rerun-if-changed={bridge}");
    }
}

struct OcctConfig {
    include_dir: std::path::PathBuf,
    library_dir: std::path::PathBuf,
    is_dynamic: bool,
}

impl OcctConfig {
    /// Find OpenCASCADE library using cmake
    fn detect() -> Self {
        println!("cargo:rerun-if-env-changed=DEP_OCCT_ROOT");

        // Add path to builtin OCCT
        #[cfg(feature = "builtin")]
        {
            occt_sys::build_occt();
            std::env::set_var("DEP_OCCT_ROOT", occt_sys::occt_path().as_os_str());
        }

        let dst =
            std::panic::catch_unwind(|| cmake::Config::new("OCCT").register_dep("occt").build());

        #[cfg(feature = "builtin")]
        let dst = dst.expect("Builtin OpenCASCADE library not found.");

        #[cfg(not(feature = "builtin"))]
        let dst = dst.expect("Pre-installed OpenCASCADE library not found. You can use `builtin` feature if you do not want to install OCCT libraries system-wide.");

        let cfg = std::fs::read_to_string(dst.join("share").join("occ_info.txt"))
            .expect("Something went wrong when detecting OpenCASCADE library.");

        let mut version_major: Option<u8> = None;
        let mut version_minor: Option<u8> = None;
        let mut include_dir: Option<std::path::PathBuf> = None;
        let mut library_dir: Option<std::path::PathBuf> = None;
        let mut is_dynamic: bool = false;

        for line in cfg.lines() {
            if let Some((var, val)) = line.split_once('=') {
                match var {
                    "VERSION_MAJOR" => version_major = val.parse().ok(),
                    "VERSION_MINOR" => version_minor = val.parse().ok(),
                    "INCLUDE_DIR" => include_dir = val.parse().ok(),
                    "LIBRARY_DIR" => library_dir = val.parse().ok(),
                    "BUILD_SHARED_LIBS" => is_dynamic = val == "ON",
                    _ => (),
                }
            }
        }

        if let (Some(version_major), Some(version_minor), Some(include_dir), Some(library_dir)) =
            (version_major, version_minor, include_dir, library_dir)
        {
            if version_major != OCCT_VERSION.0 || version_minor < OCCT_VERSION.1 {
                #[cfg(feature = "builtin")]
                panic!("Builtin OpenCASCADE library found but version is not met (found {}.{} but {}.{} required). Please fix OCCT_VERSION in build script of `opencascade-sys` crate or submodule OCCT in `occt-sys` crate.",
                       version_major, version_minor, OCCT_VERSION.0, OCCT_VERSION.1);

                #[cfg(not(feature = "builtin"))]
                panic!("Pre-installed OpenCASCADE library found but version is not met (found {}.{} but {}.{} required). Please provide required version or use `builtin` feature.",
                       version_major, version_minor, OCCT_VERSION.0, OCCT_VERSION.1);
            }

            Self { include_dir, library_dir, is_dynamic }
        } else {
            panic!("OpenCASCADE library found but something wrong with config.");
        }
    }
}

fn collect_object_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_object_files(&path, out);
            } else if path.extension().map_or(false, |ext| ext == "o") {
                out.push(path);
            }
        }
    }
}

