# Migration Guide

This guide helps you migrate from the old SIYI protocol implementation to the new feature-gated, camera-aware version.

## Overview of Changes

The new version introduces:
- Feature flags for protocol types (TCP, UDP, TTL)
- Feature flags for camera models
- Camera-specific message filtering
- Protocol-specific message filtering
- Improved module organization
- Better compile-time optimization

## Breaking Changes

### 1. Module Structure

**Old:**
```rust
use siyi_protocol::*;

let request = FirmwareVersionRequest::new();
```

**New:**
```rust
use siyi_protocol::zt30_tcp::*;

let request = FirmwareVersionRequest::new();
```

You must now specify which camera and protocol combination you're using by importing from the appropriate module.

### 2. Cargo.toml Dependencies

**Old:**
```toml
[dependencies]
siyi-protocol = "0.0.x"
```

**New:**
```toml
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "tcp"] }
```

You must specify which features (cameras and protocols) you need.

### 3. Message Availability

Messages are now filtered based on your selected camera and protocol. Attempting to use unsupported messages will result in compile-time errors.

**Old:**
```rust
// Would compile but potentially fail at runtime
let msg = ThermalImageRequest::new();
```

**New:**
```rust
// Compile error if thermal imaging not supported by selected camera
use siyi_protocol::zr10_tcp::*;  // ZR10 doesn't have thermal
// let msg = ThermalImageRequest::new();  // Won't compile
```

## Migration Steps

### Step 1: Identify Your Hardware

Determine which camera model you're using:
- ZT30 - Quad-sensor gimbal
- ZT6 - Thermal camera
- ZR30 - Long-range camera
- ZR10 - Standard gimbal
- A8mini - Compact gimbal
- A2mini - Entry-level gimbal

### Step 2: Identify Your Protocol

Determine how you communicate with the camera:
- TCP - Network (default port 37260)
- UDP - Network (default port 37260)
- TTL - Serial UART (115200 baud)

### Step 3: Update Cargo.toml

Add the appropriate features:

```toml
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "tcp"] }
```

For multiple protocols:
```toml
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "tcp", "udp"] }
```

### Step 4: Update Import Statements

**Old:**
```rust
use siyi_protocol::*;
```

**New:**
```rust
use siyi_protocol::zt30_tcp::*;
// or
use siyi_protocol::zt30_udp::*;
// or
use siyi_protocol::zt30_ttl::*;
```

### Step 5: Update Message Usage

The message API remains the same, but now with compile-time guarantees:

```rust
// This still works the same way
let request = FirmwareVersionRequest::new();
let mut buf = [0u8; MAX_MESSAGE_SIZE];
let len = request.encode(&mut buf)?;
```

## Feature Selection Examples

### Desktop Application with Network

```toml
# Cargo.toml
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "tcp"] }
```

```rust
// main.rs
use siyi_protocol::zt30_tcp::*;
use std::net::TcpStream;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("192.168.144.25:37260")?;
    
    let request = FirmwareVersionRequest::new();
    let mut msg_buf = [0u8; MAX_MESSAGE_SIZE];
    let mut frame_buf = [0u8; MAX_FRAME_SIZE];
    
    let msg_len = request.encode(&mut msg_buf)?;
    let frame = request.to_frame(&msg_buf[..msg_len]);
    let frame_len = frame.encode(&mut frame_buf)?;
    
    stream.write_all(&frame_buf[..frame_len])?;
    Ok(())
}
```

### Embedded System with Serial

```toml
# Cargo.toml
[dependencies]
siyi-protocol = { version = "0.1", features = ["zr10", "ttl"], default-features = false }
```

```rust
// main.rs (no_std)
#![no_std]
#![no_main]

use siyi_protocol::zr10_ttl::*;

#[no_mangle]
pub extern "C" fn main() {
    let request = GimbalAttitudeRequest::new();
    let mut buf = [0u8; MAX_MESSAGE_SIZE];
    
    // Stack-allocated, no heap
    if let Ok(len) = request.encode(&mut buf) {
        // Send via UART
        uart_send(&buf[..len]);
    }
}
```

### Multi-Protocol Support

