use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/reservation.proto"], &["protos"])?;

    Command::new("cargo").args(["fmt"]).output()?;
    println!("cargo:rerun-if-changed=protos/reservation.proto");
    Ok(())
}
