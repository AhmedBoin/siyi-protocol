// // This shows how to:
// // 1. Connect to the gimbal (TCP/UDP)
// // 2. Send requests and receive responses
// // 3. Handle different message types
// // 4. Manage sequence numbers

// use std::io::{Read, Write};
// use std::net::{TcpStream, UdpSocket};
// use std::time::Duration;

// // Include the generated protocol module
// pub mod siyi_protocol;
// use siyi_protocol::*;

// /// SIYI connection configuration
// pub struct SiyiConfig {
//     pub ip: String,
//     pub port: u16,
//     pub timeout: Duration,
// }

// impl Default for SiyiConfig {
//     fn default() -> Self {
//         Self {
//             ip: "192.168.144.25".to_string(),
//             port: 37260,
//             timeout: Duration::from_secs(5),
//         }
//     }
// }

// /// SIYI gimbal client (TCP mode)
// pub struct SiyiClient {
//     stream: TcpStream,
//     seq: u16,
// }

// impl SiyiClient {
//     /// Connect to gimbal via TCP
//     pub fn connect(config: &SiyiConfig) -> Result<Self, String> {
//         let addr = format!("{}:{}", config.ip, config.port);
//         let stream = TcpStream::connect(&addr).map_err(|e| format!("Connection failed: {}", e))?;

//         stream
//             .set_read_timeout(Some(config.timeout))
//             .map_err(|e| format!("Failed to set timeout: {}", e))?;

//         Ok(Self { stream, seq: 0 })
//     }

//     /// Send heartbeat (TCP only)
//     pub fn send_heartbeat(&mut self) -> Result<(), String> {
//         let frame = Frame::new(TcpHeartbeat::CMD_ID, vec![], false);
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Failed to send heartbeat: {}", e))?;

//         Ok(())
//     }

//     /// Get next sequence number
//     fn next_seq(&mut self) -> u16 {
//         let seq = self.seq;
//         self.seq = self.seq.wrapping_add(1);
//         seq
//     }

//     /// Send a request and wait for response
//     pub fn send_request(
//         &mut self,
//         cmd_id: u8,
//         data: Vec<u8>,
//     ) -> Result<SetGimbalAttitudeResponse, String> {
//         let seq = self.next_seq();
//         let mut frame = Frame::new(cmd_id, data, true);
//         frame.seq = seq;

//         let encoded = frame.encode();
//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         // Read response
//         let mut buf = vec![0u8; 1024];
//         let n = self
//             .stream
//             .read(&mut buf)
//             .map_err(|e| format!("Read failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         match Message::from_frame(&response_frame)? {
//             Message::SetGimbalAttitudeResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }

//     /// Get gimbal attitude
//     pub fn get_gimbal_attitude(&mut self) -> Result<GimbalAttitudeResponse, String> {
//         let req = GimbalAttitudeRequest::new();
//         let frame = req.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let n = self
//             .stream
//             .read(&mut buf)
//             .map_err(|e| format!("Read failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         match Message::from_frame(&response_frame)? {
//             Message::GimbalAttitudeResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }

//     /// Take a photo
//     pub fn take_photo(&mut self) -> Result<(), String> {
//         let msg = FunctionControl {
//             func_type: FunctionType::TakePhoto,
//         };
//         let frame = msg.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         // FunctionControl has no ACK, but we might receive FunctionFeedback later
//         Ok(())
//     }

//     /// Start/stop video recording
//     pub fn toggle_recording(&mut self) -> Result<(), String> {
//         let msg = FunctionControl {
//             func_type: FunctionType::StartRecording,
//         };
//         let frame = msg.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         Ok(())
//     }

//     /// Set gimbal mode
//     pub fn set_gimbal_mode(&mut self, mode: GimbalMode) -> Result<(), String> {
//         let func_type = match mode {
//             GimbalMode::Lock => FunctionType::LockMode,
//             GimbalMode::Follow => FunctionType::FollowMode,
//             GimbalMode::FPV => FunctionType::FPVMode,
//         };

//         let msg = FunctionControl {
//             func_type: func_type,
//         };
//         let frame = msg.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         Ok(())
//     }

