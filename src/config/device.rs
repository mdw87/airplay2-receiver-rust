//! Device configuration and information

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{FeatureFlags, StatusFlags};

/// Device configuration
#[derive(Debug, Clone)]
pub struct DeviceConfig {
    /// Device name visible to clients
    pub name: String,
    /// Device ID (MAC address format)
    pub device_id: String,
    /// Public identifier (UUID)
    pub public_id: Uuid,
    /// Feature flags
    pub features: FeatureFlags,
    /// Status flags
    pub status: StatusFlags,
    /// Network interface to bind to
    pub interface: Option<String>,
    /// Server port
    pub port: u16,
    /// Volume control enabled
    pub volume_enabled: bool,
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            name: "AirPlay Receiver".to_string(),
            device_id: "00:00:00:00:00:00".to_string(),
            public_id: Uuid::new_v4(),
            features: FeatureFlags::default_airplay2(),
            status: StatusFlags::empty(),
            interface: None,
            port: crate::DEFAULT_PORT,
            volume_enabled: true,
        }
    }
}

/// Device information response for /info endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    /// Audio latencies
    pub audio_latencies: Vec<AudioLatency>,
    /// Device ID (MAC address)
    pub device_id: String,
    /// Feature flags as integer
    pub features: u64,
    /// Keep alive low power mode
    pub keep_alive_low_power: bool,
    /// Keep alive send stats as body
    pub keep_alive_send_stats_as_body: bool,
    /// Manufacturer name
    pub manufacturer: String,
    /// Model name
    pub model: String,
    /// Device name
    pub name: String,
    /// Name is factory default
    pub name_is_factory_default: bool,
    /// Public identifier (UUID)
    pub pi: String,
    /// Protocol version
    pub protocol_version: String,
    /// SDK version
    pub sdk: String,
    /// Source version
    pub source_version: String,
    /// Status flags as hex string
    pub status_flags: String,
}

/// Audio latency information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioLatency {
    /// Input latency in microseconds
    pub input_latency_micros: u32,
    /// Output latency in microseconds
    pub output_latency_micros: u32,
}

impl DeviceInfo {
    /// Create device info from configuration
    pub fn from_config(config: &DeviceConfig) -> Self {
        Self {
            audio_latencies: vec![AudioLatency {
                input_latency_micros: 0,
                output_latency_micros: 400_000,
            }],
            device_id: config.device_id.clone(),
            features: config.features.bits(),
            keep_alive_low_power: true,
            keep_alive_send_stats_as_body: true,
            manufacturer: "OpenAirplay".to_string(),
            model: "Receiver".to_string(),
            name: config.name.clone(),
            name_is_factory_default: false,
            pi: config.public_id.to_string(),
            protocol_version: crate::AIRPLAY_PROTOCOL_VERSION.to_string(),
            sdk: crate::AIRPLAY_SDK_VERSION.to_string(),
            source_version: crate::SERVER_VERSION.to_string(),
            status_flags: config.status.to_hex_string(),
        }
    }
}
