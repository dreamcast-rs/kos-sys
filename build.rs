fn main() {
    // Include libpthread from KallistiOS
    println!("cargo:rustc-link-lib=pthread");

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
        .file("src/c_stubs/pvrh.c")
        .compile("kossysc");
}
