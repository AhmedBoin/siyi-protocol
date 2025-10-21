# SIYI Protocol Specification

This document describes the SIYI Gimbal Camera communication protocol and the XML definition format used to generate type-safe implementations.

## Protocol Frame Format

The SIYI protocol uses a binary frame format for all communications:

```
Byte:   0   1   2   3   4   5   6   7     8...N     N+1 N+2
      ┌───┬───┬───┬───┬───┬───┬───┬───┬─────────┬─────┬─────┐
      │STX│STX│CTR│LEN│LEN│SEQ│SEQ│CMD│  DATA   │ CRC │ CRC │
      │ H │ L │   │ L │ H │ L │ H │ ID│         │  L  │  H  │
      └───┴───┴───┴───┴───┴───┴───┴───┴─────────┴─────┴─────┘
```

### Field Descriptions

| Field | Bytes | Type | Description |
|-------|-------|------|-------------|
| STX | 2 | uint16 | Start marker (0x6655), little-endian |
| CTRL | 1 | uint8 | Control byte (bit 0: need_ack, bit 1: is_ack) |
| DATALEN | 2 | uint16 | Length of DATA field, little-endian |
| SEQ | 2 | uint16 | Sequence number, little-endian |
| CMD_ID | 1 | uint8 | Command identifier |
| DATA | N | bytes | Message payload (N = DATALEN) |
| CRC16 | 2 | uint16 | CRC16-CCITT checksum, little-endian |

### Control Byte (CTRL)

The control byte contains two flags:

- **Bit 0 (need_ack)**: Set to 1 if acknowledgment is required
- **Bit 1 (is_ack)**: Set to 1 if this is a response/acknowledgment
- **Bits 2-7**: Reserved (set to 0)

```rust
// Request (need_ack=1, is_ack=0)
CTRL = 0x01

// Response (need_ack=0, is_ack=1)
CTRL = 0x02
```

### CRC16 Calculation

The protocol uses CRC16-CCITT with polynomial G(X) = X^16 + X^12 + X^5 + 1.

The CRC is calculated over all bytes from STX through the end of DATA (excluding the CRC itself).

```rust
// Pseudocode
crc = crc16_calc(&frame[0..8+datalen], 0);
```

## Message Structure

Each message consists of:
1. A command ID (CMD_ID byte in the frame)
2. A direction (request or response)
3. Zero or more fields containing the message data

## Protocol Differences by Transport

### TTL (Serial UART)

- Baud rate: 115200
- Data bits: 8
- Stop bits: 1
- Parity: None
- Flow control: None
- Full duplex communication
- Some messages not supported (see compatibility table)

### UDP

- Default port: 37260
- Connectionless protocol
- No guaranteed delivery
- Suitable for status updates and telemetry
- Some messages not supported (see compatibility table)

### TCP

- Default port: 37260
- Connection-oriented
- Guaranteed delivery
- Required for video encoding configuration
- Heartbeat message (0x00) required to keep connection alive
- All messages supported

## Camera Model Differences

Different camera models support different subsets of the protocol:

### ZT30 (Quad-Sensor Gimbal)
- Full thermal imaging support
- Laser ranging
- AI tracking
- Video stitching modes
- All zoom and focus controls

### ZT6 (Thermal Camera)
- Full thermal imaging support
- AI tracking
- Video stitching modes
- Basic gimbal control

### ZR30 (Long-Range Camera)
- Laser ranging
- Advanced zoom control
- No thermal imaging
- Limited AI features

### ZR10 (Standard Gimbal)
- Basic zoom and focus
- Laser ranging
- Standard gimbal control
- No thermal or AI features

### A8mini (Compact Gimbal)
- AI tracking support
- Basic zoom control
- Compact message set
- Network only (TCP/UDP)

### A2mini (Entry-Level)
- Basic gimbal control
- Simple zoom control
- Limited message support
- Network only (TCP/UDP)

## XML Definition Format

The protocol is defined in `siyi_protocol.xml` using the following structure:

### Root Element

```xml
<protocol name="SIYI_Gimbal_Camera_External_SDK_Protocol" 
          stx="0x6655" 
          stx_little="true" 
          crc="crc16">
```

Attributes:
- `name`: Protocol name
- `stx`: Start marker value
- `stx_little`: Use little-endian for STX (true/false)
- `crc`: Checksum algorithm (crc16)

### Camera Models

