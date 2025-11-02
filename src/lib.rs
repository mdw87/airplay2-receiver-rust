//! # AirPlay 2 Receiver
//!
//! A high-performance AirPlay 2 audio receiver implementation in Rust.
//!
//! This library provides all the components needed to build an AirPlay 2 receiver,
//! including protocol parsing, cryptography, network handling, and audio streaming.
//!
//! ## Architecture
//!
//! The library is organized into several modules:
//!
//! - `config`: Device configuration and feature flags
//! - `crypto`: Cryptographic operations (HAP pairing, FairPlay, encryption)
//! - `protocol`: Protocol parsing (Plist, TLV8, SDP, RTSP)
//! - `network`: Network layer (HTTP/RTSP server, mDNS)
//! - `streaming`: Audio streaming (RTP, session management)
//! - `audio`: Audio decoding and output
//! - `utils`: Utility functions (logging, network interfaces)

// Module declarations
pub mod audio;
pub mod config;
pub mod crypto;
pub mod network;
pub mod protocol;
pub mod streaming;
pub mod utils;

// Re-export commonly used types
pub use config::{DeviceConfig, FeatureFlags, StatusFlags};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// AirPlay protocol version
pub const AIRPLAY_PROTOCOL_VERSION: &str = "1.1";

/// AirPlay SDK version string
pub const AIRPLAY_SDK_VERSION: &str = "AirPlay;2.0.2";

/// Default server version (affects client behavior)
/// - > 360: Triggers remote control
/// - >= 355: Triggers PTP and buffered audio
/// - <= 355: Triggers REALTIME and NTP
pub const SERVER_VERSION: &str = "366.0";

/// Standard AirPlay port
pub const DEFAULT_PORT: u16 = 7000;

/// Default audio buffer size (8MB)
pub const AIRPLAY_BUFFER_SIZE: usize = 8 * 1024 * 1024;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constants() {
        assert!(!VERSION.is_empty());
        assert_eq!(AIRPLAY_PROTOCOL_VERSION, "1.1");
        assert_eq!(DEFAULT_PORT, 7000);
    }
}
