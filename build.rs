extern crate cc;

use std::path::Path;

fn main() {
    cc::Build::new()
        .define("PREFIX", "L\"/usr/local\"")
        .define("_GNU_SOURCE", "1")
        .define("_REENTRANT", None)
        .include(Path::new("/usr/include/SDL"))
        .include(Path::new("/usr/local/include/SDL"))  // osx brew
        .include(Path::new("C:\\Users\\user1\\Documents\\SDL-1.2.15\\include"))
        .file("utils.c")
        .compile("foo");

    println!("cargo:rustc-link-lib=SDL_ttf");
    println!("cargo:rustc-link-lib=freetype");
    println!("cargo:rustc-link-lib=SDL");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=SDL_mixer");
}