//     /// Zoom control
//     pub fn zoom_control(&mut self, zoom: i8) -> Result<ManualZoomResponse, String> {
//         let req = ManualZoomRequest { zoom };
//         let frame = req.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let n = self
//             .stream
//             .read(&mut buf)
//             .map_err(|e| format!("Read failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         match Message::from_frame(&response_frame)? {
//             Message::ManualZoomResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }

//     /// Set absolute zoom level
//     pub fn set_zoom_level(&mut self, zoom_level: f32) -> Result<AbsoluteZoomResponse, String> {
//         let zoom_int = zoom_level as u8;
//         let zoom_float = ((zoom_level - zoom_int as f32) * 10.0) as u8;

//         let req = AbsoluteZoomRequest {
//             zoom_int,
//             zoom_float,
//         };
//         let frame = req.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let n = self
//             .stream
//             .read(&mut buf)
//             .map_err(|e| format!("Read failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         match Message::from_frame(&response_frame)? {
//             Message::AbsoluteZoomResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }

//     /// Get laser distance
//     pub fn get_laser_distance(&mut self) -> Result<LaserDistanceResponse, String> {
//         let req = LaserDistanceRequest::new();
//         let frame = req.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let n = self
//             .stream
//             .read(&mut buf)
//             .map_err(|e| format!("Read failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         match Message::from_frame(&response_frame)? {
//             Message::LaserDistanceResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }

//     /// Center gimbal
//     pub fn center_gimbal(
//         &mut self,
//         center_pos: CenterPosition,
//     ) -> Result<CenterGimbalResponse, String> {
//         let req = CenterGimbalRequest {
//             center_pos: center_pos,
//         };
//         let frame = req.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let n = self
//             .stream
//             .read(&mut buf)
//             .map_err(|e| format!("Read failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         match Message::from_frame(&response_frame)? {
//             Message::CenterGimbalResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }

//     /// Request data stream (attitude, laser, etc.)
//     pub fn request_data_stream(
//         &mut self,
//         data_type: DataStreamType,
//         freq: DataFrequency,
//     ) -> Result<RequestDataStreamResponse, String> {
//         let req = RequestDataStreamRequest {
//             data_type: data_type,
//             data_freq: freq,
//         };
//         let frame = req.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let n = self
//             .stream
//             .read(&mut buf)
//             .map_err(|e| format!("Read failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         match Message::from_frame(&response_frame)? {
//             Message::RequestDataStreamResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }

//     /// Get camera system info
//     pub fn get_camera_system_info(&mut self) -> Result<CameraSystemInfoResponse, String> {
//         let req = CameraSystemInfoRequest::new();
//         let frame = req.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let n = self
//             .stream
//             .read(&mut buf)
//             .map_err(|e| format!("Read failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         match Message::from_frame(&response_frame)? {
//             Message::CameraSystemInfoResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }

//     /// Request firmware version
//     pub fn get_firmware_version(&mut self) -> Result<FirmwareVersionResponse, String> {
//         let req = FirmwareVersionRequest::new();
//         let frame = req.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let n = self
//             .stream
//             .read(&mut buf)
//             .map_err(|e| format!("Read failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         match Message::from_frame(&response_frame)? {
//             Message::FirmwareVersionResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }

//     /// Request hardware ID
//     pub fn get_hardware_id(&mut self) -> Result<HardwareIdResponse, String> {
//         let req = HardwareIdRequest::new();
//         let frame = req.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let n = self
//             .stream
//             .read(&mut buf)
//             .map_err(|e| format!("Read failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         match Message::from_frame(&response_frame)? {
//             Message::HardwareIdResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }

//     /// Control gimbal rotation
//     pub fn control_gimbal_rotation(
//         &mut self,
//         yaw: i8,
//         pitch: i8,
//     ) -> Result<GimbalRotationResponse, String> {
//         let req = GimbalRotationRequest { yaw, pitch };
//         let frame = req.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let n = self
//             .stream
//             .read(&mut buf)
//             .map_err(|e| format!("Read failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         match Message::from_frame(&response_frame)? {
//             Message::GimbalRotationResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }

