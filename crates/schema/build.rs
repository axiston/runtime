#![forbid(unsafe_code)]

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

    let protos = [instance.as_path(), registry.as_path()];
    let includes = [input_dir.as_path()];
    builder.compile_protos(&protos, &includes)?;

    Ok(())
}
