<img align="right" alt="ETERNAL NEXUS logo" width="100" src="../etc/ETERNALNEXUS.png">

# ETERNAL NEXUS
Eternal Nexus is a blockchain-based MMORPG where players explore a vast multiverse powered by Solana, integrate with the Core blockchain, and collect rare NFT items to shape their journey. The game combines traditional fantasy MMORPG gameplay with futuristic blockchain features, enabling players to truly own their assets, participate in a thriving economy, and influence the game world through decentralized governance.

## 🚀 **Features**
- **Multiverse Exploration**: Discover unique realms tied to different blockchain ecosystems.
- **NFT Gear System**: Collect and trade rare NFT items.
- **Play-to-Earn Economy**: Earn tokens through gameplay and trade NFTs.
- **Cross-Chain Economy**: Interoperable assets across Solana and Core blockchains.
- **Guild Wars and Governance**: Join guilds and participate in multiplayer battles.

## 🛠️ **Prerequisites**
* [Rust (nightly-i686-pc-windows-msvc)][rust] (can be installed via [Rustup][rustup])
* [NASM][nasm]
* **Node.js**: Ensure you have Node.js installed for any frontend dependencies.

## Building
ETERNAL NEXUS uses Cargo for building (this is included with Rust by default). To build the full project, you can run `cargo build` in the root directory to build the project. This will also run the build script for each project, which invokes [NASM][nasm] to modify the binaries.

## 📞 **Troubleshooting Tips**
- If you encounter issues during the build process, ensure that your Rust toolchain is up to date and that NASM is properly installed.
- Refer to the main README.md for additional setup instructions and troubleshooting tips.

[nasm]: https://nasm.us
[rust]: https://www.rust-lang.org/
[rustup]: https://rustup.rs/
