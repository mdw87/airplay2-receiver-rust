//! Feature and status flags for AirPlay 2
//!
//! This module defines the feature flags and status flags used in the AirPlay 2 protocol.

use bitflags::bitflags;

bitflags! {
    /// AirPlay 2 feature flags
    ///
    /// These flags indicate the capabilities supported by the receiver.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FeatureFlags: u64 {
        // TODO: Define all feature flags from Python reference
        // For now, just a placeholder
        const AUDIO = 1 << 9;           // Ft09: AirPlayAudio
        const FAIRPLAY = 1 << 14;       // Ft14: MFiSoft_FairPlay
        const AUDIO_META_PROGRESS = 1 << 16;  // Ft16: AudioMetaProgress
        const RECEIVE_AUDIO_ALAC = 1 << 19;   // Ft19: ReceiveAudioALAC
        const RECEIVE_AUDIO_AAC = 1 << 20;    // Ft20: ReceiveAudioAAC_LC
    }
}

impl FeatureFlags {
    /// Get the default AirPlay 2 feature flags
    /// Result: 0x0001c300405f4200
    pub fn default_airplay2() -> Self {
        // TODO: Implement full default flags from Python reference
        Self::AUDIO | Self::FAIRPLAY | Self::AUDIO_META_PROGRESS
    }

    /// Format as hex string for protocol use
    pub fn to_hex_string(&self) -> String {
        let bits = self.bits();
        if bits <= 0xFFFFFFFF {
            format!("0x{:x}", bits)
        } else {
            // Split into two 32-bit parts for compatibility
            format!("0x{:x},0x{:x}", bits & 0xFFFFFFFF, (bits >> 32) & 0xFFFFFFFF)
        }
    }
}

bitflags! {
    /// AirPlay 2 status flags
    ///
    /// These flags indicate the current status of the receiver.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusFlags: u32 {
        const RECV_SESS_ACTIVE = 1 << 0;  // Receiving session active
        const HKAC_FLAG = 1 << 1;         // HomeKit Access Control enabled
        const PW_SET_FLAG = 1 << 2;       // Password protection enabled
    }
}

impl StatusFlags {
    /// Format as hex string for protocol use
    pub fn to_hex_string(&self) -> String {
        format!("0x{:x}", self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_flags() {
        let flags = FeatureFlags::AUDIO | FeatureFlags::FAIRPLAY;
        assert!(flags.contains(FeatureFlags::AUDIO));
        assert!(flags.contains(FeatureFlags::FAIRPLAY));
        assert!(!flags.contains(FeatureFlags::RECEIVE_AUDIO_ALAC));
    }

    #[test]
    fn test_status_flags() {
        let flags = StatusFlags::RECV_SESS_ACTIVE;
        assert!(flags.contains(StatusFlags::RECV_SESS_ACTIVE));
        assert!(!flags.contains(StatusFlags::HKAC_FLAG));
    }
}
