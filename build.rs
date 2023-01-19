use std::path::PathBuf;

use cmake;

fn main() -> miette::Result<()> {
    // let cmake build the MaterialX library
    let dst = cmake::Config::new("MaterialX").build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:warning={}", dst.display());
    println!("cargo:rustc-link-lib=static=MaterialXCore");
    println!("cargo:rustc-link-lib=static=MaterialXFormat");
    println!("cargo:rustc-link-lib=static=MaterialXGenGlsl");
    println!("cargo:rustc-link-lib=static=MaterialXGenShader");

    let path = std::path::PathBuf::from("src");
    let material_x = std::path::PathBuf::from(format!("{}/include", dst.display()));
    let mut b = autocxx_build::Builder::new("src/main.rs", [&path, &material_x]).build()?;
    b.flag_if_supported("-std=c++14").compile("autocxx-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    Ok(())
}
