use std::{path::PathBuf, env};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //tonic_build::compile_protos("src/proto/voting.proto")?;

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("proto_descriptor.bin"))
        .compile(
            &["src/proto/voting.proto"],
            &["src/proto"],
        )?;

    Ok(())
}
