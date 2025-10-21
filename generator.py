#!/usr/bin/env python3
"""
SIYI Protocol XML to Rust Code Generator (V4 - Lifetime-Free)
Converts XML protocol definitions to bare-metal compatible Rust code with
NO LIFETIMES. All data is owned in fixed-size buffers.
"""
import xml.etree.ElementTree as ET
import sys
from typing import List, Tuple, Optional, Dict, Set
from collections import defaultdict
import argparse

PRIMITIVE_TYPES = {
    'uint8': ('u8', 1),
    'int8': ('i8', 1),
    'uint16': ('u16', 2),
    'int16': ('i16', 2),
    'uint32': ('u32', 4),
    'int32': ('i32', 4),
    'uint64': ('u64', 8),
    'int64': ('i64', 8),
    'float32': ('f32', 4),
    'float64': ('f64', 8),
}

def parse_camera_models(root):
    """Parse available camera models from XML"""
    models = []
    camera_models = root.find("camera_models")
    if camera_models is not None:
        for model in camera_models.findall("model"):
            models.append(model.attrib['name'])
    return models

def parse_protocol_types(root):
    """Parse available protocol types from XML"""
    protocols = []
    protocol_types = root.find("protocol_types")
    if protocol_types is not None:
        for ptype in protocol_types.findall("type"):
            protocols.append(ptype.attrib['name'])
    return protocols

def parse_list_attr(attr_str: Optional[str]) -> List[str]:
    """Parse comma-separated attribute into list"""
    if not attr_str:
        return []
    return [s.strip() for s in attr_str.split(',')]

def calculate_max_message_size(all_messages):
    """Calculate the maximum possible message size"""
    max_size = 0
    for msg_data in all_messages:
        _name, _cmd_id, _direction, _msg_desc, fields, _protocols, _cameras = msg_data
        size = 0
        for field_data in fields:
            _fname, ftype, _enum_type, _fdesc, _field_cameras = field_data
            if ftype in PRIMITIVE_TYPES:
                size += PRIMITIVE_TYPES[ftype][1]
            elif ftype.startswith("bytes["):
                arr_size = int(ftype[6:-1])
                size += arr_size
            elif ftype == 'enum':
                size += 1
            else:
                size += 256
        max_size = max(max_size, size)
    return max(max_size, 512)

def parse_enum(e) -> Tuple[str, List[Tuple[str, int, str]]]:
    """Parse enum element"""
    name = e.attrib['name']
    variants = []
    for v in e.findall("variant"):
        vname = v.attrib['name']
        value = int(v.attrib['value'], 0)
        desc = v.attrib.get('description', '')
        variants.append((vname, value, desc))
    return name, variants

def parse_message(m, all_cameras: List[str]) -> Tuple:
    """Parse message element with protocol and camera filtering"""
    name = m.attrib['name']
    cmd_id = int(m.attrib['id'], 0)
    direction = m.attrib.get('direction', 'request')
    msg_desc = m.attrib.get('description', '')
    
    protocols = parse_list_attr(m.attrib.get('protocols'))
    cameras = parse_list_attr(m.attrib.get('cameras'))
    
    if not protocols: protocols = ['TTL', 'UDP', 'TCP']
    if not cameras: cameras = all_cameras.copy()
    
    fields = []
    for f in m.findall("field"):
        fname = f.attrib['name']
        ftype = f.attrib['type']
        fdesc = f.attrib.get('description', '')
        fenum_type = f.attrib.get('enum_type', None)
        
        field_cameras = parse_list_attr(f.attrib.get('cameras'))
        if not field_cameras: field_cameras = cameras.copy()
        
        # If the type is 'bytes', we create two fields: the buffer and its length
        if ftype == "bytes":
            fields.append((fname, f"[u8; MAX_MESSAGE_SIZE]", None, fdesc, field_cameras))
            fields.append((f"{fname}_len", "u16", None, f"Length of {fname}", field_cameras))
        else:
            fields.append((fname, ftype, fenum_type, fdesc, field_cameras))
    
    return name, cmd_id, direction, msg_desc, fields, protocols, cameras

def get_rust_type(ftype: str, enum_type: Optional[str]) -> str:
    """Get Rust type for a field"""
    if ftype == 'enum':
        return enum_type if enum_type else 'u8'
    elif ftype in PRIMITIVE_TYPES:
        return PRIMITIVE_TYPES[ftype][0]
    elif ftype.startswith("[u8;"): # Handle our custom bytes type
        return ftype
    elif ftype.startswith("bytes["):
        size = ftype[6:-1]
        return f"[u8; {size}]"
    else:
        return ftype # Should handle u16 etc.

