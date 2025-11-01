# Python Reference Implementation Notes

This document contains key information extracted from the Python ap2-receiver implementation to guide the Rust conversion.

---

## Key Constants & Configuration

### Feature Flags (Default Configuration)
```python
FEATURES = FeatureFlags.GetDefaultAirplayTwoFlags(FeatureFlags)
# Results in: 0x0001c300405f4200
# Binary: Ft48|Ft47|Ft46|Ft41|Ft40|Ft30|Ft22|Ft20|Ft19|Ft18|Ft17|Ft16|Ft14|Ft09
```

**Important Features:**
- Ft09: AirPlayAudio
- Ft14: MFiSoft_FairPlay
- Ft16: AudioMetaProgress
- Ft17: AudioMetaTxtDAAP
- Ft18: ReceiveAudioPCM
- Ft19: ReceiveAudioALAC
- Ft20: ReceiveAudioAAC_LC
- Ft22: AudioUnencrypted
- Ft30: UnifiedAdvertisingInfo
- Ft40: BufferedAudio
- Ft41: PTPClock
- Ft46: HomeKitPairing
- Ft47: PeerManagement
- Ft48: TransientPairing

### Status Flags
- `RecvSessActive`: Set when streaming session is active
- `HKACFlag`: HomeKit Access Control enabled
- `PWSetFlag`: Password protection enabled

### Server Configuration
```python
SERVER_VERSION = "366.0"  # Dictates client behavior
PORT = 7000  # Standard AirPlay port
AIRPLAY_BUFFER = 8388608  # 8MB buffer (1024 * 8192)
```

**Version behavior:**
- \> 360: Triggers remote control
- \<= 355: Triggers REALTIME and NTP (vs buffered)
- \>= 355: Triggers PTP and buffered
- \<= 350: Prevents shk in streams
- 200: Triggers ANNOUNCE (APv1)

---

## HTTP Content Types

```python
HTTP_CT_BPLIST = "application/x-apple-binary-plist"
HTTP_CT_OCTET = "application/octet-stream"
HTTP_CT_PARAM = "text/parameters"
HTTP_CT_IMAGE_JPEG = "image/jpeg"
HTTP_CT_IMAGE = "image/"
HTTP_CT_DMAP = "application/x-dmap-tagged"
HTTP_CT_PAIR = "application/pairing+tlv8"
```

---

## RTSP/HTTP Endpoints

### GET Endpoints
- `/info` - Returns device capabilities and configuration

### POST Endpoints
- `/command` - Remote control commands
- `/feedback` - Stream feedback
- `/audioMode` - Audio mode configuration
- `/auth-setup` - Authentication setup
- `/fp-setup` - FairPlay setup
- `/fp-setup2` - FairPlay setup v2 (alias for auth-setup)
- `/pair-pin-start` - PIN pairing initiation
- `/pair-setup` - HAP pairing setup (SRP)
- `/pair-verify` - HAP pairing verification (Ed25519)
- `/pair-add` - Add paired controller
- `/pair-remove` - Remove paired controller
- `/pair-list` - List paired controllers
- `/configure` - HomeKit configuration

### RTSP Methods (AirPlay v1 compatibility)
- `OPTIONS` - Capability negotiation
- `ANNOUNCE` - Stream announcement (with SDP)
- `SETUP` - Stream setup
- `RECORD` - Start recording/playback
- `FLUSH` - Flush buffer
- `FLUSHBUFFERED` - Flush buffered stream
- `TEARDOWN` - End session
- `GET_PARAMETER` - Get parameter (e.g., volume)
- `SET_PARAMETER` - Set parameter (e.g., volume, progress)
- `SETRATEANCHORTIME` - Play/pause control
- `SETPEERS` - Set timing peers
- `SETPEERSX` - Extended timing peers (requires Ft52)
- `SETMAGICCOOKIE` - iOS 17+ magic cookie

---

## Device Info Structure

```python
device_info = {
    'audioLatencies': [
        {
            'inputLatencyMicros': 0,
            'outputLatencyMicros': 400000,
        },
    ],
    'deviceID': DEVICE_ID,  # MAC address
    'features': int(FEATURES),  # Feature flags
    'keepAliveLowPower': True,
    'keepAliveSendStatsAsBody': True,
    'manufacturer': 'OpenAirplay',
    'model': 'Receiver',
    'name': DEV_NAME,
    'nameIsFactoryDefault': False,
    'pi': PI.decode(),  # Public ID (UUID)
    'protocolVersion': '1.1',
    'sdk': 'AirPlay;2.0.2',
    'sourceVersion': SERVER_VERSION,
    'statusFlags': get_hex_bitmask(STATUS_FLAGS),
}
```

---

## mDNS Properties

Service: `_airplay._tcp.local.`

```python
mdns_props = {
    "acl": HK_ACL_LEVEL,  # 0=everyone, 1=users, 2=admin
    "deviceid": DEVICE_ID,  # MAC address
    "features": get_hex_bitmask(FEATURES),
    "flags": get_hex_bitmask(STATUS_FLAGS),
    "gcgl": "0",  # Group Contains Group Leader
    "gid": "<uuid>",  # Group UUID
    "manufacturer": "OpenAirplay",
    "model": "Airplay2-Receiver",
    "name": DEV_NAME,
    "protovers": "1.1",
    "rsf": "0x0",  # Required Sender Features
    "serialNumber": DEVICE_ID,
    "srcvers": SERVER_VERSION,
    
    # RAOP-specific
    "ch": "2",  # Channels
    "cn": "0,1,2",  # Compression: PCM, ALAC, AAC
    "pi": PI.decode(),  # Pairing UUID
    "pk": LTPK_OBJ.get_pub_string(),  # Ed25519 public key
    "sf": get_hex_bitmask(STATUS_FLAGS),
}
```

