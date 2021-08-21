use std::env;

#[test]
fn test_ffmpeg() {
    // for dynamic library
    {
        env::set_var("VCPKGRS_DYNAMIC", "1");

        let mut cfg = vcpkg::Config::new();
        cfg.target_triplet("x64-windows").copy_dlls(false);

        // will do config: 1. cargo:include={} 2. cargo:rustc-link-search=native={} 3. cargo:rustc-link-lib={}
        let lib = cfg.find_package("opencv4").unwrap();

        // only show added cargo config info
        println!("lib: {:?}", lib.cargo_metadata);
    }
    // for static library
    // ...
}
