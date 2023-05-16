extern crate cc;

fn main() {
    println!("cargo:include=libstretch/");
    cc::Build::new()
        .cpp(true)
        .file("libstretch/stretch.cpp")
        .flag_if_supported("-std=c++11")
        .cpp_link_stdlib("stdc++")
        .compile("stretch");
}
