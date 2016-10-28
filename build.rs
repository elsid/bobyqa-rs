use std::fs::File;

fn build(out: &mut File) -> bool {
    use std::path::Path;
    use std::process::Command;
    use std::str::from_utf8;
    use std::io::Write;
    use std::fs::create_dir_all;
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("bobyqa-cpp").join("build");
    create_dir_all(&dir).unwrap();
    let date = Command::new("date").output().unwrap();
    write!(out, "# {}", from_utf8(&date.stdout[..]).unwrap()).unwrap();
    writeln!(out, "# cmake").unwrap();
    let cmake = Command::new("cmake")
        .arg("-DCMAKE_VERBOSE_MAKEFILE=1").arg("-DCMAKE_BUILD_TYPE=Release").arg("..")
        .current_dir(&dir).output().unwrap();
    write!(out, "{}", from_utf8(&cmake.stdout[..]).unwrap()).unwrap();
    write!(out, "{}", from_utf8(&cmake.stderr[..]).unwrap()).unwrap();
    writeln!(out, "{}", cmake.status).unwrap();
    if !cmake.status.success() {
        return false;
    }
    writeln!(out, "# make").unwrap();
    let make = Command::new("make").current_dir(&dir).output().unwrap();
    write!(out, "{}", from_utf8(&make.stdout[..]).unwrap()).unwrap();
    write!(out, "{}", from_utf8(&make.stderr[..]).unwrap()).unwrap();
    writeln!(out, "{}", make.status).unwrap();
    if !make.status.success() {
        return false;
    }
    println!("cargo:rustc-link-search=native={}", dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=bobyqa");
    true
}

fn main() {
    use std::fs::canonicalize;
    let mut out = File::create("build.log").unwrap();
    let log = canonicalize("build.log").unwrap();
    assert!(build(&mut out), format!("build failed, see log: {:?}", log));
}
