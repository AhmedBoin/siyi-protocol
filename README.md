# SIYI Protocol

A type-safe, no_std compatible Rust implementation of the SIYI Gimbal Camera SDK protocol. This crate provides message definitions and serialization/deserialization for communicating with SIYI camera systems over TCP, UDP, or serial (TTL) connections.

## Features

- **No heap allocation** - Works in `no_std` environments without `alloc`
- **Zero-copy parsing** - Efficient deserialization with minimal overhead
- **Compile-time filtering** - Only include messages for your specific hardware
- **Protocol verification** - Type-safe message construction with CRC16 validation
- **Feature-gated modules** - Small binary size by including only what you need

## Supported Hardware

- **ZT30** - Quad-sensor gimbal camera
- **ZT6** - Thermal imaging camera
- **ZR30** - Long-range camera system  
- **ZR10** - Standard gimbal camera
- **A8mini** - Compact gimbal camera
- **A2mini** - Entry-level gimbal camera

## Supported Protocols

- **TCP** - Network communication over TCP/IP
- **UDP** - Network communication over UDP
- **TTL** - Serial UART communication

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
siyi-protocol = "0.1"
```

### Selecting Your Configuration

Use feature flags to include only the messages your hardware supports. This significantly reduces code size and compilation time.

```toml
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "tcp"] }
```

For multiple protocols or cameras:

```toml
[dependencies]
siyi-protocol = { version = "0.1", features = ["zr10", "tcp", "udp"] }
```

To include everything (not recommended for embedded):

```toml
[dependencies]
siyi-protocol = { version = "0.1", features = ["all"] }
```

## Quick Start

### Serializing Messages

```rust
use siyi_protocol::zt30_tcp::*;

fn main() {
    // Create a request for firmware version
    let request = FirmwareVersionRequest::new();
    
    // Encode the message to a buffer
    let mut msg_buf = [0u8; MAX_MESSAGE_SIZE];
    let msg_len = request.encode(&mut msg_buf).unwrap();
    
    // Create a frame with the encoded message
    let mut frame_buf = [0u8; MAX_FRAME_SIZE];
    let frame = request.to_frame(&msg_buf[..msg_len]);
    let frame_len = frame.encode(&mut frame_buf).unwrap();
    
    // Send frame_buf[..frame_len] over your transport
    // socket.send(&frame_buf[..frame_len])?;
}
```

### Deserializing Messages

```rust
use siyi_protocol::zt30_tcp::*;

fn main() {
    // Receive data from your transport
    let mut recv_buf = [0u8; MAX_FRAME_SIZE];
    // let recv_len = socket.recv(&mut recv_buf)?;
    
    // Decode the frame
    let frame = Frame::decode(&recv_buf[..recv_len]).unwrap();
    
    // Decode the message from the frame
    let message = Message::from_frame(&frame).unwrap();
    
    // Handle the message
    match message {
        Message::FirmwareVersionResponse(resp) => {
            println!("Camera FW: {}", resp.camera_firmware_ver);
            println!("Gimbal FW: {}", resp.gimbal_firmware_ver);
        }
        Message::GimbalAttitudeResponse(resp) => {
            println!("Yaw: {:.1}°", resp.yaw as f32 / 10.0);
            println!("Pitch: {:.1}°", resp.pitch as f32 / 10.0);
        }
        _ => println!("Other message received"),
    }
}
```

## Protocol Overview

The SIYI protocol uses a binary frame format:

```
┌─────────┬──────┬─────────┬─────┬────────┬──────┬────────┐
│   STX   │ CTRL │ DATALEN │ SEQ │ CMD_ID │ DATA │  CRC16 │
│ 2 bytes │  1   │    2    │  2  │   1    │  N   │   2    │
└─────────┴──────┴─────────┴─────┴────────┴──────┴────────┘
```

- **STX**: Start marker (0x6655, little-endian)
- **CTRL**: Control flags (bit 0: need_ack, bit 1: is_ack)
- **DATALEN**: Payload length (little-endian)
- **SEQ**: Sequence number (little-endian)
- **CMD_ID**: Command identifier
- **DATA**: Message payload
- **CRC16**: Checksum using CRC16-CCITT

All multi-byte fields use little-endian byte order.

## Message Categories

### Camera Control
- `FirmwareVersionRequest/Response` - Get firmware versions
- `HardwareIdRequest/Response` - Get device hardware ID
- `AutoFocusRequest/Response` - Trigger autofocus
- `ManualZoomRequest/Response` - Control zoom level
- `AbsoluteZoomRequest/Response` - Set specific zoom level

### Gimbal Control
- `GimbalRotationRequest/Response` - Control gimbal movement
- `GimbalAttitudeRequest/Response` - Get current angles
- `SetGimbalAttitudeRequest/Response` - Set target angles
- `CenterGimbalRequest/Response` - Reset to center position
- `GimbalModeRequest/Response` - Get/set gimbal mode

### Recording and Photo
- `FunctionControl` - Take photo, start/stop recording
- `FunctionFeedback` - Camera feedback events
- `CameraSystemInfoRequest/Response` - Get system status

### Thermal Imaging (ZT30, ZT6)
- `GetTemperatureAtPointRequest/Response` - Point temperature
- `LocalTemperatureMeasurementRequest/Response` - Area temperature
- `GlobalTemperatureMeasurementRequest/Response` - Full frame
- `PseudoColorRequest/Response` - Get/set thermal palette
- `ThermalGainModeRequest/Response` - Configure gain mode

### Laser Ranging (ZT30, ZR10, ZR30)
- `LaserDistanceRequest/Response` - Get range measurement
- `LaserTargetLocationRequest/Response` - Get target GPS coordinates
- `SetLaserStateRequest/Response` - Enable/disable laser

### Video Configuration (TCP only)
- `EncodingParamsRequest/Response` - Get encoding settings
- `SetEncodingParamsRequest/Response` - Configure video encoding
- `VideoStitchingModeRequest/Response` - Multi-sensor stitching

### AI Features (ZT30, ZT6, A8mini)
- `AiModeStatusRequest/Response` - Check AI module status
- `AiTrackingCoordinateStream` - Real-time tracking data
- `SetAiTrackingStreamStatusRequest/Response` - Control tracking output

## Memory Requirements

The crate uses fixed-size buffers with compile-time known sizes. No heap allocation is required.

- **Message encoding buffer**: ~512 bytes (stack)
- **Frame encoding buffer**: ~522 bytes (stack)
- **Per-message overhead**: Varies by message type (typically 4-64 bytes)

All buffers are stack-allocated. The exact size depends on which messages you include via feature flags.

## Transport Layer Integration

This crate provides only message definitions and serialization. You need to implement your own transport layer:

### TCP Example

```rust
use std::net::TcpStream;
use std::io::{Read, Write};
use siyi_protocol::zt30_tcp::*;