//     /// Set gimbal attitude (angles)
//     pub fn set_gimbal_attitude(
//         &mut self,
//         yaw: f32,
//         pitch: f32,
//     ) -> Result<SetGimbalAttitudeResponse, String> {
//         // Convert angles to protocol format (multiply by 10)
//         let yaw_i16 = (yaw * 10.0) as i16;
//         let pitch_i16 = (pitch * 10.0) as i16;

//         let req = SetGimbalAttitudeRequest {
//             yaw: yaw_i16,
//             pitch: pitch_i16,
//         };
//         let frame = req.to_frame();
//         let encoded = frame.encode();

//         self.stream
//             .write_all(&encoded)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let n = self
//             .stream
//             .read(&mut buf)
//             .map_err(|e| format!("Read failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         // Verify sequence number
//         if response_frame.seq != self.seq {
//             return Err(format!(
//                 "Sequence mismatch: sent {}, got {}",
//                 self.seq, response_frame.seq
//             ));
//         }

//         // SetGimbalAttitudeResponse::from_frame(&response_frame)
//         match Message::from_frame(&response_frame)? {
//             Message::SetGimbalAttitudeResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }
// }

// /// SIYI gimbal client (UDP mode)
// pub struct SiyiClientUdp {
//     socket: UdpSocket,
//     remote_addr: String,
//     seq: u16,
// }

// impl SiyiClientUdp {
//     /// Connect to gimbal via UDP
//     pub fn connect(config: &SiyiConfig) -> Result<Self, String> {
//         let socket = UdpSocket::bind("0.0.0.0:0")
//             .map_err(|e| format!("Failed to bind UDP socket: {}", e))?;

//         socket
//             .set_read_timeout(Some(config.timeout))
//             .map_err(|e| format!("Failed to set timeout: {}", e))?;

//         let remote_addr = format!("{}:{}", config.ip, config.port);

//         Ok(Self {
//             socket,
//             remote_addr,
//             seq: 0,
//         })
//     }

//     fn next_seq(&mut self) -> u16 {
//         let seq = self.seq;
//         self.seq = self.seq.wrapping_add(1);
//         seq
//     }

//     /// Send request and wait for response
//     pub fn send_request(&mut self, cmd_id: u8, data: Vec<u8>) -> Result<Message, String> {
//         let seq = self.next_seq();
//         let mut frame = Frame::new(cmd_id, data, true);
//         frame.seq = seq;

//         let encoded = frame.encode();
//         self.socket
//             .send_to(&encoded, &self.remote_addr)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let (n, _) = self
//             .socket
//             .recv_from(&mut buf)
//             .map_err(|e| format!("Receive failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         if response_frame.seq != seq {
//             return Err(format!(
//                 "Sequence mismatch: expected {}, got {}",
//                 seq, response_frame.seq
//             ));
//         }

//         Message::from_frame(&response_frame)
//     }

//     /// Get firmware version (UDP)
//     pub fn get_firmware_version(&mut self) -> Result<FirmwareVersionResponse, String> {
//         let req = FirmwareVersionRequest::new();
//         let frame = req.to_frame();
//         let encoded = frame.encode();

//         self.socket
//             .send_to(&encoded, &self.remote_addr)
//             .map_err(|e| format!("Send failed: {}", e))?;

//         let mut buf = vec![0u8; 1024];
//         let (n, _) = self
//             .socket
//             .recv_from(&mut buf)
//             .map_err(|e| format!("Receive failed: {}", e))?;

//         let response_frame = Frame::decode(&buf[..n])?;

//         match Message::from_frame(&response_frame)? {
//             Message::FirmwareVersionResponse(resp) => Ok(resp),
//             _ => Err("Unexpected response type".into()),
//         }
//     }
// }

// // ============================================================================
// // Additional Helper Functions
// // ============================================================================

// /// Parse firmware version into human-readable format
// pub fn parse_firmware_version(version: u32) -> String {
//     let major = (version >> 16) & 0xFF;
//     let minor = (version >> 8) & 0xFF;
//     let patch = version & 0xFF;
//     format!("v{}.{}.{}", major, minor, patch)
// }

