# SIYI Protocol Code Generator

A protocol code generator that converts SIYI Gimbal Camera SDK protocol definitions from XML to type-safe Rust code. The generator produces bare-metal compatible code with no heap allocations, making it suitable for embedded systems, RTOS environments, and standard applications.

## Overview

This tool reads the SIYI protocol specification from an XML file and generates Rust message structures, serialization/deserialization logic, and a byte-by-byte state machine parser for serial communication.

### Design Philosophy

The protocol definition is maintained in XML format to:
- Enable easy updates when SIYI releases protocol changes
- Allow generation of implementations in other programming languages
- Provide a single source of truth for the protocol specification
- Separate protocol definition from implementation details

## Features

### Generated Code Characteristics

- **No heap allocation**: Works without `alloc` or `std`
- **State machine parser**: Byte-by-byte parsing for UART/Serial
- **CRC16 validation**: Built-in checksum verification
- **Type-safe enumerations**: Protocol enums with validation
- **Fixed-size buffers**: Compile-time known memory requirements

### Protocol Format

The SIYI protocol uses the following frame structure:

```
+--------+------+---------+-----+--------+------+---------+-------+
| STX    | CTRL | DATALEN | SEQ | CMD_ID | DATA | CRC16   |       |
| 2 bytes| 1    | 2       | 2   | 1      | N    | 2 bytes | Total |
+--------+------+---------+-----+--------+------+---------+-------+
```

- **STX**: Start marker (0x6655, little-endian)
- **CTRL**: Control byte (bit 0: need_ack, bit 1: is_ack)
- **DATALEN**: Data payload length (little-endian)
- **SEQ**: Frame sequence number (little-endian)
- **CMD_ID**: Command identifier
- **DATA**: Message payload
- **CRC16**: Checksum (little-endian)

## Usage

### Basic Generation

Generate bare-metal compatible code (default):

```bash
python gen_rust_from_xml_nostd.py siyi_protocol.xml -o protocol.rs
```

Generate with standard library support:

```bash
python gen_rust_from_xml_nostd.py siyi_protocol.xml --std -o protocol.rs
```

### Command Line Options

```
usage: gen_rust_from_xml3_nostd_stateful.py [-h] [-o OUTPUT] [--std] xml_file

positional arguments:
  xml_file              Protocol XML file

optional arguments:
  -h, --help            show this help message and exit
  -o OUTPUT, --output OUTPUT
                        Output file path (stdout if not specified)
  --std                 Generate with std support (default: no_std)
```

## Integration

The generated file contains message definitions and protocol utilities. You'll need to integrate it with your own transport layer and application logic.

### Example: Serial/UART Communication

```rust
use protocol::*;

let mut parser = FrameParser::new();
let mut uart = /* your UART peripheral */;

loop {
    if let Some(byte) = uart.read_byte() {
        match parser.feed(byte) {
            Ok(Some(frame)) => {
                // Complete frame received
                match Message::decode(&frame) {
                    Ok(msg) => handle_message(msg),
                    Err(e) => log_error(e),
                }
            }
            Ok(None) => {
                // Still receiving frame
            }
            Err(e) => {
                // Frame error, parser automatically resets
                log_error(e);
            }
        }
    }
}
```

### Example: UDP/TCP Communication

```rust
use protocol::*;

let mut socket = /* your socket */;
let mut buffer = [0u8; MAX_FRAME_SIZE];

loop {
    let len = socket.recv(&mut buffer)?;
    
    match bytes_to_message(&buffer[..len]) {
        Ok(message) => {
            // Process complete message
            match message {
                Message::FirmwareVersionRequest(_) => send_version(),
                Message::GimbalAttitudeResponse(resp) => {
                    println!("Yaw: {}, Pitch: {}", resp.yaw, resp.pitch);
                }
                _ => {}
            }
        }
        Err(e) => eprintln!("Decode error: {:?}", e),
    }
}
```

### Helper Functions

The generator provides four main helper functions:

```rust
// Serialize frame to bytes (adds STX and CRC16)
pub fn frame_to_bytes(frame: &Frame, buf: &mut [u8]) -> Result<usize, EncodeError>

// Deserialize bytes to frame (validates STX and CRC16)
pub fn bytes_to_frame(data: &[u8]) -> Result<Frame, DecodeError>

// Serialize message to complete frame bytes
pub fn message_to_bytes(msg: &Message, buf: &mut [u8]) -> Result<usize, EncodeError>

// Deserialize bytes to message (validates and decodes)
pub fn bytes_to_message(data: &[u8]) -> Result<Message, DecodeError>
```

