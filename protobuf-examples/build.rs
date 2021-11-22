use protobuf_codegen::{Codegen, Customize};

// option (rustproto.carllerche_bytes_for_bytes_all) = true;
// option (rustproto.carllerche_bytes_for_string_all) = true;
// option (rustproto.serde_derive_all) = true;
// option (rustproto.generate_accessors_all) = true;

fn main() {
    Codegen::new()
        .pure()
        .customize(Customize {
            gen_mod_rs: Some(true),
            generate_accessors: Some(true),
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