def generate_field_encode(fname: str, ftype: str, enum_type: Optional[str], indent: str = "        ") -> List[str]:
    """Generate encoding code for a field"""
    lines = []
    # Skip explicit length fields, they are handled by their parent buffer
    if fname.endswith("_len"):
        return []

    if ftype.startswith("[u8; MAX_MESSAGE_SIZE]"):
        lines.append(f"{indent}let len = self.{fname}_len as usize;")
        lines.append(f"{indent}if idx + len > buf.len() {{ return Err(EncodeError::BufferTooSmall); }}")
        lines.append(f"{indent}buf[idx..idx+len].copy_from_slice(&self.{fname}[..len]);")
        lines.append(f"{indent}idx += len;")
    elif ftype == 'enum':
        lines.append(f"{indent}if idx >= buf.len() {{ return Err(EncodeError::BufferTooSmall); }}")
        lines.append(f"{indent}buf[idx] = self.{fname} as u8;")
        lines.append(f"{indent}idx += 1;")
    elif ftype in PRIMITIVE_TYPES:
        _rustt, size = PRIMITIVE_TYPES[ftype]
        lines.append(f"{indent}if idx + {size} > buf.len() {{ return Err(EncodeError::BufferTooSmall); }}")
        lines.append(f"{indent}buf[idx..idx+{size}].copy_from_slice(&self.{fname}.to_le_bytes());")
        lines.append(f"{indent}idx += {size};")
    elif ftype.startswith("bytes["):
        size = ftype[6:-1]
        lines.append(f"{indent}if idx + {size} > buf.len() {{ return Err(EncodeError::BufferTooSmall); }}")
        lines.append(f"{indent}buf[idx..idx+{size}].copy_from_slice(&self.{fname});")
        lines.append(f"{indent}idx += {size};")
    
    return lines

def generate_field_decode(fname: str, ftype: str, enum_type: Optional[str], indent: str = "        ") -> List[str]:
    """Generate decoding code for a field"""
    lines = []
    if fname.endswith("_len"):
        return []

    if ftype.startswith("[u8; MAX_MESSAGE_SIZE]"):
        lines.append(f"{indent}let remaining_len = data.len() - idx;")
        lines.append(f"{indent}if remaining_len > MAX_MESSAGE_SIZE {{ return Err(DecodeError::NotEnoughBytes); }}")
        lines.append(f"{indent}let mut {fname} = [0u8; MAX_MESSAGE_SIZE];")
        lines.append(f"{indent}{fname}[..remaining_len].copy_from_slice(&data[idx..]);")
        lines.append(f"{indent}let {fname}_len = remaining_len as u16;")
        lines.append(f"{indent}idx += remaining_len;")
    elif ftype == 'enum':
        lines.append(f"{indent}if idx >= data.len() {{ return Err(DecodeError::NotEnoughBytes); }}")
        lines.append(f"{indent}let {fname} = {enum_type}::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;")
        lines.append(f"{indent}idx += 1;")
    elif ftype in PRIMITIVE_TYPES:
        rustt, size = PRIMITIVE_TYPES[ftype]
        lines.append(f"{indent}if data.len() < idx + {size} {{ return Err(DecodeError::NotEnoughBytes); }}")
        lines.append(f"{indent}let {fname} = {rustt}::from_le_bytes(data[idx..idx+{size}].try_into().map_err(|_| DecodeError::ConversionError)?);")
        lines.append(f"{indent}idx += {size};")
    elif ftype.startswith("bytes["):
        size = ftype[6:-1]
        lines.append(f"{indent}if data.len() < idx + {size} {{ return Err(DecodeError::NotEnoughBytes); }}")
        lines.append(f"{indent}let {fname}: [u8; {size}] = data[idx..idx+{size}].try_into().map_err(|_| DecodeError::ConversionError)?;")
        lines.append(f"{indent}idx += {size};")
        
    return lines

def generate_default_value(ftype: str, enum_type: Optional[str]) -> str:
    """Generate default value for a field"""
    if ftype == 'enum':
        return f"{enum_type}::default()"
    elif ftype in PRIMITIVE_TYPES:
        rust_type = PRIMITIVE_TYPES[ftype][0]
        return "0.0" if rust_type in ('f32', 'f64') else "0"
    elif ftype.startswith("[u8;"): # Handle our custom bytes type
        return "[0u8; MAX_MESSAGE_SIZE]"
    elif ftype.startswith("bytes["):
        size = ftype[6:-1]
        return f"[0u8; {size}]"
    else:
        return "0" # for _len fields