let mut stream = TcpStream::connect("192.168.144.25:37260")?;

// Send request
let request = GimbalAttitudeRequest::new();
let mut msg_buf = [0u8; MAX_MESSAGE_SIZE];
let mut frame_buf = [0u8; MAX_FRAME_SIZE];

let msg_len = request.encode(&mut msg_buf)?;
let frame = request.to_frame(&msg_buf[..msg_len]);
let frame_len = frame.encode(&mut frame_buf)?;

stream.write_all(&frame_buf[..frame_len])?;

// Receive response
let mut recv_buf = [0u8; MAX_FRAME_SIZE];
let recv_len = stream.read(&mut recv_buf)?;

let frame = Frame::decode(&recv_buf[..recv_len])?;
let message = Message::from_frame(&frame)?;
```

### Serial (TTL) Example

```rust
use serialport::SerialPort;
use siyi_protocol::zr10_ttl::*;

let mut port = serialport::new("/dev/ttyUSB0", 115200)
    .timeout(Duration::from_millis(100))
    .open()?;

// Send and receive similar to TCP example
```

## Error Handling

The crate provides two error types:

```rust
pub enum EncodeError {
    BufferTooSmall,
}

pub enum DecodeError {
    FrameTooShort,
    InvalidStx,
    FrameIncomplete,
    CrcMismatch,
    NotEnoughBytes,
    InvalidEnumValue,
    ConversionError,
    UnknownCmdId,
}
```

Both implement `Debug`, `Clone`, `Copy`, `PartialEq`, and `Eq` for easy error handling.

## Code Generation

This crate is generated from an XML protocol definition using a Python script. The XML format allows:

- Single source of truth for the protocol
- Easy updates when SIYI releases new firmware
- Generation of implementations in other languages
- Clear documentation of protocol differences between camera models

To regenerate the code after modifying the XML:

```bash
./generate_all.sh
```

This creates optimized modules for each camera/protocol combination.

## Feature Flag Reference

### Protocol Features
- `tcp` - Enable TCP protocol messages
- `udp` - Enable UDP protocol messages  
- `ttl` - Enable serial (TTL) protocol messages

### Camera Features
- `zt30` - SIYI ZT30 camera support
- `zt6` - SIYI ZT6 camera support
- `zr30` - SIYI ZR30 camera support
- `zr10` - SIYI ZR10 camera support
- `a8mini` - SIYI A8mini camera support
- `a2mini` - SIYI A2mini camera support

### Combined Feature
- `all` - Enable all protocols and cameras (large binary size)

## Examples

See the `examples/` directory:

- `serialize_messages.rs` - Demonstrates message encoding
- `deserialize_messages.rs` - Demonstrates message decoding

Run examples with:

```bash
cargo run --example serialize_messages --features "zt30,tcp"
cargo run --example deserialize_messages --features "zt30,tcp"
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

Protocol specification based on the SIYI Gimbal Camera External SDK Protocol documentation.

## Contributing

Contributions are welcome! Please ensure:

1. Protocol changes are made in the XML file, not generated Rust code
2. Run `./generate_all.sh` after XML modifications
3. Test with real hardware when possible
4. Update documentation for any API changes

## Support

For protocol questions, refer to the official SIYI SDK documentation. For issues with this crate, please file an issue on the repository.