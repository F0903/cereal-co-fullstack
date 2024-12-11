use std::{env, fs, io, path::Path};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    // Detect the target directory
    let profile = env::var("PROFILE").unwrap(); // `debug` or `release`
    let target_dir = Path::new("target").join(profile);

    let src_dir = Path::new("static");
    let dest_dir = target_dir.join("static");

    copy_dir_all(src_dir, dest_dir).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
}
