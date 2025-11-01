# Current Task

## Status: 📋 Planning Phase

---

## What We're Working On

**Phase**: Project Planning & Documentation  
**Date Started**: [Current Date]  
**Target Completion**: Today

---

## Objectives

✅ Create ARCHITECTURE.md - High-level system design  
✅ Create PROGRESS.md - Track implementation progress  
✅ Create PYTHON_REFERENCE.md - Key details from Python implementation  
🚧 Create CURRENT_TASK.md - This file!  
⏳ Review and refine all planning documents

---

## Next Session Plan

### Immediate Next Steps (Session 2)
1. **Set up Cargo project structure**
   - Create workspace with `cargo new --lib airplay2-receiver`
   - Set up module directories (config/, crypto/, protocol/, etc.)
   - Add initial dependencies to Cargo.toml
   - Configure basic logging with tracing

2. **First implementation: FeatureFlags**
   - Create `src/config/flags.rs`
   - Use `bitflags!` macro to define FeatureFlags
   - Implement all 64 feature bits from Python
   - Add helper methods (GetDefaultAirplayTwoFlags, etc.)
   - Write unit tests for bitflag operations

### Why Start with FeatureFlags?
- Self-contained (no external dependencies beyond bitflags crate)
- No I/O or async complexity
- Easy to test
- Core to the entire system (used in device info, mDNS, etc.)
- Good warm-up to get familiar with Rust patterns

---

## Future Tasks (Ordered by Priority)

### Week 1: Foundation
- [ ] StatusFlags implementation
- [ ] DeviceProperties struct
- [ ] CLI argument parsing with clap
- [ ] Network interface utilities (list interfaces, get MAC/IP)
- [ ] Basic logging setup

### Week 2: Protocol Basics
- [ ] Plist parsing/writing with `plist` crate
- [ ] TLV8 encoding/decoding
- [ ] Basic RTSP message parsing
- [ ] SDP parser

### Week 3: Crypto Foundation
- [ ] LTPK (Long-term public key) management
- [ ] HAP pairing state machine (structure only, no I/O yet)
- [ ] Ed25519 key operations
- [ ] ChaCha20-Poly1305 encryption/decryption helpers

### Week 4: Network Layer Basics
- [ ] HTTP/RTSP server with Tokio
- [ ] Request routing
- [ ] GET /info endpoint (no pairing required)
- [ ] mDNS service registration

### Week 5+: Advanced Features
- Full HAP pairing flow
- FairPlay encryption
- RTP streaming
- Audio decoding and output

---

## Questions to Resolve

1. **ALAC Decoder**: 
   - Option A: Use `symphonia` crate (full-featured, heavier)
   - Option B: Use `alac` crate (lighter, ALAC-only)
   - Option C: Custom implementation
   - **Decision needed**: Research both options

2. **HTTP Server Framework**:
   - Option A: `axum` (higher-level, easier routing)
   - Option B: `hyper` (lower-level, more control)
   - Option C: `actix-web` (feature-rich, different runtime)
   - **Leaning towards**: axum for simplicity, can migrate to hyper if needed

3. **Zero-Copy Strategy**:
   - Use `bytes::Bytes` throughout?
   - When to clone vs reference?
   - **Decision**: Research best practices for network data

4. **Error Handling Strategy**:
   - Custom error types per module?
   - Use `anyhow` for application errors?
   - Use `thiserror` for library errors?
   - **Decision**: Mix of both - thiserror for lib, anyhow for main

5. **PTP Clock Implementation**:
   - Can we use existing crates?
   - Need custom implementation?
   - **Needs research**: PTP on Rust

---

## Resources & References

### Rust Crates to Evaluate
- `bitflags` - Feature/status flags ✓
- `plist` - Binary plist support ✓
- `ed25519-dalek` - Ed25519 signatures ✓
- `chacha20poly1305` - Encryption ✓
- `srp` - HAP pairing ✓
- `tokio` - Async runtime ✓
- `axum` - HTTP server (to evaluate)
- `mdns-sd` - mDNS (to evaluate)
- `cpal` - Audio output ✓
- `symphonia` - Audio decoding (to evaluate)
- `get_if_addrs` - Network interfaces ✓
- `clap` - CLI parsing ✓

### Documentation to Read
- AirPlay 2 protocol (unofficial docs)
- HAP specification
- RTP/RTCP RFCs
- PTP/NTP specifications

---

## Success Criteria

### For Planning Phase (Current)
- ✅ All planning documents created
- ⏳ Documents reviewed and validated
- ⏳ Clear roadmap for first implementation
- ⏳ Cargo project structure defined

### For Next Session (FeatureFlags Implementation)
- [ ] Cargo project compiles
- [ ] FeatureFlags module implemented
- [ ] All 64 feature bits defined
- [ ] Unit tests passing
- [ ] Can print default flags in hex format

### For MVP (Minimum Viable Product)
- [ ] Can be discovered via mDNS
- [ ] Can respond to /info requests
- [ ] Can complete HAP pairing
- [ ] Can establish encrypted session
- [ ] Can receive and decrypt RTP audio
- [ ] Can decode ALAC audio
- [ ] Can output audio to speakers

---

## Notes & Observations

### From Python Analysis
- The Python implementation is ~1500 lines in the main file
- Heavy use of global variables (we'll use proper state management in Rust)
- Threading model will map well to Tokio tasks
- Many edge cases handled - need to preserve this logic
- Good logging is crucial for debugging

### Rust Advantages We'll Leverage
- Type safety for protocol messages
- Zero-cost abstractions for hot paths
- Fearless concurrency with Tokio
- Better memory management (no GC pauses)
- Cross-compilation for Raspberry Pi

### Challenges to Anticipate
- Crypto interoperability (must match Python exactly)
- Audio codec integration
- Platform-specific audio APIs
- Real-time performance requirements
- Debugging without a client device initially

---

## Communication Plan

When working with Claude:
1. **Start each session** by reviewing PROGRESS.md and CURRENT_TASK.md
2. **Update CURRENT_TASK.md** before starting new work
3. **Implement small chunks** - one function or struct at a time
4. **Test immediately** after each implementation
5. **Update PROGRESS.md** when completing tasks
6. **Ask questions** when uncertain about approach
7. **Document decisions** in relevant .md files

---

## Ready for Next Session!

The planning documents are complete. Next time you want to work on this project:

1. Open this Claude Project
2. Say something like: "I'm ready to start implementing! Let's begin with setting up the Cargo project and implementing FeatureFlags"
3. We'll work through it step by step

Would you like to:
- A) Start implementing right now (FeatureFlags)?
- B) Review/refine any of the planning documents?
- C) Save these documents and come back later?