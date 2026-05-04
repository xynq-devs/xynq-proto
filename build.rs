use std::{
    fs,
    io::Result,
    path::{
        Path,
        PathBuf,
    },
};

use prost_build::Config;

const PROTOBUF_DIR: &str = "./protobuf";

fn main() -> Result<()> {
    println!("cargo::rerun-if-changed={}", PROTOBUF_DIR);

    let mut config = Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    let protos = collect_proto_files(PROTOBUF_DIR)?;
    let includes = &[PROTOBUF_DIR];

    config.compile_protos(&protos, includes)?;

    Ok(())
}

fn collect_proto_files(dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    visit(dir.as_ref(), &mut files)?;
    files.sort();
    Ok(files)
}

fn visit(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            visit(&path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "proto") {
            out.push(path);
        }
    }
    Ok(())
}
