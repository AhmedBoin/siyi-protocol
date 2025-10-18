#!/usr/bin/env python3
"""
SIYI Protocol XML to Rust Code Generator (V2 - No_std Compatible)
Converts XML protocol definitions to bare-metal compatible Rust code
- No Vec, String, or heap allocations
- Fixed-size buffers and stack-based encoding
- Simple error types without format strings
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

# Calculate maximum message size from XML
def calculate_max_message_size(all_messages):
    """Calculate the maximum possible message size"""
    max_size = 0
    for name, cmd_id, direction, msg_desc, fields in all_messages:
        size = 0
        for fname, ftype, enum_type, fdesc in fields:
            if ftype in PRIMITIVE_TYPES:
                size += PRIMITIVE_TYPES[ftype][1]
            elif ftype.startswith("bytes["):
                arr_size = int(ftype[6:-1])
                size += arr_size
            elif ftype == 'enum':
                size += 1
            else:
                # For variable bytes, we'll need to handle differently
                size += 256  # Conservative estimate
        max_size = max(max_size, size)
    return max(max_size, 512)  # Minimum 512 bytes

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

def parse_message(m) -> Tuple[str, int, str, str, List[Tuple[str, str, Optional[str], str]]]:
    """Parse message element"""
    name = m.attrib['name']
    cmd_id = int(m.attrib['id'], 0)
    direction = m.attrib.get('direction', 'request')
    msg_desc = m.attrib.get('description', '')
    
    fields = []
    for f in m.findall("field"):
        fname = f.attrib['name']
        ftype = f.attrib['type']
        fdesc = f.attrib.get('description', '')
        fenum_type = f.attrib.get('enum_type', None)
        fields.append((fname, ftype, fenum_type, fdesc))
    
    return name, cmd_id, direction, msg_desc, fields

def get_rust_type(ftype: str, enum_type: Optional[str]) -> str:
    """Get Rust type for a field"""
    if ftype == 'enum':
        return enum_type if enum_type else 'u8'
    elif ftype in PRIMITIVE_TYPES:
        return PRIMITIVE_TYPES[ftype][0]
    elif ftype.startswith("bytes["):
        size = ftype[6:-1]
        return f"[u8; {size}]"
    elif ftype == "bytes":
        return "&'a [u8]"  # Use slices instead of Vec
    else:
        return "&'a [u8]"

def generate_field_encode(fname: str, ftype: str, enum_type: Optional[str], indent: str = "        ") -> List[str]:
    """Generate encoding code for a field (no_std compatible)"""
    lines = []
    if ftype == 'enum':
        lines.append(f"{indent}if idx >= buf.len() {{ return Err(EncodeError::BufferTooSmall); }}")
        lines.append(f"{indent}buf[idx] = self.{fname} as u8;")
        lines.append(f"{indent}idx += 1;")
    elif ftype in PRIMITIVE_TYPES:
        rustt, size = PRIMITIVE_TYPES[ftype]
        lines.append(f"{indent}if idx + {size} > buf.len() {{ return Err(EncodeError::BufferTooSmall); }}")
        lines.append(f"{indent}buf[idx..idx+{size}].copy_from_slice(&self.{fname}.to_le_bytes());")
        lines.append(f"{indent}idx += {size};")
    elif ftype.startswith("bytes["):
        size = ftype[6:-1]
        lines.append(f"{indent}if idx + {size} > buf.len() {{ return Err(EncodeError::BufferTooSmall); }}")
        lines.append(f"{indent}buf[idx..idx+{size}].copy_from_slice(&self.{fname});")
        lines.append(f"{indent}idx += {size};")
    else:
        lines.append(f"{indent}let len = self.{fname}.len();")
        lines.append(f"{indent}if idx + len > buf.len() {{ return Err(EncodeError::BufferTooSmall); }}")
        lines.append(f"{indent}buf[idx..idx+len].copy_from_slice(self.{fname});")
        lines.append(f"{indent}idx += len;")
    return lines

def generate_field_decode(fname: str, ftype: str, enum_type: Optional[str], indent: str = "        ") -> List[str]:
    """Generate decoding code for a field (no_std compatible)"""
    lines = []
    if ftype == 'enum':
        lines.append(f"{indent}if idx >= data.len() {{")
        lines.append(f"{indent}    return Err(DecodeError::NotEnoughBytes);")
        lines.append(f"{indent}}}")
        lines.append(f"{indent}let {fname} = {enum_type}::from_u8(data[idx])")
        lines.append(f"{indent}    .ok_or(DecodeError::InvalidEnumValue)?;")
        lines.append(f"{indent}idx += 1;")
    elif ftype in PRIMITIVE_TYPES:
        rustt, size = PRIMITIVE_TYPES[ftype]
        lines.append(f"{indent}if data.len() < idx + {size} {{")
        lines.append(f"{indent}    return Err(DecodeError::NotEnoughBytes);")
        lines.append(f"{indent}}}")
        lines.append(f"{indent}let {fname} = {rustt}::from_le_bytes(")
        lines.append(f"{indent}    data[idx..idx+{size}].try_into().map_err(|_| DecodeError::ConversionError)?")
        lines.append(f"{indent});")
        lines.append(f"{indent}idx += {size};")
    elif ftype.startswith("bytes["):
        size = ftype[6:-1]
        lines.append(f"{indent}if data.len() < idx + {size} {{")
        lines.append(f"{indent}    return Err(DecodeError::NotEnoughBytes);")
        lines.append(f"{indent}}}")
        lines.append(f"{indent}let {fname}: [u8; {size}] = data[idx..idx+{size}]")
        lines.append(f"{indent}    .try_into().map_err(|_| DecodeError::ConversionError)?;")
        lines.append(f"{indent}idx += {size};")
    else:
        lines.append(f"{indent}let {fname} = &data[idx..];")
        lines.append(f"{indent}idx = data.len();")
    return lines

def generate_default_value(ftype: str, enum_type: Optional[str]) -> str:
    """Generate default value for a field"""
    if ftype == 'enum':
        return f"{enum_type}::default()"
    elif ftype in PRIMITIVE_TYPES:
        rust_type = PRIMITIVE_TYPES[ftype][0]
        return "0.0" if rust_type in ('f32', 'f64') else "0"
    elif ftype.startswith("bytes["):
        size = ftype[6:-1]
        return f"[0u8; {size}]"
    else:
        return "&[]"

def xml_to_rust(protocol_xml_path: str, out_path: Optional[str] = None, use_std: bool = False):
    """Main conversion function"""
    tree = ET.parse(protocol_xml_path)
    root = tree.getroot()
    
    proto_name = root.attrib.get("name", "protocol")
    stx = root.attrib.get("stx", "0x6655")
    stx_little = root.attrib.get("stx_little", "true").lower() == "true"
    
    # Parse all enums
    enums = []
    for e in root.findall("enum"):
        enum_name, variants = parse_enum(e)
        enums.append((enum_name, variants))
    
    # Parse all messages and group by CMD_ID
    messages_by_id: Dict[int, Dict[str, Tuple]] = defaultdict(dict)
    all_messages = []
    for m in root.findall("message"):
        name, cmd_id, direction, msg_desc, fields = parse_message(m)
        messages_by_id[cmd_id][direction] = (name, fields, msg_desc)
        all_messages.append((name, cmd_id, direction, msg_desc, fields))
    
    # Calculate maximum buffer sizes
    max_msg_size = calculate_max_message_size(all_messages)
    max_frame_size = max_msg_size + 10  # Header + CRC
    
    out = []
    
    # ===== HEADER =====
    out.append("// Auto-generated by SIYI Protocol XML to Rust Generator (V2 - No_std)")
    out.append(f"// Protocol: {proto_name}")
    out.append("// DO NOT EDIT - This file is automatically generated")
    out.append("")
    
    if not use_std:
        out.append("#![no_std]")
        out.append("")
    
    out.append("#![allow(dead_code, clippy::derivable_impls, unused, non_snake_case)]")
    out.append("")
    
    if not use_std:
        out.append("use core::convert::TryInto;")
    else:
        out.append("use std::convert::TryInto;")
    out.append("")
    
    # ===== CONSTANTS =====
    out.append("// ============================================================================")
    out.append("// Protocol Constants")
    out.append("// ============================================================================")
    out.append("")
    out.append(f"pub const STX: u16 = {stx};")
    out.append(f"pub const STX_LITTLE: bool = {str(stx_little).lower()};")
    out.append(f"pub const MAX_MESSAGE_SIZE: usize = {max_msg_size};")
    out.append(f"pub const MAX_FRAME_SIZE: usize = {max_frame_size};")
    out.append("")
    
    # ===== ERROR TYPES =====
    out.append("// ============================================================================")
    out.append("// Error Types (no_std compatible)")
    out.append("// ============================================================================")
    out.append("")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
    out.append("pub enum EncodeError {")
    out.append("    BufferTooSmall,")
    out.append("}")
    out.append("")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
    out.append("pub enum DecodeError {")
    out.append("    FrameTooShort,")
    out.append("    InvalidStx,")
    out.append("    FrameIncomplete,")
    out.append("    CrcMismatch,")
    out.append("    NotEnoughBytes,")
    out.append("    InvalidEnumValue,")
    out.append("    ConversionError,")
    out.append("    UnknownCmdId,")
    out.append("}")
    out.append("")
    
    # ===== CRC16 TABLE =====
    out.append("// ============================================================================")
    out.append("// CRC16 Implementation (G(X) = X^16+X^12+X^5+1)")
    out.append("// ============================================================================")
    out.append("")
    out.append("const CRC16_TAB: [u16; 256] = [")
    crc_values = [
        0x0,0x1021,0x2042,0x3063,0x4084,0x50a5,0x60c6,0x70e7,
        0x8108,0x9129,0xa14a,0xb16b,0xc18c,0xd1ad,0xe1ce,0xf1ef,
        0x1231,0x210,0x3273,0x2252,0x52b5,0x4294,0x72f7,0x62d6,
        0x9339,0x8318,0xb37b,0xa35a,0xd3bd,0xc39c,0xf3ff,0xe3de,
        0x2462,0x3443,0x420,0x1401,0x64e6,0x74c7,0x44a4,0x5485,
        0xa56a,0xb54b,0x8528,0x9509,0xe5ee,0xf5cf,0xc5ac,0xd58d,
        0x3653,0x2672,0x1611,0x630,0x76d7,0x66f6,0x5695,0x46b4,
        0xb75b,0xa77a,0x9719,0x8738,0xf7df,0xe7fe,0xd79d,0xc7bc,
        0x48c4,0x58e5,0x6886,0x78a7,0x840,0x1861,0x2802,0x3823,
        0xc9cc,0xd9ed,0xe98e,0xf9af,0x8948,0x9969,0xa90a,0xb92b,
        0x5af5,0x4ad4,0x7ab7,0x6a96,0x1a71,0xa50,0x3a33,0x2a12,
        0xdbfd,0xcbdc,0xfbbf,0xeb9e,0x9b79,0x8b58,0xbb3b,0xab1a,
        0x6ca6,0x7c87,0x4ce4,0x5cc5,0x2c22,0x3c03,0xc60,0x1c41,
        0xedae,0xfd8f,0xcdec,0xddcd,0xad2a,0xbd0b,0x8d68,0x9d49,
        0x7e97,0x6eb6,0x5ed5,0x4ef4,0x3e13,0x2e32,0x1e51,0xe70,
        0xff9f,0xefbe,0xdfdd,0xcffc,0xbf1b,0xaf3a,0x9f59,0x8f78,
        0x9188,0x81a9,0xb1ca,0xa1eb,0xd10c,0xc12d,0xf14e,0xe16f,
        0x1080,0xa1,0x30c2,0x20e3,0x5004,0x4025,0x7046,0x6067,
        0x83b9,0x9398,0xa3fb,0xb3da,0xc33d,0xd31c,0xe37f,0xf35e,
        0x2b1,0x1290,0x22f3,0x32d2,0x4235,0x5214,0x6277,0x7256,
        0xb5ea,0xa5cb,0x95a8,0x8589,0xf56e,0xe54f,0xd52c,0xc50d,
        0x34e2,0x24c3,0x14a0,0x481,0x7466,0x6447,0x5424,0x4405,
        0xa7db,0xb7fa,0x8799,0x97b8,0xe75f,0xf77e,0xc71d,0xd73c,
        0x26d3,0x36f2,0x691,0x16b0,0x6657,0x7676,0x4615,0x5634,
        0xd94c,0xc96d,0xf90e,0xe92f,0x99c8,0x89e9,0xb98a,0xa9ab,
        0x5844,0x4865,0x7806,0x6827,0x18c0,0x8e1,0x3882,0x28a3,
        0xcb7d,0xdb5c,0xeb3f,0xfb1e,0x8bf9,0x9bd8,0xabbb,0xbb9a,
        0x4a75,0x5a54,0x6a37,0x7a16,0xaf1,0x1ad0,0x2ab3,0x3a92,
        0xfd2e,0xed0f,0xdd6c,0xcd4d,0xbdaa,0xad8b,0x9de8,0x8dc9,
        0x7c26,0x6c07,0x5c64,0x4c45,0x3ca2,0x2c83,0x1ce0,0xcc1,
        0xef1f,0xff3e,0xcf5d,0xdf7c,0xaf9b,0xbfba,0x8fd9,0x9ff8,
        0x6e17,0x7e36,0x4e55,0x5e74,0x2e93,0x3eb2,0xed1,0x1ef0,
    ]
    for i in range(0, len(crc_values), 8):
        line = ",".join([f"0x{v:x}" for v in crc_values[i:i+8]])
        out.append(f"    {line},")
    out.append("];")
    out.append("")
    out.append("/// Calculate CRC16 checksum for SIYI protocol")
    out.append("#[inline]")
    out.append("pub const fn crc16_calc(data: &[u8], crc_init: u16) -> u16 {")
    out.append("    let mut crc = crc_init;")
    out.append("    let mut i = 0;")
    out.append("    while i < data.len() {")
    out.append("        let temp = (crc >> 8) as u8;")
    out.append("        let oldcrc16 = CRC16_TAB[(data[i] ^ temp) as usize];")
    out.append("        crc = (crc << 8) ^ oldcrc16;")
    out.append("        i += 1;")
    out.append("    }")
    out.append("    crc")
    out.append("}")
    out.append("")
    
    # ===== CTRL STRUCTURE =====
    out.append("// ============================================================================")
    out.append("// CTRL Field Structure")
    out.append("// ============================================================================")
    out.append("")
    out.append("/// Control byte: bit 0 = need_ack, bit 1 = is_ack")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")
    out.append("pub struct CtrlByte {")
    out.append("    pub need_ack: bool,")
    out.append("    pub is_ack: bool,")
    out.append("}")
    out.append("")
    out.append("impl CtrlByte {")
    out.append("    #[inline]")
    out.append("    pub const fn from_u8(val: u8) -> Self {")
    out.append("        CtrlByte {")
    out.append("            need_ack: (val & 0x01) != 0,")
    out.append("            is_ack: (val & 0x02) != 0,")
    out.append("        }")
    out.append("    }")
    out.append("")
    out.append("    #[inline]")
    out.append("    pub const fn to_u8(&self) -> u8 {")
    out.append("        let mut val: u8 = 0;")
    out.append("        if self.need_ack { val |= 0x01; }")
    out.append("        if self.is_ack { val |= 0x02; }")
    out.append("        val")
    out.append("    }")
    out.append("")
    out.append("    #[inline]")
    out.append("    pub const fn request() -> Self {")
    out.append("        CtrlByte { need_ack: true, is_ack: false }")
    out.append("    }")
    out.append("")
    out.append("    #[inline]")
    out.append("    pub const fn response() -> Self {")
    out.append("        CtrlByte { need_ack: false, is_ack: true }")
    out.append("    }")
    out.append("")
    out.append("    #[inline]")
    out.append("    pub const fn is_request(&self) -> bool {")
    out.append("        !self.is_ack")
    out.append("    }")
    out.append("")
    out.append("    #[inline]")
    out.append("    pub const fn is_response(&self) -> bool {")
    out.append("        self.is_ack")
    out.append("    }")
    out.append("}")
    out.append("")
    out.append("impl Default for CtrlByte {")
    out.append("    fn default() -> Self {")
    out.append("        CtrlByte::request()")
    out.append("    }")
    out.append("}")
    out.append("")
    
    # ===== ENUMERATIONS =====
    if enums:
        out.append("// ============================================================================")
        out.append("// Protocol Enumerations")
        out.append("// ============================================================================")
        out.append("")
        
        for enum_name, variants in enums:
            out.append("#[repr(u8)]")
            out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]")
            out.append(f"pub enum {enum_name} {{")
            for vname, vvalue, vdesc in variants:
                if vdesc:
                    out.append(f"    /// {vdesc}")
                out.append(f"    {vname} = {vvalue},")
            out.append("}")
            out.append("")
            
            out.append(f"impl {enum_name} {{")
            out.append(f"    #[inline]")
            out.append(f"    pub const fn from_u8(val: u8) -> Option<Self> {{")
            out.append("        match val {")
            for vname, vvalue, _ in variants:
                out.append(f"            {vvalue} => Some({enum_name}::{vname}),")
            out.append("            _ => None,")
            out.append("        }")
            out.append("    }")
            out.append("")
            out.append(f"    #[inline]")
            out.append(f"    pub const fn to_u8(self) -> u8 {{")
            out.append("        self as u8")
            out.append("    }")
            out.append("}")
            out.append("")
            
            if variants:
                first_variant = variants[0][0]
                out.append(f"impl Default for {enum_name} {{")
                out.append(f"    fn default() -> Self {{")
                out.append(f"        {enum_name}::{first_variant}")
                out.append("    }")
                out.append("}")
                out.append("")
    
    # ===== FRAME STRUCTURE =====
    out.append("// ============================================================================")
    out.append("// Protocol Frame Structure (Stack-based)")
    out.append("// ============================================================================")
    out.append("")
    out.append("/// SIYI protocol frame with fixed-size buffer")
    out.append("#[derive(Debug, Clone, Copy, PartialEq)]")
    out.append("pub struct Frame<'a> {")
    out.append("    pub ctrl: CtrlByte,")
    out.append("    pub seq: u16,")
    out.append("    pub cmd: u8,")
    out.append("    pub data: &'a [u8],")
    out.append("}")
    out.append("")
    out.append("impl<'a> Frame<'a> {")
    out.append("    /// Create a new frame")
    out.append("    #[inline]")
    out.append("    pub const fn new(cmd: u8, data: &'a [u8], is_response: bool) -> Self {")
    out.append("        Self {")
    out.append("            ctrl: if is_response { CtrlByte::response() } else { CtrlByte::request() },")
    out.append("            seq: 0,")
    out.append("            cmd,")
    out.append("            data,")
    out.append("        }")
    out.append("    }")
    out.append("")
    out.append("    /// Encode frame to buffer, returns size written")
    out.append("    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {")
    out.append("        let datalen = self.data.len();")
    out.append("        let total_len = 10 + datalen;")
    out.append("        ")
    out.append("        if buf.len() < total_len {")
    out.append("            return Err(EncodeError::BufferTooSmall);")
    out.append("        }")
    out.append("        ")
    out.append("        let mut idx = 0;")
    out.append("        ")
    out.append("        // STX")
    out.append("        if STX_LITTLE {")
    out.append("            buf[idx..idx+2].copy_from_slice(&STX.to_le_bytes());")
    out.append("        } else {")
    out.append("            buf[idx..idx+2].copy_from_slice(&STX.to_be_bytes());")
    out.append("        }")
    out.append("        idx += 2;")
    out.append("        ")
    out.append("        // CTRL")
    out.append("        buf[idx] = self.ctrl.to_u8();")
    out.append("        idx += 1;")
    out.append("        ")
    out.append("        // DATA_LEN")
    out.append("        buf[idx..idx+2].copy_from_slice(&(datalen as u16).to_le_bytes());")
    out.append("        idx += 2;")
    out.append("        ")
    out.append("        // SEQ")
    out.append("        buf[idx..idx+2].copy_from_slice(&self.seq.to_le_bytes());")
    out.append("        idx += 2;")
    out.append("        ")
    out.append("        // CMD")
    out.append("        buf[idx] = self.cmd;")
    out.append("        idx += 1;")
    out.append("        ")
    out.append("        // DATA")
    out.append("        buf[idx..idx+datalen].copy_from_slice(self.data);")
    out.append("        idx += datalen;")
    out.append("        ")
    out.append("        // CRC")
    out.append("        let crc = crc16_calc(&buf[0..idx], 0);")
    out.append("        buf[idx..idx+2].copy_from_slice(&crc.to_le_bytes());")
    out.append("        idx += 2;")
    out.append("        ")
    out.append("        Ok(idx)")
    out.append("    }")
    out.append("")
    out.append("    /// Decode frame from bytes")
    out.append("    pub fn decode(frame_bytes: &'a [u8]) -> Result<Self, DecodeError> {")
    out.append("        const MIN_SIZE: usize = 10;")
    out.append("        if frame_bytes.len() < MIN_SIZE {")
    out.append("            return Err(DecodeError::FrameTooShort);")
    out.append("        }")
    out.append("")
    out.append("        let stx = if STX_LITTLE {")
    out.append("            u16::from_le_bytes([frame_bytes[0], frame_bytes[1]])")
    out.append("        } else {")
    out.append("            u16::from_be_bytes([frame_bytes[0], frame_bytes[1]])")
    out.append("        };")
    out.append("        if stx != STX {")
    out.append("            return Err(DecodeError::InvalidStx);")
    out.append("        }")
    out.append("")
    out.append("        let ctrl = CtrlByte::from_u8(frame_bytes[2]);")
    out.append("        let datalen = u16::from_le_bytes([frame_bytes[3], frame_bytes[4]]) as usize;")
    out.append("        let seq = u16::from_le_bytes([frame_bytes[5], frame_bytes[6]]);")
    out.append("        let cmd = frame_bytes[7];")
    out.append("        ")
    out.append("        let expected_len = MIN_SIZE + datalen;")
    out.append("        if frame_bytes.len() < expected_len {")
    out.append("            return Err(DecodeError::FrameIncomplete);")
    out.append("        }")
    out.append("        ")
    out.append("        let data = &frame_bytes[8..8 + datalen];")
    out.append("        let crc_recv = u16::from_le_bytes([frame_bytes[8 + datalen], frame_bytes[8 + datalen + 1]]);")
    out.append("        let crc_calc = crc16_calc(&frame_bytes[0..8 + datalen], 0);")
    out.append("        if crc_recv != crc_calc {")
    out.append("            return Err(DecodeError::CrcMismatch);")
    out.append("        }")
    out.append("        ")
    out.append("        Ok(Frame { ctrl, seq, cmd, data })")
    out.append("    }")
    out.append("}")
    out.append("")
    out.append("// ============================================================================")
    out.append("// Message Structures (with lifetime for slices)")
    out.append("// ============================================================================")
    out.append("")

    for name, cmd_id, direction, msg_desc, fields in all_messages:
        # Check if message has any slice fields
        has_slice = any(ftype == "bytes" for _, ftype, _, _ in fields)
        lifetime = "<'a>" if has_slice else ""
        
        if msg_desc:
            out.append(f"/// {msg_desc}")
        out.append(f"/// CMD_ID: 0x{cmd_id:02X} | Type: {direction.upper()}")
        out.append("#[derive(Debug, Clone, Copy, PartialEq)]")
        out.append(f"pub struct {name}{lifetime} {{")
        if not fields:
            out.append("    _phantom: core::marker::PhantomData<()>,")
        for fname, ftype, enum_type, fdesc in fields:
            rustt = get_rust_type(ftype, enum_type)
            if fdesc:
                out.append(f"    /// {fdesc}")
            out.append(f"    pub {fname}: {rustt},")
        out.append("}")
        out.append("")
        
        out.append(f"impl{lifetime} {name}{lifetime} {{")
        out.append(f"    pub const CMD_ID: u8 = 0x{cmd_id:02X};")
        out.append(f"    pub const IS_REQUEST: bool = {str(direction == 'request').lower()};")
        out.append("")
        
        # Constructor
        if fields:
            params = ", ".join([f"{fname}: {get_rust_type(ftype, enum_type)}" for fname, ftype, enum_type, _ in fields])
            out.append(f"    pub const fn new({params}) -> Self {{")
            out.append(f"        Self {{ {', '.join([fname for fname, _, _, _ in fields])} }}")
            out.append("    }")
        else:
            out.append(f"    pub const fn new() -> Self {{")
            out.append(f"        Self {{ _phantom: core::marker::PhantomData }}")
            out.append("    }")
        out.append("")
        
        # Encode to buffer
        out.append("    /// Encode to buffer, returns size written")
        out.append("    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {")
        out.append("        let mut idx = 0;")
        for fname, ftype, enum_type, _ in fields:
            for line in generate_field_encode(fname, ftype, enum_type):
                out.append(line)
        out.append("        Ok(idx)")
        out.append("    }")
        out.append("")
        
        # to_frame helper
        out.append("    /// Create frame (requires encoding to buffer first)")
        out.append("    pub fn to_frame<'a>(&self, data_buf: &'a [u8]) -> Frame<'a> {")
        out.append(f"        Frame::new(Self::CMD_ID, data_buf, !Self::IS_REQUEST)")
        out.append("    }")
        out.append("}")
        out.append("")
        
        # TryFrom implementation
        out.append(f"impl{lifetime} {name}{lifetime} {{")
        out.append("    /// Decode from slice")
        out.append("    pub fn decode<'a>(data: &'a [u8]) -> Result<Self, DecodeError> {")
        if not fields:
            out.append("        Ok(Self { _phantom: core::marker::PhantomData })")
        else:
            out.append("        let mut idx = 0usize;")
            for fname, ftype, enum_type, _ in fields:
                for line in generate_field_decode(fname, ftype, enum_type):
                    out.append(line)
            out.append(f"        Ok({name} {{")
            for fname, _, _, _ in fields:
                out.append(f"            {fname},")
            out.append("        })")
        out.append("    }")
        out.append("}")
        out.append("")
        
        # Default implementation
        out.append(f"impl{lifetime} Default for {name}{lifetime} {{")
        out.append("    fn default() -> Self {")
        if not fields:
            out.append("        Self::new()")
        else:
            default_args = []
            for fname, ftype, enum_type, _ in fields:
                default_val = generate_default_value(ftype, enum_type)
                default_args.append(default_val)
            out.append(f"        Self::new({', '.join(default_args)})")
        out.append("    }")
        out.append("}")
        out.append("")

    # ===== UNIFIED MESSAGE ENUM =====
    out.append("// ============================================================================")
    out.append("// Unified Message Enum (with lifetime)")
    out.append("// ============================================================================")
    out.append("")
    out.append("/// All possible SIYI protocol messages")
    out.append("#[derive(Debug, Clone, Copy, PartialEq)]")
    out.append("pub enum Message {")

    for cmd_id in sorted(messages_by_id.keys()):
        for direction in ['request', 'response']:
            if direction in messages_by_id[cmd_id]:
                name = messages_by_id[cmd_id][direction][0]
                desc = messages_by_id[cmd_id][direction][2]
                fields = messages_by_id[cmd_id][direction][1]
                has_slice = any(ftype == "bytes" for _, ftype, _, _ in fields)
                lifetime = "<'a>" if has_slice else ""
                msg_type = "Request" if direction == 'request' else "Response"
                if desc:
                    out.append(f"    /// {desc} ({msg_type})")
                out.append(f"    {name}({name}{lifetime}),")

    out.append("}")
    out.append("")

    out.append("impl Message {")
    out.append("    /// Get command ID")
    out.append("    #[inline]")
    out.append("    pub const fn cmd_id(&self) -> u8 {")
    out.append("        match self {")

    for cmd_id in sorted(messages_by_id.keys()):
        for direction in ['request', 'response']:
            if direction in messages_by_id[cmd_id]:
                name = messages_by_id[cmd_id][direction][0]
                out.append(f"            Message::{name}(_) => 0x{cmd_id:02X},")

    out.append("        }")
    out.append("    }")
    out.append("")

    out.append("    /// Check if request")
    out.append("    #[inline]")
    out.append("    pub const fn is_request(&self) -> bool {")
    out.append("        match self {")

    for cmd_id in sorted(messages_by_id.keys()):
        for direction in ['request', 'response']:
            if direction in messages_by_id[cmd_id]:
                name = messages_by_id[cmd_id][direction][0]
                is_req = direction == 'request'
                out.append(f"            Message::{name}(_) => {str(is_req).lower()},")

    out.append("        }")
    out.append("    }")
    out.append("")

    out.append("    /// Check if response")
    out.append("    #[inline]")
    out.append("    pub const fn is_response(&self) -> bool {")
    out.append("        !self.is_request()")
    out.append("    }")
    out.append("")

    out.append("    /// Encode message to buffer")
    out.append("    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {")
    out.append("        match self {")

    for cmd_id in sorted(messages_by_id.keys()):
        for direction in ['request', 'response']:
            if direction in messages_by_id[cmd_id]:
                name = messages_by_id[cmd_id][direction][0]
                out.append(f"            Message::{name}(msg) => msg.encode(buf),")

    out.append("        }")
    out.append("    }")
    out.append("")

    out.append("    /// Decode from frame")
    out.append("    pub fn from_frame(frame: &Frame) -> Result<Message, DecodeError> {")
    out.append("        match frame.cmd {")

    for cmd_id in sorted(messages_by_id.keys()):
        out.append(f"            0x{cmd_id:02X} => {{")
        
        has_request = 'request' in messages_by_id[cmd_id]
        has_response = 'response' in messages_by_id[cmd_id]
        
        if has_request and has_response:
            req_name = messages_by_id[cmd_id]['request'][0]
            resp_name = messages_by_id[cmd_id]['response'][0]
            out.append(f"                if frame.ctrl.is_ack {{")
            out.append(f"                    let msg = {resp_name}::decode(frame.data)?;")
            out.append(f"                    Ok(Message::{resp_name}(msg))")
            out.append(f"                }} else {{")
            out.append(f"                    let msg = {req_name}::decode(frame.data)?;")
            out.append(f"                    Ok(Message::{req_name}(msg))")
            out.append(f"                }}")
        elif has_request:
            req_name = messages_by_id[cmd_id]['request'][0]
            out.append(f"                let msg = {req_name}::decode(frame.data)?;")
            out.append(f"                Ok(Message::{req_name}(msg))")
        else:
            resp_name = messages_by_id[cmd_id]['response'][0]
            out.append(f"                let msg = {resp_name}::decode(frame.data)?;")
            out.append(f"                Ok(Message::{resp_name}(msg))")
        
        out.append(f"            }}")

    out.append("            _ => Err(DecodeError::UnknownCmdId),")
    out.append("        }")
    out.append("    }")
    out.append("}")
    out.append("")

    # ===== HELPER FUNCTIONS =====
    out.append("// ============================================================================")
    out.append("// Helper Functions")
    out.append("// ============================================================================")
    out.append("")
    out.append("/// Encode a message to frame buffer")
    out.append("pub fn encode_message<'a>(msg: &Message, frame_buf: &mut [u8], data_buf: &mut [u8]) -> Result<usize, EncodeError> {")
    out.append("    let data_len = msg.encode(data_buf)?;")
    out.append("    let frame = Frame::new(msg.cmd_id(), &data_buf[..data_len], msg.is_response());")
    out.append("    frame.encode(frame_buf)")
    out.append("}")
    out.append("")
    out.append("/// Decode frame from buffer")
    out.append("pub fn decode_frame<'a>(data: &'a [u8]) -> Result<Frame<'a>, DecodeError> {")
    out.append("    Frame::decode(data)")
    out.append("}")
    out.append("")
    out.append("/// Decode message from buffer")
    out.append("pub fn decode_message<'a>(data: &'a [u8]) -> Result<Message, DecodeError> {")
    out.append("    let frame = Frame::decode(data)?;")
    out.append("    Message::from_frame(&frame)")
    out.append("}")
    out.append("")

    # ===== TESTS =====
    out.append("#[cfg(test)]")
    out.append("mod tests {")
    out.append("    use super::*;")
    out.append("")
    out.append("    #[test]")
    out.append("    fn test_ctrl_byte() {")
    out.append("        let ctrl = CtrlByte::request();")
    out.append("        assert!(ctrl.is_request());")
    out.append("        assert_eq!(ctrl.to_u8() & 0x01, 0x01);")
    out.append("    }")
    out.append("")
    out.append("    #[test]")
    out.append("    fn test_crc16() {")
    out.append("        let data = [0x55, 0x66, 0x01];")
    out.append("        let crc = crc16_calc(&data, 0);")
    out.append("        assert!(crc > 0);")
    out.append("    }")
    out.append("")
    out.append("    #[test]")
    out.append("    fn test_frame_encode_decode() {")
    out.append("        let data = [1, 2, 3, 4];")
    out.append("        let frame = Frame::new(0x01, &data, false);")
    out.append("        ")
    out.append("        let mut buf = [0u8; MAX_FRAME_SIZE];")
    out.append("        let len = frame.encode(&mut buf).unwrap();")
    out.append("        ")
    out.append("        let decoded = Frame::decode(&buf[..len]).unwrap();")
    out.append("        assert_eq!(frame.cmd, decoded.cmd);")
    out.append("        assert_eq!(frame.data, decoded.data);")
    out.append("    }")
    out.append("")

    if all_messages:
        first_msg = all_messages[0]
        msg_name = first_msg[0]
        
        out.append("    #[test]")
        out.append(f"    fn test_{msg_name.lower()}_roundtrip() {{")
        out.append(f"        let msg = {msg_name}::default();")
        out.append(f"        let mut buf = [0u8; MAX_MESSAGE_SIZE];")
        out.append(f"        let len = msg.encode(&mut buf).unwrap();")
        out.append(f"        let decoded = {msg_name}::decode(&buf[..len]).unwrap();")
        out.append(f"        assert_eq!(msg, decoded);")
        out.append("    }")
        out.append("")

    out.append("    #[test]")
    out.append("    fn test_buffer_too_small() {")
    out.append("        let data = [1, 2, 3, 4];")
    out.append("        let frame = Frame::new(0x01, &data, false);")
    out.append("        let mut buf = [0u8; 5]; // Too small")
    out.append("        assert_eq!(frame.encode(&mut buf), Err(EncodeError::BufferTooSmall));")
    out.append("    }")
    out.append("}")
    out.append("")

    # ===== DOCUMENTATION =====
    out.append("// ============================================================================")
    out.append("// Module Documentation")
    out.append("// ============================================================================")
    out.append("")
    out.append("/// # SIYI Gimbal Protocol (No_std Compatible)")
    out.append("///")
    out.append("/// Bare-metal compatible Rust implementation with zero heap allocations.")
    out.append("///")
    out.append("/// ## Key Features")
    out.append("///")
    out.append("/// - **No allocations**: All operations use stack-based buffers")
    out.append("/// - **Fixed-size buffers**: Compile-time known maximum sizes")
    out.append("/// - **Zero-copy slices**: Efficient data handling with lifetimes")
    out.append("/// - **Simple errors**: No String/format! requirements")
    out.append("///")
    out.append("/// ## Usage Example")
    out.append("///")
    out.append("/// ```rust")
    out.append("/// # use siyi_protocol::*;")
    out.append("/// let mut msg_buf = [0u8; MAX_MESSAGE_SIZE];")
    out.append("/// let mut frame_buf = [0u8; MAX_FRAME_SIZE];")
    out.append("///")
    out.append("/// // Create and encode a message")
    out.append("/// let req = FirmwareVersionRequest::new();")
    out.append("/// let msg_len = req.encode(&mut msg_buf).unwrap();")
    out.append("///")
    out.append("/// // Create frame")
    out.append("/// let frame = req.to_frame(&msg_buf[..msg_len]);")
    out.append("/// let frame_len = frame.encode(&mut frame_buf).unwrap();")
    out.append("///")
    out.append("/// // Send frame_buf[..frame_len] over serial/network")
    out.append("///")
    out.append("/// // Decode received data")
    out.append("/// let decoded_frame = Frame::decode(&frame_buf[..frame_len]).unwrap();")
    out.append("/// let decoded_msg = Message::from_frame(&decoded_frame).unwrap();")
    out.append("/// ```")
    out.append("#[allow(unused)]")
    out.append("const _DOCUMENTATION: () = ();")

    # Write output
    src = "\n".join(out)

    if out_path:
        with open(out_path, 'w') as f:
            f.write(src)
        print(f"Generated no_std Rust code written to: {out_path}")
    else:
        print(src)

def main():
    parser = argparse.ArgumentParser(
        description='Generate no_std Rust code from SIYI Protocol XML',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
    Examples:
    %(prog)s protocol.xml                          # Print to stdout (no_std)
    %(prog)s protocol.xml -o lib.rs                # Write to file (no_std)
    %(prog)s protocol.xml --std -o lib.rs          # Generate with std support
        """
    )
    parser.add_argument('xml_file', help='Path to protocol XML file')
    parser.add_argument('-o', '--output', help='Output file path (stdout if not specified)')
    parser.add_argument('--std', action='store_true', help='Generate with std support (default: no_std)')

    args = parser.parse_args()

    try:
        xml_to_rust(args.xml_file, args.output, use_std=args.std)
    except FileNotFoundError:
        print(f"Error: File '{args.xml_file}' not found", file=sys.stderr)
        sys.exit(1)
    except ET.ParseError as e:
        print(f"Error: Failed to parse XML: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()