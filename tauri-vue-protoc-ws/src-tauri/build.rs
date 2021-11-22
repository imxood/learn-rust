fn build_proto() {
  use protobuf_codegen_pure::Customize;
  protobuf_codegen_pure::Codegen::new()
    .customize(Customize {
      gen_mod_rs: Some(true),
      serde_derive: Some(true),
      ..Default::default()
    })
    .input("../protos/example.proto")
    .out_dir("src/protos")
    .include(".")
    .run()
    .unwrap();
}

fn main() {
  build_proto();
  tauri_build::build()
}
