use std::{result::Result, io::Error};

fn main() -> Result<(), Error> {
    prost_build::compile_protos(
        &[
            "src/protos/command.proto", 
            "src/protos/packet.proto", 
            "src/protos/replacement.proto", 
            "src/protos/vssref_command.proto", 
            "src/protos/vssref_common.proto", 
            "src/protos/vssref_placement.proto",
        ], 
        &["src/protos"]
    )?;

    Ok(())
}