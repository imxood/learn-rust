fn main() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    
    // 添加库路径
    println!("cargo:rustc-link-search=native={}/c_library/out", path);
    println!("cargo:rustc-link-search=native={}/rust_library/target/debug", path);

    // 添加 静态库或动态库 的名称
    println!("cargo:rustc-link-lib=dylib=add");
    println!("cargo:rustc-link-lib=dylib=minus");
}