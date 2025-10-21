# Quick Start Guide

Get up and running with SIYI Protocol in 5 minutes.

## Prerequisites

- Rust 1.60 or later
- SIYI gimbal camera (ZT30, ZT6, ZR30, ZR10, A8mini, or A2mini)
- Network connection (TCP/UDP) or serial cable (TTL)

## Installation

Create a new Rust project:

```bash
cargo new siyi-demo
cd siyi-demo
```

Add the dependency with features for your hardware:

```toml
# Cargo.toml
[package]
name = "siyi-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "tcp"] }
```

## Your First Program

Create `src/main.rs`:

```rust
use siyi_protocol::zt30_tcp::*;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to camera
    println!("Connecting to camera...");
    let mut stream = TcpStream::connect("192.168.144.25:37260")?;
    stream.set_read_timeout(Some(Duration::from_secs(1)))?;
    
    // Request firmware version
    println!("Requesting firmware version...");
    let request = FirmwareVersionRequest::new();
    send_message(&mut stream, &request)?;
    
    // Receive response
    let response = receive_message(&mut stream)?;
    
    match response {
        Message::FirmwareVersionResponse(resp) => {
            println!("Camera Firmware: v{}", format_version(resp.camera_firmware_ver));
            println!("Gimbal Firmware: v{}", format_version(resp.gimbal_firmware_ver));
            if resp.zoom_firmware_ver != 0 {
                println!("Zoom Firmware: v{}", format_version(resp.zoom_firmware_ver));
            }
        }
        _ => println!("Unexpected response"),
    }
    
    Ok(())
}

fn send_message<T>(stream: &mut TcpStream, msg: &T) -> Result<(), Box<dyn std::error::Error>>
where
    T: /* your trait here */
{
    let mut msg_buf = [0u8; MAX_MESSAGE_SIZE];
    let mut frame_buf = [0u8; MAX_FRAME_SIZE];
    
    let msg_len = msg.encode(&mut msg_buf)?;
    let frame = msg.to_frame(&msg_buf[..msg_len]);
    let frame_len = frame.encode(&mut frame_buf)?;
    
    stream.write_all(&frame_buf[..frame_len])?;
    Ok(())
}

fn receive_message(stream: &mut TcpStream) -> Result<Message, Box<dyn std::error::Error>> {
    let mut buf = [0u8; MAX_FRAME_SIZE];
    let len = stream.read(&mut buf)?;
    
    let frame = Frame::decode(&buf[..len])?;
    let message = Message::from_frame(&frame)?;
    
    Ok(message)
}

fn format_version(ver: u32) -> String {
    let bytes = ver.to_le_bytes();
    format!("{}.{}.{}", bytes[2], bytes[1], bytes[0])
}
```

Run it:

```bash
cargo run
```

## Common Tasks

### Get Gimbal Attitude

```rust
use siyi_protocol::zt30_tcp::*;

fn get_attitude() -> Result<(), Box<dyn std::error::Error>> {
    let request = GimbalAttitudeRequest::new();
    send_message(&mut stream, &request)?;
    
    let response = receive_message(&mut stream)?;
    if let Message::GimbalAttitudeResponse(resp) = response {
        let yaw = resp.yaw as f32 / 10.0;
        let pitch = resp.pitch as f32 / 10.0;
        let roll = resp.roll as f32 / 10.0;
        
        println!("Yaw: {:.1}°", yaw);
        println!("Pitch: {:.1}°", pitch);
        println!("Roll: {:.1}°", roll);
    }
    
    Ok(())
}
```

### Control Gimbal Rotation

```rust
use siyi_protocol::zt30_tcp::*;

fn rotate_gimbal(yaw_speed: i8, pitch_speed: i8) -> Result<(), Box<dyn std::error::Error>> {
    let request = GimbalRotationRequest::new(yaw_speed, pitch_speed);
    send_message(&mut stream, &request)?;
    Ok(())
}

// Example: Rotate right and up
rotate_gimbal(50, 30)?;  // Speed range: -100 to +100
```

### Set Absolute Gimbal Position

