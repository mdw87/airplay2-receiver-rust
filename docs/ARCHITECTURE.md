# AirPlay 2 Receiver - Rust Architecture

## Overview
This is a Rust implementation of an AirPlay 2 receiver, converted from the Python ap2-receiver project. The receiver allows devices (iPhone, iPad, Mac) to stream audio to this software running on Windows, Linux (Raspberry Pi), or other platforms.

## High-Level Architecture

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
               │
               ▼
┌─────────────────────────────────────────────────────────────┐
│                   Protocol Layer                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ HAP Pairing  │  │ FairPlay     │  │ RTSP Handler │      │
│  │ (Crypto)     │  │ Encryption   │  │              │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │ Plist Parser │  │ SDP Handler  │                        │
│  └──────────────┘  └──────────────┘                        │
└──────────────┬──────────────────────────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────────────────────────┐
│                    Streaming Layer                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ RTP Audio    │  │ RTP Control  │  │ RTP Timing   │      │
│  │ Receiver     │  │ (RTCP)       │  │ (NTP/PTP)    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└──────────────┬──────────────────────────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────────────────────────┐
│                    Audio Layer                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ ALAC Decoder │  │ AAC Decoder  │  │ Buffer Mgmt  │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │ Audio Output │  │ Volume Ctrl  │                        │
│  │ (cpal)       │  │              │                        │
│  └──────────────┘  └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
```

## Module Structure

```
airplay2-receiver/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, CLI arg parsing
│   ├── lib.rs               # Library root
│   │
│   ├── config/              # Configuration & device properties
│   │   ├── mod.rs
│   │   ├── device.rs        # Device properties, name, ACL
│   │   └── flags.rs         # Feature flags, status flags
│   │
│   ├── crypto/              # Encryption & pairing
│   │   ├── mod.rs
│   │   ├── hap.rs           # HAP pairing (SRP, Ed25519, ChaCha20)
│   │   ├── fairplay.rs      # FairPlay encryption
│   │   ├── ltpk.rs          # Long-term public key management
│   │   └── aes.rs           # AES encryption utilities
│   │
│   ├── protocol/            # Protocol parsing & handling
│   │   ├── mod.rs
│   │   ├── plist.rs         # Binary plist parsing/writing
│   │   ├── tlv8.rs          # TLV8 encoding/decoding
│   │   ├── sdp.rs           # SDP handler
│   │   ├── dmap.rs          # DMAP/DXXP parsing
│   │   └── rtsp.rs          # RTSP message handling
│   │
│   ├── network/             # Network layer
│   │   ├── mod.rs
│   │   ├── server.rs        # Main HTTP/RTSP server
│   │   ├── handler.rs       # Request handlers (AP2Handler equivalent)
│   │   ├── mdns.rs          # mDNS service announcement
│   │   └── encrypted.rs     # Encrypted socket wrapper (HAPSocket)
│   │
│   ├── streaming/           # Audio streaming
│   │   ├── mod.rs
│   │   ├── stream.rs        # Stream management
│   │   ├── rtp.rs           # RTP packet handling
│   │   ├── session.rs       # Session management
│   │   └── buffer.rs        # Audio buffer management
│   │
│   ├── audio/               # Audio decoding & output
│   │   ├── mod.rs
│   │   ├── output.rs        # Audio output abstraction (cpal)
│   │   ├── decoder.rs       # Audio decoder (ALAC, AAC)
│   │   └── volume.rs        # Volume control
│   │
│   └── utils/               # Utilities
│       ├── mod.rs
│       ├── logger.rs        # Logging setup
│       └── network.rs       # Network interface utilities
│
└── tests/
    ├── integration/
    └── unit/
```

## Key Dependencies (Cargo.toml)

```toml
[dependencies]
# Async runtime
tokio = { version = "1", features = ["full"] }
tokio-util = "0.7"

# HTTP/RTSP server
axum = "0.7"  # or hyper for lower-level control
tower = "0.4"

# Serialization
serde = { version = "1.0", features = ["derive"] }
plist = "1.6"  # Binary plist support

# Cryptography
ed25519-dalek = "2.0"
chacha20poly1305 = "0.10"
aes = "0.8"
cbc = "0.1"
srp = "0.6"  # For HAP pairing
sha2 = "0.10"
rand = "0.8"

# Network
mdns-sd = "0.10"  # mDNS service discovery
socket2 = "0.5"
get_if_addrs = "0.5.3"  # Network interface enumeration

# Audio
cpal = "0.15"  # Cross-platform audio I/O
symphonia = "0.5"  # Audio decoding (ALAC, AAC)

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# CLI
clap = { version = "4.4", features = ["derive"] }

# Utilities
anyhow = "1.0"
thiserror = "1.0"
bytes = "1.5"
bitflags = "2.4"
```

## Design Principles

1. **Async-first**: Use Tokio for all I/O operations
2. **Type-safe**: Leverage Rust's type system to prevent errors
3. **Modular**: Each module should be independently testable
4. **Zero-copy where possible**: Use `bytes::Bytes` for network data
5. **Error handling**: Use `Result<T, E>` with custom error types
6. **Cross-platform**: Abstract platform-specific code (audio output, network interfaces)

## Communication Patterns

### Request Flow
1. Client sends RTSP request → HTTP server
2. Handler parses request → Protocol layer
3. Protocol layer processes → May involve crypto
4. Response generated → Sent back to client

### Streaming Flow
1. Setup phase establishes RTP ports
2. Encrypted audio packets arrive on RTP port
3. Decryption → Decoding → Buffering
4. Audio output thread pulls from buffer → Speakers

### Event Flow
1. Events (play/pause/volume) sent by client
2. Event server receives on dedicated port
3. Commands forwarded to appropriate stream
4. Stream updates state, modifies audio output

## Platform-Specific Considerations

### Windows
- Audio: WASAPI via cpal
- Volume: Windows Audio APIs
- Network: Standard socket APIs

### Linux (Raspberry Pi)
- Audio: ALSA via cpal
- Volume: ALSA mixer APIs
- Network: Standard socket APIs
- Performance: Consider zero-copy optimizations

### Future (macOS, others)
- Audio: CoreAudio via cpal
- Similar abstractions as above

## Security Considerations

1. **Pairing**: Implement full HAP pairing with SRP and Ed25519
2. **Encryption**: ChaCha20-Poly1305 for encrypted sessions
3. **FairPlay**: Support FairPlay v2.5 for encrypted streams
4. **Access Control**: HomeKit ACL levels (everyone, users, admin)

## Performance Goals

1. **Latency**: < 500ms end-to-end
2. **Dropouts**: Zero dropouts under normal conditions
3. **CPU**: < 10% on Raspberry Pi 4
4. **Memory**: < 100MB resident

## Testing Strategy

1. **Unit tests**: Each module independently
2. **Integration tests**: Full pairing and streaming flows
3. **Real-world testing**: Against actual iOS devices
4. **Performance tests**: Stress testing with continuous streams