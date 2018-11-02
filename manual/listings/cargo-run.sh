# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Build and run in debug or release mode
cargo run
cargo run --release

# View CLI options
cargo run -- --help
cargo run --release -- --help

# Automatically open the configuration panel
cargo run -- --open
cargo run --release -- --open