```xml
<camera_models>
  <model name="ZT30" />
  <model name="ZT6" />
  <model name="ZR10" />
  <model name="ZR30" />
  <model name="A2mini" />
  <model name="A8mini" />
</camera_models>
```

### Protocol Types

```xml
<protocol_types>
  <type name="TTL" description="Serial TTL interface" />
  <type name="UDP" description="UDP network interface" />
  <type name="TCP" description="TCP network interface" />
</protocol_types>
```

### Enumerations

```xml
<enum name="GimbalMode">
  <variant name="Lock" value="0" description="Lock mode"/>
  <variant name="Follow" value="1" description="Follow mode"/>
  <variant name="FPV" value="2" description="FPV mode"/>
</enum>
```

### Messages

```xml
<message name="FirmwareVersionRequest" 
         id="0x01" 
         direction="request"
         protocols="TTL,UDP,TCP"
         cameras="ZT30,ZT6,ZR10,ZR30,A2mini,A8mini"
         description="Request firmware version">
  <!-- No fields for this request -->
</message>

<message name="FirmwareVersionResponse" 
         id="0x01" 
         direction="response"
         protocols="TTL,UDP,TCP"
         cameras="ZT30,ZT6,ZR10,ZR30,A2mini,A8mini"
         description="Firmware version response">
  <field name="camera_firmware_ver" 
         type="uint32" 
         description="Camera firmware version"
         cameras="ZT30,ZT6,ZR10,ZR30,A2mini,A8mini"/>
  <field name="gimbal_firmware_ver" 
         type="uint32" 
         description="Gimbal firmware version"
         cameras="ZT30,ZT6,ZR10,ZR30,A2mini,A8mini"/>
  <field name="zoom_firmware_ver" 
         type="uint32" 
         description="Zoom module firmware version"
         cameras="ZT30,ZR10,ZR30"/>
</message>
```

Message attributes:
- `name`: Message struct name (PascalCase)
- `id`: Command ID (hex format)
- `direction`: "request" or "response"
- `protocols`: Comma-separated list of supported protocols
- `cameras`: Comma-separated list of supported cameras
- `description`: Human-readable description

Field attributes:
- `name`: Field name (snake_case)
- `type`: Data type (see below)
- `description`: Human-readable description
- `enum_type`: Enumeration type (for enum fields)
- `cameras`: Optional camera-specific support

### Supported Field Types

#### Primitive Types
- `uint8`, `int8` - 8-bit integer
- `uint16`, `int16` - 16-bit integer (little-endian)
- `uint32`, `int32` - 32-bit integer (little-endian)
- `uint64`, `int64` - 64-bit integer (little-endian)
- `float32` - 32-bit IEEE 754 float
- `float64` - 64-bit IEEE 754 float

#### Array Types
- `bytes[N]` - Fixed-size byte array of N bytes
- `bytes` - Variable-length byte array (uses 256-byte buffer)

#### Enumeration Types
- `enum` - Enumeration value (requires `enum_type` attribute)

## Common Message Patterns

### Request-Response Pattern

Most commands follow a request-response pattern:

```
Client                          Camera
  |                               |
  |----> FirmwareVersionRequest  |
  |                               |
  |<---- FirmwareVersionResponse |
  |                               |
```

### Command-Feedback Pattern

Some commands use feedback messages instead of direct responses:

```
Client                          Camera
  |                               |
  |----> FunctionControl         |
  |      (TakePhoto)              |
  |                               |
  |<---- FunctionFeedback        |
  |      (PhotoSuccess)           |
  |                               |
```

### Streaming Pattern

Some data is streamed continuously from the camera:

```
Client                          Camera
  |                               |
  |----> SetAiTrackingStream     |
  |      (Enable)                 |
  |                               |
  |<---- AiTrackingCoordinate    |
  |<---- AiTrackingCoordinate    |
  |<---- AiTrackingCoordinate    |
  |      ...                      |
```

## Message Categories

### System Information (0x01-0x02)
- Firmware version
- Hardware ID
- System time

### Focus and Zoom (0x04-0x06, 0x0F, 0x16, 0x18)
- Auto focus
- Manual focus
- Manual zoom
- Absolute zoom
- Zoom range query

### Gimbal Control (0x07-0x08, 0x0D-0x0E, 0x19, 0x41)
- Rotation control
- Attitude get/set
- Mode control
- Center position
- Single-axis control

### Camera Functions (0x0A-0x0C, 0x0B)
- System info
- Photo/video control
- Recording status
- Function feedback