---

## SDP Handling

**Audio Formats:**
- PCM (96): Raw audio
- ALAC (96): Apple Lossless
- AAC-LC (96): AAC Low Complexity
- AAC-ELD (96): AAC Enhanced Low Delay

**Encryption Methods:**
- RSA (et=1): Legacy encryption
- FairPlay (et=3): FairPlay v1
- MFi (et=4): Made for iPhone hardware
- FairPlay v2.5 (et=5): Current standard

**Key SDP Parameters:**
- `spf`: Samples per frame
- `minlatency`: Minimum latency in samples
- `maxlatency`: Maximum latency in samples
- `sr`: Sample rate (typically 44100 or 48000)

---

## Stream Setup

### AirPlay v2 (Buffered)
1. Client sends POST /setup with stream configuration
2. Server creates Stream objects with:
   - Control port (RTCP)
   - Data port (RTP audio)
   - Timing port (NTP/PTP)
3. Server responds with port numbers

### AirPlay v1 (Realtime)
1. Client sends ANNOUNCE with SDP
2. Client sends SETUP with transport parameters
3. Server responds with allocated ports
4. Client sends RECORD to start
5. Client sends SETRATEANCHORTIME for play/pause

---

## Encryption Flow

### HAP Pairing
1. **pair-setup**: SRP-based authentication
   - Client proves knowledge of PIN/password
   - Exchange of Ed25519 keys
   - Generates shared secret

2. **pair-verify**: Session establishment
   - Client proves identity with Ed25519 signature
   - Establishes session keys
   - Upgrades to ChaCha20-Poly1305 encrypted transport

### FairPlay Setup
1. **fp-setup**: 168-byte keymsg exchange
2. Used for stream encryption (AES-128 in some modes)
3. Keys derived from keymsg using PlayFair algorithm

---

## Volume Control

### GET_PARAMETER
Client requests: `volume`
Server responds: `-30.0` (in dB, typically -144 to 0)

### SET_PARAMETER
Client sends: `volume: -15.0`
Server adjusts system volume

**Note:** Volume disabled with `-nv` flag

---

## Progress/Timing

### SET_PARAMETER progress
Format: `progress: start/current/stop`
- Timestamps in NTP format
- Used for seek bar updates

### SETRATEANCHORTIME
```python
{
    'rate': 1,  # 1=play, 0=pause
    'rtpTime': 123456  # RTP timestamp anchor
}
```

---

## Stream Types

### Buffered (Stream.BUFFERED = 110)
- Higher latency (~2 seconds)
- More resilient to network issues
- Uses PTP for clock sync
- Requires larger buffer

### Realtime (Stream.REALTIME = 96)
- Lower latency (~500ms)
- Uses NTP for clock sync
- Smaller buffer
- Less forgiving of packet loss

---

## Key Algorithms

### Apple-Response (AP1 Security)
```python
def compute_apple_response(challenge, ip_addr, hw_addr):
    # Used in OPTIONS response for AP1
    # Combines challenge + IP + MAC
    # Returns base64 response
```

### Feature Bitmask Formatting
```python
def get_hex_bitmask(features):
    if features.bit_length() <= 32:
        return hex(features)
    else:
        # Split into two 32-bit parts
        return f"{hex(features & 0xffffffff)},{hex(features >> 32 & 0xffffffff)}"
```

---

## Error Handling Patterns

- Return 404 for unsupported features/codecs
- Return 200 for successful operations
- Log warnings for disabled features
- Gracefully handle client disconnects
- Clean up streams on TEARDOWN

---

## Threading Model (Python)

- Main thread: HTTP/RTSP server
- Per-request thread: Handler instances
- Separate processes for:
  - Event server
  - Timing server (NTP/PTP)
  - Stream control
  - Audio output

**Rust equivalent:** Use Tokio tasks for async operations

---

## Important Flags Combinations

### Basic Audio Only
```
-ftxor 9 14 18 19 20 22
# Minimal working config
```

### Full AirPlay 2
```
Default flags (see above)
# Includes HomeKit, PTP, buffered audio
```

### Force AP1 Behavior
```
Remove Ft30 (UnifiedAdvertisingInfo)
Set SERVER_VERSION < 300
```

---

## Platform-Specific Notes

### Windows
- Uses comtypes for volume control
- Requires network interface GUID or name
- IPv6 link-local requires scope ID

### Linux (Raspberry Pi)
- ALSA for audio output
- Network interface by name (e.g., "wlan0")
- May need performance tuning for zero-copy

---

## Common Issues & Solutions

1. **"getaddrinfo failed"**: IPv6 link-local needs scope (`fe80::...%iface`)
2. **Dropouts**: Increase buffer size or check network performance
3. **No volume control**: Use `-nv` flag to disable
4. **Client can't find receiver**: Check mDNS registration and firewall
5. **Pairing fails**: Ensure Ft46 (HomeKitPairing) is enabled

---

## Next Steps for Rust Implementation

1. Start with pure data structures (no I/O)
2. Implement crypto primitives (can test in isolation)
3. Build protocol parsers (plist, TLV8, SDP)
4. Create network layer with mocked crypto
5. Integrate crypto into network layer
6. Add streaming layer
7. Integrate audio output