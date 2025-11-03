//! Feature and status flags for AirPlay 2
//!
//! This module defines the feature flags and status flags used in the AirPlay 2 protocol.
//!
//! ## Feature Flags
//!
//! Feature flags are 64-bit values that indicate the capabilities supported by the receiver.
//! The default AirPlay 2 configuration uses flags value: `0x0001c300405f4200`
//!
//! ## References
//!
//! Based on the Python ap2-receiver implementation and AirPlay 2 protocol documentation.

use bitflags::bitflags;

bitflags! {
    /// AirPlay 2 feature flags
    ///
    /// These flags indicate the capabilities supported by the receiver.
    /// Each flag is numbered (Ft00-Ft63) corresponding to its bit position.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FeatureFlags: u64 {
        // Audio Features (Ft00-Ft10)
        const FT00_RESERVED             = 1 << 0;   // Reserved
        const FT01_RESERVED             = 1 << 1;   // Reserved
        const FT02_RESERVED             = 1 << 2;   // Reserved
        const FT03_RESERVED             = 1 << 3;   // Reserved
        const FT04_RESERVED             = 1 << 4;   // Reserved
        const FT05_RESERVED             = 1 << 5;   // Reserved
        const FT06_RESERVED             = 1 << 6;   // Reserved
        const FT07_RESERVED             = 1 << 7;   // Reserved
        const FT08_RESERVED             = 1 << 8;   // Reserved
        const AIRPLAY_AUDIO             = 1 << 9;   // Ft09: AirPlay Audio support
        const FT10_RESERVED             = 1 << 10;  // Reserved

        // Encryption & DRM (Ft11-Ft15)
        const FT11_RESERVED             = 1 << 11;  // Reserved
        const FT12_RESERVED             = 1 << 12;  // Reserved
        const FT13_RESERVED             = 1 << 13;  // Reserved
        const MFI_SOFT_FAIRPLAY         = 1 << 14;  // Ft14: MFi Software FairPlay
        const FT15_RESERVED             = 1 << 15;  // Reserved

        // Metadata & Progress (Ft16-Ft17)
        const AUDIO_META_PROGRESS       = 1 << 16;  // Ft16: Audio metadata with progress
        const AUDIO_META_TXT_DAAP       = 1 << 17;  // Ft17: Audio metadata text (DAAP)

        // Audio Formats (Ft18-Ft21)
        const RECEIVE_AUDIO_PCM         = 1 << 18;  // Ft18: Receive audio in PCM format
        const RECEIVE_AUDIO_ALAC        = 1 << 19;  // Ft19: Receive audio in ALAC format
        const RECEIVE_AUDIO_AAC_LC      = 1 << 20;  // Ft20: Receive audio in AAC-LC format
        const FT21_RESERVED             = 1 << 21;  // Reserved

        // Audio Capabilities (Ft22-Ft29)
        const AUDIO_UNENCRYPTED         = 1 << 22;  // Ft22: Support unencrypted audio
        const FT23_RESERVED             = 1 << 23;  // Reserved
        const FT24_RESERVED             = 1 << 24;  // Reserved
        const FT25_RESERVED             = 1 << 25;  // Reserved
        const FT26_RESERVED             = 1 << 26;  // Reserved
        const FT27_RESERVED             = 1 << 27;  // Reserved
        const FT28_RESERVED             = 1 << 28;  // Reserved
        const FT29_RESERVED             = 1 << 29;  // Reserved

        // Protocol Features (Ft30-Ft39)
        const UNIFIED_ADVERTISING_INFO  = 1 << 30;  // Ft30: Unified advertising info (AirPlay 2)
        const FT31_RESERVED             = 1 << 31;  // Reserved
        const FT32_RESERVED             = 1 << 32;  // Reserved
        const FT33_RESERVED             = 1 << 33;  // Reserved
        const FT34_RESERVED             = 1 << 34;  // Reserved
        const FT35_RESERVED             = 1 << 35;  // Reserved
        const FT36_RESERVED             = 1 << 36;  // Reserved
        const FT37_RESERVED             = 1 << 37;  // Reserved
        const FT38_RESERVED             = 1 << 38;  // Reserved
        const FT39_RESERVED             = 1 << 39;  // Reserved

        // Buffering & Timing (Ft40-Ft45)
        const BUFFERED_AUDIO            = 1 << 40;  // Ft40: Buffered audio streaming
        const PTP_CLOCK                 = 1 << 41;  // Ft41: PTP clock synchronization
        const FT42_RESERVED             = 1 << 42;  // Reserved
        const FT43_RESERVED             = 1 << 43;  // Reserved
        const FT44_RESERVED             = 1 << 44;  // Reserved
        const FT45_RESERVED             = 1 << 45;  // Reserved

        // HomeKit & Security (Ft46-Ft50)
        const HOMEKIT_PAIRING           = 1 << 46;  // Ft46: HomeKit pairing support
        const PEER_MANAGEMENT           = 1 << 47;  // Ft47: Peer management
        const TRANSIENT_PAIRING         = 1 << 48;  // Ft48: Transient pairing (guest mode)
        const FT49_RESERVED             = 1 << 49;  // Reserved
        const FT50_RESERVED             = 1 << 50;  // Reserved

        // Extended Features (Ft51-Ft63)
        const FT51_RESERVED             = 1 << 51;  // Reserved
        const FT52_RESERVED             = 1 << 52;  // Ft52: Set peers extended (requires for SETPEERSX)
        const FT53_RESERVED             = 1 << 53;  // Reserved
        const FT54_RESERVED             = 1 << 54;  // Reserved
        const FT55_RESERVED             = 1 << 55;  // Reserved
        const FT56_RESERVED             = 1 << 56;  // Reserved
        const FT57_RESERVED             = 1 << 57;  // Reserved
        const FT58_RESERVED             = 1 << 58;  // Reserved
        const FT59_RESERVED             = 1 << 59;  // Reserved
        const FT60_RESERVED             = 1 << 60;  // Reserved
        const FT61_RESERVED             = 1 << 61;  // Reserved
        const FT62_RESERVED             = 1 << 62;  // Reserved
        const FT63_RESERVED             = 1 << 63;  // Reserved
    }
}

