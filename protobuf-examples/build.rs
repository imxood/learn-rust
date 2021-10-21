use protobuf_codegen_pure::Customize;

// option (rustproto.carllerche_bytes_for_bytes_all) = true;
// option (rustproto.carllerche_bytes_for_string_all) = true;
// option (rustproto.serde_derive_all) = true;
// option (rustproto.generate_accessors_all) = true;

fn main() {
    protobuf_codegen_pure::Codegen::new()
        .customize(Customize {
            gen_mod_rs: Some(true),
            // generate_accessors: Some(true),
            // carllerche_bytes_for_bytes: Some(true),
            // carllerche_bytes_for_string: Some(true),
            serde_derive: Some(true),
            ..Default::default()
        })
        .input("protos/example.proto")
        .out_dir("src/protos")
        .include(".")
        .run()
        .unwrap();
}
