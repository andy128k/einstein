extern crate cc;

use std::path::Path;

fn main() {
    cc::Build::new()
        .cpp(true)
        .define("PREFIX", "L\"/usr/local\"")
        .define("_GNU_SOURCE", "1")
        .define("_REENTRANT", None)
        .flag("-Wno-narrowing")
        .flag("-Wno-write-strings")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-type-limits")
        .flag("-Wno-unused-but-set-variable")
        .include(Path::new("/usr/include/SDL"))
        .include(Path::new("/usr/local/include/SDL"))  // osx brew
        .file("main.cpp")
        .file("screen.cpp")
        .file("resources.cpp")
        .file("utils.cpp")
        .file("widgets.cpp")
        .file("font.cpp")
        .file("messages.cpp")
        .file("formatter.cpp")
        .file("buffer.cpp")
        .file("unicode.cpp")
        .file("convert.cpp")
        .file("i18n.cpp")
        .file("sound.cpp")
        .compile("foo");

    println!("cargo:rustc-link-lib=SDL_ttf");
    println!("cargo:rustc-link-lib=freetype");
    println!("cargo:rustc-link-lib=SDL");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=SDL_mixer");
}