impl FeatureFlags {
    /// Get the default AirPlay 2 feature flags
    ///
    /// This returns the standard feature set for a fully-featured AirPlay 2 receiver.
    /// Value: `0x0001c300405f4200`
    ///
    /// Enabled features:
    /// - Ft09: AirPlayAudio
    /// - Ft14: MFiSoft_FairPlay
    /// - Ft16: AudioMetaProgress
    /// - Ft17: AudioMetaTxtDAAP
    /// - Ft18: ReceiveAudioPCM
    /// - Ft19: ReceiveAudioALAC
    /// - Ft20: ReceiveAudioAAC_LC
    /// - Ft22: AudioUnencrypted
    /// - Ft30: UnifiedAdvertisingInfo
    /// - Ft40: BufferedAudio
    /// - Ft41: PTPClock
    /// - Ft46: HomeKitPairing
    /// - Ft47: PeerManagement
    /// - Ft48: TransientPairing
    pub fn default_airplay2() -> Self {
        Self::AIRPLAY_AUDIO
            | Self::MFI_SOFT_FAIRPLAY
            | Self::AUDIO_META_PROGRESS
            | Self::AUDIO_META_TXT_DAAP
            | Self::RECEIVE_AUDIO_PCM
            | Self::RECEIVE_AUDIO_ALAC
            | Self::RECEIVE_AUDIO_AAC_LC
            | Self::AUDIO_UNENCRYPTED
            | Self::UNIFIED_ADVERTISING_INFO
            | Self::BUFFERED_AUDIO
            | Self::PTP_CLOCK
            | Self::HOMEKIT_PAIRING
            | Self::PEER_MANAGEMENT
            | Self::TRANSIENT_PAIRING
    }

    /// Get minimal audio-only feature flags
    ///
    /// This returns a minimal feature set for basic audio streaming without
    /// HomeKit pairing or advanced features.
    ///
    /// Enabled features:
    /// - Ft09: AirPlayAudio
    /// - Ft14: MFiSoft_FairPlay
    /// - Ft18: ReceiveAudioPCM
    /// - Ft19: ReceiveAudioALAC
    /// - Ft20: ReceiveAudioAAC_LC
    /// - Ft22: AudioUnencrypted
    pub fn minimal_audio() -> Self {
        Self::AIRPLAY_AUDIO
            | Self::MFI_SOFT_FAIRPLAY
            | Self::RECEIVE_AUDIO_PCM
            | Self::RECEIVE_AUDIO_ALAC
            | Self::RECEIVE_AUDIO_AAC_LC
            | Self::AUDIO_UNENCRYPTED
    }

    /// Format as hex string for protocol use
    ///
    /// Returns the feature flags formatted for use in HTTP/RTSP responses.
    /// For values > 32 bits, returns two comma-separated 32-bit hex values.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let flags = FeatureFlags::default_airplay2();
    /// assert_eq!(flags.to_hex_string(), "0x405f4200,0x1c300");
    /// ```
    pub fn to_hex_string(&self) -> String {
        let bits = self.bits();
        if bits <= 0xFFFFFFFF {
            format!("0x{:x}", bits)
        } else {
            // Split into two 32-bit parts for compatibility
            format!("0x{:x},0x{:x}", bits & 0xFFFFFFFF, (bits >> 32) & 0xFFFFFFFF)
        }
    }

