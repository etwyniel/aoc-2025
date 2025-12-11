fn main() {
    println!("cargo::rerun-if-changed=src/day01.c");
    println!("cargo::rustc-check-cfg=cfg(use_c)");
    println!("cargo:rustc-link-lib=glpk");

    cc::Build::new()
        .flags([
            "-Wall",
            "-Wextra",
            "-Werror",
            "-std=c23",
            "-g",
            "-pedantic",
            "-O3",
            "-lglpk",
        ])
        .file("src/day01.c")
        .compile("aoc2025");
}
