fn main() {
    use std::path::Path;
    use std::process::Command;
    use std::str::from_utf8;
    use std::io::Write;
    use std::fs::File;
    let mut out = File::create("build.log").unwrap();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("bobyqa-cpp");
    writeln!(&mut out, "cmake").unwrap();
    let cmake = Command::new("cmake").args(&["."]).current_dir(&dir).output().unwrap();
    write!(&mut out, "{}", from_utf8(&cmake.stdout[..]).unwrap()).unwrap();
    write!(&mut out, "{}", from_utf8(&cmake.stderr[..]).unwrap()).unwrap();
    writeln!(&mut out, "{}", cmake.status).unwrap();
    writeln!(&mut out, "make").unwrap();
    let make = Command::new("make").current_dir(&dir).output().unwrap();
    write!(&mut out, "{}", from_utf8(&make.stdout[..]).unwrap()).unwrap();
    write!(&mut out, "{}", from_utf8(&make.stderr[..]).unwrap()).unwrap();
    writeln!(&mut out, "{}", make.status).unwrap();
    println!("cargo:rustc-link-search=native={}", dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=bobyqa");
}