```toml
# Cargo.toml
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "tcp", "udp"] }
```

```rust
// main.rs
use siyi_protocol::zt30_tcp;
use siyi_protocol::zt30_udp;

fn send_tcp(request: &zt30_tcp::FirmwareVersionRequest) {
    // TCP implementation
}

fn send_udp(request: &zt30_udp::GimbalAttitudeRequest) {
    // UDP implementation
}
```

## Common Migration Issues

### Issue 1: Message Not Found

**Error:**
```
error[E0432]: unresolved import `siyi_protocol::ThermalImageRequest`
```

**Solution:**
Check if your camera supports the message. Some messages are only available on specific cameras.

```rust
// Wrong - ZR10 doesn't have thermal imaging
use siyi_protocol::zr10_tcp::ThermalImageRequest;

// Correct - Use ZT30 or ZT6 for thermal
use siyi_protocol::zt30_tcp::ThermalImageRequest;
```

### Issue 2: Module Not Found

**Error:**
```
error[E0432]: unresolved import `siyi_protocol::zt30_tcp`
```

**Solution:**
Add the required features to your Cargo.toml:

```toml
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "tcp"] }
```

### Issue 3: Binary Size Too Large

**Problem:**
Including too many features increases binary size.

**Solution:**
Only include the features you actually use:

```toml
# Before (large binary)
[dependencies]
siyi-protocol = { version = "0.1", features = ["all"] }

# After (smaller binary)
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "tcp"] }
```

### Issue 4: Parser State Machine Removed

**Old version** had a byte-by-byte state machine parser.

**New version** focuses on message definitions. You need to implement your own transport layer.

**Solution:**
Use the provided `Frame::decode()` for complete frames:

```rust
// Receive complete frame from your transport
let frame_data = receive_complete_frame()?;

// Decode
let frame = Frame::decode(&frame_data)?;
let message = Message::from_frame(&frame)?;
```

For serial communication, implement your own state machine or use a library like `postcard` or `serde`.

## API Compatibility

### Still Supported

These APIs work the same way:
- `Message::encode()`
- `Message::decode()`
- `Frame::encode()`
- `Frame::decode()`
- `Message::from_frame()`
- All message struct definitions
- All enumeration types
- CRC16 calculation

### Changed

These have changed:
- Module organization (now feature-gated)
- Import paths (now include camera and protocol)
- Message availability (filtered by camera/protocol)

### Removed

These were removed:
- State machine parser (`FrameParser`)
- Generic message module
- `bytes_to_message()` helper (use `Frame::decode()` + `Message::from_frame()`)
- `message_to_bytes()` helper (use message `encode()` + frame `encode()`)

## Testing Your Migration

### 1. Compile Test

```bash
cargo build --features "zt30,tcp"
```

### 2. Run Examples

```bash
cargo run --example serialize_messages --features "zt30,tcp"
cargo run --example deserialize_messages --features "zt30,tcp"
```

### 3. Hardware Test

Test with your actual hardware to verify compatibility.

## Gradual Migration Strategy

If you have a large codebase, migrate gradually:

### Phase 1: Update Dependencies
Update Cargo.toml with proper features, but keep old code.

### Phase 2: Update Imports
Change import statements one module at a time.

### Phase 3: Test Each Module
Test each migrated module before moving to the next.

### Phase 4: Remove Old Code
Once everything works, remove any old compatibility shims.

## Getting Help

If you encounter issues during migration:

1. Check the examples in the `examples/` directory
2. Review the API documentation
3. Open an issue on GitHub with:
   - Your hardware model
   - Protocol type
   - Error messages
   - Code sample

## Benefits of Migrating

After migration, you'll benefit from:

1. **Smaller binaries** - Only include what you use
2. **Compile-time safety** - Catch protocol errors at compile time
3. **Better documentation** - Camera and protocol support clearly documented
4. **No heap allocation** - True no_std support
5. **Faster compilation** - Less code to compile with feature gates
6. **Type safety** - Protocol and camera mismatches caught early

## Rollback Plan

If you need to temporarily rollback:

```toml
[dependencies]
siyi-protocol = "0.0.x"  # Old version
```

However, we recommend migrating as the old version will not receive updates.