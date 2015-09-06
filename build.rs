fn main() {
    use std::process::Command;
    Command::new("cmake").args(&["."]).current_dir("bobyqa-cpp").status().unwrap();
    Command::new("make").current_dir("bobyqa-cpp").status().unwrap();
    println!("cargo:rustc-link-search=native=bobyqa-cpp/lib");
    println!("cargo:rustc-link-lib=static=bobyqa");
}