def xml_to_rust(protocol_xml_path: str, out_path: Optional[str] = None, 
                protocol_filter: Optional[str] = None, camera_filter: Optional[str] = None,
                use_std: bool = False):
    """Main conversion function with filtering"""
    tree = ET.parse(protocol_xml_path)
    root = tree.getroot()
    
    proto_name = root.attrib.get("name", "protocol")
    stx = root.attrib.get("stx", "0x6655")
    stx_little = root.attrib.get("stx_little", "true").lower() == "true"
    
    all_cameras = parse_camera_models(root)
    all_protocols = parse_protocol_types(root)
    
    target_protocol = protocol_filter
    target_camera = camera_filter
    
    enums = [parse_enum(e) for e in root.findall("enum")]
    
    messages_by_id: Dict[int, Dict[str, Tuple]] = defaultdict(dict)
    all_messages = []
    
    for m in root.findall("message"):
        msg_data = parse_message(m, all_cameras)
        _name, _cmd_id, _direction, _msg_desc, fields, protocols, cameras = msg_data
        
        if target_protocol and target_protocol not in protocols: continue
        if target_camera and target_camera not in cameras: continue
        
        filtered_fields = fields
        if target_camera:
            filtered_fields = [f for f in fields if target_camera in f[4]]
        
        msg_data_filtered = (*msg_data[:4], filtered_fields, *msg_data[5:])
        messages_by_id[msg_data[1]][msg_data[2]] = msg_data_filtered
        all_messages.append(msg_data_filtered)
    
    max_msg_size = calculate_max_message_size(all_messages)
    max_frame_size = max_msg_size + 10
    
    out = []
    
    # ===== HEADER, CONSTANTS, ERRORS =====
    out.append(f"// Auto-generated from {proto_name}")
    if target_protocol: out.append(f"// Protocol: {target_protocol}")
    if target_camera: out.append(f"// Camera: {target_camera}")
    out.append("#![no_std]")
    out.append("#![allow(dead_code, clippy::derivable_impls, unused, non_snake_case)]")
    out.append("use core::convert::TryInto;")
    out.append(f"pub const STX: u16 = {stx};")
    out.append(f"pub const STX_LITTLE: bool = {str(stx_little).lower()};")
    out.append(f"pub const MAX_MESSAGE_SIZE: usize = {max_msg_size};")
    out.append(f"pub const MAX_FRAME_SIZE: usize = {max_frame_size};")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum EncodeError { BufferTooSmall }")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum DecodeError { FrameTooShort, InvalidStx, FrameIncomplete, CrcMismatch, NotEnoughBytes, InvalidEnumValue, ConversionError, UnknownCmdId }")
    out.append("")

    # ===== CRC16 TABLE =====
    out.append("const CRC16_TAB: [u16; 256] = [")
    crc_values = [ 0x0,0x1021,0x2042,0x3063,0x4084,0x50a5,0x60c6,0x70e7,0x8108,0x9129,0xa14a,0xb16b,0xc18c,0xd1ad,0xe1ce,0xf1ef,0x1231,0x210,0x3273,0x2252,0x52b5,0x4294,0x72f7,0x62d6,0x9339,0x8318,0xb37b,0xa35a,0xd3bd,0xc39c,0xf3ff,0xe3de,0x2462,0x3443,0x420,0x1401,0x64e6,0x74c7,0x44a4,0x5485,0xa56a,0xb54b,0x8528,0x9509,0xe5ee,0xf5cf,0xc5ac,0xd58d,0x3653,0x2672,0x1611,0x630,0x76d7,0x66f6,0x5695,0x46b4,0xb75b,0xa77a,0x9719,0x8738,0xf7df,0xe7fe,0xd79d,0xc7bc,0x48c4,0x58e5,0x6886,0x78a7,0x840,0x1861,0x2802,0x3823,0xc9cc,0xd9ed,0xe98e,0xf9af,0x8948,0x9969,0xa90a,0xb92b,0x5af5,0x4ad4,0x7ab7,0x6a96,0x1a71,0xa50,0x3a33,0x2a12,0xdbfd,0xcbdc,0xfbbf,0xeb9e,0x9b79,0x8b58,0xbb3b,0xab1a,0x6ca6,0x7c87,0x4ce4,0x5cc5,0x2c22,0x3c03,0xc60,0x1c41,0xedae,0xfd8f,0xcdec,0xddcd,0xad2a,0xbd0b,0x8d68,0x9d49,0x7e97,0x6eb6,0x5ed5,0x4ef4,0x3e13,0x2e32,0x1e51,0xe70,0xff9f,0xefbe,0xdfdd,0xcffc,0xbf1b,0xaf3a,0x9f59,0x8f78,0x9188,0x81a9,0xb1ca,0xa1eb,0xd10c,0xc12d,0xf14e,0xe16f,0x1080,0xa1,0x30c2,0x20e3,0x5004,0x4025,0x7046,0x6067,0x83b9,0x9398,0xa3fb,0xb3da,0xc33d,0xd31c,0xe37f,0xf35e,0x2b1,0x1290,0x22f3,0x32d2,0x4235,0x5214,0x6277,0x7256,0xb5ea,0xa5cb,0x95a8,0x8589,0xf56e,0xe54f,0xd52c,0xc50d,0x34e2,0x24c3,0x14a0,0x481,0x7466,0x6447,0x5424,0x4405,0xa7db,0xb7fa,0x8799,0x97b8,0xe75f,0xf77e,0xc71d,0xd73c,0x26d3,0x36f2,0x691,0x16b0,0x6657,0x7676,0x4615,0x5634,0xd94c,0xc96d,0xf90e,0xe92f,0x99c8,0x89e9,0xb98a,0xa9ab,0x5844,0x4865,0x7806,0x6827,0x18c0,0x8e1,0x3882,0x28a3,0xcb7d,0xdb5c,0xeb3f,0xfb1e,0x8bf9,0x9bd8,0xabbb,0xbb9a,0x4a75,0x5a54,0x6a37,0x7a16,0xaf1,0x1ad0,0x2ab3,0x3a92,0xfd2e,0xed0f,0xdd6c,0xcd4d,0xbdaa,0xad8b,0x9de8,0x8dc9,0x7c26,0x6c07,0x5c64,0x4c45,0x3ca2,0x2c83,0x1ce0,0xcc1,0xef1f,0xff3e,0xcf5d,0xdf7c,0xaf9b,0xbfba,0x8fd9,0x9ff8,0x6e17,0x7e36,0x4e55,0x5e74,0x2e93,0x3eb2,0xed1,0x1ef0]
    out.extend([f"    {','.join([f'0x{v:x}' for v in crc_values[i:i+8]])}," for i in range(0, len(crc_values), 8)])
    out.append("];")
    out.append("pub const fn crc16_calc(data: &[u8], crc_init: u16) -> u16 { let mut crc = crc_init; let mut i = 0; while i < data.len() { crc = (crc << 8) ^ CRC16_TAB[((crc >> 8) as u8 ^ data[i]) as usize]; i += 1; } crc }")
    out.append("")

    # ===== CTRL, ENUMS, FRAME =====
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct CtrlByte { pub need_ack: bool, pub is_ack: bool }")
    out.append("impl CtrlByte { pub const fn from_u8(val: u8) -> Self { Self { need_ack: (val & 1) != 0, is_ack: (val & 2) != 0 } } pub const fn to_u8(&self) -> u8 { (if self.need_ack { 1 } else { 0 }) | (if self.is_ack { 2 } else { 0 }) } pub const fn request() -> Self { Self { need_ack: true, is_ack: false } } pub const fn response() -> Self { Self { need_ack: false, is_ack: true } } pub const fn is_request(&self) -> bool { !self.is_ack } pub const fn is_response(&self) -> bool { self.is_ack } }")
    out.append("impl Default for CtrlByte { fn default() -> Self { Self::request() } }")
    for name, variants in enums:
        out.append(f"#[repr(u8)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)] pub enum {name} {{")
        out.extend([f"    {vname} = {vvalue}," for vname, vvalue, _ in variants])
        out.append("}")
        out.append(f"impl {name} {{ pub const fn from_u8(val: u8) -> Option<Self> {{ match val {{")
        out.extend([f"    {vvalue} => Some(Self::{vname})," for vname, vvalue, _ in variants])
        out.append("    _ => None, } } pub const fn to_u8(self) -> u8 { self as u8 } }")
        if variants: out.append(f"impl Default for {name} {{ fn default() -> Self {{ Self::{variants[0][0]} }} }}")
    out.append("#[derive(Debug, Clone, Copy, PartialEq)] pub struct Frame { pub ctrl: CtrlByte, pub seq: u16, pub cmd: u8, pub data: [u8; MAX_MESSAGE_SIZE], pub data_len: u16 }")
    out.append("impl Frame { pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> { let total_len = 10 + self.data_len as usize; if buf.len() < total_len { return Err(EncodeError::BufferTooSmall); } buf[0..2].copy_from_slice(&if STX_LITTLE { STX.to_le_bytes() } else { STX.to_be_bytes() }); buf[2] = self.ctrl.to_u8(); buf[3..5].copy_from_slice(&self.data_len.to_le_bytes()); buf[5..7].copy_from_slice(&self.seq.to_le_bytes()); buf[7] = self.cmd; buf[8..8+self.data_len as usize].copy_from_slice(&self.data[..self.data_len as usize]); let crc = crc16_calc(&buf[0..8+self.data_len as usize], 0); buf[8+self.data_len as usize..total_len].copy_from_slice(&crc.to_le_bytes()); Ok(total_len) }")
    out.append("pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> { if buf.len() < 10 { return Err(DecodeError::FrameTooShort); } let stx = if STX_LITTLE { u16::from_le_bytes(buf[0..2].try_into().unwrap()) } else { u16::from_be_bytes(buf[0..2].try_into().unwrap()) }; if stx != STX { return Err(DecodeError::InvalidStx); } let data_len = u16::from_le_bytes(buf[3..5].try_into().unwrap()) as usize; let expected_len = 10 + data_len; if buf.len() < expected_len { return Err(DecodeError::FrameIncomplete); } let crc_recv = u16::from_le_bytes(buf[expected_len-2..expected_len].try_into().unwrap()); if crc_recv != crc16_calc(&buf[..expected_len-2], 0) { return Err(DecodeError::CrcMismatch); } let mut data = [0u8; MAX_MESSAGE_SIZE]; data[..data_len].copy_from_slice(&buf[8..8+data_len]); Ok(Self { ctrl: CtrlByte::from_u8(buf[2]), seq: u16::from_le_bytes(buf[5..7].try_into().unwrap()), cmd: buf[7], data, data_len: data_len as u16 }) } }")
    out.append("")

    # ===== MESSAGE STRUCTS =====
    out.append("// ============================================================================")
    out.append("// Message Structures")
    out.append("// ============================================================================")
    for name, cmd_id, direction, msg_desc, fields, _, _ in all_messages:
        out.append(f"#[derive(Debug, Clone, Copy, PartialEq)] pub struct {name} {{")
        out.extend([f"    pub {fname}: {get_rust_type(ftype, enum_type)}," for fname, ftype, enum_type, _, _ in fields])
        out.append("}")
        out.append(f"impl {name} {{")
        out.append(f"    pub const CMD_ID: u8 = 0x{cmd_id:02X};")
        out.append(f"    pub const IS_REQUEST: bool = {str(direction == 'request').lower()};")
        out.append("    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> { let mut idx = 0; ")
        out.extend(["".join(generate_field_encode(f[0], f[1], f[2], "")) for f in fields])
        out.append(" Ok(idx) }")
        out.append("    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> { let mut idx = 0; ")
        out.extend(["".join(generate_field_decode(f[0], f[1], f[2], "")) for f in fields])
        field_names = ", ".join([f[0] for f in fields])
        out.append(f" Ok(Self {{ {field_names} }}) }}")
        out.append("}")
        out.append(f"impl Default for {name} {{ fn default() -> Self {{ Self {{")
        out.extend([f"    {f[0]}: {generate_default_value(f[1], f[2])}," for f in fields])
        out.append("}} }")
    out.append("")

    # ===== UNIFIED MESSAGE ENUM & HELPERS =====
    out.append("#[derive(Debug, Clone, Copy, PartialEq)] pub enum Message {")
    out.extend([f"    {msg[0]}({msg[0]})," for msg in all_messages])
    out.append("}")
    out.append("impl Message {")
    out.append("    pub const fn cmd_id(&self) -> u8 { match self {")
    out.extend([f"        Self::{msg[0]}(_) => {msg[0]}::CMD_ID," for msg in all_messages])
    out.append("    }}")
    out.append("    pub const fn is_request(&self) -> bool { match self {")
    out.extend([f"        Self::{msg[0]}(_) => {msg[0]}::IS_REQUEST," for msg in all_messages])
    out.append("    }}")
    out.append("    pub const fn is_response(&self) -> bool { !self.is_request() }")
    out.append("    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> { match self {")
    out.extend([f"        Self::{msg[0]}(m) => m.encode(buf)," for msg in all_messages])
    out.append("    }}")
    out.append("    pub fn from_frame(frame: &Frame) -> Result<Self, DecodeError> {")
    out.append("        let data = &frame.data[..frame.data_len as usize];")
    out.append("        match frame.cmd {")
    for cmd_id in sorted(messages_by_id.keys()):
        out.append(f"            0x{cmd_id:02X} => {{")
        req = messages_by_id[cmd_id].get('request')
        resp = messages_by_id[cmd_id].get('response')
        if req and resp: out.append(f" if frame.ctrl.is_response() {{ Ok(Self::{resp[0]}({resp[0]}::decode(data)?)) }} else {{ Ok(Self::{req[0]}({req[0]}::decode(data)?)) }}")
        elif req: out.append(f" Ok(Self::{req[0]}({req[0]}::decode(data)?))")
        elif resp: out.append(f" Ok(Self::{resp[0]}({resp[0]}::decode(data)?))")
        out.append("}")
    out.append("            _ => Err(DecodeError::UnknownCmdId),")
    out.append("        }")
    out.append("    }")
    out.append("}")
    out.append("pub fn encode_message(msg: &Message, frame_buf: &mut [u8]) -> Result<usize, EncodeError> { let mut data_buf = [0u8; MAX_MESSAGE_SIZE]; let data_len = msg.encode(&mut data_buf)?; let mut frame = Frame { ctrl: if msg.is_response() { CtrlByte::response() } else { CtrlByte::request() }, seq: 0, cmd: msg.cmd_id(), data: [0u8; MAX_MESSAGE_SIZE], data_len: data_len as u16 }; frame.data[..data_len].copy_from_slice(&data_buf[..data_len]); frame.encode(frame_buf) }")
    out.append("pub fn decode_message(buf: &[u8]) -> Result<Message, DecodeError> { let frame = Frame::decode(buf)?; Message::from_frame(&frame) }")
    
    # ===== TESTS =====
    if all_messages:
      out.append("#[cfg(test)] mod tests { use super::*; #[test] fn test_message_roundtrip() {")
      test_msg = all_messages[0]
      test_msg_name = test_msg[0]
      test_msg_fields = test_msg[4]
      out.append(f" let mut msg = {test_msg_name}::default();")
      # Add some data to test bytes fields if they exist
      for fname, ftype, _, _, _ in test_msg_fields:
          if ftype.startswith("[u8;"):
              out.append(f" msg.{fname}[0..4].copy_from_slice(&[1,2,3,4]); msg.{fname}_len = 4;")

      out.append(" let wrapped_msg = Message::" + test_msg_name + "(msg);")
      out.append(" let mut frame_buf = [0u8; MAX_FRAME_SIZE];")
      out.append(" let len = encode_message(&wrapped_msg, &mut frame_buf).unwrap();")
      out.append(" let decoded_msg = decode_message(&frame_buf[..len]).unwrap();")
      out.append(" assert_eq!(wrapped_msg, decoded_msg);")
      out.append("}}")

    # ===== MODULE DOCUMENTATION =====
    out.append("")
    out.append("// ============================================================================")
    out.append("// Module Documentation")
    out.append("// ============================================================================")
    out.append("")
    out.append("/// # SIYI Protocol - Generated Module")
    out.append("///")
    if target_camera and target_protocol:
        out.append(f"/// This module contains message definitions for the **{target_camera}** camera")
        out.append(f"/// using the **{target_protocol}** protocol.")
    elif target_camera:
        out.append(f"/// This module contains message definitions for the **{target_camera}** camera.")
    elif target_protocol:
        out.append(f"/// This module contains message definitions using the **{target_protocol}** protocol.")
    else:
        out.append("/// This module contains all SIYI protocol message definitions.")
    out.append("///")
    out.append("/// ## Features")
    out.append("///")
    out.append("/// - **No heap allocation**: All operations use stack-allocated buffers")
    out.append("/// - **No lifetimes**: All data is owned in fixed-size arrays")
    out.append("/// - **CRC16 validation**: Automatic frame integrity checking")
    out.append("/// - **Type-safe enums**: Protocol enumerations with validation")
    out.append("/// - **No_std compatible**: Works in bare-metal environments")
    out.append("///")
    out.append("/// ## Quick Start")
    out.append("///")
    out.append("/// ### Encoding a Message")
    out.append("///")
    out.append("/// ```rust")
    if target_camera and target_protocol:
        out.append(f"/// use siyi_protocol::{target_camera.lower()}_{target_protocol.lower()}::*;")
    else:
        out.append("/// use siyi_protocol::*;")
    out.append("///")
    out.append("/// // Create a request")
    out.append("/// let request = FirmwareVersionRequest::default();")
    out.append("///")
    out.append("/// // Encode to frame buffer")
    out.append("/// let mut frame_buf = [0u8; MAX_FRAME_SIZE];")
    out.append("/// let msg = Message::FirmwareVersionRequest(request);")
    out.append("/// let frame_len = encode_message(&msg, &mut frame_buf).unwrap();")
    out.append("///")
    out.append("/// // Send frame_buf[..frame_len] over your transport layer")
    out.append("/// ```")
    out.append("///")
    out.append("/// ### Decoding a Message")
    out.append("///")
    out.append("/// ```rust")
    if target_camera and target_protocol:
        out.append(f"/// use siyi_protocol::{target_camera.lower()}_{target_protocol.lower()}::*;")
    else:
        out.append("/// use siyi_protocol::*;")
    out.append("///")
    out.append("/// // Receive data from your transport layer")
    out.append("/// let received_data: &[u8] = /* ... */;")
    out.append("///")
    out.append("/// // Decode the frame")
    out.append("/// match decode_message(received_data) {")
    out.append("///     Ok(Message::FirmwareVersionResponse(resp)) => {")
    out.append("///         println!(\"Camera FW: {}\", resp.camera_firmware_ver);")
    out.append("///     }")
    out.append("///     Ok(msg) => println!(\"Other message: {:?}\", msg),")
    out.append("///     Err(e) => eprintln!(\"Decode error: {:?}\", e),")
    out.append("/// }")
    out.append("/// ```")
    out.append("///")
    out.append("/// ## Protocol Frame Format")
    out.append("///")
    out.append("/// ```text")
    out.append("/// +--------+------+---------+-----+--------+------+---------+")
    out.append("/// | STX    | CTRL | DATALEN | SEQ | CMD_ID | DATA | CRC16   |")
    out.append("/// | 2 bytes| 1    | 2       | 2   | 1      | N    | 2 bytes |")
    out.append("/// +--------+------+---------+-----+--------+------+---------+")
    out.append("/// ```")
    out.append("///")
    out.append("/// - **STX**: Start marker (0x6655, little-endian)")
    out.append("/// - **CTRL**: Control byte (bit 0: need_ack, bit 1: is_ack)")
    out.append("/// - **DATALEN**: Data payload length (little-endian)")
    out.append("/// - **SEQ**: Sequence number (little-endian)")
    out.append("/// - **CMD_ID**: Command identifier")
    out.append("/// - **DATA**: Message payload")
    out.append("/// - **CRC16**: CRC16-CCITT checksum (little-endian)")
    out.append("///")
    out.append("/// ## Available Messages")
    out.append("///")
    
    # Group messages by category
    categories = {
        "System Information": [],
        "Gimbal Control": [],
        "Camera Functions": [],
        "Focus and Zoom": [],
        "Thermal Imaging": [],
        "Laser Ranging": [],
        "AI Features": [],
        "Video Configuration": [],
        "Data Streams": [],
        "Configuration": [],
    }
    
    for name, cmd_id, direction, msg_desc, _, _, _ in sorted(all_messages, key=lambda x: x[1]):
        msg_type = "Request" if direction == "request" else "Response"
        desc = msg_desc if msg_desc else f"{msg_type} message"
        
        # Categorize messages
        if cmd_id in [0x01, 0x02, 0x40]:
            categories["System Information"].append((name, cmd_id, desc))
        elif cmd_id in [0x07, 0x08, 0x0D, 0x0E, 0x19, 0x26, 0x27, 0x41]:
            categories["Gimbal Control"].append((name, cmd_id, desc))
        elif cmd_id in [0x0A, 0x0B, 0x0C]:
            categories["Camera Functions"].append((name, cmd_id, desc))
        elif cmd_id in [0x04, 0x05, 0x06, 0x0F, 0x16, 0x18]:
            categories["Focus and Zoom"].append((name, cmd_id, desc))
        elif cmd_id in range(0x12, 0x15) or cmd_id in [0x1A, 0x1B] or cmd_id in range(0x33, 0x48):
            categories["Thermal Imaging"].append((name, cmd_id, desc))
        elif cmd_id in [0x15, 0x17, 0x31, 0x32]:
            categories["Laser Ranging"].append((name, cmd_id, desc))
        elif cmd_id in [0x4D, 0x4E, 0x50, 0x51]:
            categories["AI Features"].append((name, cmd_id, desc))
        elif cmd_id in [0x10, 0x11, 0x20, 0x21]:
            categories["Video Configuration"].append((name, cmd_id, desc))
        elif cmd_id in range(0x22, 0x2B):
            categories["Data Streams"].append((name, cmd_id, desc))
        else:
            categories["Configuration"].append((name, cmd_id, desc))
    
    for category, messages in categories.items():
        if messages:
            out.append(f"/// ### {category}")
            out.append("///")
            for name, cmd_id, desc in messages:
                out.append(f"/// - [`{name}`] (0x{cmd_id:02X}): {desc}")
            out.append("///")
    
    out.append("/// ## Constants")
    out.append("///")
    out.append(f"/// - [`STX`]: Protocol start marker (0x{int(stx, 16):04X})")
    out.append(f"/// - [`MAX_MESSAGE_SIZE`]: Maximum message payload size ({max_msg_size} bytes)")
    out.append(f"/// - [`MAX_FRAME_SIZE`]: Maximum complete frame size ({max_frame_size} bytes)")
    out.append("///")
    out.append("/// ## Error Types")
    out.append("///")
    out.append("/// - [`EncodeError`]: Errors that can occur during message encoding")
    out.append("///   - `BufferTooSmall`: Output buffer is too small for the message")
    out.append("///")
    out.append("/// - [`DecodeError`]: Errors that can occur during message decoding")
    out.append("///   - `FrameTooShort`: Frame is shorter than minimum size")
    out.append("///   - `InvalidStx`: Start marker does not match expected value")
    out.append("///   - `FrameIncomplete`: Frame is incomplete based on length field")
    out.append("///   - `CrcMismatch`: CRC check failed")
    out.append("///   - `NotEnoughBytes`: Not enough bytes to decode field")
    out.append("///   - `InvalidEnumValue`: Enum value is not valid")
    out.append("///   - `ConversionError`: Type conversion failed")
    out.append("///   - `UnknownCmdId`: Unknown command ID")
    out.append("///")
    out.append("/// ## Memory Requirements")
    out.append("///")
    out.append(f"/// - Message encoding buffer: {max_msg_size} bytes (stack)")
    out.append(f"/// - Frame encoding buffer: {max_frame_size} bytes (stack)")
    out.append("/// - Per-message overhead: Varies by message type")
    out.append("///")
    out.append("/// All buffers are stack-allocated. No heap allocation is required.")
    out.append("///")
    out.append("/// ## Protocol-Specific Notes")
    out.append("///")
    if target_protocol == "TCP":
        out.append("/// ### TCP Protocol")
        out.append("///")
        out.append("/// - Default port: 37260")
        out.append("/// - Connection-oriented with guaranteed delivery")
        out.append("/// - Requires periodic heartbeat messages (0x00) to maintain connection")
        out.append("/// - Recommended heartbeat interval: 1-2 seconds")
        out.append("/// - All messages are supported over TCP")
    elif target_protocol == "UDP":
        out.append("/// ### UDP Protocol")
        out.append("///")
        out.append("/// - Default port: 37260")
        out.append("/// - Connectionless with no delivery guarantee")
        out.append("/// - Suitable for telemetry and status updates")
        out.append("/// - Implement retry logic for critical commands")
        out.append("/// - Some messages may not be supported (check camera documentation)")
    elif target_protocol == "TTL":
        out.append("/// ### TTL (Serial) Protocol")
        out.append("///")
        out.append("/// - Baud rate: 115200")
        out.append("/// - Data bits: 8, Stop bits: 1, Parity: None")
        out.append("/// - Full duplex communication")
        out.append("/// - Implement frame synchronization for byte-by-byte reception")
        out.append("/// - Some messages may not be supported (check camera documentation)")
    out.append("///")
    
    if target_camera:
        out.append("/// ## Camera-Specific Notes")
        out.append("///")
        out.append(f"/// ### {target_camera}")
        out.append("///")
        if target_camera == "ZT30":
            out.append("/// - Quad-sensor gimbal camera")
            out.append("/// - Full thermal imaging support")
            out.append("/// - Laser ranging available")
            out.append("/// - AI tracking features")
            out.append("/// - Video stitching modes")
        elif target_camera == "ZT6":
            out.append("/// - Thermal imaging camera")
            out.append("/// - AI tracking features")
            out.append("/// - Video stitching modes")
            out.append("/// - No laser ranging")
        elif target_camera == "ZR30":
            out.append("/// - Long-range camera system")
            out.append("/// - Laser ranging available")
            out.append("/// - Advanced zoom control")
            out.append("/// - No thermal imaging")
        elif target_camera == "ZR10":
            out.append("/// - Standard gimbal camera")
            out.append("/// - Laser ranging available")
            out.append("/// - Basic zoom and focus")
            out.append("/// - No thermal or AI features")
        elif target_camera == "A8mini":
            out.append("/// - Compact gimbal camera")
            out.append("/// - AI tracking support")
            out.append("/// - Basic zoom control")
            out.append("/// - Network only (TCP/UDP)")
        elif target_camera == "A2mini":
            out.append("/// - Entry-level gimbal camera")
            out.append("/// - Basic gimbal control")
            out.append("/// - Simple zoom control")
            out.append("/// - Network only (TCP/UDP)")
        out.append("///")
    
    out.append("/// ## Data Encoding Notes")
    out.append("///")
    out.append("/// ### Angles")
    out.append("///")
    out.append("/// Angles are encoded as integers multiplied by 10:")
    out.append("///")
    out.append("/// ```rust")
    out.append("/// // Encoding: 45.5 degrees")
    out.append("/// let angle_deg = 45.5;")
    out.append("/// let angle_protocol = (angle_deg * 10.0) as i16;  // 455")
    out.append("///")
    out.append("/// // Decoding:")
    out.append("/// let received_value = 455i16;")
    out.append("/// let angle_deg = received_value as f32 / 10.0;  // 45.5")
    out.append("/// ```")
    out.append("///")
    out.append("/// ### Temperatures")
    out.append("///")
    out.append("/// Temperatures are encoded as integers multiplied by 100:")
    out.append("///")
    out.append("/// ```rust")
    out.append("/// // Encoding: 25.37°C")
    out.append("/// let temp_celsius = 25.37;")
    out.append("/// let temp_protocol = (temp_celsius * 100.0) as u16;  // 2537")
    out.append("///")
    out.append("/// // Decoding:")
    out.append("/// let received_value = 2537u16;")
    out.append("/// let temp_celsius = received_value as f32 / 100.0;  // 25.37")
    out.append("/// ```")
    out.append("///")
    out.append("/// ### Distances")
    out.append("///")
    out.append("/// Laser distances are measured in decimeters (dm):")
    out.append("///")
    out.append("/// ```rust")
    out.append("/// // Encoding: 150 meters")
    out.append("/// let distance_m = 150.0;")
    out.append("/// let distance_dm = (distance_m * 10.0) as u16;  // 1500")
    out.append("///")
    out.append("/// // Decoding:")
    out.append("/// let received_value = 1500u16;")
    out.append("/// let distance_m = received_value as f32 / 10.0;  // 150.0")
    out.append("/// ```")
    out.append("///")
    out.append("/// Minimum valid distance: 5.0 meters (50 dm)")
    out.append("///")
    out.append("/// ## Examples")
    out.append("///")
    out.append("/// ### Getting Gimbal Attitude")
    out.append("///")
    out.append("/// ```rust")
    if target_camera and target_protocol:
        out.append(f"/// use siyi_protocol::{target_camera.lower()}_{target_protocol.lower()}::*;")
    else:
        out.append("/// use siyi_protocol::*;")
    out.append("///")
    out.append("/// let request = GimbalAttitudeRequest::default();")
    out.append("/// let msg = Message::GimbalAttitudeRequest(request);")
    out.append("///")
    out.append("/// let mut frame_buf = [0u8; MAX_FRAME_SIZE];")
    out.append("/// let len = encode_message(&msg, &mut frame_buf).unwrap();")
    out.append("///")
    out.append("/// // Send frame_buf[..len] and receive response")
    out.append("/// // let response_data: &[u8] = receive_from_camera();")
    out.append("///")
    out.append("/// // Decode response")
    out.append("/// // match decode_message(response_data) {")
    out.append("/// //     Ok(Message::GimbalAttitudeResponse(resp)) => {")
    out.append("/// //         let yaw = resp.yaw as f32 / 10.0;")
    out.append("/// //         let pitch = resp.pitch as f32 / 10.0;")
    out.append("/// //         println!(\"Yaw: {:.1}°, Pitch: {:.1}°\", yaw, pitch);")
    out.append("/// //     }")
    out.append("/// //     _ => {}")
    out.append("/// // }")
    out.append("/// ```")
    out.append("///")
    out.append("/// ### Setting Gimbal Position")
    out.append("///")
    out.append("/// ```rust")
    if target_camera and target_protocol:
        out.append(f"/// use siyi_protocol::{target_camera.lower()}_{target_protocol.lower()}::*;")
    else:
        out.append("/// use siyi_protocol::*;")
    out.append("///")
    out.append("/// // Set to 45° yaw, -30° pitch")
    out.append("/// let yaw = (45.0 * 10.0) as i16;")
    out.append("/// let pitch = (-30.0 * 10.0) as i16;")
    out.append("///")
    out.append("/// let mut request = SetGimbalAttitudeRequest::default();")
    out.append("/// request.yaw = yaw;")
    out.append("/// request.pitch = pitch;")
    out.append("///")
    out.append("/// let msg = Message::SetGimbalAttitudeRequest(request);")
    out.append("/// let mut frame_buf = [0u8; MAX_FRAME_SIZE];")
    out.append("/// let len = encode_message(&msg, &mut frame_buf).unwrap();")
    out.append("/// ```")
    out.append("///")
    out.append("/// ## See Also")
    out.append("///")
    out.append("/// - [SIYI SDK Documentation](https://shop.siyi.biz/)")
    out.append("/// - [Protocol Specification](https://github.com/AhmedBoin/siyi-protocol/blob/main/PROTOCOL.md)")
    out.append("/// - [Examples](https://github.com/AhmedBoin/siyi-protocol/tree/main/examples)")
    out.append("///")
    out.append("#[allow(unused)]")
    out.append("const _DOCUMENTATION: () = ();")

    src = "\n".join(out)
    if out_path:
        with open(out_path, 'w') as f: f.write(src)
        print(f"Generated Rust code for protocol '{target_protocol or 'all'}' and camera '{target_camera or 'all'}' to: {out_path}")
    else:
        print(src)

def main():
    parser = argparse.ArgumentParser(
        description='Generate lifetime-free no_std Rust code from SIYI Protocol XML.',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Generate a universal library for all protocols and cameras
  %(prog)s siyi_protocol.xml -o src/universal.rs

  # Generate a library specifically for the ZT30 camera over UDP
  %(prog)s siyi_protocol.xml --protocol UDP --camera ZT30 -o src/zt30_udp.rs
        """
    )
    parser.add_argument('xml_file', help='Path to the protocol XML file')
    parser.add_argument('-o', '--output', help='Output file path (prints to stdout if not specified)')
    parser.add_argument('--protocol', help='Filter by protocol type (e.g., TTL, UDP, TCP)')
    parser.add_argument('--camera', help='Filter by camera model (e.g., ZT30, A8mini)')
    
    args = parser.parse_args()

    try:
        xml_to_rust(args.xml_file, args.output, protocol_filter=args.protocol, camera_filter=args.camera)
    except FileNotFoundError:
        print(f"Error: File '{args.xml_file}' not found.", file=sys.stderr)
        sys.exit(1)
    except ET.ParseError as e:
        print(f"Error: Failed to parse XML file: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"An unexpected error occurred: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()