// /// Convert angle from protocol format (int16 * 10) to degrees
// pub fn protocol_angle_to_degrees(angle: i16) -> f32 {
//     angle as f32 / 10.0
// }

// /// Convert degrees to protocol format
// pub fn degrees_to_protocol_angle(degrees: f32) -> i16 {
//     (degrees * 10.0) as i16
// }

// /// Convert laser distance from decimeters to meters
// pub fn laser_distance_to_meters(distance: u16) -> f32 {
//     distance as f32 / 10.0
// }

// /// Convert temperature from protocol format to Celsius
// pub fn protocol_temp_to_celsius(temp: u16) -> f32 {
//     temp as f32 / 100.0
// }

// #[cfg(test)]
// mod test {
//     // ============================================================================
//     // Example Usage
//     // ============================================================================
//     #[test]
//     fn main() -> Result<(), String> {
//         use super::*;

//         println!("SIYI Gimbal Protocol Example\n");

//         // Configuration
//         let config = SiyiConfig::default();

//         // Connect via TCP
//         println!("Connecting to gimbal at {}:{}...", config.ip, config.port);
//         let mut client = SiyiClient::connect(&config)?;
//         println!("✓ Connected!\n");

//         // Get firmware version
//         println!("Requesting firmware version...");
//         let fw_version = client.get_firmware_version()?;
//         println!("Camera FW: 0x{:08X}", fw_version.camera_firmware_ver);
//         println!("Gimbal FW: 0x{:08X}", fw_version.gimbal_firmware_ver);
//         println!("Zoom FW:   0x{:08X}\n", fw_version.zoom_firmware_ver);

//         // Get hardware ID
//         println!("Requesting hardware ID...");
//         let hw_id = client.get_hardware_id()?;
//         println!("Hardware ID: {:?}\n", hw_id.hardware_id);

//         // Get gimbal attitude
//         println!("Getting gimbal attitude...");
//         let attitude = client.get_gimbal_attitude()?;
//         println!("Yaw:   {:.1}°", attitude.yaw as f32 / 10.0);
//         println!("Pitch: {:.1}°", attitude.pitch as f32 / 10.0);
//         println!("Roll:  {:.1}°\n", attitude.roll as f32 / 10.0);

//         // Center gimbal
//         println!("Centering gimbal...");
//         let result = client.center_gimbal(CenterPosition::CenterOnly)?;
//         if let BooleanStatus::Success = result.status {
//             println!("✓ Gimbal centered\n");
//         }

//         // Set gimbal to specific angle
//         println!("Setting gimbal to 0° yaw, -45° pitch...");
//         let set_result = client.set_gimbal_attitude(0.0, -45.0)?;
//         println!("Current position after set:");
//         println!("  Yaw:   {:.1}°", set_result.yaw as f32 / 10.0);
//         println!("  Pitch: {:.1}°", set_result.pitch as f32 / 10.0);
//         println!("  Roll:  {:.1}°\n", set_result.roll as f32 / 10.0);

//         // Get camera system info
//         println!("Getting camera system info...");
//         let sys_info = client.get_camera_system_info()?;
//         println!("Recording: {:?}", sys_info.record_status);
//         println!("Gimbal mode: {:?}", sys_info.gimbal_motion_mode);
//         println!("Mounting: {:?}", sys_info.gimbal_mounting_dir);

//         // Zoom control
//         println!("\nZoom in...");
//         let zoom_result = client.zoom_control(1)?;
//         println!(
//             "Current zoom: {:.1}x",
//             zoom_result.zoom_multiple as f32 / 10.0
//         );

//         std::thread::sleep(std::time::Duration::from_secs(1));

//         println!("Stop zoom...");
//         client.zoom_control(0)?;

//         // Request attitude data stream at 10Hz
//         println!("\nRequesting attitude data stream at 10Hz...");
//         client.request_data_stream(DataStreamType::Attitude, DataFrequency::Hz10)?;
//         println!("✓ Data stream configured");

//         // Send heartbeat (TCP only)
//         println!("\nSending TCP heartbeat...");
//         client.send_heartbeat()?;
//         println!("✓ Heartbeat sent");

//         println!("\n✓ All operations completed successfully!");

//         Ok(())
//     }
// }

pub mod siyi_protocol;
