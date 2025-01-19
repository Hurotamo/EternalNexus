<img align="right" alt="ETERNAL NEXUS logo" width="100" src="../etc/ETERNALNEXUS.png">

# ETERNAL NEXUS
Eternal Nexus is a blockchain-based MMORPG where players explore a vast multiverse powered by Solana, integrate with the Core blockchain, and collect rare NFT items to shape their journey. The game combines the traditional fantasy MMORPG gameplay with futuristic blockchain features, enabling players to truly own their assets, participate in a thriving economy, and influence the game world through decentralized governance.

## Prerequisites
* [Rust (nightly-i686-pc-windows-msvc)][rust] (can be installed via [Rustup][rustup])
* [NASM][nasm]

## Building
ETERNAL NEXUS uses Cargo for building (this is included with Rust by default). To build the full
project, you can run `cargo build` in the root directory to build the project. This will also
run the buildscript for each project, which also invokes [NASM][nasm] to modify the binaries.

[nasm]: https://nasm.us
[rust]: https://www.rust-lang.org/
[rustup]: https://rustup.rs/