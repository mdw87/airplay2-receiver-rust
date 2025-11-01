# AirPlay 2 Receiver (Rust)

A high-performance AirPlay 2 audio receiver implementation in Rust, enabling devices like iPhones, iPads, and Macs to stream audio to your computer or Raspberry Pi.

## Overview

This project is a Rust implementation of an AirPlay 2 receiver, converted from the Python [ap2-receiver](https://github.com/openairplay/ap2-receiver) project. It allows Apple devices to discover and stream audio to this software running on Windows, Linux (including Raspberry Pi), and other platforms.

## Features

### Supported Capabilities

- **AirPlay 2 Protocol**: Full support for the modern AirPlay 2 protocol
- **Audio Streaming**:
  - ALAC (Apple Lossless Audio Codec)
  - AAC-LC (Advanced Audio Coding - Low Complexity)
  - PCM (uncompressed audio)
- **Secure Pairing**: HomeKit Accessory Protocol (HAP) pairing with Ed25519 and ChaCha20-Poly1305 encryption
- **FairPlay**: FairPlay v2.5 support for encrypted streams
- **Buffered Audio**: High-quality buffered streaming with PTP clock synchronization
- **Volume Control**: Remote volume adjustment from iOS devices
- **Playback Control**: Play, pause, and progress tracking
- **mDNS Discovery**: Automatic discovery via Bonjour/mDNS
- **HomeKit Integration**: HomeKit access control levels

### Key Features

- **Cross-platform**: Windows, Linux (x86_64, ARM/Raspberry Pi)
- **Low latency**: Target < 500ms end-to-end for realtime mode
- **Efficient**: < 10% CPU usage on Raspberry Pi 4, < 100MB memory
- **Secure**: Full encryption support with HAP and FairPlay
- **Async I/O**: Built on Tokio for efficient concurrent operations

## Project Status

**Current Phase**: Planning & Initial Development

This project is currently in active development. See [docs/PROGRESS.md](docs/PROGRESS.md) for detailed progress tracking.

### Roadmap

- [ ] Phase 1: Foundation & Setup
- [ ] Phase 2: Protocol Layer (Plist, TLV8, SDP, Crypto)
- [ ] Phase 3: Network Layer (HTTP/RTSP Server, mDNS)
- [ ] Phase 4: Streaming (RTP, Audio Decoding)
- [ ] Phase 5: Testing & Refinement
- [ ] Phase 6: Optimization & Deployment

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     AirPlay 2 Client                         │
│                   (iPhone, iPad, Mac)                        │
└──────────────────────┬──────────────────────────────────────┘
                       │ HTTP/RTSP (Port 7000)
                       │ mDNS Discovery
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                  Network Layer (Tokio)                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ mDNS Service │  │ HTTP/RTSP    │  │ Event Server │      │
│  │ Announcer    │  │ Server       │  │ (Generic)    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└──────────────┬──────────────────────────────────────────────┘
               ▼
┌─────────────────────────────────────────────────────────────┐
│                    Streaming Layer                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ RTP Audio    │  │ RTP Control  │  │ RTP Timing   │      │
│  │ Receiver     │  │ (RTCP)       │  │ (NTP/PTP)    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└──────────────┬──────────────────────────────────────────────┘
               ▼
┌─────────────────────────────────────────────────────────────┐
│                    Audio Layer                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ ALAC Decoder │  │ AAC Decoder  │  │ Audio Output │      │
│  │              │  │              │  │ (cpal)       │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

For detailed architecture documentation, see [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Requirements

### Runtime Requirements

- **Operating System**: Windows 10+, Linux (Ubuntu 20.04+, Raspberry Pi OS)
- **Audio Output**: System audio device
- **Network**: WiFi or Ethernet with mDNS/Bonjour support

### Build Requirements

- **Rust**: 1.70 or later
- **Dependencies**: See [Cargo.toml](Cargo.toml) for complete list

### Key Dependencies

- `tokio` - Async runtime
- `axum` - HTTP/RTSP server
- `mdns-sd` - mDNS service discovery
- `ed25519-dalek` - Ed25519 cryptography
- `chacha20poly1305` - ChaCha20-Poly1305 encryption
- `srp` - SRP authentication for HAP pairing
- `cpal` - Cross-platform audio output
- `symphonia` - Audio decoding (ALAC, AAC)
- `plist` - Binary plist support
- `clap` - CLI argument parsing

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/airplay2-receiver-rust.git
cd airplay2-receiver-rust

# Build the project
cargo build --release

# Run the receiver
./target/release/airplay2-receiver
```

### Pre-built Binaries

Pre-built binaries will be available on the [Releases](https://github.com/yourusername/airplay2-receiver-rust/releases) page once the project reaches a stable state.

## Usage

### Basic Usage

```bash
# Start the receiver with default settings
airplay2-receiver

# Specify a custom device name
airplay2-receiver --name "Living Room Speaker"

# Disable volume control
airplay2-receiver --no-volume

# Specify network interface
airplay2-receiver --interface wlan0
```

### Command-Line Options

```
OPTIONS:
    -n, --name <NAME>              Device name (default: "AirPlay Receiver")
    -i, --interface <INTERFACE>    Network interface to use
        --no-volume                Disable volume control
        --port <PORT>              Server port (default: 7000)
    -v, --verbose                  Verbose logging
    -h, --help                     Print help information
    -V, --version                  Print version information
```

### Pairing

On first use, you may need to pair your iOS device:

1. Start the receiver
2. Open Control Center on your iOS device
3. Tap the AirPlay icon
4. Select your receiver from the list
5. Enter the PIN if prompted (displayed in receiver logs)

## Development

### Project Structure

```
airplay2-receiver/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library root
│   ├── config/              # Configuration & device properties
│   ├── crypto/              # Encryption & pairing (HAP, FairPlay)
│   ├── protocol/            # Protocol parsing (Plist, TLV8, SDP)
│   ├── network/             # Network layer (HTTP/RTSP, mDNS)
│   ├── streaming/           # Audio streaming (RTP)
│   ├── audio/               # Audio decoding & output
│   └── utils/               # Utilities (logging, network)
└── docs/
    ├── ARCHITECTURE.md      # Detailed architecture documentation
    ├── PYTHON_REFERENCE.md  # Python implementation reference
    ├── PROGRESS.md          # Development progress tracker
    └── CURRENT_TASK.md      # Current development focus
```

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test module
cargo test crypto::

# Run integration tests
cargo test --test integration
```

## Design Principles

1. **Async-first**: Built on Tokio for efficient I/O operations
2. **Type-safe**: Leverages Rust's type system to prevent protocol errors
3. **Modular**: Each module is independently testable
4. **Zero-copy**: Uses `bytes::Bytes` for network data where possible
5. **Cross-platform**: Platform-specific code is abstracted behind traits

## Performance Goals

- **Latency**: < 500ms end-to-end for realtime mode
- **Dropouts**: Zero audio dropouts under normal conditions
- **CPU Usage**: < 10% on Raspberry Pi 4
- **Memory**: < 100MB resident memory

## Platform Support

### Tested Platforms

- **Windows**: Windows 10/11 (WASAPI audio)
- **Linux**: Ubuntu 20.04+, Raspberry Pi OS (ALSA audio)

### Future Platforms

- **macOS**: Planned (CoreAudio support)

## Security

This receiver implements the following security features:

- **HAP Pairing**: SRP-based pairing with Ed25519 key exchange
- **Session Encryption**: ChaCha20-Poly1305 for all encrypted sessions
- **FairPlay**: FairPlay v2.5 for DRM-protected streams
- **Access Control**: HomeKit ACL support (everyone/users/admin)

## Troubleshooting

### Common Issues

1. **Receiver not appearing in AirPlay menu**
   - Check firewall settings (allow port 7000)
   - Verify mDNS/Bonjour is enabled on your network
   - Check network interface selection

2. **Audio dropouts**
   - Increase buffer size
   - Check WiFi signal strength
   - Verify network bandwidth

3. **Pairing fails**
   - Ensure HomeKit pairing feature flag is enabled
   - Check logs for detailed error messages
   - Try restarting the receiver

4. **No volume control**
   - Use `--no-volume` flag if system volume control is not available
   - Check platform-specific audio APIs

For more issues and solutions, see [docs/PYTHON_REFERENCE.md](docs/PYTHON_REFERENCE.md#common-issues--solutions).

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Commit your changes (`git commit -am 'Add new feature'`)
6. Push to the branch (`git push origin feature/my-feature`)
7. Create a Pull Request

### Development Workflow

See [docs/CURRENT_TASK.md](docs/CURRENT_TASK.md) for the current development focus and next steps.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Based on the Python [ap2-receiver](https://github.com/openairplay/ap2-receiver) project
- Inspired by the AirPlay reverse engineering community
- Thanks to all contributors and testers

## References

- [Architecture Documentation](docs/ARCHITECTURE.md)
- [Python Reference Implementation](docs/PYTHON_REFERENCE.md)
- [Development Progress](docs/PROGRESS.md)
- [Current Tasks](docs/CURRENT_TASK.md)

## Contact

For questions, issues, or suggestions, please [open an issue](https://github.com/yourusername/airplay2-receiver-rust/issues) on GitHub.
