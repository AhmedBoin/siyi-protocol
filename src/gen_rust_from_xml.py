#!/usr/bin/env python3
"""
SIYI Protocol XML to Rust Code Generator (V2)
Converts XML protocol definitions to type-safe Rust code with unified message handling
- CTRL field split into need_ack (bit 0) and is_ack (bit 1)
- Single Message enum combining request/response with automatic type detection
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
        return "Vec<u8>"
    else:
        return "Vec<u8>"

def generate_field_encode(fname: str, ftype: str, enum_type: Optional[str], indent: str = "        ") -> List[str]:
    """Generate encoding code for a field"""
    lines = []
    if ftype == 'enum':
        lines.append(f"{indent}buf.push(self.{fname} as u8);")
    elif ftype in PRIMITIVE_TYPES:
        lines.append(f"{indent}buf.extend(&self.{fname}.to_le_bytes());")
    elif ftype.startswith("bytes["):
        lines.append(f"{indent}buf.extend(&self.{fname});")
    else:
        lines.append(f"{indent}buf.extend(&self.{fname});")
    return lines

def generate_field_decode(fname: str, ftype: str, enum_type: Optional[str], indent: str = "        ") -> List[str]:
    """Generate decoding code for a field"""
    lines = []
    if ftype == 'enum':
        lines.append(f"{indent}if idx >= data.len() {{")
        lines.append(f"{indent}    return Err(format!(\"Not enough bytes for field '{fname}'\"));")
        lines.append(f"{indent}}}")
        lines.append(f"{indent}let {fname} = {enum_type}::from_u8(data[idx])")
        lines.append(f"{indent}    .ok_or_else(|| format!(\"Invalid {enum_type} value: {{}}\", data[idx]))?;")
        lines.append(f"{indent}idx += 1;")
    elif ftype in PRIMITIVE_TYPES:
        rustt, size = PRIMITIVE_TYPES[ftype]
        lines.append(f"{indent}if data.len() < idx + {size} {{")
        lines.append(f"{indent}    return Err(format!(\"Not enough bytes for field '{fname}'\"));")
        lines.append(f"{indent}}}")
        lines.append(f"{indent}let {fname} = {rustt}::from_le_bytes(data[idx..idx+{size}].try_into().unwrap());")
        lines.append(f"{indent}idx += {size};")
    elif ftype.startswith("bytes["):
        size = ftype[6:-1]
        lines.append(f"{indent}if data.len() < idx + {size} {{")
        lines.append(f"{indent}    return Err(format!(\"Not enough bytes for field '{fname}'\"));")
        lines.append(f"{indent}}}")
        lines.append(f"{indent}let {fname}: [u8; {size}] = data[idx..idx+{size}].try_into().unwrap();")
        lines.append(f"{indent}idx += {size};")
    else:
        lines.append(f"{indent}let {fname} = data[idx..].to_vec();")
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
        return "Vec::new()"

def xml_to_rust(protocol_xml_path: str, out_path: Optional[str] = None):
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
    
    out = []
    
    # ===== HEADER =====
    out.append("// Auto-generated by SIYI Protocol XML to Rust Generator (V2)")
    out.append(f"// Protocol: {proto_name}")
    out.append("// DO NOT EDIT - This file is automatically generated")
    out.append("")
    out.append("#![allow(dead_code, clippy::derivable_impls, unused, non_snake_case)]")
    out.append("")
    out.append("use std::convert::TryInto;")
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
    out.append("pub fn crc16_calc(data: &[u8], crc_init: u16) -> u16 {")
    out.append("    let mut crc = crc_init;")
    out.append("    for &byte in data {")
    out.append("        let temp = (crc >> 8) as u8;")
    out.append("        let oldcrc16 = CRC16_TAB[(byte ^ temp) as usize];")
    out.append("        crc = (crc << 8) ^ oldcrc16;")
    out.append("    }")
    out.append("    crc")
    out.append("}")
    out.append("")
    
    # ===== CONSTANTS =====
    out.append("// ============================================================================")
    out.append("// Protocol Constants")
    out.append("// ============================================================================")
    out.append("")
    out.append(f"pub const STX: u16 = {stx};")
    out.append(f"pub const STX_LITTLE: bool = {str(stx_little).lower()};")
    out.append("")
    
    # ===== CTRL STRUCTURE =====
    out.append("// ============================================================================")
    out.append("// CTRL Field Structure")
    out.append("// ============================================================================")
    out.append("")
    out.append("/// Control byte: bit 0 = need_ack, bit 1 = is_ack")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")
    out.append("pub struct CtrlByte {")
    out.append("    /// Bit 0: 1 if ACK is needed/this is ACK response")
    out.append("    pub need_ack: bool,")
    out.append("    /// Bit 1: 1 if this is an ACK response, 0 if request")
    out.append("    pub is_ack: bool,")
    out.append("}")
    out.append("")
    out.append("impl CtrlByte {")
    out.append("    /// Create from u8 byte")
    out.append("    pub fn from_u8(val: u8) -> Self {")
    out.append("        CtrlByte {")
    out.append("            need_ack: (val & 0x01) != 0,")
    out.append("            is_ack: (val & 0x02) != 0,")
    out.append("        }")
    out.append("    }")
    out.append("")
    out.append("    /// Convert to u8 byte")
    out.append("    pub fn to_u8(&self) -> u8 {")
    out.append("        let mut val: u8 = 0;")
    out.append("        if self.need_ack { val |= 0x01; }")
    out.append("        if self.is_ack { val |= 0x02; }")
    out.append("        val")
    out.append("    }")
    out.append("")
    out.append("    /// Create request (need_ack=true, is_ack=false)")
    out.append("    pub fn request() -> Self {")
    out.append("        CtrlByte { need_ack: true, is_ack: false }")
    out.append("    }")
    out.append("")
    out.append("    /// Create response (is_ack=true)")
    out.append("    pub fn response() -> Self {")
    out.append("        CtrlByte { need_ack: false, is_ack: true }")
    out.append("    }")
    out.append("")
    out.append("    /// Check if this is a request")
    out.append("    pub fn is_request(&self) -> bool {")
    out.append("        !self.is_ack")
    out.append("    }")
    out.append("")
    out.append("    /// Check if this is a response")
    out.append("    pub fn is_response(&self) -> bool {")
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
            out.append(f"    pub fn from_u8(val: u8) -> Option<Self> {{")
            out.append("        match val {")
            for vname, vvalue, _ in variants:
                out.append(f"            {vvalue} => Some({enum_name}::{vname}),")
            out.append("            _ => None,")
            out.append("        }")
            out.append("    }")
            out.append("")
            out.append(f"    pub fn to_u8(self) -> u8 {{")
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
    out.append("// Protocol Frame Structure")
    out.append("// ============================================================================")
    out.append("")
    out.append("/// SIYI protocol frame with header, data, and CRC")
    out.append("#[derive(Debug, Clone, PartialEq)]")
    out.append("pub struct Frame {")
    out.append("    pub ctrl: CtrlByte,")
    out.append("    pub seq: u16,")
    out.append("    pub cmd: u8,")
    out.append("    pub data: Vec<u8>,")
    out.append("}")
    out.append("")
    out.append("impl Frame {")
    out.append("    /// Create a new frame")
    out.append("    pub fn new(cmd: u8, data: Vec<u8>, is_response: bool) -> Self {")
    out.append("        Self {")
    out.append("            ctrl: if is_response { CtrlByte::response() } else { CtrlByte::request() },")
    out.append("            seq: 0,")
    out.append("            cmd,")
    out.append("            data,")
    out.append("        }")
    out.append("    }")
    out.append("")
    out.append("    /// Encode frame to bytes with CRC")
    out.append("    pub fn encode(&self) -> Vec<u8> {")
    out.append("        let mut buf: Vec<u8> = Vec::new();")
    out.append("        if STX_LITTLE {")
    out.append("            buf.extend(&STX.to_le_bytes());")
    out.append("        } else {")
    out.append("            buf.extend(&STX.to_be_bytes());")
    out.append("        }")
    out.append("        buf.push(self.ctrl.to_u8());")
    out.append("        let datalen: u16 = self.data.len().try_into().unwrap();")
    out.append("        buf.extend(&datalen.to_le_bytes());")
    out.append("        buf.extend(&self.seq.to_le_bytes());")
    out.append("        buf.push(self.cmd);")
    out.append("        buf.extend(&self.data);")
    out.append("        let crc = crc16_calc(&buf, 0);")
    out.append("        buf.extend(&crc.to_le_bytes());")
    out.append("        buf")
    out.append("    }")
    out.append("")
    out.append("    /// Decode frame from bytes")
    out.append("    pub fn decode(frame_bytes: &[u8]) -> Result<Frame, String> {")
    out.append("        const MIN_SIZE: usize = 10;")
    out.append("        if frame_bytes.len() < MIN_SIZE {")
    out.append("            return Err(format!(\"Frame too short: {} bytes\", frame_bytes.len()));")
    out.append("        }")
    out.append("")
    out.append("        let stx = if STX_LITTLE {")
    out.append("            u16::from_le_bytes(frame_bytes[0..2].try_into().unwrap())")
    out.append("        } else {")
    out.append("            u16::from_be_bytes(frame_bytes[0..2].try_into().unwrap())")
    out.append("        };")
    out.append("        if stx != STX {")
    out.append("            return Err(format!(\"Invalid STX: 0x{:04X}\", stx));")
    out.append("        }")
    out.append("")
    out.append("        let ctrl = CtrlByte::from_u8(frame_bytes[2]);")
    out.append("        let datalen = u16::from_le_bytes(frame_bytes[3..5].try_into().unwrap()) as usize;")
    out.append("        let seq = u16::from_le_bytes(frame_bytes[5..7].try_into().unwrap());")
    out.append("        let cmd = frame_bytes[7];")
    out.append("")
    out.append("        let expected_len = MIN_SIZE + datalen;")
    out.append("        if frame_bytes.len() < expected_len {")
    out.append("            return Err(format!(\"Frame incomplete\"));")
    out.append("        }")
    out.append("")
    out.append("        let data = frame_bytes[8..8 + datalen].to_vec();")
    out.append("        let crc_recv = u16::from_le_bytes(frame_bytes[8 + datalen..8 + datalen + 2].try_into().unwrap());")
    out.append("        let crc_calc = crc16_calc(&frame_bytes[0..8 + datalen], 0);")
    out.append("        if crc_recv != crc_calc {")
    out.append("            return Err(format!(\"CRC mismatch\"));")
    out.append("        }")
    out.append("")
    out.append("        Ok(Frame { ctrl, seq, cmd, data })")
    out.append("    }")
    out.append("}")
    out.append("")
    
    # ===== MESSAGE STRUCTURES =====
    out.append("// ============================================================================")
    out.append("// Message Structures")
    out.append("// ============================================================================")
    out.append("")
    
    for name, cmd_id, direction, msg_desc, fields in all_messages:
        if msg_desc:
            out.append(f"/// {msg_desc}")
        out.append(f"/// CMD_ID: 0x{cmd_id:02X} | Type: {direction.upper()}")
        out.append("#[derive(Debug, Clone, PartialEq)]")
        out.append(f"pub struct {name} {{")
        if not fields:
            out.append("    _phantom: std::marker::PhantomData<()>,")
        for fname, ftype, enum_type, fdesc in fields:
            rustt = get_rust_type(ftype, enum_type)
            if fdesc:
                out.append(f"    /// {fdesc}")
            out.append(f"    pub {fname}: {rustt},")
        out.append("}")
        out.append("")
        
        out.append(f"impl {name} {{")
        out.append(f"    pub const CMD_ID: u8 = 0x{cmd_id:02X};")
        out.append(f"    pub const IS_REQUEST: bool = {str(direction == 'request').lower()};")
        out.append("")
        
        if fields:
            params = ", ".join([f"{fname}: {get_rust_type(ftype, enum_type)}" for fname, ftype, enum_type, _ in fields])
            out.append(f"    pub fn new({params}) -> Self {{")
            out.append(f"        Self {{ {', '.join([fname for fname, _, _, _ in fields])} }}")
            out.append("    }")
        else:
            out.append(f"    pub fn new() -> Self {{")
            out.append(f"        Self {{ _phantom: std::marker::PhantomData }}")
            out.append("    }")
        out.append("")
        
        out.append("    pub fn to_bytes(&self) -> Vec<u8> {")
        out.append("        let mut buf: Vec<u8> = Vec::new();")
        for fname, ftype, enum_type, _ in fields:
            for line in generate_field_encode(fname, ftype, enum_type):
                out.append(line)
        out.append("        buf")
        out.append("    }")
        out.append("")
        
        out.append("    pub fn to_frame(&self) -> Frame {")
        out.append(f"        Frame::new(Self::CMD_ID, self.to_bytes(), !Self::IS_REQUEST)")
        out.append("    }")
        out.append("}")
        out.append("")
        
        out.append(f"impl std::convert::TryFrom<&[u8]> for {name} {{")
        out.append("    type Error = String;")
        out.append("    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {")
        if not fields:
            out.append("        Ok(Self { _phantom: std::marker::PhantomData })")
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
        
        out.append(f"impl Default for {name} {{")
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
    out.append("// Unified Message Enum")
    out.append("// ============================================================================")
    out.append("")
    out.append("/// All possible SIYI protocol messages (request and response)")
    out.append("/// Use is_request() and is_response() to distinguish message type")
    out.append("#[derive(Debug, Clone, PartialEq)]")
    out.append("pub enum Message {")
    
    for cmd_id in sorted(messages_by_id.keys()):
        for direction in ['request', 'response']:
            if direction in messages_by_id[cmd_id]:
                name = messages_by_id[cmd_id][direction][0]
                desc = messages_by_id[cmd_id][direction][2]
                msg_type = "Request" if direction == 'request' else "Response"
                if desc:
                    out.append(f"    /// {desc} ({msg_type})")
                out.append(f"    {name}({name}),")
    
    out.append("}")
    out.append("")
    
    out.append("impl Message {")
    out.append("    /// Get command ID for this message")
    out.append("    pub fn cmd_id(&self) -> u8 {")
    out.append("        match self {")
    
    for cmd_id in sorted(messages_by_id.keys()):
        for direction in ['request', 'response']:
            if direction in messages_by_id[cmd_id]:
                name = messages_by_id[cmd_id][direction][0]
                out.append(f"            Message::{name}(_) => 0x{cmd_id:02X},")
    
    out.append("        }")
    out.append("    }")
    out.append("")
    
    out.append("    /// Check if this is a request message")
    out.append("    pub fn is_request(&self) -> bool {")
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
    
    out.append("    /// Check if this is a response message")
    out.append("    pub fn is_response(&self) -> bool {")
    out.append("        !self.is_request()")
    out.append("    }")
    out.append("")
    
    out.append("    /// Serialize message to bytes")
    out.append("    pub fn to_bytes(&self) -> Vec<u8> {")
    out.append("        match self {")
    
    for cmd_id in sorted(messages_by_id.keys()):
        for direction in ['request', 'response']:
            if direction in messages_by_id[cmd_id]:
                name = messages_by_id[cmd_id][direction][0]
                out.append(f"            Message::{name}(msg) => msg.to_bytes(),")
    
    out.append("        }")
    out.append("    }")
    out.append("")
    
    out.append("    /// Convert message to frame for transmission")
    out.append("    pub fn to_frame(&self) -> Frame {")
    out.append("        match self {")
    
    for cmd_id in sorted(messages_by_id.keys()):
        for direction in ['request', 'response']:
            if direction in messages_by_id[cmd_id]:
                name = messages_by_id[cmd_id][direction][0]
                out.append(f"            Message::{name}(msg) => msg.to_frame(),")
    
    out.append("        }")
    out.append("    }")
    out.append("")
    
    out.append("    /// Decode a message from a frame")
    out.append("    pub fn from_frame(frame: &Frame) -> Result<Message, String> {")
    out.append("        match frame.cmd {")
    
    for cmd_id in sorted(messages_by_id.keys()):
        out.append(f"            0x{cmd_id:02X} => {{")
        
        has_request = 'request' in messages_by_id[cmd_id]
        has_response = 'response' in messages_by_id[cmd_id]
        
        if has_request and has_response:
            # Both request and response for this ID - distinguish by ctrl.is_ack
            req_name = messages_by_id[cmd_id]['request'][0]
            resp_name = messages_by_id[cmd_id]['response'][0]
            out.append(f"                if frame.ctrl.is_ack {{")
            out.append(f"                    let msg = {resp_name}::try_from(frame.data.as_slice())?;")
            out.append(f"                    Ok(Message::{resp_name}(msg))")
            out.append(f"                }} else {{")
            out.append(f"                    let msg = {req_name}::try_from(frame.data.as_slice())?;")
            out.append(f"                    Ok(Message::{req_name}(msg))")
            out.append(f"                }}")
        elif has_request:
            req_name = messages_by_id[cmd_id]['request'][0]
            out.append(f"                let msg = {req_name}::try_from(frame.data.as_slice())?;")
            out.append(f"                Ok(Message::{req_name}(msg))")
        else:
            resp_name = messages_by_id[cmd_id]['response'][0]
            out.append(f"                let msg = {resp_name}::try_from(frame.data.as_slice())?;")
            out.append(f"                Ok(Message::{resp_name}(msg))")
        
        out.append(f"            }}")
    
    out.append("            _ => Err(format!(\"Unknown CMD_ID: 0x{:02X}\", frame.cmd)),")
    out.append("        }")
    out.append("    }")
    out.append("}")
    out.append("")
    
    # ===== HELPER FUNCTIONS =====
    out.append("// ============================================================================")
    out.append("// Helper Functions")
    out.append("// ============================================================================")
    out.append("")
    out.append("/// Encode a message to bytes ready for transmission")
    out.append("pub fn encode_message(msg: &Message) -> Vec<u8> {")
    out.append("    msg.to_frame().encode()")
    out.append("}")
    out.append("")
    out.append("/// Decode a frame into a message")
    out.append("pub fn decode_message(data: &[u8]) -> Result<Message, String> {")
    out.append("    Message::from_frame(&Frame::decode(data)?)")
    out.append("}")
    out.append("")
    out.append("/// Encode a frame to bytes ready for transmission")
    out.append("pub fn encode_frame(msg: &Frame) -> Vec<u8> {")
    out.append("    msg.encode()")
    out.append("}")
    out.append("")
    out.append("/// Decode bytes into a frame")
    out.append("pub fn decode_frame(data: &[u8]) -> Result<Frame, String> {")
    out.append("    Frame::decode(data)")
    out.append("}")
    out.append("")
    
    # ===== TESTS =====
    out.append("#[cfg(test)]")
    out.append("mod tests {")
    out.append("    use super::*;")
    out.append("")
    out.append("    #[test]")
    out.append("    fn test_ctrl_byte_conversions() {")
    out.append("        let ctrl_req = CtrlByte::request();")
    out.append("        assert!(!ctrl_req.is_ack);")
    out.append("        assert!(ctrl_req.is_request());")
    out.append("")
    out.append("        let ctrl_resp = CtrlByte::response();")
    out.append("        assert!(ctrl_resp.is_ack);")
    out.append("        assert!(ctrl_resp.is_response());")
    out.append("")
    out.append("        let byte = ctrl_req.to_u8();")
    out.append("        let restored = CtrlByte::from_u8(byte);")
    out.append("        assert_eq!(ctrl_req, restored);")
    out.append("    }")
    out.append("")
    out.append("    #[test]")
    out.append("    fn test_crc16_calculation() {")
    out.append("        let data = [0x55, 0x66, 0x01, 0x00, 0x00, 0x01, 0x00, 0x01];")
    out.append("        let crc = crc16_calc(&data, 0);")
    out.append("        assert!(crc > 0);")
    out.append("    }")
    out.append("")
    out.append("    #[test]")
    out.append("    fn test_frame_encode_decode_roundtrip() {")
    out.append("        let original = Frame {")
    out.append("            ctrl: CtrlByte::request(),")
    out.append("            seq: 123,")
    out.append("            cmd: 0x01,")
    out.append("            data: vec![1, 2, 3, 4],")
    out.append("        };")
    out.append("")
    out.append("        let encoded = original.encode();")
    out.append("        let decoded = Frame::decode(&encoded).expect(\"Failed to decode\");")
    out.append("")
    out.append("        assert_eq!(original.ctrl, decoded.ctrl);")
    out.append("        assert_eq!(original.seq, decoded.seq);")
    out.append("        assert_eq!(original.cmd, decoded.cmd);")
    out.append("        assert_eq!(original.data, decoded.data);")
    out.append("    }")
    out.append("")
    out.append("    #[test]")
    out.append("    fn test_frame_validation() {")
    out.append("        let result = Frame::decode(&[0x55, 0x66]);")
    out.append("        assert!(result.is_err());")
    out.append("")
    out.append("        let bad_stx = [0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00];")
    out.append("        let result = Frame::decode(&bad_stx);")
    out.append("        assert!(result.is_err());")
    out.append("    }")
    out.append("")
    
    if enums:
        first_enum_name, first_variants = enums[0]
        if first_variants:
            first_variant = first_variants[0]
            out.append("    #[test]")
            out.append("    fn test_enum_conversion() {")
            out.append(f"        let val = {first_enum_name}::{first_variant[0]};")
            out.append(f"        let u8_val = val.to_u8();")
            out.append(f"        let converted = {first_enum_name}::from_u8(u8_val).unwrap();")
            out.append(f"        assert_eq!(val, converted);")
            out.append("")
            out.append(f"        let invalid = {first_enum_name}::from_u8(255);")
            out.append(f"        assert!(invalid.is_none());")
            out.append("    }")
            out.append("")
    
    if all_messages:
        first_msg = all_messages[0]
        msg_name = first_msg[0]
        
        out.append("    #[test]")
        out.append(f"    fn test_{msg_name.lower()}_message() {{")
        out.append(f"        let msg = {msg_name}::default();")
        out.append(f"        let bytes = msg.to_bytes();")
        out.append(f"        let decoded = {msg_name}::try_from(bytes.as_slice()).expect(\"Failed to decode\");")
        out.append(f"        assert_eq!(msg, decoded);")
        out.append("    }")
        out.append("")
        
        out.append("    #[test]")
        out.append(f"    fn test_{msg_name.lower()}_to_frame() {{")
        out.append(f"        let msg = {msg_name}::default();")
        out.append(f"        let frame = msg.to_frame();")
        out.append(f"        assert_eq!(frame.cmd, {msg_name}::CMD_ID);")
        out.append("    }")
        out.append("")
        
        out.append("    #[test]")
        out.append(f"    fn test_{msg_name.lower()}_message_enum() {{")
        out.append(f"        let msg_struct = {msg_name}::default();")
        out.append(f"        let msg_enum = Message::{msg_name}(msg_struct.clone());")
        out.append(f"        assert_eq!(msg_enum.cmd_id(), {msg_name}::CMD_ID);")
        out.append(f"        assert_eq!(msg_enum.is_request(), {msg_name}::IS_REQUEST);")
        out.append("    }")
        out.append("")
    
    out.append("    #[test]")
    out.append("    fn test_empty_frame() {")
    out.append("        let frame = Frame::new(0x00, vec![], false);")
    out.append("        let encoded = frame.encode();")
    out.append("        let decoded = Frame::decode(&encoded).expect(\"Failed to decode\");")
    out.append("        assert_eq!(frame.cmd, decoded.cmd);")
    out.append("        assert_eq!(frame.data.len(), 0);")
    out.append("    }")
    out.append("}")
    out.append("")
    
    # ===== DOCUMENTATION =====
    out.append("// ============================================================================")
    out.append("// Module Documentation")
    out.append("// ============================================================================")
    out.append("")
    out.append("/// # SIYI Gimbal Protocol (V2)")
    out.append("///")
    out.append("/// Type-safe Rust implementation with unified message handling.")
    out.append("///")
    out.append("/// ## Key Features")
    out.append("///")
    out.append("/// - **CtrlByte Structure**: Proper bit-field handling for request/response distinction")
    out.append("/// - **Unified Message Enum**: Single enum for all message types")
    out.append("/// - **Automatic Type Detection**: Messages identified by CMD_ID and CTRL bits")
    out.append("/// - **CRC16 Validation**: All frames verified with CRC16 checksum")
    out.append("///")
    out.append("/// ## Frame Structure")
    out.append("///")
    out.append("/// | Field    | Bytes | Description                    |")
    out.append("/// |----------|-------|--------------------------------|")
    out.append("/// | STX      | 2     | Start marker (0x6655)          |")
    out.append("/// | CTRL     | 1     | Bit 0: need_ack, Bit 1: is_ack |")
    out.append("/// | Data_len | 2     | Data field length              |")
    out.append("/// | SEQ      | 2     | Frame sequence                 |")
    out.append("/// | CMD_ID   | 1     | Command ID                     |")
    out.append("/// | DATA     | N     | Message payload                |")
    out.append("/// | CRC16    | 2     | CRC16 checksum                 |")
    out.append("///")
    out.append("/// ## Usage Example")
    out.append("///")
    out.append("/// ```rust")
    out.append("/// # use siyi_protocol::*;")
    out.append("/// // Create a request message")
    out.append("/// let req = FirmwareVersionRequest::new();")
    out.append("/// let msg = Message::FirmwareVersionRequest(req);")
    out.append("///")
    out.append("/// // Encode to bytes")
    out.append("/// let bytes = encode_message(&msg);")
    out.append("///")
    out.append("/// // Receive response bytes")
    out.append("/// let frame = decode_frame(&response_bytes)?;")
    out.append("/// let response = decode_message(&frame)?;")
    out.append("///")
    out.append("/// if response.is_response() {")
    out.append("///     println!(\"Got response for CMD 0x{:02X}\", response.cmd_id());")
    out.append("/// }")
    out.append("/// ```")
    out.append("#[allow(unused)]")
    out.append("const _DOCUMENTATION: () = ();")
    
    # Write output
    src = "\n".join(out)
    
    if out_path:
        with open(out_path, 'w') as f:
            f.write(src)
        print(f"Generated Rust code written to: {out_path}")
    else:
        print(src)

def main():
    parser = argparse.ArgumentParser(
        description='Generate Rust code from SIYI Protocol XML',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s protocol.xml                    # Print to stdout
  %(prog)s protocol.xml -o lib.rs          # Write to file
  %(prog)s protocol.xml --output siyi.rs   # Alternative syntax
        """
    )
    parser.add_argument('xml_file', help='Path to protocol XML file')
    parser.add_argument('-o', '--output', help='Output file path (stdout if not specified)')
    
    args = parser.parse_args()
    
    try:
        xml_to_rust(args.xml_file, args.output)
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