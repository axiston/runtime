#![forbid(unsafe_code)]

use std::fs::create_dir_all;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let generate_client = cfg!(feature = "client");
    let generate_server = cfg!(feature = "server");

    let builder = tonic_build::configure()
        .build_server(generate_server)
        .build_client(generate_client)
        .build_transport(true);

    let input_dir = PathBuf::from("./protobuf/");
    let instance = input_dir.join("./instance.proto");
    let registry = input_dir.join("./registry.proto");

    let output_dir = PathBuf::from("./generated/");
    create_dir_all(output_dir.as_path())?;

    let protos = [instance.as_path(), registry.as_path()];
    let includes = [input_dir.as_path()];
    builder.out_dir(output_dir).compile(&protos, &includes)?;

    Ok(())
}
