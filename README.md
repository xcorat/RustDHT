# RustDHT

A P2P networking implementation using Rust and libp2p, with WebRTC support for browser-to-native communication.

## Features

- Native Rust P2P server with TCP and WebRTC support
- Browser-compatible WASM client
- WebRTC-based peer connectivity
- Basic ping functionality for testing connections

## Components

- `src/bin/server.rs`: Native P2P server implementation
- `src/lib.rs`: WASM client implementation
- `www/`: Web interface for browser testing

## Requirements

- Rust 2021 edition
- Node.js (for web interface)
- wasm-pack (for WASM compilation)

## Building

### Development Build

1. Build WASM module:
```bash
wasm-pack build
```

2. Build native server:
```bash
cargo build
```

### Release Build

1. Build optimized release version:
```bash
cargo build --release
```

2. Cross-platform builds:
```bash
# Install target toolchains
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu

# Build for macOS
cargo build --release --target x86_64-apple-darwin
```

3. Docker build:
```bash
docker build -t rustdht-server .
```

See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed deployment instructions.

## Running

1. Start server:
```bash
cargo run --bin server
```

2. Start web client:
```bash
# From www directory
npm install
npm start
```

3. Connect using browser at `http://localhost:8080`

## Transport Protocols

- TCP (native-to-native)
- WebRTC (browser-to-native, browser-to-browser)

## Dependencies

- libp2p: Core P2P networking
- tokio: Async runtime
- wasm-bindgen: WASM bindings
- web-sys: Web APIs for WebRTC
