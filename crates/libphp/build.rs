use std::{env, path::{Path, PathBuf}, process::Command};
use bindgen::Builder;

const PHP_VERSION: &str = "8.2";

fn main() {
    println!("cargo:rerun-if-changed=bindings/libphp.h");
    println!("cargo:rerun-if-changed=bindings/libphp.c");

    let target = out_path("");
    let spc_path = out_path("spc");

    if ! spc_path.exists() {
        run_command_or_fail(
            &target,
            "git",
            &[
                "clone",
                "https://github.com/crazywhalecc/static-php-cli.git",
                "spc",
                "--depth=1"
            ]
        );

        run_command_or_fail(
            &spc_path,
            "composer",
            &["update", "--no-dev", "--no-plugins", "-n"]
        );

        run_command_or_fail(
            &spc_path,
            "php",
            &[
                "bin/spc",
                "download",
                "php-src,pkg-config,micro",
                format!("--with-php={}", PHP_VERSION).as_str(),
            ]
        );
    }

    run_command_or_fail(
        &spc_path,
        "php",
        &[
            "bin/spc",
            "build",
            "opcache",
            "--build-embed",
            "--enable-zts",
        ]
    );

    let include_dir = spc_path.join("buildroot/include/php");
    let lib_dir = spc_path.join("buildroot/lib");

    println!("cargo:rustc-link-lib=static=php");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    let includes = ["", "Zend", "main", "TSRM"].map(|path| format!("-I{}/{}", &include_dir.display(), &path));
    let bindings = Builder::default()
        .clang_args(&includes)
        .derive_default(true)
        .header("bindings/libphp.h")
        .generate()
        .expect("Failed to generate PHP bindings.");

    let bindings_path = out_path("bindings.rs");
    
    bindings.write_to_file(&bindings_path).expect("Failed to write generated bindings.");

    cc::Build::new()
        .file("bindings/libphp.c")
        .includes(
            &includes
                .iter()
                .map(|include| include.as_str()[2..].to_string())
                .collect::<Vec<String>>()
        )
        .flag("-fPIC")
        .flag("-m64")
        .static_flag(true)
        .compile("libphp");
}

fn out_path(path: &str) -> PathBuf {
    let out = env::var("OUT_DIR").unwrap();
    PathBuf::from(out).join(path)
}

fn run_command_or_fail(path: &Path, cmd: &str, args: &[&str]) {
    eprintln!("Running command: {} {}", cmd, args.join(" "));

    Command::new(cmd).current_dir(path).args(args).status().expect("Failed to run command.");
}