```rust
use siyi_protocol::zt30_tcp::*;

fn set_position(yaw_deg: f32, pitch_deg: f32) -> Result<(), Box<dyn std::error::Error>> {
    // Convert degrees to protocol format (deg * 10)
    let yaw = (yaw_deg * 10.0) as i16;
    let pitch = (pitch_deg * 10.0) as i16;
    
    let request = SetGimbalAttitudeRequest::new(yaw, pitch);
    send_message(&mut stream, &request)?;
    
    Ok(())
}

// Example: Point to 45° yaw, -30° pitch
set_position(45.0, -30.0)?;
```

### Control Zoom

```rust
use siyi_protocol::zt30_tcp::*;

fn set_zoom(zoom_level: f32) -> Result<(), Box<dyn std::error::Error>> {
    // zoom_level: 1.0 to 30.0 (or camera's max)
    let zoom_int = zoom_level as u8;
    let zoom_float = ((zoom_level - zoom_int as f32) * 10.0) as u8;
    
    let request = AbsoluteZoomRequest::new(zoom_int, zoom_float);
    send_message(&mut stream, &request)?;
    
    Ok(())
}

// Example: Set zoom to 5.5x
set_zoom(5.5)?;
```

### Take Photo

```rust
use siyi_protocol::zt30_tcp::*;

fn take_photo() -> Result<(), Box<dyn std::error::Error>> {
    let request = FunctionControl::new(FunctionType::TakePhoto);
    send_message(&mut stream, &request)?;
    
    // Wait for feedback
    let response = receive_message(&mut stream)?;
    if let Message::FunctionFeedback(feedback) = response {
        match feedback.info_type {
            FeedbackInfoType::PhotoSuccess => println!("Photo captured!"),
            FeedbackInfoType::PhotoFailed => println!("Photo failed!"),
            _ => {}
        }
    }
    
    Ok(())
}
```

### Start/Stop Recording

```rust
use siyi_protocol::zt30_tcp::*;

fn toggle_recording() -> Result<(), Box<dyn std::error::Error>> {
    let request = FunctionControl::new(FunctionType::StartRecording);
    send_message(&mut stream, &request)?;
    
    // Wait for feedback
    let response = receive_message(&mut stream)?;
    if let Message::FunctionFeedback(feedback) = response {
        match feedback.info_type {
            FeedbackInfoType::RecordStarted => println!("Recording started"),
            FeedbackInfoType::RecordStopped => println!("Recording stopped"),
            FeedbackInfoType::RecordFailed => println!("Recording failed"),
            _ => {}
        }
    }
    
    Ok(())
}
```

### Get Laser Distance (ZT30, ZR10, ZR30)

```rust
use siyi_protocol::zt30_tcp::*;

fn get_laser_distance() -> Result<(), Box<dyn std::error::Error>> {
    let request = LaserDistanceRequest::new();
    send_message(&mut stream, &request)?;
    
    let response = receive_message(&mut stream)?;
    if let Message::LaserDistanceResponse(resp) = response {
        let distance_m = resp.laser_distance as f32 / 10.0;
        println!("Distance: {:.1} meters", distance_m);
    }
    
    Ok(())
}
```

### Get Temperature (ZT30, ZT6 Thermal Cameras)

```rust
use siyi_protocol::zt30_tcp::*;

fn get_point_temperature(x: u16, y: u16) -> Result<(), Box<dyn std::error::Error>> {
    let request = GetTemperatureAtPointRequest::new(
        x, 
        y, 
        TempMeasurementFlag::Once
    );
    send_message(&mut stream, &request)?;
    
    let response = receive_message(&mut stream)?;
    if let Message::GetTemperatureAtPointResponse(resp) = response {
        let temp_c = resp.temp as f32 / 100.0;
        println!("Temperature at ({}, {}): {:.2}°C", x, y, temp_c);
    }
    
    Ok(())
}
```

## Choosing Your Configuration

### By Camera Model