## XML Protocol Definition

### Structure

The XML file defines the protocol using these elements:

```xml
<protocol name="SIYI_Protocol" stx="0x6655" stx_little="true" crc="crc16">
  <enum name="GimbalMode">
    <variant name="Lock" value="0" description="Lock mode"/>
    <variant name="Follow" value="1" description="Follow mode"/>
  </enum>
  
  <message name="FirmwareVersionRequest" id="0x01" direction="request">
    <!-- No fields for this request -->
  </message>
  
  <message name="FirmwareVersionResponse" id="0x01" direction="response">
    <field name="camera_firmware_ver" type="uint32" description="Camera firmware version"/>
    <field name="gimbal_firmware_ver" type="uint32" description="Gimbal firmware version"/>
  </message>
</protocol>
```

### Supported Field Types

- Primitives: `uint8`, `int8`, `uint16`, `int16`, `uint32`, `int32`, `uint64`, `int64`, `float32`, `float64`
- Fixed arrays: `bytes[N]` where N is the array size
- Variable data: `bytes` (generates fixed 256-byte buffer with length field)
- Enumerations: `enum` with `enum_type` attribute

### Adding New Messages

1. Open `siyi_protocol.xml`
2. Add your message definition:

```xml
<message name="MyNewRequest" id="0xAB" direction="request">
  <field name="param1" type="uint16" description="First parameter"/>
  <field name="param2" type="int32" description="Second parameter"/>
</message>

<message name="MyNewResponse" id="0xAB" direction="response">
  <field name="result" type="uint8" description="Result code"/>
</message>
```

3. Regenerate the Rust code:

```bash
python gen_rust_from_xml_nostd.py siyi_protocol.xml -o siyi_protocol.rs
```

## Memory Requirements

The generated code uses fixed-size buffers with compile-time known sizes:

- `MAX_MESSAGE_SIZE`: Calculated from largest message in protocol (default: 512 bytes)
- `MAX_FRAME_SIZE`: `MAX_MESSAGE_SIZE + 10` bytes for frame overhead

Stack usage per operation:
- Frame encoding: ~522 bytes
- Frame parsing: ~522 bytes (state machine buffer)
- Message structures: Varies by message type

No heap allocation is required. All operations use stack-allocated buffers.

## State Machine Parser

The byte-by-byte parser handles common serial communication issues:

- **Shifted frames**: Correctly resyncs when STX bytes are misaligned
- **Partial frames**: Accumulates bytes until complete frame received
- **CRC validation**: Automatically validates and rejects corrupted frames
- **Auto-reset**: Resets state after frame completion or errors

### Parser States

The parser transitions through these states:

1. `Stx1`: Waiting for first STX byte
2. `Stx2`: Waiting for second STX byte
3. `Ctrl`: Reading control byte
4. `Len1/Len2`: Reading data length
5. `Seq1/Seq2`: Reading sequence number
6. `Cmd`: Reading command ID
7. `Data`: Accumulating payload bytes
8. `Crc1/Crc2`: Reading and validating checksum

## Extending to Other Languages

The XML-based approach allows generation of protocol implementations in other languages:

1. Create a new generator script (e.g., `gen_cpp_from_xml.py`, `gen_python_from_xml.py`)
2. Parse the same `siyi_protocol.xml` file
3. Generate language-specific structures and serialization code
4. Implement the same state machine logic for serial communication

The protocol definition remains unchanged across all language implementations.

## Requirements

- Python 3.6 or higher (for code generation)
- Rust 1.60 or higher (for generated code)

No external Python dependencies required. The generator uses only standard library modules.

## License

This tool is provided as-is for use with SIYI gimbal camera systems. Refer to SIYI's official documentation for protocol specifications and licensing terms.

## Related Documentation

- SIYI SDK Protocol Documentation
- SIYI Gimbal Camera User Manual
- CRC16 CCITT Standard (G(X) = X^16 + X^12 + X^5 + 1)

## Notes

The generated code is the message definition layer. You are responsible for implementing:
- Transport layer (UART, UDP, TCP)
- Connection management
- Message routing and handling
- Application-specific logic
- Error recovery strategies

The state machine parser handles frame-level parsing but does not manage connections or implement retry logic.