### Video Configuration (0x10-0x11, 0x20-0x21)
- Stitching modes
- Encoding parameters
- Stream configuration

### Thermal Imaging (0x12-0x14, 0x1A-0x1B, 0x33-0x3C, 0x42-0x47, 0x4F)
- Temperature measurement
- Pseudo-color palettes
- Gain modes
- Environmental correction
- Threshold settings
- Shutter control

### Laser Ranging (0x15, 0x17, 0x31-0x32)
- Distance measurement
- Target location
- Laser enable/disable

### AI Features (0x4D-0x4E, 0x50-0x51)
- AI mode status
- Tracking coordinate stream
- Stream control

### Data Streams (0x22-0x2A)
- Aircraft attitude
- RC channels
- Encoder angles
- Motor voltage

### Configuration (0x30, 0x48-0x4C, 0x70-0x71, 0x80-0x82)
- UTC time
- SD card format
- File naming
- HDMI OSD
- Weak control mode
- Network settings
- Reboot

## Angle Representation

Angles in the protocol are typically represented as integers multiplied by 10:

```rust
// Example: 45.5 degrees
let angle_deg = 45.5;
let angle_protocol = (angle_deg * 10.0) as i16;  // 455

// Decoding
let received_value = 455i16;
let angle_deg = received_value as f32 / 10.0;  // 45.5
```

Common angle fields:
- Gimbal yaw, pitch, roll
- Target angles for positioning

## Temperature Representation

Temperature values are represented as integers multiplied by 100:

```rust
// Example: 25.37°C
let temp_celsius = 25.37;
let temp_protocol = (temp_celsius * 100.0) as u16;  // 2537

// Decoding
let received_value = 2537u16;
let temp_celsius = received_value as f32 / 100.0;  // 25.37
```

## Distance Representation

Laser distance is measured in decimeters (dm):

```rust
// Example: 150 meters
let distance_m = 150.0;
let distance_dm = (distance_m * 10.0) as u16;  // 1500

// Decoding
let received_value = 1500u16;
let distance_m = received_value as f32 / 10.0;  // 150.0
```

Minimum valid distance: 5.0 meters (50 dm)

## GPS Coordinates

GPS coordinates use degE7 format (degrees * 10^7):

```rust
// Example: 37.7749° N latitude
let lat_deg = 37.7749;
let lat_dege7 = (lat_deg * 1e7) as i32;  // 377749000

// Decoding
let received_value = 377749000i32;
let lat_deg = received_value as f64 / 1e7;  // 37.7749
```

## Error Handling

The protocol uses several error detection mechanisms:

1. **CRC16 validation** - Detects transmission errors
2. **Frame format validation** - Checks STX marker and length
3. **Enumeration validation** - Validates enum values
4. **Status codes** - Many responses include success/failure status

Common error responses:
- `BooleanStatus::Failed` (0) - Operation failed
- `BooleanStatus::Success` (1) - Operation succeeded

## Best Practices

### For TCP Communication
1. Send heartbeat (0x00) every 1-2 seconds
2. Wait for response before sending next command
3. Implement timeout handling (500ms recommended)
4. Reconnect on connection loss

### For UDP Communication
1. Implement retry logic for critical commands
2. Don't rely on delivery guarantees
3. Use for telemetry and status updates
4. Expect packet loss in poor network conditions

### For TTL Communication
1. Use 115200 baud rate
2. Implement frame synchronization
3. Handle partial frame reception
4. Check for buffer overruns

### General Guidelines
1. Validate CRC before processing messages
2. Check camera model compatibility before sending commands
3. Use sequence numbers to detect lost messages
4. Implement proper error recovery
5. Log protocol errors for debugging

## Code Generation

The Python generator script reads the XML and produces:

1. Enumeration definitions
2. Message structure definitions
3. Encode/decode implementations
4. CRC16 calculation
5. Frame parsing logic
6. Helper functions
7. Documentation comments
8. Unit tests

Generated code is optimized for:
- Zero heap allocation
- Compile-time feature gating
- Type safety
- Embedded systems compatibility

## Version History

The protocol has evolved across firmware versions. Check the SIYI documentation for:
- Supported firmware versions
- Protocol changes between versions
- Deprecated commands
- New feature additions

## References

- SIYI SDK Protocol Documentation
- SIYI Gimbal Camera User Manuals
- CRC16-CCITT Standard
- IEEE 754 Floating Point Standard