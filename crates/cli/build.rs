#![forbid(unsafe_code)]

use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let builder = tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .build_transport(true);

    let dir = PathBuf::from("./protobuf/");
    let instance = dir.join("./instance.proto");
    let registry = dir.join("./registry.proto");

    let protos = [instance.as_path(), registry.as_path()];
    let includes = [dir.as_path()];
    builder.compile(&protos, &includes)?;

    Ok(())
}
