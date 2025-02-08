fn main() {
    // Include library paths from KallistiOS environment
    let kos_ldflags = std::env::var("KOS_LDFLAGS")
        .expect("Missing $KOS_LDFLAGS -- KallistiOS environment not sourced!");
    for lib_path in kos_ldflags
        .split_whitespace()
        .filter(|x| x.starts_with("-L"))
    {
        println!("cargo:rustc-link-search=native={}", lib_path);
    }

    cc::Build::new()
        .compiler("kos-cc")
        .try_flags_from_environment("KOS_CFLAGS")
        .expect("Missing $KOS_CFLAGS -- KallistiOS environment not sourced!")
        .file("src/arch/cache.c")
        .file("src/dc/fmath.c")
        .file("src/dc/g2bus.c")
        .file("src/dc/matrix.c")
        .file("src/dc/pvr/pvr.c")
        .file("src/dc/sq.c")
        .file("src/dc/vec3f.c")
        .compile("kossysc");
}
