use cc::Build;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=ooz");
    let mut builder = Build::new();
    builder
        .cpp(true)
        .files(
            std::fs::read_dir("ooz")?
                .filter_map(|x| match x {
                    Ok(x) => {
                        if x.file_name().to_str()?.ends_with(".cpp") {
                            Some(x)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                })
                .map(|x| x.path()),
        )
        .compile("ooz");
    Ok(())
}
