fn main() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
 
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-search=native={}/c_library/build/Debug", path);
    }
    
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-search=native={}/c_library/build", path);
    }
}