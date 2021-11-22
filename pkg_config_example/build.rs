fn main() {
    println!("PKG_CONFIG_PATH: {:?}", std::env::var("PKG_CONFIG_PATH"));
    let library = pkg_config::probe_library("realsense2")
        .expect("pkg-config failed to find realsense2 package");
    println!("library: {:?}", &library);
}
