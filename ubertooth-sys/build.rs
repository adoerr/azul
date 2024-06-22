use std::{env, path::PathBuf};

use anyhow::Result;

fn main() -> Result<()> {
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=ubertooth");
    println!("cargo:rerun-if-changed=ubertooth.h");

    bindgen::Builder::default()
        .header("/usr/local/include/ubertooth.h")
        .generate()?
        .write_to_file(PathBuf::from(env::var("OUT_DIR")?).join("ubertooth.rs"))?;

    Ok(())
}