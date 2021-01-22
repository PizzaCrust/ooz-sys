use cc::Build;
use std::path::Path;
use std::fs::DirEntry;
use diffy::{Patch, apply, ApplyError};

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    if !Path::new("ooz/kraken_patched.cpp").exists() {
        if !Path::new("ooz/kraken.cpp").exists() {
            panic!("Kraken.cpp doesn't exist. Please fetch git submodules.")
        }
        let patch = std::fs::read_to_string("kraken.patch")?;
        let patch = Patch::from_str(&patch)?;
        let text = std::fs::read_to_string("ooz/kraken.cpp")?;
        let text = apply(&text, &patch)?;
        std::fs::write("ooz/kraken_patched.cpp", text)?;
        std::fs::remove_file("ooz/kraken.cpp")?;
    }
    Build::new()
        .cpp(true)
        .files(std::fs::read_dir("ooz")?.filter_map(|x| match x {
            Ok(x) => if x.file_name().to_str()?.ends_with(".cpp") { Some(x) } else { None },
            Err(_) => None
        }).map(|x| x.path()))
        .compile("ooz");
    Ok(())
}