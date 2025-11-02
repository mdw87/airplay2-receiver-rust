//! Configuration and device properties
//!
//! This module contains device configuration, feature flags, and status flags
//! used throughout the AirPlay 2 receiver.

mod device;
mod flags;

pub use device::{DeviceConfig, DeviceInfo};
pub use flags::{FeatureFlags, StatusFlags};