```toml
# ZT30 (Quad-sensor with thermal and laser)
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "tcp"] }

# ZT6 (Thermal camera)
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt6", "tcp"] }

# ZR10 (Standard gimbal with laser)
[dependencies]
siyi-protocol = { version = "0.1", features = ["zr10", "tcp"] }

# ZR30 (Long-range with laser)
[dependencies]
siyi-protocol = { version = "0.1", features = ["zr30", "tcp"] }

# A8mini (Compact with AI)
[dependencies]
siyi-protocol = { version = "0.1", features = ["a8mini", "tcp"] }

# A2mini (Entry-level)
[dependencies]
siyi-protocol = { version = "0.1", features = ["a2mini", "tcp"] }
```

### By Protocol Type

```toml
# TCP (most features, requires heartbeat)
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "tcp"] }

# UDP (lightweight, no guaranteed delivery)
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "udp"] }

# TTL (serial UART, 115200 baud)
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "ttl"] }
```

### Multiple Protocols

```toml
# Support both TCP and UDP
[dependencies]
siyi-protocol = { version = "0.1", features = ["zt30", "tcp", "udp"] }
```

## TCP Connection Tips

### Keep-Alive Heartbeat

TCP connections require periodic heartbeat messages:

```rust
use std::thread;
use std::time::Duration;

fn heartbeat_loop(mut stream: TcpStream) {
    thread::spawn(move || {
        loop {
            let heartbeat = TcpHeartbeat::new();
            if send_message(&mut stream, &heartbeat).is_err() {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}
```

### Connection Setup

```rust
use std::net::TcpStream;
use std::time::Duration;

fn connect_camera(ip: &str) -> Result<TcpStream, Box<dyn std::error::Error>> {
    let addr = format!("{}:37260", ip);
    let mut stream = TcpStream::connect(&addr)?;
    
    // Set timeouts
    stream.set_read_timeout(Some(Duration::from_millis(500)))?;
    stream.set_write_timeout(Some(Duration::from_millis(500)))?;
    
    // Disable Nagle's algorithm for low latency
    stream.set_nodelay(true)?;
    
    Ok(stream)
}
```

## UDP Communication

```rust
use std::net::UdpSocket;

fn udp_example() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("192.168.144.25:37260")?;
    
    // Send request
    let request = GimbalAttitudeRequest::new();
    let mut msg_buf = [0u8; MAX_MESSAGE_SIZE];
    let mut frame_buf = [0u8; MAX_FRAME_SIZE];
    
    let msg_len = request.encode(&mut msg_buf)?;
    let frame = request.to_frame(&msg_buf[..msg_len]);
    let frame_len = frame.encode(&mut frame_buf)?;
    
    socket.send(&frame_buf[..frame_len])?;
    
    // Receive response
    let mut recv_buf = [0u8; MAX_FRAME_SIZE];
    let (amt, _) = socket.recv_from(&mut recv_buf)?;
    
    let frame = Frame::decode(&recv_buf[..amt])?;
    let message = Message::from_frame(&frame)?;
    
    Ok(())
}
```

## Serial (TTL) Communication

```toml
# Add serialport dependency
[dependencies]
siyi-protocol = { version = "0.1", features = ["zr10", "ttl"] }
serialport = "4.2"
```

```rust
use serialport::SerialPort;
use std::time::Duration;
use siyi_protocol::zr10_ttl::*;

fn serial_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut port = serialport::new("/dev/ttyUSB0", 115200)
        .timeout(Duration::from_millis(100))
        .open()?;
    
    // Send request
    let request = FirmwareVersionRequest::new();
    let mut msg_buf = [0u8; MAX_MESSAGE_SIZE];
    let mut frame_buf = [0u8; MAX_FRAME_SIZE];
    
    let msg_len = request.encode(&mut msg_buf)?;
    let frame = request.to_frame(&msg_buf[..msg_len]);
    let frame_len = frame.encode(&mut frame_buf)?;
    
    port.write_all(&frame_buf[..frame_len])?;
    
    // Receive response
    let mut recv_buf = [0u8; MAX_FRAME_SIZE];
    let len = port.read(&mut recv_buf)?;
    
    let frame = Frame::decode(&recv_buf[..len])?;
    let message = Message::from_frame(&frame)?;
    
    Ok(())
}
```

