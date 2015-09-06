fn main() {
    use std::path::Path;
    use std::process::Command;
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("bobyqa-cpp");
    Command::new("cmake").args(&["."]).current_dir(&dir).status().unwrap();
    Command::new("make").current_dir(&dir).status().unwrap();
    println!("cargo:rustc-link-search=native={}", dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=bobyqa");
}
