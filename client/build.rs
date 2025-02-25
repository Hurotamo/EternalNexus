use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::env;

/// The entry point for the ETERNAL NEXUS build script, which runs some additional code
/// before building the DLL. This includes modifying the assembly of the `game` binary.
fn main() {
    // Set the GIT_HASH environment variable
    let commit_hash_output = Command::new("git").args(&["rev-parse", "--short", "HEAD"]).output().unwrap();
    let git_hash = String::from_utf8(commit_hash_output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    // Run the assembler
    if let Err(e) = assemble(git_hash) {
        eprintln!("Error assembling: {}", e);
        std::process::exit(1);
    }
}

/// Run the assembler and codegen
///
/// # Arguments
/// * `hash` - The current Git hash.
fn assemble(hash: String) -> std::io::Result<()> {
    // Generate the assembly file with the Git information.
    let text = format!("\
        %ifndef metadata_asm\n\
        %define metadata_asm\n\
        git_hash   db   \"{}\", 0\n\
        %endif\
    ", hash.trim());
    let mut file = File::create("asm/metadata.asm")?;
    file.write_all(text.as_bytes())?;

    // Run the assembler
    let status = Command::new("nasm")
        .args(&["-o", "../game.exe"])
        .arg("asm/client.asm")
        .status()?;
    if !status.success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "nasm failed"));
    }

    Ok(())
}