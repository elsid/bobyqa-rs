fn main() {
    use std::path::Path;
    use std::process::Command;
    use std::str::from_utf8;
    use std::io::Write;
    use std::fs::{File, create_dir_all};
    let mut out = File::create("build.log").unwrap();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("bobyqa-cpp").join("build");
    create_dir_all(&dir).unwrap();
    let date = Command::new("date").output().unwrap();
    write!(&mut out, "# {}", from_utf8(&date.stdout[..]).unwrap()).unwrap();
    writeln!(&mut out, "# cmake").unwrap();
    let cmake = Command::new("cmake")
        .arg("-DCMAKE_VERBOSE_MAKEFILE=1").arg("-DCMAKE_BUILD_TYPE=Release").arg("..")
        .current_dir(&dir).output().unwrap();
    write!(&mut out, "{}", from_utf8(&cmake.stdout[..]).unwrap()).unwrap();
    write!(&mut out, "{}", from_utf8(&cmake.stderr[..]).unwrap()).unwrap();
    writeln!(&mut out, "{}", cmake.status).unwrap();
    assert!(cmake.status.success());
    writeln!(&mut out, "# make").unwrap();
    let make = Command::new("make").current_dir(&dir).output().unwrap();
    write!(&mut out, "{}", from_utf8(&make.stdout[..]).unwrap()).unwrap();
    write!(&mut out, "{}", from_utf8(&make.stderr[..]).unwrap()).unwrap();
    writeln!(&mut out, "{}", make.status).unwrap();
    assert!(make.status.success());
    println!("cargo:rustc-link-search=native={}", dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=bobyqa");
}