    /// Check if this is an AirPlay 2 configuration
    ///
    /// Returns true if Ft30 (UnifiedAdvertisingInfo) is enabled,
    /// which indicates AirPlay 2 vs legacy AirPlay 1.
    pub fn is_airplay2(&self) -> bool {
        self.contains(Self::UNIFIED_ADVERTISING_INFO)
    }

    /// Check if buffered audio is supported
    ///
    /// Returns true if both buffered audio and PTP clock are enabled.
    pub fn supports_buffered_audio(&self) -> bool {
        self.contains(Self::BUFFERED_AUDIO) && self.contains(Self::PTP_CLOCK)
    }

    /// Check if HomeKit pairing is supported
    pub fn supports_homekit(&self) -> bool {
        self.contains(Self::HOMEKIT_PAIRING)
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
    fn test_feature_flags_individual() {
        // Test individual flag bits
        assert_eq!(FeatureFlags::AIRPLAY_AUDIO.bits(), 1 << 9);
        assert_eq!(FeatureFlags::MFI_SOFT_FAIRPLAY.bits(), 1 << 14);
        assert_eq!(FeatureFlags::AUDIO_META_PROGRESS.bits(), 1 << 16);
        assert_eq!(FeatureFlags::AUDIO_META_TXT_DAAP.bits(), 1 << 17);
        assert_eq!(FeatureFlags::RECEIVE_AUDIO_PCM.bits(), 1 << 18);
        assert_eq!(FeatureFlags::RECEIVE_AUDIO_ALAC.bits(), 1 << 19);
        assert_eq!(FeatureFlags::RECEIVE_AUDIO_AAC_LC.bits(), 1 << 20);
        assert_eq!(FeatureFlags::AUDIO_UNENCRYPTED.bits(), 1 << 22);
        assert_eq!(FeatureFlags::UNIFIED_ADVERTISING_INFO.bits(), 1 << 30);
        assert_eq!(FeatureFlags::BUFFERED_AUDIO.bits(), 1 << 40);
        assert_eq!(FeatureFlags::PTP_CLOCK.bits(), 1 << 41);
        assert_eq!(FeatureFlags::HOMEKIT_PAIRING.bits(), 1 << 46);
        assert_eq!(FeatureFlags::PEER_MANAGEMENT.bits(), 1 << 47);
        assert_eq!(FeatureFlags::TRANSIENT_PAIRING.bits(), 1 << 48);
    }

    #[test]
    fn test_default_airplay2_value() {
        // Test that default_airplay2() returns the correct value: 0x0001c300405f4200
        let flags = FeatureFlags::default_airplay2();
        assert_eq!(flags.bits(), 0x0001c300405f4200);
    }

    #[test]
    fn test_default_airplay2_contains() {
        let flags = FeatureFlags::default_airplay2();

        // Should contain these flags
        assert!(flags.contains(FeatureFlags::AIRPLAY_AUDIO));
        assert!(flags.contains(FeatureFlags::MFI_SOFT_FAIRPLAY));
        assert!(flags.contains(FeatureFlags::AUDIO_META_PROGRESS));
        assert!(flags.contains(FeatureFlags::AUDIO_META_TXT_DAAP));
        assert!(flags.contains(FeatureFlags::RECEIVE_AUDIO_PCM));
        assert!(flags.contains(FeatureFlags::RECEIVE_AUDIO_ALAC));
        assert!(flags.contains(FeatureFlags::RECEIVE_AUDIO_AAC_LC));
        assert!(flags.contains(FeatureFlags::AUDIO_UNENCRYPTED));
        assert!(flags.contains(FeatureFlags::UNIFIED_ADVERTISING_INFO));
        assert!(flags.contains(FeatureFlags::BUFFERED_AUDIO));
        assert!(flags.contains(FeatureFlags::PTP_CLOCK));
        assert!(flags.contains(FeatureFlags::HOMEKIT_PAIRING));
        assert!(flags.contains(FeatureFlags::PEER_MANAGEMENT));
        assert!(flags.contains(FeatureFlags::TRANSIENT_PAIRING));

        // Should NOT contain reserved flags
        assert!(!flags.contains(FeatureFlags::FT00_RESERVED));
        assert!(!flags.contains(FeatureFlags::FT52_RESERVED));
    }

    #[test]
    fn test_minimal_audio() {
        let flags = FeatureFlags::minimal_audio();

        // Should contain basic audio flags
        assert!(flags.contains(FeatureFlags::AIRPLAY_AUDIO));
        assert!(flags.contains(FeatureFlags::MFI_SOFT_FAIRPLAY));
        assert!(flags.contains(FeatureFlags::RECEIVE_AUDIO_PCM));
        assert!(flags.contains(FeatureFlags::RECEIVE_AUDIO_ALAC));
        assert!(flags.contains(FeatureFlags::RECEIVE_AUDIO_AAC_LC));
        assert!(flags.contains(FeatureFlags::AUDIO_UNENCRYPTED));

        // Should NOT contain HomeKit or advanced features
        assert!(!flags.contains(FeatureFlags::HOMEKIT_PAIRING));
        assert!(!flags.contains(FeatureFlags::UNIFIED_ADVERTISING_INFO));
        assert!(!flags.contains(FeatureFlags::BUFFERED_AUDIO));
    }

    #[test]
    fn test_hex_string_formatting() {
        // Test small value (< 32 bits)
        let small_flags = FeatureFlags::AIRPLAY_AUDIO;
        assert_eq!(small_flags.to_hex_string(), "0x200");

        // Test large value (> 32 bits) - should be split
        let large_flags = FeatureFlags::default_airplay2();
        assert_eq!(large_flags.to_hex_string(), "0x405f4200,0x1c300");
    }

    #[test]
    fn test_is_airplay2() {
        let ap2_flags = FeatureFlags::default_airplay2();
        assert!(ap2_flags.is_airplay2());

        let minimal_flags = FeatureFlags::minimal_audio();
        assert!(!minimal_flags.is_airplay2());
    }

    #[test]
    fn test_supports_buffered_audio() {
        let flags = FeatureFlags::default_airplay2();
        assert!(flags.supports_buffered_audio());

        let flags_no_buffer = FeatureFlags::AIRPLAY_AUDIO | FeatureFlags::PTP_CLOCK;
        assert!(!flags_no_buffer.supports_buffered_audio());

        let flags_no_ptp = FeatureFlags::AIRPLAY_AUDIO | FeatureFlags::BUFFERED_AUDIO;
        assert!(!flags_no_ptp.supports_buffered_audio());
    }

    #[test]
    fn test_supports_homekit() {
        let flags = FeatureFlags::default_airplay2();
        assert!(flags.supports_homekit());

        let minimal_flags = FeatureFlags::minimal_audio();
        assert!(!minimal_flags.supports_homekit());
    }

    #[test]
    fn test_bitwise_operations() {
        let mut flags = FeatureFlags::empty();
        assert!(!flags.contains(FeatureFlags::AIRPLAY_AUDIO));

        // Add a flag
        flags |= FeatureFlags::AIRPLAY_AUDIO;
        assert!(flags.contains(FeatureFlags::AIRPLAY_AUDIO));

        // Add another flag
        flags |= FeatureFlags::RECEIVE_AUDIO_ALAC;
        assert!(flags.contains(FeatureFlags::AIRPLAY_AUDIO));
        assert!(flags.contains(FeatureFlags::RECEIVE_AUDIO_ALAC));

        // Remove a flag
        flags.remove(FeatureFlags::AIRPLAY_AUDIO);
        assert!(!flags.contains(FeatureFlags::AIRPLAY_AUDIO));
        assert!(flags.contains(FeatureFlags::RECEIVE_AUDIO_ALAC));
    }

    #[test]
    fn test_status_flags_individual() {
        assert_eq!(StatusFlags::RECV_SESS_ACTIVE.bits(), 1 << 0);
        assert_eq!(StatusFlags::HKAC_FLAG.bits(), 1 << 1);
        assert_eq!(StatusFlags::PW_SET_FLAG.bits(), 1 << 2);
    }

    #[test]
    fn test_status_flags_combinations() {
        let flags = StatusFlags::RECV_SESS_ACTIVE | StatusFlags::HKAC_FLAG;
        assert!(flags.contains(StatusFlags::RECV_SESS_ACTIVE));
        assert!(flags.contains(StatusFlags::HKAC_FLAG));
        assert!(!flags.contains(StatusFlags::PW_SET_FLAG));
    }

    #[test]
    fn test_status_flags_hex_string() {
        let flags = StatusFlags::empty();
        assert_eq!(flags.to_hex_string(), "0x0");

        let flags = StatusFlags::RECV_SESS_ACTIVE;
        assert_eq!(flags.to_hex_string(), "0x1");

        let flags = StatusFlags::RECV_SESS_ACTIVE | StatusFlags::HKAC_FLAG | StatusFlags::PW_SET_FLAG;
        assert_eq!(flags.to_hex_string(), "0x7");
    }

    #[test]
    fn test_feature_flags_are_copy() {
        // Verify that FeatureFlags implements Copy
        let flags1 = FeatureFlags::default_airplay2();
        let flags2 = flags1; // Should copy, not move
        assert_eq!(flags1, flags2);
        // Both should still be usable
        assert!(flags1.contains(FeatureFlags::AIRPLAY_AUDIO));
        assert!(flags2.contains(FeatureFlags::AIRPLAY_AUDIO));
    }
}
