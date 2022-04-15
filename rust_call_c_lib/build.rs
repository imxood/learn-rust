fn main() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
 
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-search=native={}/c_library/build/Debug", path);
        println!("cargo:rustc-link-search=native={}/rust_library/target/debug", path);
        println!("cargo:rustc-link-lib=dylib=add");
        println!("cargo:rustc-link-lib=dylib=minus");
    }
    
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-search=native={}/c_library/build", path);
        println!("cargo:rustc-link-search=native={}/rust_library/target/debug", path);
        println!("cargo:rustc-link-lib=dylib=add");
        println!("cargo:rustc-link-lib=dylib=minus");
    }
}