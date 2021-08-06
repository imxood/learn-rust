#[test]
fn test_pkg() {
    // Probe libary
    let library = pkg_config::probe_library("realsense2")
        .expect("pkg-config failed to find realsense2 package");
    println!("library: {:?}", &library);
}
