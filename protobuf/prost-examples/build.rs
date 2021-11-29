use std::io::Result;

fn main() -> Result<()> {
    // std::env::set_var("RUST_LOG", "DEBUG");
    // env_logger::init();

    // let path = prost_build::protoc();
    // let path = format!("protoc path: {:?}", &path);
    // Command::new("echo")
    //     .arg(path)
    //     .spawn()
    //     .expect("failed to spawn process");

    // let path = prost_build::protoc_include();
    // let path = format!("protoc_include path: {:?}", &path);
    // Command::new("echo")
    //     .arg(path)
    //     .spawn()
    //     .expect("failed to spawn process");

    // prost_build::compile_protos(&["src/example.proto"], &["src/"])?;

    prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
    Ok(())
}