## Error Handling

```rust
use siyi_protocol::zt30_tcp::*;

fn robust_send() -> Result<(), Box<dyn std::error::Error>> {
    let request = FirmwareVersionRequest::new();
    let mut msg_buf = [0u8; MAX_MESSAGE_SIZE];
    let mut frame_buf = [0u8; MAX_FRAME_SIZE];
    
    // Encode message
    let msg_len = match request.encode(&mut msg_buf) {
        Ok(len) => len,
        Err(EncodeError::BufferTooSmall) => {
            eprintln!("Buffer too small for message");
            return Err("Encode failed".into());
        }
    };
    
    // Encode frame
    let frame = request.to_frame(&msg_buf[..msg_len]);
    let frame_len = match frame.encode(&mut frame_buf) {
        Ok(len) => len,
        Err(EncodeError::BufferTooSmall) => {
            eprintln!("Buffer too small for frame");
            return Err("Frame encode failed".into());
        }
    };
    
    // Send over network
    stream.write_all(&frame_buf[..frame_len])?;
    
    Ok(())
}

fn robust_receive() -> Result<Message, Box<dyn std::error::Error>> {
    let mut buf = [0u8; MAX_FRAME_SIZE];
    let len = stream.read(&mut buf)?;
    
    // Decode frame
    let frame = match Frame::decode(&buf[..len]) {
        Ok(f) => f,
        Err(DecodeError::InvalidStx) => {
            eprintln!("Invalid start marker");
            return Err("Bad frame".into());
        }
        Err(DecodeError::CrcMismatch) => {
            eprintln!("CRC check failed");
            return Err("Corrupted frame".into());
        }
        Err(e) => {
            eprintln!("Decode error: {:?}", e);
            return Err("Decode failed".into());
        }
    };
    
    // Decode message
    let message = Message::from_frame(&frame)?;
    Ok(message)
}
```

## Testing Without Hardware

You can test message encoding/decoding without hardware:

```rust
use siyi_protocol::zt30_tcp::*;

fn test_encoding() {
    let request = FirmwareVersionRequest::new();
    let mut msg_buf = [0u8; MAX_MESSAGE_SIZE];
    let mut frame_buf = [0u8; MAX_FRAME_SIZE];
    
    // Encode
    let msg_len = request.encode(&mut msg_buf).unwrap();
    let frame = request.to_frame(&msg_buf[..msg_len]);
    let frame_len = frame.encode(&mut frame_buf).unwrap();
    
    println!("Encoded frame: {} bytes", frame_len);
    println!("Hex: {:02x?}", &frame_buf[..frame_len]);
    
    // Decode
    let decoded_frame = Frame::decode(&frame_buf[..frame_len]).unwrap();
    let decoded_msg = Message::from_frame(&decoded_frame).unwrap();
    
    println!("Decoded successfully: {:?}", decoded_msg);
}
```

## Next Steps

1. **Read the full README** - Detailed information about all features
2. **Check examples/** - Working examples for common tasks
3. **Review PROTOCOL.md** - Deep dive into protocol details
4. **Browse the API docs** - `cargo doc --open --features "all"`
5. **Join discussions** - Ask questions, share experiences

## Common Issues

### Camera Not Responding

1. Check IP address (default: 192.168.144.25)
2. Verify port (default: 37260)
3. Ensure camera is powered on
4. Check network connectivity
5. Try sending heartbeat first (TCP only)

### Compilation Errors

1. Verify correct features in Cargo.toml
2. Match camera model to your hardware
3. Check import paths (`use siyi_protocol::zt30_tcp::*;`)

### Wrong Data Format

1. Remember angle scaling (multiply by 10)
2. Remember temperature scaling (multiply by 100)
3. Check endianness (little-endian for all multi-byte values)

## Support

- GitHub Issues: Report bugs and request features
- GitHub Discussions: Ask questions and share experiences
- Examples: Check the `examples/` directory for more code

Happy coding!