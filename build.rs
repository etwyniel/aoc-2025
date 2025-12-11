fn main() {
    println!("cargo::rerun-if-changed=src/day01.c");
    println!("cargo::rustc-check-cfg=cfg(use_c)");

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
