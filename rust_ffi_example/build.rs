fn main() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    #[cfg(target_os = "linux")]
    {
        // 添加 add library 路径
        println!("cargo:rustc-link-search=native={}/c_library/out", &path);
        println!(
            "cargo:rustc-link-search=native={}/rust_library/target/debug",
            &path
        );

        // 添加 静态库或动态库 的名称
        println!("cargo:rustc-link-lib=dylib=minus");
        println!("cargo:rustc-link-lib=dylib=add");
    }

    #[cfg(target_os = "windows")]
    {
        // 添加 library 路径
        println!(
            "cargo:rustc-link-search=native={}/c_library/out/Debug",
            &path
        );
        println!(
            "cargo:rustc-link-search=native={}/rust_library/target/debug",
            &path
        );

        // 添加 库名字
        println!("cargo:rustc-link-lib=dylib=add");
        println!("cargo:rustc-link-lib=dylib=minus.dll");
    }
}
