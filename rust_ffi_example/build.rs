fn main() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    #[cfg(target_os = "linux")]
    {
        // 添加 add library 路径
        println!("cargo:rustc-link-search=native={}/c_library_dyn/out", &path);
        println!(
            "cargo:rustc-link-search=native={}/c_library_static/out",
            &path
        );
        println!(
            "cargo:rustc-link-search=native={}/rust_library_dyn/target/debug",
            &path
        );
        println!(
            "cargo:rustc-link-search=native={}/rust_library_static/target/debug",
            &path
        );
        println!(
            "cargo:rustc-link-search=native={}/rust_library_static1/target/debug",
            &path
        );

        // 添加 静态库或动态库 的名称
        println!("cargo:rustc-link-lib=dylib=add_dyn");
        println!("cargo:rustc-link-lib=static=add_static");
        println!("cargo:rustc-link-lib=dylib=minus_dyn");
        println!("cargo:rustc-link-lib=static=minus_static");
        println!("cargo:rustc-link-lib=static=minus_static1");
    }

    #[cfg(target_os = "windows")]
    {
        // 添加 library 路径
        println!(
            "cargo:rustc-link-search=native={}/c_library_dyn/out/Debug",
            &path
        );
        println!(
            "cargo:rustc-link-search=native={}/c_library_static/out/Debug",
            &path
        );
        println!(
            "cargo:rustc-link-search=native={}/rust_library_dyn/target/debug",
            &path
        );
        println!(
            "cargo:rustc-link-search=native={}/rust_library_static/target/debug",
            &path
        );

        // 添加 库名字
        println!("cargo:rustc-link-lib=dylib=add_dyn");
        println!("cargo:rustc-link-lib=static=add_static");
        println!("cargo:rustc-link-lib=dylib=minus_dyn.dll");
        println!("cargo:rustc-link-lib=static=minus_static");
    }
}
