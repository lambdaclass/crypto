fn main() {
    println!("cargo:rerun-if-changed=c_code/test_sve.c");
    println!("cargo:rerun-if-changed=c_code/test_sve.h");
    cc::Build::new()
        .file("c_code/test_sve.c")
        .flag("-march=armv8-a+sve")
        .compile("sve");
}
