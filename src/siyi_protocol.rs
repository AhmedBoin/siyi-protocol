// Auto-generated SIYI Protocol - No Lifetimes - State Machine Parser
// Protocol: SIYI_Gimbal_Camera_External_SDK_Protocol

#![no_std]
#![allow(dead_code, unused, non_snake_case)]

use core::convert::TryInto;

pub const STX: u16 = 0x6655;
pub const MAX_MESSAGE_SIZE: usize = 512;
pub const MAX_FRAME_SIZE: usize = 522;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeError {
    BufferTooSmall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    InvalidStx,
    FrameIncomplete,
    CrcMismatch,
    NotEnoughBytes,
    InvalidEnumValue,
    ConversionError,
    UnknownCmdId,
}

const CRC16_TAB: [u16; 256] = [
    0x0, 0x1021, 0x2042, 0x3063, 0x4084, 0x50a5, 0x60c6, 0x70e7, 0x8108, 0x9129, 0xa14a, 0xb16b,
    0xc18c, 0xd1ad, 0xe1ce, 0xf1ef, 0x1231, 0x210, 0x3273, 0x2252, 0x52b5, 0x4294, 0x72f7, 0x62d6,
    0x9339, 0x8318, 0xb37b, 0xa35a, 0xd3bd, 0xc39c, 0xf3ff, 0xe3de, 0x2462, 0x3443, 0x420, 0x1401,
    0x64e6, 0x74c7, 0x44a4, 0x5485, 0xa56a, 0xb54b, 0x8528, 0x9509, 0xe5ee, 0xf5cf, 0xc5ac, 0xd58d,
    0x3653, 0x2672, 0x1611, 0x630, 0x76d7, 0x66f6, 0x5695, 0x46b4, 0xb75b, 0xa77a, 0x9719, 0x8738,
    0xf7df, 0xe7fe, 0xd79d, 0xc7bc, 0x48c4, 0x58e5, 0x6886, 0x78a7, 0x840, 0x1861, 0x2802, 0x3823,
    0xc9cc, 0xd9ed, 0xe98e, 0xf9af, 0x8948, 0x9969, 0xa90a, 0xb92b, 0x5af5, 0x4ad4, 0x7ab7, 0x6a96,
    0x1a71, 0xa50, 0x3a33, 0x2a12, 0xdbfd, 0xcbdc, 0xfbbf, 0xeb9e, 0x9b79, 0x8b58, 0xbb3b, 0xab1a,
    0x6ca6, 0x7c87, 0x4ce4, 0x5cc5, 0x2c22, 0x3c03, 0xc60, 0x1c41, 0xedae, 0xfd8f, 0xcdec, 0xddcd,
    0xad2a, 0xbd0b, 0x8d68, 0x9d49, 0x7e97, 0x6eb6, 0x5ed5, 0x4ef4, 0x3e13, 0x2e32, 0x1e51, 0xe70,
    0xff9f, 0xefbe, 0xdfdd, 0xcffc, 0xbf1b, 0xaf3a, 0x9f59, 0x8f78, 0x9188, 0x81a9, 0xb1ca, 0xa1eb,
    0xd10c, 0xc12d, 0xf14e, 0xe16f, 0x1080, 0xa1, 0x30c2, 0x20e3, 0x5004, 0x4025, 0x7046, 0x6067,
    0x83b9, 0x9398, 0xa3fb, 0xb3da, 0xc33d, 0xd31c, 0xe37f, 0xf35e, 0x2b1, 0x1290, 0x22f3, 0x32d2,
    0x4235, 0x5214, 0x6277, 0x7256, 0xb5ea, 0xa5cb, 0x95a8, 0x8589, 0xf56e, 0xe54f, 0xd52c, 0xc50d,
    0x34e2, 0x24c3, 0x14a0, 0x481, 0x7466, 0x6447, 0x5424, 0x4405, 0xa7db, 0xb7fa, 0x8799, 0x97b8,
    0xe75f, 0xf77e, 0xc71d, 0xd73c, 0x26d3, 0x36f2, 0x691, 0x16b0, 0x6657, 0x7676, 0x4615, 0x5634,
    0xd94c, 0xc96d, 0xf90e, 0xe92f, 0x99c8, 0x89e9, 0xb98a, 0xa9ab, 0x5844, 0x4865, 0x7806, 0x6827,
    0x18c0, 0x8e1, 0x3882, 0x28a3, 0xcb7d, 0xdb5c, 0xeb3f, 0xfb1e, 0x8bf9, 0x9bd8, 0xabbb, 0xbb9a,
    0x4a75, 0x5a54, 0x6a37, 0x7a16, 0xaf1, 0x1ad0, 0x2ab3, 0x3a92, 0xfd2e, 0xed0f, 0xdd6c, 0xcd4d,
    0xbdaa, 0xad8b, 0x9de8, 0x8dc9, 0x7c26, 0x6c07, 0x5c64, 0x4c45, 0x3ca2, 0x2c83, 0x1ce0, 0xcc1,
    0xef1f, 0xff3e, 0xcf5d, 0xdf7c, 0xaf9b, 0xbfba, 0x8fd9, 0x9ff8, 0x6e17, 0x7e36, 0x4e55, 0x5e74,
    0x2e93, 0x3eb2, 0xed1, 0x1ef0,
];

pub const fn crc16_calc(data: &[u8]) -> u16 {
    let mut crc = 0u16;
    let mut i = 0;
    while i < data.len() {
        crc = (crc << 8) ^ CRC16_TAB[((crc >> 8) as u8 ^ data[i]) as usize];
        i += 1;
    }
    crc
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CtrlByte {
    pub need_ack: bool,
    pub is_ack: bool,
}
impl CtrlByte {
    pub const fn from_u8(v: u8) -> Self {
        Self {
            need_ack: (v & 1) != 0,
            is_ack: (v & 2) != 0,
        }
    }
    pub const fn to_u8(&self) -> u8 {
        (self.need_ack as u8) | ((self.is_ack as u8) << 1)
    }
    pub const fn request() -> Self {
        Self {
            need_ack: true,
            is_ack: false,
        }
    }
    pub const fn response() -> Self {
        Self {
            need_ack: false,
            is_ack: true,
        }
    }
}
impl Default for CtrlByte {
    fn default() -> Self {
        Self::request()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanStatus {
    /// Failed or Disabled
    Failed = 0,
    /// Success or Enabled
    Success = 1,
}
impl BooleanStatus {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Failed),
            1 => Some(Self::Success),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for BooleanStatus {
    fn default() -> Self {
        Self::Failed
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanOnOff {
    /// Off or Disabled
    Off = 0,
    /// On or Enabled
    On = 1,
}
impl BooleanOnOff {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Off),
            1 => Some(Self::On),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for BooleanOnOff {
    fn default() -> Self {
        Self::Off
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GimbalMode {
    /// Lock mode - gimbal maintains absolute orientation
    Lock = 0,
    /// Follow mode - gimbal follows vehicle yaw
    Follow = 1,
    /// FPV mode - gimbal follows all vehicle movements
    FPV = 2,
}
impl GimbalMode {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Lock),
            1 => Some(Self::Follow),
            2 => Some(Self::FPV),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for GimbalMode {
    fn default() -> Self {
        Self::Lock
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GimbalMountingDir {
    Reserved = 0,
    /// Normal mounting
    Normal = 1,
    /// Upside down mounting
    Inverted = 2,
}
impl GimbalMountingDir {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Reserved),
            1 => Some(Self::Normal),
            2 => Some(Self::Inverted),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for GimbalMountingDir {
    fn default() -> Self {
        Self::Reserved
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoOutput {
    /// HDMI output ON, CVBS OFF
    HDMI = 0,
    /// HDMI output OFF, CVBS ON
    CVBS = 1,
}
impl VideoOutput {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::HDMI),
            1 => Some(Self::CVBS),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for VideoOutput {
    fn default() -> Self {
        Self::HDMI
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingStatus {
    NotRecording = 0,
    Recording = 1,
    /// No TF card
    NoCard = 2,
    /// Video data loss
    DataLoss = 3,
}
impl RecordingStatus {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::NotRecording),
            1 => Some(Self::Recording),
            2 => Some(Self::NoCard),
            3 => Some(Self::DataLoss),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for RecordingStatus {
    fn default() -> Self {
        Self::NotRecording
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionType {
    TakePhoto = 0,
    /// Not supported
    HDRToggle = 1,
    StartRecording = 2,
    LockMode = 3,
    FollowMode = 4,
    FPVMode = 5,
    /// Requires reboot
    EnableHDMI = 6,
    /// Requires reboot
    EnableCVBS = 7,
    /// Disable HDMI/CVBS, requires reboot
    DisableVideo = 8,
    TiltDownward = 9,
    ZoomLinkage = 10,
}
impl FunctionType {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::TakePhoto),
            1 => Some(Self::HDRToggle),
            2 => Some(Self::StartRecording),
            3 => Some(Self::LockMode),
            4 => Some(Self::FollowMode),
            5 => Some(Self::FPVMode),
            6 => Some(Self::EnableHDMI),
            7 => Some(Self::EnableCVBS),
            8 => Some(Self::DisableVideo),
            9 => Some(Self::TiltDownward),
            10 => Some(Self::ZoomLinkage),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for FunctionType {
    fn default() -> Self {
        Self::TakePhoto
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedbackInfoType {
    PhotoSuccess = 0,
    PhotoFailed = 1,
    HDROn = 2,
    HDROff = 3,
    RecordFailed = 4,
    RecordStarted = 5,
    RecordStopped = 6,
}
impl FeedbackInfoType {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::PhotoSuccess),
            1 => Some(Self::PhotoFailed),
            2 => Some(Self::HDROn),
            3 => Some(Self::HDROff),
            4 => Some(Self::RecordFailed),
            5 => Some(Self::RecordStarted),
            6 => Some(Self::RecordStopped),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for FeedbackInfoType {
    fn default() -> Self {
        Self::PhotoSuccess
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CenterPosition {
    /// One-key center
    CenterOnly = 1,
    /// Center then look down
    CenterDownward = 2,
    Center = 3,
    Downward = 4,
}
impl CenterPosition {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::CenterOnly),
            2 => Some(Self::CenterDownward),
            3 => Some(Self::Center),
            4 => Some(Self::Downward),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for CenterPosition {
    fn default() -> Self {
        Self::CenterOnly
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PseudoColor {
    WhiteHot = 0,
    Reserved = 1,
    Sepia = 2,
    Ironbow = 3,
    Rainbow = 4,
    Night = 5,
    Aurora = 6,
    RedHot = 7,
    Jungle = 8,
    Medical = 9,
    BlackHot = 10,
    GloryHot = 11,
}
impl PseudoColor {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::WhiteHot),
            1 => Some(Self::Reserved),
            2 => Some(Self::Sepia),
            3 => Some(Self::Ironbow),
            4 => Some(Self::Rainbow),
            5 => Some(Self::Night),
            6 => Some(Self::Aurora),
            7 => Some(Self::RedHot),
            8 => Some(Self::Jungle),
            9 => Some(Self::Medical),
            10 => Some(Self::BlackHot),
            11 => Some(Self::GloryHot),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for PseudoColor {
    fn default() -> Self {
        Self::WhiteHot
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoEncType {
    H264 = 1,
    H265 = 2,
}
impl VideoEncType {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::H264),
            2 => Some(Self::H265),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for VideoEncType {
    fn default() -> Self {
        Self::H264
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamType {
    Recording = 0,
    Main = 1,
    Sub = 2,
}
impl StreamType {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Recording),
            1 => Some(Self::Main),
            2 => Some(Self::Sub),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for StreamType {
    fn default() -> Self {
        Self::Recording
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TempMeasurementFlag {
    Disable = 0,
    Once = 1,
    /// 5Hz continuous measurement
    Continuous = 2,
}
impl TempMeasurementFlag {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Disable),
            1 => Some(Self::Once),
            2 => Some(Self::Continuous),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for TempMeasurementFlag {
    fn default() -> Self {
        Self::Disable
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFrequency {
    Off = 0,
    Hz2 = 1,
    Hz4 = 2,
    Hz5 = 3,
    Hz10 = 4,
    Hz20 = 5,
    Hz50 = 6,
    Hz100 = 7,
}
impl DataFrequency {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Off),
            1 => Some(Self::Hz2),
            2 => Some(Self::Hz4),
            3 => Some(Self::Hz5),
            4 => Some(Self::Hz10),
            5 => Some(Self::Hz20),
            6 => Some(Self::Hz50),
            7 => Some(Self::Hz100),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for DataFrequency {
    fn default() -> Self {
        Self::Off
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataStreamType {
    Attitude = 1,
    LaserRange = 2,
    MagneticEncoder = 3,
    MotorVoltage = 4,
}
impl DataStreamType {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Attitude),
            2 => Some(Self::LaserRange),
            3 => Some(Self::MagneticEncoder),
            4 => Some(Self::MotorVoltage),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for DataStreamType {
    fn default() -> Self {
        Self::Attitude
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalOutputMode {
    /// 30fps normal mode
    Fps30 = 0,
    /// 25fps + temperature frame
    Fps25WithTemp = 1,
}
impl ThermalOutputMode {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Fps30),
            1 => Some(Self::Fps25WithTemp),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for ThermalOutputMode {
    fn default() -> Self {
        Self::Fps30
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalGainMode {
    LowGain = 0,
    HighGain = 1,
}
impl ThermalGainMode {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::LowGain),
            1 => Some(Self::HighGain),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for ThermalGainMode {
    fn default() -> Self {
        Self::LowGain
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AITrackingStatus {
    /// Normal tracking (AI)
    NormalTracking = 0,
    /// Can recover
    IntermittentLoss = 1,
    Lost = 2,
    UserCanceled = 3,
    /// Tracking any object
    NormalTrackingAny = 4,
}
impl AITrackingStatus {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::NormalTracking),
            1 => Some(Self::IntermittentLoss),
            2 => Some(Self::Lost),
            3 => Some(Self::UserCanceled),
            4 => Some(Self::NormalTrackingAny),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for AITrackingStatus {
    fn default() -> Self {
        Self::NormalTracking
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AITargetType {
    Human = 0,
    Car = 1,
    Bus = 2,
    Truck = 3,
    AnyObject = 255,
}
impl AITargetType {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Human),
            1 => Some(Self::Car),
            2 => Some(Self::Bus),
            3 => Some(Self::Truck),
            255 => Some(Self::AnyObject),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for AITargetType {
    fn default() -> Self {
        Self::Human
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlMode {
    AttitudeMode = 0,
    WeakMode = 1,
    MiddleMode = 2,
    FPVMode = 3,
    MotorClose = 4,
}
impl ControlMode {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::AttitudeMode),
            1 => Some(Self::WeakMode),
            2 => Some(Self::MiddleMode),
            3 => Some(Self::FPVMode),
            4 => Some(Self::MotorClose),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for ControlMode {
    fn default() -> Self {
        Self::AttitudeMode
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Picture = 0,
    TempRawFile = 1,
    RecordVideo = 2,
}
impl FileType {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Picture),
            1 => Some(Self::TempRawFile),
            2 => Some(Self::RecordVideo),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for FileType {
    fn default() -> Self {
        Self::Picture
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileNameType {
    Reserve = 0,
    Index = 1,
    TimeStamp = 2,
}
impl FileNameType {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Reserve),
            1 => Some(Self::Index),
            2 => Some(Self::TimeStamp),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for FileNameType {
    fn default() -> Self {
        Self::Reserve
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalThresholdPrecision {
    MaxAccurate = 1,
    MidAccurate = 2,
    MinAccurate = 3,
}
impl ThermalThresholdPrecision {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::MaxAccurate),
            2 => Some(Self::MidAccurate),
            3 => Some(Self::MinAccurate),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for ThermalThresholdPrecision {
    fn default() -> Self {
        Self::MaxAccurate
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleAxisControl {
    YawControl = 0,
    PitchControl = 1,
}
impl SingleAxisControl {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::YawControl),
            1 => Some(Self::PitchControl),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Default for SingleAxisControl {
    fn default() -> Self {
        Self::YawControl
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub ctrl: CtrlByte,
    pub seq: u16,
    pub cmd: u8,
    pub data: [u8; MAX_MESSAGE_SIZE],
    pub data_len: u16,
}
impl Frame {
    pub fn new(cmd: u8, is_response: bool) -> Self {
        Self {
            ctrl: if is_response {
                CtrlByte::response()
            } else {
                CtrlByte::request()
            },
            seq: 0,
            cmd,
            data: [0; MAX_MESSAGE_SIZE],
            data_len: 0,
        }
    }
    pub fn data_slice(&self) -> &[u8] {
        &self.data[..self.data_len as usize]
    }
}
impl Default for Frame {
    fn default() -> Self {
        Self::new(0, false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Stx1,
    Stx2(u8),
    Ctrl,
    Len1,
    Len2(u8),
    Seq1,
    Seq2(u8),
    Cmd,
    Data(usize),
    Crc1,
    Crc2(u8),
}

#[derive(Debug, Clone)]
pub struct FrameParser {
    state: State,
    buf: [u8; MAX_FRAME_SIZE],
    idx: usize,
    data_len: u16,
}
impl FrameParser {
    pub fn new() -> Self {
        Self {
            state: State::Stx1,
            buf: [0; MAX_FRAME_SIZE],
            idx: 0,
            data_len: 0,
        }
    }
    pub fn reset(&mut self) {
        self.state = State::Stx1;
        self.idx = 0;
        self.data_len = 0;
    }
    pub fn feed(&mut self, b: u8) -> Result<Option<Frame>, DecodeError> {
        let stx_lo = (STX & 0xFF) as u8;
        let stx_hi = (STX >> 8) as u8;
        match self.state {
            State::Stx1 => {
                if b == stx_lo {
                    self.buf[0] = b;
                    self.idx = 1;
                    self.state = State::Stx2(b);
                }
                Ok(None)
            }
            State::Stx2(_) => {
                if b == stx_hi {
                    self.buf[1] = b;
                    self.idx = 2;
                    self.state = State::Ctrl;
                } else if b == stx_lo {
                    self.buf[0] = b;
                    self.idx = 1;
                    self.state = State::Stx2(b);
                } else {
                    self.reset();
                }
                Ok(None)
            }
            State::Ctrl => {
                self.buf[self.idx] = b;
                self.idx += 1;
                self.state = State::Len1;
                Ok(None)
            }
            State::Len1 => {
                self.buf[self.idx] = b;
                self.idx += 1;
                self.state = State::Len2(b);
                Ok(None)
            }
            State::Len2(lo) => {
                self.buf[self.idx] = b;
                self.idx += 1;
                self.data_len = u16::from_le_bytes([lo, b]);
                self.state = State::Seq1;
                Ok(None)
            }
            State::Seq1 => {
                self.buf[self.idx] = b;
                self.idx += 1;
                self.state = State::Seq2(b);
                Ok(None)
            }
            State::Seq2(_) => {
                self.buf[self.idx] = b;
                self.idx += 1;
                self.state = State::Cmd;
                Ok(None)
            }
            State::Cmd => {
                self.buf[self.idx] = b;
                self.idx += 1;
                self.state = if self.data_len == 0 {
                    State::Crc1
                } else {
                    State::Data(0)
                };
                Ok(None)
            }
            State::Data(cnt) => {
                self.buf[self.idx] = b;
                self.idx += 1;
                if cnt + 1 >= self.data_len as usize {
                    self.state = State::Crc1;
                } else {
                    self.state = State::Data(cnt + 1);
                }
                Ok(None)
            }
            State::Crc1 => {
                self.buf[self.idx] = b;
                self.idx += 1;
                self.state = State::Crc2(b);
                Ok(None)
            }
            State::Crc2(_) => {
                self.buf[self.idx] = b;
                self.idx += 1;
                let frame_data = &self.buf[..self.idx - 2];
                let crc_recv = u16::from_le_bytes([self.buf[self.idx - 2], b]);
                let crc_calc = crc16_calc(frame_data);
                if crc_recv != crc_calc {
                    self.reset();
                    return Err(DecodeError::CrcMismatch);
                }
                let mut frame = Frame::default();
                frame.ctrl = CtrlByte::from_u8(self.buf[2]);
                frame.seq = u16::from_le_bytes([self.buf[5], self.buf[6]]);
                frame.cmd = self.buf[7];
                frame.data_len = self.data_len;
                frame.data[..self.data_len as usize]
                    .copy_from_slice(&self.buf[8..8 + self.data_len as usize]);
                self.reset();
                Ok(Some(frame))
            }
        }
    }
}
impl Default for FrameParser {
    fn default() -> Self {
        Self::new()
    }
}

/// TCP heartbeat keepalive
/// CMD_ID: 0x00 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct TcpHeartbeat {
    _phantom: core::marker::PhantomData<()>,
}
impl TcpHeartbeat {
    pub const CMD_ID: u8 = 0x00;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for TcpHeartbeat {
    fn default() -> Self {
        Self::new()
    }
}

/// Request firmware version
/// CMD_ID: 0x01 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct FirmwareVersionRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl FirmwareVersionRequest {
    pub const CMD_ID: u8 = 0x01;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for FirmwareVersionRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Firmware version response
/// CMD_ID: 0x01 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct FirmwareVersionResponse {
    /// Camera firmware version (ignore highest byte)
    pub camera_firmware_ver: u32,
    /// Gimbal firmware version
    pub gimbal_firmware_ver: u32,
    /// Zoom module firmware version
    pub zoom_firmware_ver: u32,
}
impl FirmwareVersionResponse {
    pub const CMD_ID: u8 = 0x01;
    pub const IS_REQUEST: bool = false;
    pub fn new(camera_firmware_ver: u32, gimbal_firmware_ver: u32, zoom_firmware_ver: u32) -> Self {
        Self {
            camera_firmware_ver,
            gimbal_firmware_ver,
            zoom_firmware_ver,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.camera_firmware_ver.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.gimbal_firmware_ver.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.zoom_firmware_ver.to_le_bytes());
        idx += 4;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let camera_firmware_ver = u32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let gimbal_firmware_ver = u32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let zoom_firmware_ver = u32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        Ok(Self {
            camera_firmware_ver,
            gimbal_firmware_ver,
            zoom_firmware_ver,
        })
    }
}
impl Default for FirmwareVersionResponse {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Get hardware ID
/// CMD_ID: 0x02 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareIdRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl HardwareIdRequest {
    pub const CMD_ID: u8 = 0x02;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for HardwareIdRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Hardware ID response
/// CMD_ID: 0x02 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareIdResponse {
    /// 10-digit hardware ID string
    pub hardware_id: [u8; 12],
}
impl HardwareIdResponse {
    pub const CMD_ID: u8 = 0x02;
    pub const IS_REQUEST: bool = false;
    pub fn new(hardware_id: [u8; 12]) -> Self {
        Self { hardware_id }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 12 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 12].copy_from_slice(&self.hardware_id);
        idx += 12;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 12 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let mut hardware_id = [0u8; 12];
        hardware_id.copy_from_slice(&data[idx..idx + 12]);
        idx += 12;
        Ok(Self { hardware_id })
    }
}
impl Default for HardwareIdResponse {
    fn default() -> Self {
        Self::new([0u8; 12])
    }
}

/// Trigger auto focus
/// CMD_ID: 0x04 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct AutoFocusRequest {
    /// 1: Trigger single autofocus
    pub auto_focus: u8,
    /// X coordinate (video stream width)
    pub touch_x: u16,
    /// Y coordinate (video stream height)
    pub touch_y: u16,
}
impl AutoFocusRequest {
    pub const CMD_ID: u8 = 0x04;
    pub const IS_REQUEST: bool = true;
    pub fn new(auto_focus: u8, touch_x: u16, touch_y: u16) -> Self {
        Self {
            auto_focus,
            touch_x,
            touch_y,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.auto_focus.to_le_bytes());
        idx += 1;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.touch_x.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.touch_y.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let auto_focus = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let touch_x = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let touch_y = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self {
            auto_focus,
            touch_x,
            touch_y,
        })
    }
}
impl Default for AutoFocusRequest {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Auto focus response
/// CMD_ID: 0x04 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct AutoFocusResponse {
    /// Success or Error
    pub status: BooleanStatus,
}
impl AutoFocusResponse {
    pub const CMD_ID: u8 = 0x04;
    pub const IS_REQUEST: bool = false;
    pub fn new(status: BooleanStatus) -> Self {
        Self { status }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.status as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let status = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { status })
    }
}
impl Default for AutoFocusResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Manual zoom control
/// CMD_ID: 0x05 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct ManualZoomRequest {
    /// 1: Zoom in, 0: Stop, -1: Zoom out
    pub zoom: i8,
}
impl ManualZoomRequest {
    pub const CMD_ID: u8 = 0x05;
    pub const IS_REQUEST: bool = true;
    pub fn new(zoom: i8) -> Self {
        Self { zoom }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.zoom.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let zoom = i8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self { zoom })
    }
}
impl Default for ManualZoomRequest {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Manual zoom response
/// CMD_ID: 0x05 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct ManualZoomResponse {
    /// Current zoom level * 10
    pub zoom_multiple: u16,
}
impl ManualZoomResponse {
    pub const CMD_ID: u8 = 0x05;
    pub const IS_REQUEST: bool = false;
    pub fn new(zoom_multiple: u16) -> Self {
        Self { zoom_multiple }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.zoom_multiple.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let zoom_multiple = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self { zoom_multiple })
    }
}
impl Default for ManualZoomResponse {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Manual focus control
/// CMD_ID: 0x06 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct ManualFocusRequest {
    /// 1: Far, 0: Stop, -1: Near
    pub focus: i8,
}
impl ManualFocusRequest {
    pub const CMD_ID: u8 = 0x06;
    pub const IS_REQUEST: bool = true;
    pub fn new(focus: i8) -> Self {
        Self { focus }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.focus.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let focus = i8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self { focus })
    }
}
impl Default for ManualFocusRequest {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Manual focus response
/// CMD_ID: 0x06 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct ManualFocusResponse {
    /// Success or Error
    pub status: BooleanStatus,
}
impl ManualFocusResponse {
    pub const CMD_ID: u8 = 0x06;
    pub const IS_REQUEST: bool = false;
    pub fn new(status: BooleanStatus) -> Self {
        Self { status }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.status as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let status = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { status })
    }
}
impl Default for ManualFocusResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Control gimbal rotation
/// CMD_ID: 0x07 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct GimbalRotationRequest {
    /// Yaw speed -100 to +100
    pub yaw: i8,
    /// Pitch speed -100 to +100
    pub pitch: i8,
}
impl GimbalRotationRequest {
    pub const CMD_ID: u8 = 0x07;
    pub const IS_REQUEST: bool = true;
    pub fn new(yaw: i8, pitch: i8) -> Self {
        Self { yaw, pitch }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.yaw.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.pitch.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let yaw = i8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pitch = i8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self { yaw, pitch })
    }
}
impl Default for GimbalRotationRequest {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

/// Gimbal rotation response
/// CMD_ID: 0x07 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct GimbalRotationResponse {
    /// Success or Error
    pub status: BooleanStatus,
}
impl GimbalRotationResponse {
    pub const CMD_ID: u8 = 0x07;
    pub const IS_REQUEST: bool = false;
    pub fn new(status: BooleanStatus) -> Self {
        Self { status }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.status as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let status = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { status })
    }
}
impl Default for GimbalRotationResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Center gimbal
/// CMD_ID: 0x08 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct CenterGimbalRequest {
    /// Center position mode
    pub center_pos: CenterPosition,
}
impl CenterGimbalRequest {
    pub const CMD_ID: u8 = 0x08;
    pub const IS_REQUEST: bool = true;
    pub fn new(center_pos: CenterPosition) -> Self {
        Self { center_pos }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.center_pos as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let center_pos = CenterPosition::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { center_pos })
    }
}
impl Default for CenterGimbalRequest {
    fn default() -> Self {
        Self::new(CenterPosition::default())
    }
}

/// Center response
/// CMD_ID: 0x08 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct CenterGimbalResponse {
    /// Success or Error
    pub status: BooleanStatus,
}
impl CenterGimbalResponse {
    pub const CMD_ID: u8 = 0x08;
    pub const IS_REQUEST: bool = false;
    pub fn new(status: BooleanStatus) -> Self {
        Self { status }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.status as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let status = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { status })
    }
}
impl Default for CenterGimbalResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Request camera system info
/// CMD_ID: 0x0A | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct CameraSystemInfoRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl CameraSystemInfoRequest {
    pub const CMD_ID: u8 = 0x0A;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for CameraSystemInfoRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Camera system info
/// CMD_ID: 0x0A | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct CameraSystemInfoResponse {
    pub reserved1: u8,
    /// HDR status
    pub hdr_status: BooleanOnOff,
    pub reserved2: u8,
    /// Recording status
    pub record_status: RecordingStatus,
    /// Current gimbal mode
    pub gimbal_motion_mode: GimbalMode,
    /// Mounting direction
    pub gimbal_mounting_dir: GimbalMountingDir,
    /// Video output status
    pub video_output: VideoOutput,
    /// Zoom linkage status
    pub zoom_linkage: BooleanOnOff,
}
impl CameraSystemInfoResponse {
    pub const CMD_ID: u8 = 0x0A;
    pub const IS_REQUEST: bool = false;
    pub fn new(
        reserved1: u8,
        hdr_status: BooleanOnOff,
        reserved2: u8,
        record_status: RecordingStatus,
        gimbal_motion_mode: GimbalMode,
        gimbal_mounting_dir: GimbalMountingDir,
        video_output: VideoOutput,
        zoom_linkage: BooleanOnOff,
    ) -> Self {
        Self {
            reserved1,
            hdr_status,
            reserved2,
            record_status,
            gimbal_motion_mode,
            gimbal_mounting_dir,
            video_output,
            zoom_linkage,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.reserved1.to_le_bytes());
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.hdr_status as u8;
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.reserved2.to_le_bytes());
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.record_status as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.gimbal_motion_mode as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.gimbal_mounting_dir as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.video_output as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.zoom_linkage as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let reserved1 = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let hdr_status = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let reserved2 = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let record_status =
            RecordingStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let gimbal_motion_mode =
            GimbalMode::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let gimbal_mounting_dir =
            GimbalMountingDir::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let video_output = VideoOutput::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let zoom_linkage = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            reserved1,
            hdr_status,
            reserved2,
            record_status,
            gimbal_motion_mode,
            gimbal_mounting_dir,
            video_output,
            zoom_linkage,
        })
    }
}
impl Default for CameraSystemInfoResponse {
    fn default() -> Self {
        Self::new(
            0,
            BooleanOnOff::default(),
            0,
            RecordingStatus::default(),
            GimbalMode::default(),
            GimbalMountingDir::default(),
            VideoOutput::default(),
            BooleanOnOff::default(),
        )
    }
}

/// Function feedback (sent by camera)
/// CMD_ID: 0x0B | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionFeedback {
    /// Feedback type
    pub info_type: FeedbackInfoType,
}
impl FunctionFeedback {
    pub const CMD_ID: u8 = 0x0B;
    pub const IS_REQUEST: bool = false;
    pub fn new(info_type: FeedbackInfoType) -> Self {
        Self { info_type }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.info_type as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let info_type =
            FeedbackInfoType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { info_type })
    }
}
impl Default for FunctionFeedback {
    fn default() -> Self {
        Self::new(FeedbackInfoType::default())
    }
}

/// Photo/video/mode control
/// CMD_ID: 0x0C | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionControl {
    /// Function to execute
    pub func_type: FunctionType,
}
impl FunctionControl {
    pub const CMD_ID: u8 = 0x0C;
    pub const IS_REQUEST: bool = true;
    pub fn new(func_type: FunctionType) -> Self {
        Self { func_type }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.func_type as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let func_type = FunctionType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { func_type })
    }
}
impl Default for FunctionControl {
    fn default() -> Self {
        Self::new(FunctionType::default())
    }
}

/// Request gimbal attitude
/// CMD_ID: 0x0D | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct GimbalAttitudeRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl GimbalAttitudeRequest {
    pub const CMD_ID: u8 = 0x0D;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for GimbalAttitudeRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Gimbal attitude data
/// CMD_ID: 0x0D | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct GimbalAttitudeResponse {
    /// Yaw angle * 10 (degrees)
    pub yaw: i16,
    /// Pitch angle * 10 (degrees)
    pub pitch: i16,
    /// Roll angle * 10 (degrees)
    pub roll: i16,
    /// Yaw angular velocity
    pub yaw_velocity: i16,
    /// Pitch angular velocity
    pub pitch_velocity: i16,
    /// Roll angular velocity
    pub roll_velocity: i16,
}
impl GimbalAttitudeResponse {
    pub const CMD_ID: u8 = 0x0D;
    pub const IS_REQUEST: bool = false;
    pub fn new(
        yaw: i16,
        pitch: i16,
        roll: i16,
        yaw_velocity: i16,
        pitch_velocity: i16,
        roll_velocity: i16,
    ) -> Self {
        Self {
            yaw,
            pitch,
            roll,
            yaw_velocity,
            pitch_velocity,
            roll_velocity,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.yaw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.pitch.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.roll.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.yaw_velocity.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.pitch_velocity.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.roll_velocity.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let yaw = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pitch = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let roll = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let yaw_velocity = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pitch_velocity = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let roll_velocity = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self {
            yaw,
            pitch,
            roll,
            yaw_velocity,
            pitch_velocity,
            roll_velocity,
        })
    }
}
impl Default for GimbalAttitudeResponse {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0)
    }
}

/// Set gimbal angles
/// CMD_ID: 0x0E | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetGimbalAttitudeRequest {
    /// Target yaw * 10 (degrees)
    pub yaw: i16,
    /// Target pitch * 10 (degrees)
    pub pitch: i16,
}
impl SetGimbalAttitudeRequest {
    pub const CMD_ID: u8 = 0x0E;
    pub const IS_REQUEST: bool = true;
    pub fn new(yaw: i16, pitch: i16) -> Self {
        Self { yaw, pitch }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.yaw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.pitch.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let yaw = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pitch = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self { yaw, pitch })
    }
}
impl Default for SetGimbalAttitudeRequest {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

/// Set attitude response
/// CMD_ID: 0x0E | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetGimbalAttitudeResponse {
    /// Current yaw * 10
    pub yaw: i16,
    /// Current pitch * 10
    pub pitch: i16,
    /// Current roll * 10
    pub roll: i16,
}
impl SetGimbalAttitudeResponse {
    pub const CMD_ID: u8 = 0x0E;
    pub const IS_REQUEST: bool = false;
    pub fn new(yaw: i16, pitch: i16, roll: i16) -> Self {
        Self { yaw, pitch, roll }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.yaw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.pitch.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.roll.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let yaw = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pitch = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let roll = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self { yaw, pitch, roll })
    }
}
impl Default for SetGimbalAttitudeResponse {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Set absolute zoom level
/// CMD_ID: 0x0F | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct AbsoluteZoomRequest {
    /// Integer part of zoom (1-30)
    pub zoom_int: u8,
    /// Decimal part (0-9)
    pub zoom_float: u8,
}
impl AbsoluteZoomRequest {
    pub const CMD_ID: u8 = 0x0F;
    pub const IS_REQUEST: bool = true;
    pub fn new(zoom_int: u8, zoom_float: u8) -> Self {
        Self {
            zoom_int,
            zoom_float,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.zoom_int.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.zoom_float.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let zoom_int = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let zoom_float = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self {
            zoom_int,
            zoom_float,
        })
    }
}
impl Default for AbsoluteZoomRequest {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

/// Absolute zoom response
/// CMD_ID: 0x0F | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct AbsoluteZoomResponse {
    /// Success
    pub status: BooleanStatus,
}
impl AbsoluteZoomResponse {
    pub const CMD_ID: u8 = 0x0F;
    pub const IS_REQUEST: bool = false;
    pub fn new(status: BooleanStatus) -> Self {
        Self { status }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.status as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let status = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { status })
    }
}
impl Default for AbsoluteZoomResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Request video stitching mode
/// CMD_ID: 0x10 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct VideoStitchingModeRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl VideoStitchingModeRequest {
    pub const CMD_ID: u8 = 0x10;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for VideoStitchingModeRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Video stitching mode
/// CMD_ID: 0x10 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct VideoStitchingModeResponse {
    /// 0-8: various stitching/non-stitching modes
    pub vdisp_mode: u8,
}
impl VideoStitchingModeResponse {
    pub const CMD_ID: u8 = 0x10;
    pub const IS_REQUEST: bool = false;
    pub fn new(vdisp_mode: u8) -> Self {
        Self { vdisp_mode }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.vdisp_mode.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let vdisp_mode = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self { vdisp_mode })
    }
}
impl Default for VideoStitchingModeResponse {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Set video stitching mode
/// CMD_ID: 0x11 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetVideoStitchingModeRequest {
    /// 0-8: stitching mode selection
    pub vdisp_mode: u8,
}
impl SetVideoStitchingModeRequest {
    pub const CMD_ID: u8 = 0x11;
    pub const IS_REQUEST: bool = true;
    pub fn new(vdisp_mode: u8) -> Self {
        Self { vdisp_mode }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.vdisp_mode.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let vdisp_mode = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self { vdisp_mode })
    }
}
impl Default for SetVideoStitchingModeRequest {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Set mode response
/// CMD_ID: 0x11 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetVideoStitchingModeResponse {
    /// Confirmed mode
    pub vdisp_mode: u8,
}
impl SetVideoStitchingModeResponse {
    pub const CMD_ID: u8 = 0x11;
    pub const IS_REQUEST: bool = false;
    pub fn new(vdisp_mode: u8) -> Self {
        Self { vdisp_mode }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.vdisp_mode.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let vdisp_mode = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self { vdisp_mode })
    }
}
impl Default for SetVideoStitchingModeResponse {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Get temperature at point
/// CMD_ID: 0x12 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct GetTemperatureAtPointRequest {
    /// X coordinate
    pub x: u16,
    /// Y coordinate
    pub y: u16,
    /// Measurement mode
    pub get_temp_flag: TempMeasurementFlag,
}
impl GetTemperatureAtPointRequest {
    pub const CMD_ID: u8 = 0x12;
    pub const IS_REQUEST: bool = true;
    pub fn new(x: u16, y: u16, get_temp_flag: TempMeasurementFlag) -> Self {
        Self {
            x,
            y,
            get_temp_flag,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.x.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.y.to_le_bytes());
        idx += 2;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.get_temp_flag as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let x = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let y = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let get_temp_flag =
            TempMeasurementFlag::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            x,
            y,
            get_temp_flag,
        })
    }
}
impl Default for GetTemperatureAtPointRequest {
    fn default() -> Self {
        Self::new(0, 0, TempMeasurementFlag::default())
    }
}

/// Point temperature
/// CMD_ID: 0x12 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct GetTemperatureAtPointResponse {
    /// Temperature * 100 (Celsius)
    pub temp: u16,
    /// X coordinate
    pub x: u16,
    /// Y coordinate
    pub y: u16,
}
impl GetTemperatureAtPointResponse {
    pub const CMD_ID: u8 = 0x12;
    pub const IS_REQUEST: bool = false;
    pub fn new(temp: u16, x: u16, y: u16) -> Self {
        Self { temp, x, y }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.x.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.y.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let x = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let y = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self { temp, x, y })
    }
}
impl Default for GetTemperatureAtPointResponse {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Measure temperature in rectangle
/// CMD_ID: 0x13 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct LocalTemperatureMeasurementRequest {
    /// Rectangle start X
    pub startx: u16,
    /// Rectangle start Y
    pub starty: u16,
    /// Rectangle end X
    pub endx: u16,
    /// Rectangle end Y
    pub endy: u16,
    /// Measurement mode
    pub get_temp_flag: TempMeasurementFlag,
}
impl LocalTemperatureMeasurementRequest {
    pub const CMD_ID: u8 = 0x13;
    pub const IS_REQUEST: bool = true;
    pub fn new(
        startx: u16,
        starty: u16,
        endx: u16,
        endy: u16,
        get_temp_flag: TempMeasurementFlag,
    ) -> Self {
        Self {
            startx,
            starty,
            endx,
            endy,
            get_temp_flag,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.startx.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.starty.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.endx.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.endy.to_le_bytes());
        idx += 2;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.get_temp_flag as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let startx = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let starty = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let endx = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let endy = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let get_temp_flag =
            TempMeasurementFlag::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            startx,
            starty,
            endx,
            endy,
            get_temp_flag,
        })
    }
}
impl Default for LocalTemperatureMeasurementRequest {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, TempMeasurementFlag::default())
    }
}

/// Local temperature data
/// CMD_ID: 0x13 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct LocalTemperatureMeasurementResponse {
    pub startx: u16,
    pub starty: u16,
    pub endx: u16,
    pub endy: u16,
    /// Max temp * 100
    pub temp_max: u16,
    /// Min temp * 100
    pub temp_min: u16,
    /// Max temp X coord
    pub temp_max_x: u16,
    /// Max temp Y coord
    pub temp_max_y: u16,
    /// Min temp X coord
    pub temp_min_x: u16,
    /// Min temp Y coord
    pub temp_min_y: u16,
}
impl LocalTemperatureMeasurementResponse {
    pub const CMD_ID: u8 = 0x13;
    pub const IS_REQUEST: bool = false;
    pub fn new(
        startx: u16,
        starty: u16,
        endx: u16,
        endy: u16,
        temp_max: u16,
        temp_min: u16,
        temp_max_x: u16,
        temp_max_y: u16,
        temp_min_x: u16,
        temp_min_y: u16,
    ) -> Self {
        Self {
            startx,
            starty,
            endx,
            endy,
            temp_max,
            temp_min,
            temp_max_x,
            temp_max_y,
            temp_min_x,
            temp_min_y,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.startx.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.starty.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.endx.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.endy.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp_max.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp_min.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp_max_x.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp_max_y.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp_min_x.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp_min_y.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let startx = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let starty = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let endx = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let endy = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp_max = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp_min = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp_max_x = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp_max_y = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp_min_x = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp_min_y = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self {
            startx,
            starty,
            endx,
            endy,
            temp_max,
            temp_min,
            temp_max_x,
            temp_max_y,
            temp_min_x,
            temp_min_y,
        })
    }
}
impl Default for LocalTemperatureMeasurementResponse {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    }
}

/// Measure global temperature
/// CMD_ID: 0x14 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalTemperatureMeasurementRequest {
    /// Measurement mode
    pub get_temp_flag: TempMeasurementFlag,
}
impl GlobalTemperatureMeasurementRequest {
    pub const CMD_ID: u8 = 0x14;
    pub const IS_REQUEST: bool = true;
    pub fn new(get_temp_flag: TempMeasurementFlag) -> Self {
        Self { get_temp_flag }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.get_temp_flag as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let get_temp_flag =
            TempMeasurementFlag::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { get_temp_flag })
    }
}
impl Default for GlobalTemperatureMeasurementRequest {
    fn default() -> Self {
        Self::new(TempMeasurementFlag::default())
    }
}

/// Global temperature data
/// CMD_ID: 0x14 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalTemperatureMeasurementResponse {
    /// Max temp * 100
    pub temp_max: u16,
    /// Min temp * 100
    pub temp_min: u16,
    pub temp_max_x: u16,
    pub temp_max_y: u16,
    pub temp_min_x: u16,
    pub temp_min_y: u16,
}
impl GlobalTemperatureMeasurementResponse {
    pub const CMD_ID: u8 = 0x14;
    pub const IS_REQUEST: bool = false;
    pub fn new(
        temp_max: u16,
        temp_min: u16,
        temp_max_x: u16,
        temp_max_y: u16,
        temp_min_x: u16,
        temp_min_y: u16,
    ) -> Self {
        Self {
            temp_max,
            temp_min,
            temp_max_x,
            temp_max_y,
            temp_min_x,
            temp_min_y,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp_max.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp_min.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp_max_x.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp_max_y.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp_min_x.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.temp_min_y.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp_max = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp_min = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp_max_x = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp_max_y = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp_min_x = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let temp_min_y = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self {
            temp_max,
            temp_min,
            temp_max_x,
            temp_max_y,
            temp_min_x,
            temp_min_y,
        })
    }
}
impl Default for GlobalTemperatureMeasurementResponse {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0)
    }
}

/// Request laser rangefinder distance
/// CMD_ID: 0x15 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct LaserDistanceRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl LaserDistanceRequest {
    pub const CMD_ID: u8 = 0x15;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for LaserDistanceRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Laser distance
/// CMD_ID: 0x15 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct LaserDistanceResponse {
    /// Distance in decimeters (dm), min 50
    pub laser_distance: u16,
}
impl LaserDistanceResponse {
    pub const CMD_ID: u8 = 0x15;
    pub const IS_REQUEST: bool = false;
    pub fn new(laser_distance: u16) -> Self {
        Self { laser_distance }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.laser_distance.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let laser_distance = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self { laser_distance })
    }
}
impl Default for LaserDistanceResponse {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Request max zoom range
/// CMD_ID: 0x16 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct MaxZoomRangeRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl MaxZoomRangeRequest {
    pub const CMD_ID: u8 = 0x16;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for MaxZoomRangeRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Max zoom range
/// CMD_ID: 0x16 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct MaxZoomRangeResponse {
    /// Max zoom integer part
    pub zoom_max_int: u8,
    /// Max zoom decimal part
    pub zoom_max_float: u8,
}
impl MaxZoomRangeResponse {
    pub const CMD_ID: u8 = 0x16;
    pub const IS_REQUEST: bool = false;
    pub fn new(zoom_max_int: u8, zoom_max_float: u8) -> Self {
        Self {
            zoom_max_int,
            zoom_max_float,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.zoom_max_int.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.zoom_max_float.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let zoom_max_int = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let zoom_max_float = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self {
            zoom_max_int,
            zoom_max_float,
        })
    }
}
impl Default for MaxZoomRangeResponse {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

/// Request target lat/lon
/// CMD_ID: 0x17 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct LaserTargetLocationRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl LaserTargetLocationRequest {
    pub const CMD_ID: u8 = 0x17;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for LaserTargetLocationRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Target location
/// CMD_ID: 0x17 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct LaserTargetLocationResponse {
    /// Longitude * 1e7 (WGS84)
    pub lon_degE7: i32,
    /// Latitude * 1e7 (WGS84)
    pub lat_degE7: i32,
}
impl LaserTargetLocationResponse {
    pub const CMD_ID: u8 = 0x17;
    pub const IS_REQUEST: bool = false;
    pub fn new(lon_degE7: i32, lat_degE7: i32) -> Self {
        Self {
            lon_degE7,
            lat_degE7,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.lon_degE7.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.lat_degE7.to_le_bytes());
        idx += 4;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let lon_degE7 = i32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let lat_degE7 = i32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        Ok(Self {
            lon_degE7,
            lat_degE7,
        })
    }
}
impl Default for LaserTargetLocationResponse {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

/// Request current zoom level
/// CMD_ID: 0x18 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct CurrentZoomRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl CurrentZoomRequest {
    pub const CMD_ID: u8 = 0x18;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for CurrentZoomRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Current zoom
/// CMD_ID: 0x18 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct CurrentZoomResponse {
    /// Zoom integer part
    pub zoom_int: u8,
    /// Zoom decimal part
    pub zoom_float: u8,
}
impl CurrentZoomResponse {
    pub const CMD_ID: u8 = 0x18;
    pub const IS_REQUEST: bool = false;
    pub fn new(zoom_int: u8, zoom_float: u8) -> Self {
        Self {
            zoom_int,
            zoom_float,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.zoom_int.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.zoom_float.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let zoom_int = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let zoom_float = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self {
            zoom_int,
            zoom_float,
        })
    }
}
impl Default for CurrentZoomResponse {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

/// Request current gimbal mode
/// CMD_ID: 0x19 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct GimbalModeRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl GimbalModeRequest {
    pub const CMD_ID: u8 = 0x19;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for GimbalModeRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Gimbal mode
/// CMD_ID: 0x19 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct GimbalModeResponse {
    /// Current mode
    pub gimbal_mode: GimbalMode,
}
impl GimbalModeResponse {
    pub const CMD_ID: u8 = 0x19;
    pub const IS_REQUEST: bool = false;
    pub fn new(gimbal_mode: GimbalMode) -> Self {
        Self { gimbal_mode }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.gimbal_mode as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let gimbal_mode = GimbalMode::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { gimbal_mode })
    }
}
impl Default for GimbalModeResponse {
    fn default() -> Self {
        Self::new(GimbalMode::default())
    }
}

/// Request thermal pseudo-color
/// CMD_ID: 0x1A | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct PseudoColorRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl PseudoColorRequest {
    pub const CMD_ID: u8 = 0x1A;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for PseudoColorRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Pseudo-color
/// CMD_ID: 0x1A | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct PseudoColorResponse {
    /// Current color palette
    pub pseudo_color: PseudoColor,
}
impl PseudoColorResponse {
    pub const CMD_ID: u8 = 0x1A;
    pub const IS_REQUEST: bool = false;
    pub fn new(pseudo_color: PseudoColor) -> Self {
        Self { pseudo_color }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.pseudo_color as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pseudo_color = PseudoColor::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { pseudo_color })
    }
}
impl Default for PseudoColorResponse {
    fn default() -> Self {
        Self::new(PseudoColor::default())
    }
}

/// Set thermal pseudo-color
/// CMD_ID: 0x1B | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetPseudoColorRequest {
    /// Color palette to set
    pub pseudo_color: PseudoColor,
}
impl SetPseudoColorRequest {
    pub const CMD_ID: u8 = 0x1B;
    pub const IS_REQUEST: bool = true;
    pub fn new(pseudo_color: PseudoColor) -> Self {
        Self { pseudo_color }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.pseudo_color as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pseudo_color = PseudoColor::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { pseudo_color })
    }
}
impl Default for SetPseudoColorRequest {
    fn default() -> Self {
        Self::new(PseudoColor::default())
    }
}

/// Set pseudo-color response
/// CMD_ID: 0x1B | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetPseudoColorResponse {
    /// Confirmed color
    pub pseudo_color: PseudoColor,
}
impl SetPseudoColorResponse {
    pub const CMD_ID: u8 = 0x1B;
    pub const IS_REQUEST: bool = false;
    pub fn new(pseudo_color: PseudoColor) -> Self {
        Self { pseudo_color }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.pseudo_color as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pseudo_color = PseudoColor::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { pseudo_color })
    }
}
impl Default for SetPseudoColorResponse {
    fn default() -> Self {
        Self::new(PseudoColor::default())
    }
}

/// Request camera encoding params
/// CMD_ID: 0x20 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct EncodingParamsRequest {
    /// Which stream
    pub stream_type: StreamType,
}
impl EncodingParamsRequest {
    pub const CMD_ID: u8 = 0x20;
    pub const IS_REQUEST: bool = true;
    pub fn new(stream_type: StreamType) -> Self {
        Self { stream_type }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.stream_type as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let stream_type = StreamType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { stream_type })
    }
}
impl Default for EncodingParamsRequest {
    fn default() -> Self {
        Self::new(StreamType::default())
    }
}

/// Encoding parameters
/// CMD_ID: 0x20 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct EncodingParamsResponse {
    pub stream_type: StreamType,
    /// H264/H265
    pub video_enc_type: VideoEncType,
    /// Width
    pub resolution_w: u16,
    /// Height
    pub resolution_h: u16,
    /// Bitrate in Kbps
    pub video_bitrate: u16,
    /// Frame rate
    pub video_framerate: u8,
}
impl EncodingParamsResponse {
    pub const CMD_ID: u8 = 0x20;
    pub const IS_REQUEST: bool = false;
    pub fn new(
        stream_type: StreamType,
        video_enc_type: VideoEncType,
        resolution_w: u16,
        resolution_h: u16,
        video_bitrate: u16,
        video_framerate: u8,
    ) -> Self {
        Self {
            stream_type,
            video_enc_type,
            resolution_w,
            resolution_h,
            video_bitrate,
            video_framerate,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.stream_type as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.video_enc_type as u8;
        idx += 1;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.resolution_w.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.resolution_h.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.video_bitrate.to_le_bytes());
        idx += 2;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.video_framerate.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let stream_type = StreamType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let video_enc_type =
            VideoEncType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let resolution_w = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let resolution_h = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let video_bitrate = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let video_framerate = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self {
            stream_type,
            video_enc_type,
            resolution_w,
            resolution_h,
            video_bitrate,
            video_framerate,
        })
    }
}
impl Default for EncodingParamsResponse {
    fn default() -> Self {
        Self::new(StreamType::default(), VideoEncType::default(), 0, 0, 0, 0)
    }
}

/// Set camera encoding params
/// CMD_ID: 0x21 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetEncodingParamsRequest {
    pub stream_type: StreamType,
    pub video_enc_type: VideoEncType,
    /// Width (1920/1280)
    pub resolution_w: u16,
    /// Height (1080/720)
    pub resolution_h: u16,
    /// Bitrate in Kbps
    pub video_bitrate: u16,
    pub reserved: u8,
}
impl SetEncodingParamsRequest {
    pub const CMD_ID: u8 = 0x21;
    pub const IS_REQUEST: bool = true;
    pub fn new(
        stream_type: StreamType,
        video_enc_type: VideoEncType,
        resolution_w: u16,
        resolution_h: u16,
        video_bitrate: u16,
        reserved: u8,
    ) -> Self {
        Self {
            stream_type,
            video_enc_type,
            resolution_w,
            resolution_h,
            video_bitrate,
            reserved,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.stream_type as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.video_enc_type as u8;
        idx += 1;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.resolution_w.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.resolution_h.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.video_bitrate.to_le_bytes());
        idx += 2;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.reserved.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let stream_type = StreamType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let video_enc_type =
            VideoEncType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let resolution_w = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let resolution_h = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let video_bitrate = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let reserved = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self {
            stream_type,
            video_enc_type,
            resolution_w,
            resolution_h,
            video_bitrate,
            reserved,
        })
    }
}
impl Default for SetEncodingParamsRequest {
    fn default() -> Self {
        Self::new(StreamType::default(), VideoEncType::default(), 0, 0, 0, 0)
    }
}

/// Set encoding response
/// CMD_ID: 0x21 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetEncodingParamsResponse {
    pub stream_type: StreamType,
    /// Success or Failure
    pub status: BooleanStatus,
}
impl SetEncodingParamsResponse {
    pub const CMD_ID: u8 = 0x21;
    pub const IS_REQUEST: bool = false;
    pub fn new(stream_type: StreamType, status: BooleanStatus) -> Self {
        Self {
            stream_type,
            status,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.stream_type as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.status as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let stream_type = StreamType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let status = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            stream_type,
            status,
        })
    }
}
impl Default for SetEncodingParamsResponse {
    fn default() -> Self {
        Self::new(StreamType::default(), BooleanStatus::default())
    }
}

/// Send aircraft attitude to gimbal
/// CMD_ID: 0x22 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SendAircraftAttitude {
    /// Time since boot (ms)
    pub time_boot_ms: u32,
    /// Roll (radians)
    pub roll: f32,
    /// Pitch (radians)
    pub pitch: f32,
    /// Yaw (radians)
    pub yaw: f32,
    /// Roll rate (rad/s)
    pub rollspeed: f32,
    /// Pitch rate (rad/s)
    pub pitchspeed: f32,
    /// Yaw rate (rad/s)
    pub yawspeed: f32,
}
impl SendAircraftAttitude {
    pub const CMD_ID: u8 = 0x22;
    pub const IS_REQUEST: bool = true;
    pub fn new(
        time_boot_ms: u32,
        roll: f32,
        pitch: f32,
        yaw: f32,
        rollspeed: f32,
        pitchspeed: f32,
        yawspeed: f32,
    ) -> Self {
        Self {
            time_boot_ms,
            roll,
            pitch,
            yaw,
            rollspeed,
            pitchspeed,
            yawspeed,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.time_boot_ms.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.roll.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.pitch.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.yaw.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.rollspeed.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.pitchspeed.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.yawspeed.to_le_bytes());
        idx += 4;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let time_boot_ms = u32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let roll = f32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pitch = f32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let yaw = f32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let rollspeed = f32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pitchspeed = f32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let yawspeed = f32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        Ok(Self {
            time_boot_ms,
            roll,
            pitch,
            yaw,
            rollspeed,
            pitchspeed,
            yawspeed,
        })
    }
}
impl Default for SendAircraftAttitude {
    fn default() -> Self {
        Self::new(0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }
}

/// Send RC channel data to gimbal
/// CMD_ID: 0x23 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SendRcChannelDataRequest {
    /// RC channel 1 value [us]
    pub chan1_raw: u16,
    /// RC channel 2 value [us]
    pub chan2_raw: u16,
    /// RC channel 3 value [us]
    pub chan3_raw: u16,
    /// RC channel 4 value [us]
    pub chan4_raw: u16,
    /// RC channel 5 value [us]
    pub chan5_raw: u16,
    /// RC channel 6 value [us]
    pub chan6_raw: u16,
    /// RC channel 7 value [us]
    pub chan7_raw: u16,
    /// RC channel 8 value [us]
    pub chan8_raw: u16,
    /// RC channel 9 value [us]
    pub chan9_raw: u16,
    /// RC channel 10 value [us]
    pub chan10_raw: u16,
    /// RC channel 11 value [us]
    pub chan11_raw: u16,
    /// RC channel 12 value [us]
    pub chan12_raw: u16,
    /// RC channel 13 value [us]
    pub chan13_raw: u16,
    /// RC channel 14 value [us]
    pub chan14_raw: u16,
    /// RC channel 15 value [us]
    pub chan15_raw: u16,
    /// RC channel 16 value [us]
    pub chan16_raw: u16,
    /// RC channel 17 value [us]
    pub chan17_raw: u16,
    /// RC channel 18 value [us]
    pub chan18_raw: u16,
    /// Total number of RC channels
    pub chancount: u8,
    /// Receive signal strength indicator [0-254], 255: invalid
    pub rssi: u8,
}
impl SendRcChannelDataRequest {
    pub const CMD_ID: u8 = 0x23;
    pub const IS_REQUEST: bool = true;
    pub fn new(
        chan1_raw: u16,
        chan2_raw: u16,
        chan3_raw: u16,
        chan4_raw: u16,
        chan5_raw: u16,
        chan6_raw: u16,
        chan7_raw: u16,
        chan8_raw: u16,
        chan9_raw: u16,
        chan10_raw: u16,
        chan11_raw: u16,
        chan12_raw: u16,
        chan13_raw: u16,
        chan14_raw: u16,
        chan15_raw: u16,
        chan16_raw: u16,
        chan17_raw: u16,
        chan18_raw: u16,
        chancount: u8,
        rssi: u8,
    ) -> Self {
        Self {
            chan1_raw,
            chan2_raw,
            chan3_raw,
            chan4_raw,
            chan5_raw,
            chan6_raw,
            chan7_raw,
            chan8_raw,
            chan9_raw,
            chan10_raw,
            chan11_raw,
            chan12_raw,
            chan13_raw,
            chan14_raw,
            chan15_raw,
            chan16_raw,
            chan17_raw,
            chan18_raw,
            chancount,
            rssi,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan1_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan2_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan3_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan4_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan5_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan6_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan7_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan8_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan9_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan10_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan11_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan12_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan13_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan14_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan15_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan16_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan17_raw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.chan18_raw.to_le_bytes());
        idx += 2;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.chancount.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.rssi.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan1_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan2_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan3_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan4_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan5_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan6_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan7_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan8_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan9_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan10_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan11_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan12_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan13_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan14_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan15_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan16_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan17_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chan18_raw = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let chancount = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let rssi = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self {
            chan1_raw,
            chan2_raw,
            chan3_raw,
            chan4_raw,
            chan5_raw,
            chan6_raw,
            chan7_raw,
            chan8_raw,
            chan9_raw,
            chan10_raw,
            chan11_raw,
            chan12_raw,
            chan13_raw,
            chan14_raw,
            chan15_raw,
            chan16_raw,
            chan17_raw,
            chan18_raw,
            chancount,
            rssi,
        })
    }
}
impl Default for SendRcChannelDataRequest {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    }
}

/// Request flight controller to send data stream
/// CMD_ID: 0x24 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct RequestFlightControllerDataStreamRequest {
    /// Data stream type
    pub data_type: DataStreamType,
    /// Output frequency
    pub data_freq: DataFrequency,
}
impl RequestFlightControllerDataStreamRequest {
    pub const CMD_ID: u8 = 0x24;
    pub const IS_REQUEST: bool = true;
    pub fn new(data_type: DataStreamType, data_freq: DataFrequency) -> Self {
        Self {
            data_type,
            data_freq,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.data_type as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.data_freq as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let data_type = DataStreamType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let data_freq = DataFrequency::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            data_type,
            data_freq,
        })
    }
}
impl Default for RequestFlightControllerDataStreamRequest {
    fn default() -> Self {
        Self::new(DataStreamType::default(), DataFrequency::default())
    }
}

/// Flight controller data stream response
/// CMD_ID: 0x24 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct RequestFlightControllerDataStreamResponse {
    /// Confirmed data type
    pub data_type: DataStreamType,
}
impl RequestFlightControllerDataStreamResponse {
    pub const CMD_ID: u8 = 0x24;
    pub const IS_REQUEST: bool = false;
    pub fn new(data_type: DataStreamType) -> Self {
        Self { data_type }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.data_type as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let data_type = DataStreamType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { data_type })
    }
}
impl Default for RequestFlightControllerDataStreamResponse {
    fn default() -> Self {
        Self::new(DataStreamType::default())
    }
}

/// Request gimbal data stream
/// CMD_ID: 0x25 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct RequestDataStreamRequest {
    /// Data stream type
    pub data_type: DataStreamType,
    /// Frequency
    pub data_freq: DataFrequency,
}
impl RequestDataStreamRequest {
    pub const CMD_ID: u8 = 0x25;
    pub const IS_REQUEST: bool = true;
    pub fn new(data_type: DataStreamType, data_freq: DataFrequency) -> Self {
        Self {
            data_type,
            data_freq,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.data_type as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.data_freq as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let data_type = DataStreamType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let data_freq = DataFrequency::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            data_type,
            data_freq,
        })
    }
}
impl Default for RequestDataStreamRequest {
    fn default() -> Self {
        Self::new(DataStreamType::default(), DataFrequency::default())
    }
}

/// Data stream config response
/// CMD_ID: 0x25 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct RequestDataStreamResponse {
    /// Confirmed data type
    pub data_type: DataStreamType,
}
impl RequestDataStreamResponse {
    pub const CMD_ID: u8 = 0x25;
    pub const IS_REQUEST: bool = false;
    pub fn new(data_type: DataStreamType) -> Self {
        Self { data_type }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.data_type as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let data_type = DataStreamType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { data_type })
    }
}
impl Default for RequestDataStreamResponse {
    fn default() -> Self {
        Self::new(DataStreamType::default())
    }
}

/// Request magnetic encoder angles
/// CMD_ID: 0x26 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct MagneticEncoderAngleRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl MagneticEncoderAngleRequest {
    pub const CMD_ID: u8 = 0x26;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for MagneticEncoderAngleRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Magnetic encoder angles
/// CMD_ID: 0x26 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct MagneticEncoderAngleResponse {
    /// Yaw * 10
    pub yaw_angle: i16,
    /// Pitch * 10
    pub pitch_angle: i16,
    /// Roll * 10
    pub roll_angle: i16,
}
impl MagneticEncoderAngleResponse {
    pub const CMD_ID: u8 = 0x26;
    pub const IS_REQUEST: bool = false;
    pub fn new(yaw_angle: i16, pitch_angle: i16, roll_angle: i16) -> Self {
        Self {
            yaw_angle,
            pitch_angle,
            roll_angle,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.yaw_angle.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.pitch_angle.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.roll_angle.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let yaw_angle = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pitch_angle = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let roll_angle = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self {
            yaw_angle,
            pitch_angle,
            roll_angle,
        })
    }
}
impl Default for MagneticEncoderAngleResponse {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Request gimbal control mode
/// CMD_ID: 0x27 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct GimbalControlModeRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl GimbalControlModeRequest {
    pub const CMD_ID: u8 = 0x27;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for GimbalControlModeRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Gimbal control mode
/// CMD_ID: 0x27 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct GimbalControlModeResponse {
    /// Current control mode
    pub control_mode: ControlMode,
}
impl GimbalControlModeResponse {
    pub const CMD_ID: u8 = 0x27;
    pub const IS_REQUEST: bool = false;
    pub fn new(control_mode: ControlMode) -> Self {
        Self { control_mode }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.control_mode as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let control_mode = ControlMode::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { control_mode })
    }
}
impl Default for GimbalControlModeResponse {
    fn default() -> Self {
        Self::new(ControlMode::default())
    }
}

/// Request weak control threshold
/// CMD_ID: 0x28 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct WeakControlThresholdRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl WeakControlThresholdRequest {
    pub const CMD_ID: u8 = 0x28;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for WeakControlThresholdRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Weak control threshold data
/// CMD_ID: 0x28 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct WeakControlThresholdResponse {
    /// Voltage limit range * 10 (10-50)
    pub weak_control_limit_value: i16,
    /// Voltage threshold * 10 (20-50)
    pub voltage_threshold: i16,
    /// Angular error threshold * 10 (30-300)
    pub angular_error_threshold: i16,
}
impl WeakControlThresholdResponse {
    pub const CMD_ID: u8 = 0x28;
    pub const IS_REQUEST: bool = false;
    pub fn new(
        weak_control_limit_value: i16,
        voltage_threshold: i16,
        angular_error_threshold: i16,
    ) -> Self {
        Self {
            weak_control_limit_value,
            voltage_threshold,
            angular_error_threshold,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.weak_control_limit_value.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.voltage_threshold.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.angular_error_threshold.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let weak_control_limit_value = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let voltage_threshold = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let angular_error_threshold = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self {
            weak_control_limit_value,
            voltage_threshold,
            angular_error_threshold,
        })
    }
}
impl Default for WeakControlThresholdResponse {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Set weak control threshold
/// CMD_ID: 0x29 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetWeakControlThresholdRequest {
    /// Voltage limit range * 10 (10-50)
    pub weak_control_limit_value: i16,
    /// Voltage threshold * 10 (20-50)
    pub voltage_threshold: i16,
    /// Angular error threshold * 10 (30-300)
    pub angular_error_threshold: i16,
}
impl SetWeakControlThresholdRequest {
    pub const CMD_ID: u8 = 0x29;
    pub const IS_REQUEST: bool = true;
    pub fn new(
        weak_control_limit_value: i16,
        voltage_threshold: i16,
        angular_error_threshold: i16,
    ) -> Self {
        Self {
            weak_control_limit_value,
            voltage_threshold,
            angular_error_threshold,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.weak_control_limit_value.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.voltage_threshold.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.angular_error_threshold.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let weak_control_limit_value = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let voltage_threshold = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let angular_error_threshold = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self {
            weak_control_limit_value,
            voltage_threshold,
            angular_error_threshold,
        })
    }
}
impl Default for SetWeakControlThresholdRequest {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Set threshold response
/// CMD_ID: 0x29 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetWeakControlThresholdResponse {
    /// Success or Failure
    pub status: BooleanStatus,
}
impl SetWeakControlThresholdResponse {
    pub const CMD_ID: u8 = 0x29;
    pub const IS_REQUEST: bool = false;
    pub fn new(status: BooleanStatus) -> Self {
        Self { status }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.status as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let status = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { status })
    }
}
impl Default for SetWeakControlThresholdResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Request motor voltage
/// CMD_ID: 0x2A | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct MotorVoltageRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl MotorVoltageRequest {
    pub const CMD_ID: u8 = 0x2A;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for MotorVoltageRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Motor voltage data
/// CMD_ID: 0x2A | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct MotorVoltageResponse {
    /// Yaw motor voltage * 1000
    pub yaw_voltage: i16,
    /// Pitch motor voltage * 1000
    pub pitch_voltage: i16,
    /// Roll motor voltage * 1000
    pub roll_voltage: i16,
}
impl MotorVoltageResponse {
    pub const CMD_ID: u8 = 0x2A;
    pub const IS_REQUEST: bool = false;
    pub fn new(yaw_voltage: i16, pitch_voltage: i16, roll_voltage: i16) -> Self {
        Self {
            yaw_voltage,
            pitch_voltage,
            roll_voltage,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.yaw_voltage.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.pitch_voltage.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.roll_voltage.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let yaw_voltage = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pitch_voltage = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let roll_voltage = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self {
            yaw_voltage,
            pitch_voltage,
            roll_voltage,
        })
    }
}
impl Default for MotorVoltageResponse {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Set UTC time
/// CMD_ID: 0x30 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetUtcTimeRequest {
    /// UNIX epoch time (microseconds)
    pub timestamp: u64,
}
impl SetUtcTimeRequest {
    pub const CMD_ID: u8 = 0x30;
    pub const IS_REQUEST: bool = true;
    pub fn new(timestamp: u64) -> Self {
        Self { timestamp }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 8 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 8].copy_from_slice(&self.timestamp.to_le_bytes());
        idx += 8;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 8 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let timestamp = u64::from_le_bytes(
            data[idx..idx + 8]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 8;
        Ok(Self { timestamp })
    }
}
impl Default for SetUtcTimeRequest {
    fn default() -> Self {
        Self::new(0)
    }
}

/// UTC time response
/// CMD_ID: 0x30 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetUtcTimeResponse {
    /// Success or Invalid
    pub status: BooleanStatus,
}
impl SetUtcTimeResponse {
    pub const CMD_ID: u8 = 0x30;
    pub const IS_REQUEST: bool = false;
    pub fn new(status: BooleanStatus) -> Self {
        Self { status }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.status as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let status = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { status })
    }
}
impl Default for SetUtcTimeResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Request gimbal system information
/// CMD_ID: 0x31 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct GimbalSystemInfoRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl GimbalSystemInfoRequest {
    pub const CMD_ID: u8 = 0x31;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for GimbalSystemInfoRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Gimbal system info
/// CMD_ID: 0x31 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct GimbalSystemInfoResponse {
    /// Laser enabled or disabled
    pub laser_state: BooleanOnOff,
}
impl GimbalSystemInfoResponse {
    pub const CMD_ID: u8 = 0x31;
    pub const IS_REQUEST: bool = false;
    pub fn new(laser_state: BooleanOnOff) -> Self {
        Self { laser_state }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.laser_state as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let laser_state = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { laser_state })
    }
}
impl Default for GimbalSystemInfoResponse {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Set laser ranging state
/// CMD_ID: 0x32 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetLaserStateRequest {
    /// Enable or Disable
    pub laser_state: BooleanOnOff,
}
impl SetLaserStateRequest {
    pub const CMD_ID: u8 = 0x32;
    pub const IS_REQUEST: bool = true;
    pub fn new(laser_state: BooleanOnOff) -> Self {
        Self { laser_state }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.laser_state as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let laser_state = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { laser_state })
    }
}
impl Default for SetLaserStateRequest {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Laser state response
/// CMD_ID: 0x32 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetLaserStateResponse {
    /// Success or Failed
    pub status: BooleanStatus,
}
impl SetLaserStateResponse {
    pub const CMD_ID: u8 = 0x32;
    pub const IS_REQUEST: bool = false;
    pub fn new(status: BooleanStatus) -> Self {
        Self { status }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.status as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let status = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { status })
    }
}
impl Default for SetLaserStateResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Request thermal output mode
/// CMD_ID: 0x33 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalOutputModeRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl ThermalOutputModeRequest {
    pub const CMD_ID: u8 = 0x33;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for ThermalOutputModeRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Thermal output mode
/// CMD_ID: 0x33 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalOutputModeResponse {
    /// Output mode
    pub mode: ThermalOutputMode,
}
impl ThermalOutputModeResponse {
    pub const CMD_ID: u8 = 0x33;
    pub const IS_REQUEST: bool = false;
    pub fn new(mode: ThermalOutputMode) -> Self {
        Self { mode }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.mode as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let mode = ThermalOutputMode::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { mode })
    }
}
impl Default for ThermalOutputModeResponse {
    fn default() -> Self {
        Self::new(ThermalOutputMode::default())
    }
}

/// Set thermal output mode
/// CMD_ID: 0x34 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalOutputModeRequest {
    /// Output mode to set
    pub mode: ThermalOutputMode,
}
impl SetThermalOutputModeRequest {
    pub const CMD_ID: u8 = 0x34;
    pub const IS_REQUEST: bool = true;
    pub fn new(mode: ThermalOutputMode) -> Self {
        Self { mode }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.mode as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let mode = ThermalOutputMode::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { mode })
    }
}
impl Default for SetThermalOutputModeRequest {
    fn default() -> Self {
        Self::new(ThermalOutputMode::default())
    }
}

/// Set thermal output mode response
/// CMD_ID: 0x34 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalOutputModeResponse {
    /// Confirmed mode
    pub mode: ThermalOutputMode,
}
impl SetThermalOutputModeResponse {
    pub const CMD_ID: u8 = 0x34;
    pub const IS_REQUEST: bool = false;
    pub fn new(mode: ThermalOutputMode) -> Self {
        Self { mode }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.mode as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let mode = ThermalOutputMode::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { mode })
    }
}
impl Default for SetThermalOutputModeResponse {
    fn default() -> Self {
        Self::new(ThermalOutputMode::default())
    }
}

/// Get single temperature frame
/// CMD_ID: 0x35 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct GetSingleTemperatureFrameRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl GetSingleTemperatureFrameRequest {
    pub const CMD_ID: u8 = 0x35;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for GetSingleTemperatureFrameRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Single temperature frame response
/// CMD_ID: 0x35 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct GetSingleTemperatureFrameResponse {
    /// Acquisition successful or failed
    pub ack: BooleanStatus,
}
impl GetSingleTemperatureFrameResponse {
    pub const CMD_ID: u8 = 0x35;
    pub const IS_REQUEST: bool = false;
    pub fn new(ack: BooleanStatus) -> Self {
        Self { ack }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.ack as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ack = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { ack })
    }
}
impl Default for GetSingleTemperatureFrameResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Request thermal gain mode
/// CMD_ID: 0x37 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalGainModeRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl ThermalGainModeRequest {
    pub const CMD_ID: u8 = 0x37;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for ThermalGainModeRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Thermal gain mode
/// CMD_ID: 0x37 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalGainModeResponse {
    /// Current gain mode
    pub ir_gain: ThermalGainMode,
}
impl ThermalGainModeResponse {
    pub const CMD_ID: u8 = 0x37;
    pub const IS_REQUEST: bool = false;
    pub fn new(ir_gain: ThermalGainMode) -> Self {
        Self { ir_gain }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.ir_gain as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ir_gain = ThermalGainMode::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { ir_gain })
    }
}
impl Default for ThermalGainModeResponse {
    fn default() -> Self {
        Self::new(ThermalGainMode::default())
    }
}

/// Set thermal gain mode
/// CMD_ID: 0x38 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalGainModeRequest {
    /// Gain mode to set
    pub ir_gain: ThermalGainMode,
}
impl SetThermalGainModeRequest {
    pub const CMD_ID: u8 = 0x38;
    pub const IS_REQUEST: bool = true;
    pub fn new(ir_gain: ThermalGainMode) -> Self {
        Self { ir_gain }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.ir_gain as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ir_gain = ThermalGainMode::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { ir_gain })
    }
}
impl Default for SetThermalGainModeRequest {
    fn default() -> Self {
        Self::new(ThermalGainMode::default())
    }
}

/// Set thermal gain mode response
/// CMD_ID: 0x38 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalGainModeResponse {
    /// Confirmed gain mode
    pub ir_gain: ThermalGainMode,
}
impl SetThermalGainModeResponse {
    pub const CMD_ID: u8 = 0x38;
    pub const IS_REQUEST: bool = false;
    pub fn new(ir_gain: ThermalGainMode) -> Self {
        Self { ir_gain }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.ir_gain as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ir_gain = ThermalGainMode::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { ir_gain })
    }
}
impl Default for SetThermalGainModeResponse {
    fn default() -> Self {
        Self::new(ThermalGainMode::default())
    }
}

/// Request thermal env correction params
/// CMD_ID: 0x39 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalEnvCorrectionParamsRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl ThermalEnvCorrectionParamsRequest {
    pub const CMD_ID: u8 = 0x39;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for ThermalEnvCorrectionParamsRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Thermal env correction params
/// CMD_ID: 0x39 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalEnvCorrectionParamsResponse {
    /// Distance (m) * 100
    pub dist: u16,
    /// Target emissivity (%) * 100
    pub ems: u16,
    /// Environmental humidity (%) * 100
    pub hum: u16,
    /// Atmospheric temperature (°C) * 100
    pub ta: u16,
    /// Reflective temperature (°C) * 100
    pub tu: u16,
}
impl ThermalEnvCorrectionParamsResponse {
    pub const CMD_ID: u8 = 0x39;
    pub const IS_REQUEST: bool = false;
    pub fn new(dist: u16, ems: u16, hum: u16, ta: u16, tu: u16) -> Self {
        Self {
            dist,
            ems,
            hum,
            ta,
            tu,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.dist.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.ems.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.hum.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.ta.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.tu.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let dist = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ems = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let hum = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ta = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let tu = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self {
            dist,
            ems,
            hum,
            ta,
            tu,
        })
    }
}
impl Default for ThermalEnvCorrectionParamsResponse {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0)
    }
}

/// Set thermal env correction params
/// CMD_ID: 0x3A | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalEnvCorrectionParamsRequest {
    /// Distance (m) * 100
    pub dist: u16,
    /// Target emissivity (%) * 100
    pub ems: u16,
    /// Environmental humidity (%) * 100
    pub hum: u16,
    /// Atmospheric temperature (°C) * 100
    pub ta: u16,
    /// Reflective temperature (°C) * 100
    pub tu: u16,
}
impl SetThermalEnvCorrectionParamsRequest {
    pub const CMD_ID: u8 = 0x3A;
    pub const IS_REQUEST: bool = true;
    pub fn new(dist: u16, ems: u16, hum: u16, ta: u16, tu: u16) -> Self {
        Self {
            dist,
            ems,
            hum,
            ta,
            tu,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.dist.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.ems.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.hum.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.ta.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.tu.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let dist = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ems = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let hum = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ta = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let tu = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self {
            dist,
            ems,
            hum,
            ta,
            tu,
        })
    }
}
impl Default for SetThermalEnvCorrectionParamsRequest {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0)
    }
}

/// Set env correction params response
/// CMD_ID: 0x3A | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalEnvCorrectionParamsResponse {
    /// Successfully set or failed
    pub ack: BooleanStatus,
}
impl SetThermalEnvCorrectionParamsResponse {
    pub const CMD_ID: u8 = 0x3A;
    pub const IS_REQUEST: bool = false;
    pub fn new(ack: BooleanStatus) -> Self {
        Self { ack }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.ack as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ack = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { ack })
    }
}
impl Default for SetThermalEnvCorrectionParamsResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Request env correction switch
/// CMD_ID: 0x3B | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalEnvCorrectionSwitchRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl ThermalEnvCorrectionSwitchRequest {
    pub const CMD_ID: u8 = 0x3B;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for ThermalEnvCorrectionSwitchRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Env correction switch
/// CMD_ID: 0x3B | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalEnvCorrectionSwitchResponse {
    /// Off or On
    pub env_correct: BooleanOnOff,
}
impl ThermalEnvCorrectionSwitchResponse {
    pub const CMD_ID: u8 = 0x3B;
    pub const IS_REQUEST: bool = false;
    pub fn new(env_correct: BooleanOnOff) -> Self {
        Self { env_correct }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.env_correct as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let env_correct = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { env_correct })
    }
}
impl Default for ThermalEnvCorrectionSwitchResponse {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Set env correction switch
/// CMD_ID: 0x3C | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalEnvCorrectionSwitchRequest {
    /// Off or On
    pub env_correct: BooleanOnOff,
}
impl SetThermalEnvCorrectionSwitchRequest {
    pub const CMD_ID: u8 = 0x3C;
    pub const IS_REQUEST: bool = true;
    pub fn new(env_correct: BooleanOnOff) -> Self {
        Self { env_correct }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.env_correct as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let env_correct = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { env_correct })
    }
}
impl Default for SetThermalEnvCorrectionSwitchRequest {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Set env correction switch response
/// CMD_ID: 0x3C | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalEnvCorrectionSwitchResponse {
    /// Off or On
    pub env_correct: BooleanOnOff,
}
impl SetThermalEnvCorrectionSwitchResponse {
    pub const CMD_ID: u8 = 0x3C;
    pub const IS_REQUEST: bool = false;
    pub fn new(env_correct: BooleanOnOff) -> Self {
        Self { env_correct }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.env_correct as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let env_correct = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { env_correct })
    }
}
impl Default for SetThermalEnvCorrectionSwitchResponse {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Send GPS raw data to gimbal
/// CMD_ID: 0x3E | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SendGpsData {
    /// Time since boot (ms)
    pub time_boot_ms: u32,
    /// Latitude [degE7]
    pub lat: i32,
    /// Longitude [degE7]
    pub lon: i32,
    /// Altitude MSL (cm)
    pub alt: i32,
    /// Altitude above WGS84 (cm)
    pub alt_ellipsoid: i32,
    /// X Speed [m*1000/s]
    pub vn: i32,
    /// Y Speed [m*1000/s]
    pub ve: i32,
    /// Z Speed [m*1000/s]
    pub vd: i32,
}
impl SendGpsData {
    pub const CMD_ID: u8 = 0x3E;
    pub const IS_REQUEST: bool = true;
    pub fn new(
        time_boot_ms: u32,
        lat: i32,
        lon: i32,
        alt: i32,
        alt_ellipsoid: i32,
        vn: i32,
        ve: i32,
        vd: i32,
    ) -> Self {
        Self {
            time_boot_ms,
            lat,
            lon,
            alt,
            alt_ellipsoid,
            vn,
            ve,
            vd,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.time_boot_ms.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.lat.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.lon.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.alt.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.alt_ellipsoid.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.vn.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.ve.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.vd.to_le_bytes());
        idx += 4;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let time_boot_ms = u32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let lat = i32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let lon = i32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let alt = i32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let alt_ellipsoid = i32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let vn = i32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ve = i32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let vd = i32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        Ok(Self {
            time_boot_ms,
            lat,
            lon,
            alt,
            alt_ellipsoid,
            vn,
            ve,
            vd,
        })
    }
}
impl Default for SendGpsData {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0)
    }
}

/// Request system time
/// CMD_ID: 0x40 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SystemTimeRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl SystemTimeRequest {
    pub const CMD_ID: u8 = 0x40;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for SystemTimeRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// System time
/// CMD_ID: 0x40 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SystemTimeResponse {
    /// UNIX epoch time (μs)
    pub time_unix_usec: u64,
    /// Time since system startup (ms)
    pub time_boot_ms: u32,
}
impl SystemTimeResponse {
    pub const CMD_ID: u8 = 0x40;
    pub const IS_REQUEST: bool = false;
    pub fn new(time_unix_usec: u64, time_boot_ms: u32) -> Self {
        Self {
            time_unix_usec,
            time_boot_ms,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 8 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 8].copy_from_slice(&self.time_unix_usec.to_le_bytes());
        idx += 8;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.time_boot_ms.to_le_bytes());
        idx += 4;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 8 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let time_unix_usec = u64::from_le_bytes(
            data[idx..idx + 8]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 8;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let time_boot_ms = u32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        Ok(Self {
            time_unix_usec,
            time_boot_ms,
        })
    }
}
impl Default for SystemTimeResponse {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

/// Set single-axis attitude angle
/// CMD_ID: 0x41 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SingleAxisAttitudeRequest {
    /// Target angle * 10
    pub angle: i16,
    /// Axis to control
    pub single_control_flag: SingleAxisControl,
}
impl SingleAxisAttitudeRequest {
    pub const CMD_ID: u8 = 0x41;
    pub const IS_REQUEST: bool = true;
    pub fn new(angle: i16, single_control_flag: SingleAxisControl) -> Self {
        Self {
            angle,
            single_control_flag,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.angle.to_le_bytes());
        idx += 2;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.single_control_flag as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let angle = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let single_control_flag =
            SingleAxisControl::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            angle,
            single_control_flag,
        })
    }
}
impl Default for SingleAxisAttitudeRequest {
    fn default() -> Self {
        Self::new(0, SingleAxisControl::default())
    }
}

/// Single-axis attitude response
/// CMD_ID: 0x41 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SingleAxisAttitudeResponse {
    /// Current yaw * 10
    pub yaw: i16,
    /// Current pitch * 10
    pub pitch: i16,
    /// Current roll * 10
    pub roll: i16,
}
impl SingleAxisAttitudeResponse {
    pub const CMD_ID: u8 = 0x41;
    pub const IS_REQUEST: bool = false;
    pub fn new(yaw: i16, pitch: i16, roll: i16) -> Self {
        Self { yaw, pitch, roll }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.yaw.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.pitch.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.roll.to_le_bytes());
        idx += 2;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let yaw = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pitch = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let roll = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        Ok(Self { yaw, pitch, roll })
    }
}
impl Default for SingleAxisAttitudeResponse {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Request thermal threshold switch
/// CMD_ID: 0x42 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalThresholdSwitchRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl ThermalThresholdSwitchRequest {
    pub const CMD_ID: u8 = 0x42;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for ThermalThresholdSwitchRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Thermal threshold switch
/// CMD_ID: 0x42 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalThresholdSwitchResponse {
    /// Off or On
    pub ir_thresh_sta: BooleanOnOff,
}
impl ThermalThresholdSwitchResponse {
    pub const CMD_ID: u8 = 0x42;
    pub const IS_REQUEST: bool = false;
    pub fn new(ir_thresh_sta: BooleanOnOff) -> Self {
        Self { ir_thresh_sta }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.ir_thresh_sta as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ir_thresh_sta =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { ir_thresh_sta })
    }
}
impl Default for ThermalThresholdSwitchResponse {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Set thermal threshold switch
/// CMD_ID: 0x43 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalThresholdSwitchRequest {
    /// Off or On
    pub ir_thresh_sta: BooleanOnOff,
}
impl SetThermalThresholdSwitchRequest {
    pub const CMD_ID: u8 = 0x43;
    pub const IS_REQUEST: bool = true;
    pub fn new(ir_thresh_sta: BooleanOnOff) -> Self {
        Self { ir_thresh_sta }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.ir_thresh_sta as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ir_thresh_sta =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { ir_thresh_sta })
    }
}
impl Default for SetThermalThresholdSwitchRequest {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Set thermal threshold switch response
/// CMD_ID: 0x43 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalThresholdSwitchResponse {
    /// Off or On
    pub ir_thresh_sta: BooleanOnOff,
}
impl SetThermalThresholdSwitchResponse {
    pub const CMD_ID: u8 = 0x43;
    pub const IS_REQUEST: bool = false;
    pub fn new(ir_thresh_sta: BooleanOnOff) -> Self {
        Self { ir_thresh_sta }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.ir_thresh_sta as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ir_thresh_sta =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { ir_thresh_sta })
    }
}
impl Default for SetThermalThresholdSwitchResponse {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Request thermal threshold params
/// CMD_ID: 0x44 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalThresholdParamsRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl ThermalThresholdParamsRequest {
    pub const CMD_ID: u8 = 0x44;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for ThermalThresholdParamsRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Thermal threshold params
/// CMD_ID: 0x44 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalThresholdParamsResponse {
    /// Threshold region 1: hide or display
    pub thresh1_switch: BooleanOnOff,
    /// Threshold 1 min temp
    pub thresh1_temp_min: i16,
    /// Threshold 1 max temp
    pub thresh1_temp_max: i16,
    /// Threshold 1 color R
    pub thresh1_color_r: u8,
    /// Threshold 1 color G
    pub thresh1_color_g: u8,
    /// Threshold 1 color B
    pub thresh1_color_b: u8,
    /// Threshold region 2: hide or display
    pub thresh2_switch: BooleanOnOff,
    /// Threshold 2 min temp
    pub thresh2_temp_min: i16,
    /// Threshold 2 max temp
    pub thresh2_temp_max: i16,
    /// Threshold 2 color R
    pub thresh2_color_r: u8,
    /// Threshold 2 color G
    pub thresh2_color_g: u8,
    /// Threshold 2 color B
    pub thresh2_color_b: u8,
    /// Threshold region 3: hide or display
    pub thresh3_switch: BooleanOnOff,
    /// Threshold 3 min temp
    pub thresh3_temp_min: i16,
    /// Threshold 3 max temp
    pub thresh3_temp_max: i16,
    /// Threshold 3 color R
    pub thresh3_color_r: u8,
    /// Threshold 3 color G
    pub thresh3_color_g: u8,
    /// Threshold 3 color B
    pub thresh3_color_b: u8,
}
impl ThermalThresholdParamsResponse {
    pub const CMD_ID: u8 = 0x44;
    pub const IS_REQUEST: bool = false;
    pub fn new(
        thresh1_switch: BooleanOnOff,
        thresh1_temp_min: i16,
        thresh1_temp_max: i16,
        thresh1_color_r: u8,
        thresh1_color_g: u8,
        thresh1_color_b: u8,
        thresh2_switch: BooleanOnOff,
        thresh2_temp_min: i16,
        thresh2_temp_max: i16,
        thresh2_color_r: u8,
        thresh2_color_g: u8,
        thresh2_color_b: u8,
        thresh3_switch: BooleanOnOff,
        thresh3_temp_min: i16,
        thresh3_temp_max: i16,
        thresh3_color_r: u8,
        thresh3_color_g: u8,
        thresh3_color_b: u8,
    ) -> Self {
        Self {
            thresh1_switch,
            thresh1_temp_min,
            thresh1_temp_max,
            thresh1_color_r,
            thresh1_color_g,
            thresh1_color_b,
            thresh2_switch,
            thresh2_temp_min,
            thresh2_temp_max,
            thresh2_color_r,
            thresh2_color_g,
            thresh2_color_b,
            thresh3_switch,
            thresh3_temp_min,
            thresh3_temp_max,
            thresh3_color_r,
            thresh3_color_g,
            thresh3_color_b,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.thresh1_switch as u8;
        idx += 1;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.thresh1_temp_min.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.thresh1_temp_max.to_le_bytes());
        idx += 2;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh1_color_r.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh1_color_g.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh1_color_b.to_le_bytes());
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.thresh2_switch as u8;
        idx += 1;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.thresh2_temp_min.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.thresh2_temp_max.to_le_bytes());
        idx += 2;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh2_color_r.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh2_color_g.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh2_color_b.to_le_bytes());
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.thresh3_switch as u8;
        idx += 1;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.thresh3_temp_min.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.thresh3_temp_max.to_le_bytes());
        idx += 2;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh3_color_r.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh3_color_g.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh3_color_b.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh1_switch =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh1_temp_min = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh1_temp_max = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh1_color_r = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh1_color_g = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh1_color_b = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh2_switch =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh2_temp_min = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh2_temp_max = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh2_color_r = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh2_color_g = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh2_color_b = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh3_switch =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh3_temp_min = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh3_temp_max = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh3_color_r = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh3_color_g = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh3_color_b = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self {
            thresh1_switch,
            thresh1_temp_min,
            thresh1_temp_max,
            thresh1_color_r,
            thresh1_color_g,
            thresh1_color_b,
            thresh2_switch,
            thresh2_temp_min,
            thresh2_temp_max,
            thresh2_color_r,
            thresh2_color_g,
            thresh2_color_b,
            thresh3_switch,
            thresh3_temp_min,
            thresh3_temp_max,
            thresh3_color_r,
            thresh3_color_g,
            thresh3_color_b,
        })
    }
}
impl Default for ThermalThresholdParamsResponse {
    fn default() -> Self {
        Self::new(
            BooleanOnOff::default(),
            0,
            0,
            0,
            0,
            0,
            BooleanOnOff::default(),
            0,
            0,
            0,
            0,
            0,
            BooleanOnOff::default(),
            0,
            0,
            0,
            0,
            0,
        )
    }
}

/// Set thermal threshold params
/// CMD_ID: 0x45 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalThresholdParamsRequest {
    /// Threshold region 1: hide or display
    pub thresh1_switch: BooleanOnOff,
    /// Threshold 1 min temp
    pub thresh1_temp_min: i16,
    /// Threshold 1 max temp
    pub thresh1_temp_max: i16,
    /// Threshold 1 color R
    pub thresh1_color_r: u8,
    /// Threshold 1 color G
    pub thresh1_color_g: u8,
    /// Threshold 1 color B
    pub thresh1_color_b: u8,
    /// Threshold region 2: hide or display
    pub thresh2_switch: BooleanOnOff,
    /// Threshold 2 min temp
    pub thresh2_temp_min: i16,
    /// Threshold 2 max temp
    pub thresh2_temp_max: i16,
    /// Threshold 2 color R
    pub thresh2_color_r: u8,
    /// Threshold 2 color G
    pub thresh2_color_g: u8,
    /// Threshold 2 color B
    pub thresh2_color_b: u8,
    /// Threshold region 3: hide or display
    pub thresh3_switch: BooleanOnOff,
    /// Threshold 3 min temp
    pub thresh3_temp_min: i16,
    /// Threshold 3 max temp
    pub thresh3_temp_max: i16,
    /// Threshold 3 color R
    pub thresh3_color_r: u8,
    /// Threshold 3 color G
    pub thresh3_color_g: u8,
    /// Threshold 3 color B
    pub thresh3_color_b: u8,
}
impl SetThermalThresholdParamsRequest {
    pub const CMD_ID: u8 = 0x45;
    pub const IS_REQUEST: bool = true;
    pub fn new(
        thresh1_switch: BooleanOnOff,
        thresh1_temp_min: i16,
        thresh1_temp_max: i16,
        thresh1_color_r: u8,
        thresh1_color_g: u8,
        thresh1_color_b: u8,
        thresh2_switch: BooleanOnOff,
        thresh2_temp_min: i16,
        thresh2_temp_max: i16,
        thresh2_color_r: u8,
        thresh2_color_g: u8,
        thresh2_color_b: u8,
        thresh3_switch: BooleanOnOff,
        thresh3_temp_min: i16,
        thresh3_temp_max: i16,
        thresh3_color_r: u8,
        thresh3_color_g: u8,
        thresh3_color_b: u8,
    ) -> Self {
        Self {
            thresh1_switch,
            thresh1_temp_min,
            thresh1_temp_max,
            thresh1_color_r,
            thresh1_color_g,
            thresh1_color_b,
            thresh2_switch,
            thresh2_temp_min,
            thresh2_temp_max,
            thresh2_color_r,
            thresh2_color_g,
            thresh2_color_b,
            thresh3_switch,
            thresh3_temp_min,
            thresh3_temp_max,
            thresh3_color_r,
            thresh3_color_g,
            thresh3_color_b,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.thresh1_switch as u8;
        idx += 1;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.thresh1_temp_min.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.thresh1_temp_max.to_le_bytes());
        idx += 2;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh1_color_r.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh1_color_g.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh1_color_b.to_le_bytes());
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.thresh2_switch as u8;
        idx += 1;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.thresh2_temp_min.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.thresh2_temp_max.to_le_bytes());
        idx += 2;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh2_color_r.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh2_color_g.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh2_color_b.to_le_bytes());
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.thresh3_switch as u8;
        idx += 1;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.thresh3_temp_min.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.thresh3_temp_max.to_le_bytes());
        idx += 2;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh3_color_r.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh3_color_g.to_le_bytes());
        idx += 1;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.thresh3_color_b.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh1_switch =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh1_temp_min = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh1_temp_max = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh1_color_r = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh1_color_g = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh1_color_b = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh2_switch =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh2_temp_min = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh2_temp_max = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh2_color_r = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh2_color_g = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh2_color_b = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh3_switch =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh3_temp_min = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh3_temp_max = i16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh3_color_r = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh3_color_g = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let thresh3_color_b = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self {
            thresh1_switch,
            thresh1_temp_min,
            thresh1_temp_max,
            thresh1_color_r,
            thresh1_color_g,
            thresh1_color_b,
            thresh2_switch,
            thresh2_temp_min,
            thresh2_temp_max,
            thresh2_color_r,
            thresh2_color_g,
            thresh2_color_b,
            thresh3_switch,
            thresh3_temp_min,
            thresh3_temp_max,
            thresh3_color_r,
            thresh3_color_g,
            thresh3_color_b,
        })
    }
}
impl Default for SetThermalThresholdParamsRequest {
    fn default() -> Self {
        Self::new(
            BooleanOnOff::default(),
            0,
            0,
            0,
            0,
            0,
            BooleanOnOff::default(),
            0,
            0,
            0,
            0,
            0,
            BooleanOnOff::default(),
            0,
            0,
            0,
            0,
            0,
        )
    }
}

/// Set thermal threshold params response
/// CMD_ID: 0x45 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalThresholdParamsResponse {
    /// Success or failed
    pub ack: BooleanStatus,
}
impl SetThermalThresholdParamsResponse {
    pub const CMD_ID: u8 = 0x45;
    pub const IS_REQUEST: bool = false;
    pub fn new(ack: BooleanStatus) -> Self {
        Self { ack }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.ack as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ack = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { ack })
    }
}
impl Default for SetThermalThresholdParamsResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Request thermal threshold precision
/// CMD_ID: 0x46 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalThresholdPrecisionRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl ThermalThresholdPrecisionRequest {
    pub const CMD_ID: u8 = 0x46;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for ThermalThresholdPrecisionRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Thermal threshold precision
/// CMD_ID: 0x46 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalThresholdPrecisionResponse {
    /// Precision level
    pub precision: ThermalThresholdPrecision,
}
impl ThermalThresholdPrecisionResponse {
    pub const CMD_ID: u8 = 0x46;
    pub const IS_REQUEST: bool = false;
    pub fn new(precision: ThermalThresholdPrecision) -> Self {
        Self { precision }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.precision as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let precision =
            ThermalThresholdPrecision::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { precision })
    }
}
impl Default for ThermalThresholdPrecisionResponse {
    fn default() -> Self {
        Self::new(ThermalThresholdPrecision::default())
    }
}

/// Set thermal threshold precision
/// CMD_ID: 0x47 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalThresholdPrecisionRequest {
    /// Precision level
    pub precision: ThermalThresholdPrecision,
}
impl SetThermalThresholdPrecisionRequest {
    pub const CMD_ID: u8 = 0x47;
    pub const IS_REQUEST: bool = true;
    pub fn new(precision: ThermalThresholdPrecision) -> Self {
        Self { precision }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.precision as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let precision =
            ThermalThresholdPrecision::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { precision })
    }
}
impl Default for SetThermalThresholdPrecisionRequest {
    fn default() -> Self {
        Self::new(ThermalThresholdPrecision::default())
    }
}

/// Set thermal threshold precision response
/// CMD_ID: 0x47 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetThermalThresholdPrecisionResponse {
    /// Confirmed precision
    pub precision: ThermalThresholdPrecision,
}
impl SetThermalThresholdPrecisionResponse {
    pub const CMD_ID: u8 = 0x47;
    pub const IS_REQUEST: bool = false;
    pub fn new(precision: ThermalThresholdPrecision) -> Self {
        Self { precision }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.precision as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let precision =
            ThermalThresholdPrecision::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { precision })
    }
}
impl Default for SetThermalThresholdPrecisionResponse {
    fn default() -> Self {
        Self::new(ThermalThresholdPrecision::default())
    }
}

/// Format SD card
/// CMD_ID: 0x48 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct FormatSdCardRequest {
    /// Format failed or successful
    pub format_sta: BooleanStatus,
}
impl FormatSdCardRequest {
    pub const CMD_ID: u8 = 0x48;
    pub const IS_REQUEST: bool = true;
    pub fn new(format_sta: BooleanStatus) -> Self {
        Self { format_sta }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.format_sta as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let format_sta = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { format_sta })
    }
}
impl Default for FormatSdCardRequest {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Format SD card response
/// CMD_ID: 0x48 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct FormatSdCardResponse {
    /// Format failed or successful
    pub format_sta: BooleanStatus,
}
impl FormatSdCardResponse {
    pub const CMD_ID: u8 = 0x48;
    pub const IS_REQUEST: bool = false;
    pub fn new(format_sta: BooleanStatus) -> Self {
        Self { format_sta }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.format_sta as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let format_sta = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { format_sta })
    }
}
impl Default for FormatSdCardResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// Get picture name type
/// CMD_ID: 0x49 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct GetPictureNameTypeRequest {
    /// File type
    pub file_type: FileType,
}
impl GetPictureNameTypeRequest {
    pub const CMD_ID: u8 = 0x49;
    pub const IS_REQUEST: bool = true;
    pub fn new(file_type: FileType) -> Self {
        Self { file_type }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.file_type as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let file_type = FileType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { file_type })
    }
}
impl Default for GetPictureNameTypeRequest {
    fn default() -> Self {
        Self::new(FileType::default())
    }
}

/// Picture name type
/// CMD_ID: 0x49 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct GetPictureNameTypeResponse {
    /// File type
    pub file_type: FileType,
    /// File naming type
    pub file_name_type: FileNameType,
}
impl GetPictureNameTypeResponse {
    pub const CMD_ID: u8 = 0x49;
    pub const IS_REQUEST: bool = false;
    pub fn new(file_type: FileType, file_name_type: FileNameType) -> Self {
        Self {
            file_type,
            file_name_type,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.file_type as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.file_name_type as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let file_type = FileType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let file_name_type =
            FileNameType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            file_type,
            file_name_type,
        })
    }
}
impl Default for GetPictureNameTypeResponse {
    fn default() -> Self {
        Self::new(FileType::default(), FileNameType::default())
    }
}

/// Set picture name type
/// CMD_ID: 0x4A | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetPictureNameTypeRequest {
    /// File type
    pub file_type: FileType,
    /// File naming type
    pub file_name_type: FileNameType,
}
impl SetPictureNameTypeRequest {
    pub const CMD_ID: u8 = 0x4A;
    pub const IS_REQUEST: bool = true;
    pub fn new(file_type: FileType, file_name_type: FileNameType) -> Self {
        Self {
            file_type,
            file_name_type,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.file_type as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.file_name_type as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let file_type = FileType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let file_name_type =
            FileNameType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            file_type,
            file_name_type,
        })
    }
}
impl Default for SetPictureNameTypeRequest {
    fn default() -> Self {
        Self::new(FileType::default(), FileNameType::default())
    }
}

/// Set picture name type response
/// CMD_ID: 0x4A | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetPictureNameTypeResponse {
    /// File type
    pub file_type: FileType,
    /// File naming type
    pub file_name_type: FileNameType,
}
impl SetPictureNameTypeResponse {
    pub const CMD_ID: u8 = 0x4A;
    pub const IS_REQUEST: bool = false;
    pub fn new(file_type: FileType, file_name_type: FileNameType) -> Self {
        Self {
            file_type,
            file_name_type,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.file_type as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.file_name_type as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let file_type = FileType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let file_name_type =
            FileNameType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            file_type,
            file_name_type,
        })
    }
}
impl Default for SetPictureNameTypeResponse {
    fn default() -> Self {
        Self::new(FileType::default(), FileNameType::default())
    }
}

/// Request HDMI OSD status
/// CMD_ID: 0x4B | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct HdmiOsdStatusRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl HdmiOsdStatusRequest {
    pub const CMD_ID: u8 = 0x4B;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for HdmiOsdStatusRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// HDMI OSD status
/// CMD_ID: 0x4B | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct HdmiOsdStatusResponse {
    /// Off or On
    pub osd_sta: BooleanOnOff,
}
impl HdmiOsdStatusResponse {
    pub const CMD_ID: u8 = 0x4B;
    pub const IS_REQUEST: bool = false;
    pub fn new(osd_sta: BooleanOnOff) -> Self {
        Self { osd_sta }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.osd_sta as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let osd_sta = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { osd_sta })
    }
}
impl Default for HdmiOsdStatusResponse {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Set HDMI OSD status
/// CMD_ID: 0x4C | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetHdmiOsdStatusRequest {
    /// Off or On
    pub osd_sta: BooleanOnOff,
}
impl SetHdmiOsdStatusRequest {
    pub const CMD_ID: u8 = 0x4C;
    pub const IS_REQUEST: bool = true;
    pub fn new(osd_sta: BooleanOnOff) -> Self {
        Self { osd_sta }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.osd_sta as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let osd_sta = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { osd_sta })
    }
}
impl Default for SetHdmiOsdStatusRequest {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Set HDMI OSD status response
/// CMD_ID: 0x4C | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetHdmiOsdStatusResponse {
    /// Off or On
    pub osd_sta: BooleanOnOff,
}
impl SetHdmiOsdStatusResponse {
    pub const CMD_ID: u8 = 0x4C;
    pub const IS_REQUEST: bool = false;
    pub fn new(osd_sta: BooleanOnOff) -> Self {
        Self { osd_sta }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.osd_sta as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let osd_sta = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { osd_sta })
    }
}
impl Default for SetHdmiOsdStatusResponse {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Get AI mode status
/// CMD_ID: 0x4D | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct AiModeStatusRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl AiModeStatusRequest {
    pub const CMD_ID: u8 = 0x4D;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for AiModeStatusRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// AI mode status
/// CMD_ID: 0x4D | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct AiModeStatusResponse {
    /// Not enabled or AI mode enabled
    pub sta: BooleanOnOff,
}
impl AiModeStatusResponse {
    pub const CMD_ID: u8 = 0x4D;
    pub const IS_REQUEST: bool = false;
    pub fn new(sta: BooleanOnOff) -> Self {
        Self { sta }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.sta as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let sta = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { sta })
    }
}
impl Default for AiModeStatusResponse {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Get AI tracking stream status
/// CMD_ID: 0x4E | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct AiTrackingStreamStatusRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl AiTrackingStreamStatusRequest {
    pub const CMD_ID: u8 = 0x4E;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for AiTrackingStreamStatusRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// AI tracking stream status
/// CMD_ID: 0x4E | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct AiTrackingStreamStatusResponse {
    /// 0: Not enabled, 1: Outputting, 2: AI not enabled, 3: Tracking not enabled
    pub sta: u8,
}
impl AiTrackingStreamStatusResponse {
    pub const CMD_ID: u8 = 0x4E;
    pub const IS_REQUEST: bool = false;
    pub fn new(sta: u8) -> Self {
        Self { sta }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 1 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 1].copy_from_slice(&self.sta.to_le_bytes());
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 1 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let sta = u8::from_le_bytes(
            data[idx..idx + 1]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 1;
        Ok(Self { sta })
    }
}
impl Default for AiTrackingStreamStatusResponse {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Manually update thermal shutter
/// CMD_ID: 0x4F | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateThermalShutterRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl UpdateThermalShutterRequest {
    pub const CMD_ID: u8 = 0x4F;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for UpdateThermalShutterRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Update thermal shutter response
/// CMD_ID: 0x4F | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateThermalShutterResponse {
    /// Update successful or failed
    pub ack: BooleanStatus,
}
impl UpdateThermalShutterResponse {
    pub const CMD_ID: u8 = 0x4F;
    pub const IS_REQUEST: bool = false;
    pub fn new(ack: BooleanStatus) -> Self {
        Self { ack }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.ack as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ack = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { ack })
    }
}
impl Default for UpdateThermalShutterResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

/// AI tracking coordinate stream
/// CMD_ID: 0x50 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct AiTrackingCoordinateStream {
    /// Target tracking coordinate X
    pub pos_x: u16,
    /// Target tracking coordinate Y
    pub pos_y: u16,
    /// Target tracking box width
    pub pos_width: u16,
    /// Target tracking box height
    pub pos_height: u16,
    /// Target type ID
    pub target_id: AITargetType,
    /// Tracking status
    pub track_sta: AITrackingStatus,
}
impl AiTrackingCoordinateStream {
    pub const CMD_ID: u8 = 0x50;
    pub const IS_REQUEST: bool = false;
    pub fn new(
        pos_x: u16,
        pos_y: u16,
        pos_width: u16,
        pos_height: u16,
        target_id: AITargetType,
        track_sta: AITrackingStatus,
    ) -> Self {
        Self {
            pos_x,
            pos_y,
            pos_width,
            pos_height,
            target_id,
            track_sta,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.pos_x.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.pos_y.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.pos_width.to_le_bytes());
        idx += 2;
        if idx + 2 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 2].copy_from_slice(&self.pos_height.to_le_bytes());
        idx += 2;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.target_id as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.track_sta as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pos_x = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pos_y = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pos_width = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if data.len() < idx + 2 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let pos_height = u16::from_le_bytes(
            data[idx..idx + 2]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 2;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let target_id = AITargetType::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let track_sta =
            AITrackingStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            pos_x,
            pos_y,
            pos_width,
            pos_height,
            target_id,
            track_sta,
        })
    }
}
impl Default for AiTrackingCoordinateStream {
    fn default() -> Self {
        Self::new(
            0,
            0,
            0,
            0,
            AITargetType::default(),
            AITrackingStatus::default(),
        )
    }
}

/// Set AI tracking stream status
/// CMD_ID: 0x51 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetAiTrackingStreamStatusRequest {
    /// Enable or Disable output
    pub track_action: BooleanOnOff,
}
impl SetAiTrackingStreamStatusRequest {
    pub const CMD_ID: u8 = 0x51;
    pub const IS_REQUEST: bool = true;
    pub fn new(track_action: BooleanOnOff) -> Self {
        Self { track_action }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.track_action as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let track_action = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { track_action })
    }
}
impl Default for SetAiTrackingStreamStatusRequest {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Set AI tracking stream response
/// CMD_ID: 0x51 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetAiTrackingStreamStatusResponse {
    /// Output enabled or disabled
    pub sta: BooleanOnOff,
}
impl SetAiTrackingStreamStatusResponse {
    pub const CMD_ID: u8 = 0x51;
    pub const IS_REQUEST: bool = false;
    pub fn new(sta: BooleanOnOff) -> Self {
        Self { sta }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.sta as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let sta = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { sta })
    }
}
impl Default for SetAiTrackingStreamStatusResponse {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Request weak control mode
/// CMD_ID: 0x70 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct WeakControlModeRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl WeakControlModeRequest {
    pub const CMD_ID: u8 = 0x70;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for WeakControlModeRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Weak control mode
/// CMD_ID: 0x70 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct WeakControlModeResponse {
    /// Enabled or Disabled
    pub weak_mode_state: BooleanOnOff,
}
impl WeakControlModeResponse {
    pub const CMD_ID: u8 = 0x70;
    pub const IS_REQUEST: bool = false;
    pub fn new(weak_mode_state: BooleanOnOff) -> Self {
        Self { weak_mode_state }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.weak_mode_state as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let weak_mode_state =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { weak_mode_state })
    }
}
impl Default for WeakControlModeResponse {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Set weak control mode
/// CMD_ID: 0x71 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetWeakControlModeRequest {
    /// Enable or Disable
    pub weak_mode_state: BooleanOnOff,
}
impl SetWeakControlModeRequest {
    pub const CMD_ID: u8 = 0x71;
    pub const IS_REQUEST: bool = true;
    pub fn new(weak_mode_state: BooleanOnOff) -> Self {
        Self { weak_mode_state }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.weak_mode_state as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let weak_mode_state =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { weak_mode_state })
    }
}
impl Default for SetWeakControlModeRequest {
    fn default() -> Self {
        Self::new(BooleanOnOff::default())
    }
}

/// Set weak control mode response
/// CMD_ID: 0x71 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetWeakControlModeResponse {
    /// Success or Failure
    pub sta: BooleanStatus,
    /// Enabled or Disabled
    pub weak_mode_state: BooleanOnOff,
}
impl SetWeakControlModeResponse {
    pub const CMD_ID: u8 = 0x71;
    pub const IS_REQUEST: bool = false;
    pub fn new(sta: BooleanStatus, weak_mode_state: BooleanOnOff) -> Self {
        Self {
            sta,
            weak_mode_state,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.sta as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.weak_mode_state as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let sta = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let weak_mode_state =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            sta,
            weak_mode_state,
        })
    }
}
impl Default for SetWeakControlModeResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default(), BooleanOnOff::default())
    }
}

/// Gimbal camera soft reboot
/// CMD_ID: 0x80 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SoftRebootRequest {
    /// No action or Camera reboot
    pub camera_reboot: BooleanOnOff,
    /// No action or Gimbal reboot
    pub gimbal_reset: BooleanOnOff,
}
impl SoftRebootRequest {
    pub const CMD_ID: u8 = 0x80;
    pub const IS_REQUEST: bool = true;
    pub fn new(camera_reboot: BooleanOnOff, gimbal_reset: BooleanOnOff) -> Self {
        Self {
            camera_reboot,
            gimbal_reset,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.camera_reboot as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.gimbal_reset as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let camera_reboot =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let gimbal_reset = BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            camera_reboot,
            gimbal_reset,
        })
    }
}
impl Default for SoftRebootRequest {
    fn default() -> Self {
        Self::new(BooleanOnOff::default(), BooleanOnOff::default())
    }
}

/// Soft reboot response
/// CMD_ID: 0x80 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SoftRebootResponse {
    /// No action or Camera rebooting
    pub camera_reboot_sta: BooleanOnOff,
    /// No action or Gimbal rebooting
    pub gimbal_reset_sta: BooleanOnOff,
}
impl SoftRebootResponse {
    pub const CMD_ID: u8 = 0x80;
    pub const IS_REQUEST: bool = false;
    pub fn new(camera_reboot_sta: BooleanOnOff, gimbal_reset_sta: BooleanOnOff) -> Self {
        Self {
            camera_reboot_sta,
            gimbal_reset_sta,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.camera_reboot_sta as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.gimbal_reset_sta as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let camera_reboot_sta =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let gimbal_reset_sta =
            BooleanOnOff::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            camera_reboot_sta,
            gimbal_reset_sta,
        })
    }
}
impl Default for SoftRebootResponse {
    fn default() -> Self {
        Self::new(BooleanOnOff::default(), BooleanOnOff::default())
    }
}

/// Get gimbal camera IP address
/// CMD_ID: 0x81 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct GetIpAddressRequest {
    _phantom: core::marker::PhantomData<()>,
}
impl GetIpAddressRequest {
    pub const CMD_ID: u8 = 0x81;
    pub const IS_REQUEST: bool = true;
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            _phantom: core::marker::PhantomData,
        })
    }
}
impl Default for GetIpAddressRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// IP address
/// CMD_ID: 0x81 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct GetIpAddressResponse {
    /// IP Address
    pub ip: u32,
    /// Subnet Mask
    pub mask: u32,
    /// Gateway
    pub gateway: u32,
}
impl GetIpAddressResponse {
    pub const CMD_ID: u8 = 0x81;
    pub const IS_REQUEST: bool = false;
    pub fn new(ip: u32, mask: u32, gateway: u32) -> Self {
        Self { ip, mask, gateway }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.ip.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.mask.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.gateway.to_le_bytes());
        idx += 4;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ip = u32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let mask = u32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let gateway = u32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        Ok(Self { ip, mask, gateway })
    }
}
impl Default for GetIpAddressResponse {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Set gimbal camera IP address
/// CMD_ID: 0x82 | REQUEST
#[derive(Debug, Clone, PartialEq)]
pub struct SetIpAddressRequest {
    /// IP Address
    pub ip: u32,
    /// Subnet Mask
    pub mask: u32,
    /// Gateway
    pub gateway: u32,
}
impl SetIpAddressRequest {
    pub const CMD_ID: u8 = 0x82;
    pub const IS_REQUEST: bool = true;
    pub fn new(ip: u32, mask: u32, gateway: u32) -> Self {
        Self { ip, mask, gateway }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.ip.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.mask.to_le_bytes());
        idx += 4;
        if idx + 4 > buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx..idx + 4].copy_from_slice(&self.gateway.to_le_bytes());
        idx += 4;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ip = u32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let mask = u32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        if data.len() < idx + 4 {
            return Err(DecodeError::NotEnoughBytes);
        }
        let gateway = u32::from_le_bytes(
            data[idx..idx + 4]
                .try_into()
                .map_err(|_| DecodeError::ConversionError)?,
        );
        idx += 4;
        Ok(Self { ip, mask, gateway })
    }
}
impl Default for SetIpAddressRequest {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Set IP address response
/// CMD_ID: 0x82 | RESPONSE
#[derive(Debug, Clone, PartialEq)]
pub struct SetIpAddressResponse {
    /// Successfully set or failed
    pub ack: BooleanStatus,
}
impl SetIpAddressResponse {
    pub const CMD_ID: u8 = 0x82;
    pub const IS_REQUEST: bool = false;
    pub fn new(ack: BooleanStatus) -> Self {
        Self { ack }
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.ack as u8;
        idx += 1;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let ack = BooleanStatus::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self { ack })
    }
}
impl Default for SetIpAddressResponse {
    fn default() -> Self {
        Self::new(BooleanStatus::default())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    /// TCP heartbeat keepalive
    TcpHeartbeat(TcpHeartbeat),
    /// Request firmware version
    FirmwareVersionRequest(FirmwareVersionRequest),
    /// Firmware version response
    FirmwareVersionResponse(FirmwareVersionResponse),
    /// Get hardware ID
    HardwareIdRequest(HardwareIdRequest),
    /// Hardware ID response
    HardwareIdResponse(HardwareIdResponse),
    /// Trigger auto focus
    AutoFocusRequest(AutoFocusRequest),
    /// Auto focus response
    AutoFocusResponse(AutoFocusResponse),
    /// Manual zoom control
    ManualZoomRequest(ManualZoomRequest),
    /// Manual zoom response
    ManualZoomResponse(ManualZoomResponse),
    /// Manual focus control
    ManualFocusRequest(ManualFocusRequest),
    /// Manual focus response
    ManualFocusResponse(ManualFocusResponse),
    /// Control gimbal rotation
    GimbalRotationRequest(GimbalRotationRequest),
    /// Gimbal rotation response
    GimbalRotationResponse(GimbalRotationResponse),
    /// Center gimbal
    CenterGimbalRequest(CenterGimbalRequest),
    /// Center response
    CenterGimbalResponse(CenterGimbalResponse),
    /// Request camera system info
    CameraSystemInfoRequest(CameraSystemInfoRequest),
    /// Camera system info
    CameraSystemInfoResponse(CameraSystemInfoResponse),
    /// Function feedback (sent by camera)
    FunctionFeedback(FunctionFeedback),
    /// Photo/video/mode control
    FunctionControl(FunctionControl),
    /// Request gimbal attitude
    GimbalAttitudeRequest(GimbalAttitudeRequest),
    /// Gimbal attitude data
    GimbalAttitudeResponse(GimbalAttitudeResponse),
    /// Set gimbal angles
    SetGimbalAttitudeRequest(SetGimbalAttitudeRequest),
    /// Set attitude response
    SetGimbalAttitudeResponse(SetGimbalAttitudeResponse),
    /// Set absolute zoom level
    AbsoluteZoomRequest(AbsoluteZoomRequest),
    /// Absolute zoom response
    AbsoluteZoomResponse(AbsoluteZoomResponse),
    /// Request video stitching mode
    VideoStitchingModeRequest(VideoStitchingModeRequest),
    /// Video stitching mode
    VideoStitchingModeResponse(VideoStitchingModeResponse),
    /// Set video stitching mode
    SetVideoStitchingModeRequest(SetVideoStitchingModeRequest),
    /// Set mode response
    SetVideoStitchingModeResponse(SetVideoStitchingModeResponse),
    /// Get temperature at point
    GetTemperatureAtPointRequest(GetTemperatureAtPointRequest),
    /// Point temperature
    GetTemperatureAtPointResponse(GetTemperatureAtPointResponse),
    /// Measure temperature in rectangle
    LocalTemperatureMeasurementRequest(LocalTemperatureMeasurementRequest),
    /// Local temperature data
    LocalTemperatureMeasurementResponse(LocalTemperatureMeasurementResponse),
    /// Measure global temperature
    GlobalTemperatureMeasurementRequest(GlobalTemperatureMeasurementRequest),
    /// Global temperature data
    GlobalTemperatureMeasurementResponse(GlobalTemperatureMeasurementResponse),
    /// Request laser rangefinder distance
    LaserDistanceRequest(LaserDistanceRequest),
    /// Laser distance
    LaserDistanceResponse(LaserDistanceResponse),
    /// Request max zoom range
    MaxZoomRangeRequest(MaxZoomRangeRequest),
    /// Max zoom range
    MaxZoomRangeResponse(MaxZoomRangeResponse),
    /// Request target lat/lon
    LaserTargetLocationRequest(LaserTargetLocationRequest),
    /// Target location
    LaserTargetLocationResponse(LaserTargetLocationResponse),
    /// Request current zoom level
    CurrentZoomRequest(CurrentZoomRequest),
    /// Current zoom
    CurrentZoomResponse(CurrentZoomResponse),
    /// Request current gimbal mode
    GimbalModeRequest(GimbalModeRequest),
    /// Gimbal mode
    GimbalModeResponse(GimbalModeResponse),
    /// Request thermal pseudo-color
    PseudoColorRequest(PseudoColorRequest),
    /// Pseudo-color
    PseudoColorResponse(PseudoColorResponse),
    /// Set thermal pseudo-color
    SetPseudoColorRequest(SetPseudoColorRequest),
    /// Set pseudo-color response
    SetPseudoColorResponse(SetPseudoColorResponse),
    /// Request camera encoding params
    EncodingParamsRequest(EncodingParamsRequest),
    /// Encoding parameters
    EncodingParamsResponse(EncodingParamsResponse),
    /// Set camera encoding params
    SetEncodingParamsRequest(SetEncodingParamsRequest),
    /// Set encoding response
    SetEncodingParamsResponse(SetEncodingParamsResponse),
    /// Send aircraft attitude to gimbal
    SendAircraftAttitude(SendAircraftAttitude),
    /// Send RC channel data to gimbal
    SendRcChannelDataRequest(SendRcChannelDataRequest),
    /// Request flight controller to send data stream
    RequestFlightControllerDataStreamRequest(RequestFlightControllerDataStreamRequest),
    /// Flight controller data stream response
    RequestFlightControllerDataStreamResponse(RequestFlightControllerDataStreamResponse),
    /// Request gimbal data stream
    RequestDataStreamRequest(RequestDataStreamRequest),
    /// Data stream config response
    RequestDataStreamResponse(RequestDataStreamResponse),
    /// Request magnetic encoder angles
    MagneticEncoderAngleRequest(MagneticEncoderAngleRequest),
    /// Magnetic encoder angles
    MagneticEncoderAngleResponse(MagneticEncoderAngleResponse),
    /// Request gimbal control mode
    GimbalControlModeRequest(GimbalControlModeRequest),
    /// Gimbal control mode
    GimbalControlModeResponse(GimbalControlModeResponse),
    /// Request weak control threshold
    WeakControlThresholdRequest(WeakControlThresholdRequest),
    /// Weak control threshold data
    WeakControlThresholdResponse(WeakControlThresholdResponse),
    /// Set weak control threshold
    SetWeakControlThresholdRequest(SetWeakControlThresholdRequest),
    /// Set threshold response
    SetWeakControlThresholdResponse(SetWeakControlThresholdResponse),
    /// Request motor voltage
    MotorVoltageRequest(MotorVoltageRequest),
    /// Motor voltage data
    MotorVoltageResponse(MotorVoltageResponse),
    /// Set UTC time
    SetUtcTimeRequest(SetUtcTimeRequest),
    /// UTC time response
    SetUtcTimeResponse(SetUtcTimeResponse),
    /// Request gimbal system information
    GimbalSystemInfoRequest(GimbalSystemInfoRequest),
    /// Gimbal system info
    GimbalSystemInfoResponse(GimbalSystemInfoResponse),
    /// Set laser ranging state
    SetLaserStateRequest(SetLaserStateRequest),
    /// Laser state response
    SetLaserStateResponse(SetLaserStateResponse),
    /// Request thermal output mode
    ThermalOutputModeRequest(ThermalOutputModeRequest),
    /// Thermal output mode
    ThermalOutputModeResponse(ThermalOutputModeResponse),
    /// Set thermal output mode
    SetThermalOutputModeRequest(SetThermalOutputModeRequest),
    /// Set thermal output mode response
    SetThermalOutputModeResponse(SetThermalOutputModeResponse),
    /// Get single temperature frame
    GetSingleTemperatureFrameRequest(GetSingleTemperatureFrameRequest),
    /// Single temperature frame response
    GetSingleTemperatureFrameResponse(GetSingleTemperatureFrameResponse),
    /// Request thermal gain mode
    ThermalGainModeRequest(ThermalGainModeRequest),
    /// Thermal gain mode
    ThermalGainModeResponse(ThermalGainModeResponse),
    /// Set thermal gain mode
    SetThermalGainModeRequest(SetThermalGainModeRequest),
    /// Set thermal gain mode response
    SetThermalGainModeResponse(SetThermalGainModeResponse),
    /// Request thermal env correction params
    ThermalEnvCorrectionParamsRequest(ThermalEnvCorrectionParamsRequest),
    /// Thermal env correction params
    ThermalEnvCorrectionParamsResponse(ThermalEnvCorrectionParamsResponse),
    /// Set thermal env correction params
    SetThermalEnvCorrectionParamsRequest(SetThermalEnvCorrectionParamsRequest),
    /// Set env correction params response
    SetThermalEnvCorrectionParamsResponse(SetThermalEnvCorrectionParamsResponse),
    /// Request env correction switch
    ThermalEnvCorrectionSwitchRequest(ThermalEnvCorrectionSwitchRequest),
    /// Env correction switch
    ThermalEnvCorrectionSwitchResponse(ThermalEnvCorrectionSwitchResponse),
    /// Set env correction switch
    SetThermalEnvCorrectionSwitchRequest(SetThermalEnvCorrectionSwitchRequest),
    /// Set env correction switch response
    SetThermalEnvCorrectionSwitchResponse(SetThermalEnvCorrectionSwitchResponse),
    /// Send GPS raw data to gimbal
    SendGpsData(SendGpsData),
    /// Request system time
    SystemTimeRequest(SystemTimeRequest),
    /// System time
    SystemTimeResponse(SystemTimeResponse),
    /// Set single-axis attitude angle
    SingleAxisAttitudeRequest(SingleAxisAttitudeRequest),
    /// Single-axis attitude response
    SingleAxisAttitudeResponse(SingleAxisAttitudeResponse),
    /// Request thermal threshold switch
    ThermalThresholdSwitchRequest(ThermalThresholdSwitchRequest),
    /// Thermal threshold switch
    ThermalThresholdSwitchResponse(ThermalThresholdSwitchResponse),
    /// Set thermal threshold switch
    SetThermalThresholdSwitchRequest(SetThermalThresholdSwitchRequest),
    /// Set thermal threshold switch response
    SetThermalThresholdSwitchResponse(SetThermalThresholdSwitchResponse),
    /// Request thermal threshold params
    ThermalThresholdParamsRequest(ThermalThresholdParamsRequest),
    /// Thermal threshold params
    ThermalThresholdParamsResponse(ThermalThresholdParamsResponse),
    /// Set thermal threshold params
    SetThermalThresholdParamsRequest(SetThermalThresholdParamsRequest),
    /// Set thermal threshold params response
    SetThermalThresholdParamsResponse(SetThermalThresholdParamsResponse),
    /// Request thermal threshold precision
    ThermalThresholdPrecisionRequest(ThermalThresholdPrecisionRequest),
    /// Thermal threshold precision
    ThermalThresholdPrecisionResponse(ThermalThresholdPrecisionResponse),
    /// Set thermal threshold precision
    SetThermalThresholdPrecisionRequest(SetThermalThresholdPrecisionRequest),
    /// Set thermal threshold precision response
    SetThermalThresholdPrecisionResponse(SetThermalThresholdPrecisionResponse),
    /// Format SD card
    FormatSdCardRequest(FormatSdCardRequest),
    /// Format SD card response
    FormatSdCardResponse(FormatSdCardResponse),
    /// Get picture name type
    GetPictureNameTypeRequest(GetPictureNameTypeRequest),
    /// Picture name type
    GetPictureNameTypeResponse(GetPictureNameTypeResponse),
    /// Set picture name type
    SetPictureNameTypeRequest(SetPictureNameTypeRequest),
    /// Set picture name type response
    SetPictureNameTypeResponse(SetPictureNameTypeResponse),
    /// Request HDMI OSD status
    HdmiOsdStatusRequest(HdmiOsdStatusRequest),
    /// HDMI OSD status
    HdmiOsdStatusResponse(HdmiOsdStatusResponse),
    /// Set HDMI OSD status
    SetHdmiOsdStatusRequest(SetHdmiOsdStatusRequest),
    /// Set HDMI OSD status response
    SetHdmiOsdStatusResponse(SetHdmiOsdStatusResponse),
    /// Get AI mode status
    AiModeStatusRequest(AiModeStatusRequest),
    /// AI mode status
    AiModeStatusResponse(AiModeStatusResponse),
    /// Get AI tracking stream status
    AiTrackingStreamStatusRequest(AiTrackingStreamStatusRequest),
    /// AI tracking stream status
    AiTrackingStreamStatusResponse(AiTrackingStreamStatusResponse),
    /// Manually update thermal shutter
    UpdateThermalShutterRequest(UpdateThermalShutterRequest),
    /// Update thermal shutter response
    UpdateThermalShutterResponse(UpdateThermalShutterResponse),
    /// AI tracking coordinate stream
    AiTrackingCoordinateStream(AiTrackingCoordinateStream),
    /// Set AI tracking stream status
    SetAiTrackingStreamStatusRequest(SetAiTrackingStreamStatusRequest),
    /// Set AI tracking stream response
    SetAiTrackingStreamStatusResponse(SetAiTrackingStreamStatusResponse),
    /// Request weak control mode
    WeakControlModeRequest(WeakControlModeRequest),
    /// Weak control mode
    WeakControlModeResponse(WeakControlModeResponse),
    /// Set weak control mode
    SetWeakControlModeRequest(SetWeakControlModeRequest),
    /// Set weak control mode response
    SetWeakControlModeResponse(SetWeakControlModeResponse),
    /// Gimbal camera soft reboot
    SoftRebootRequest(SoftRebootRequest),
    /// Soft reboot response
    SoftRebootResponse(SoftRebootResponse),
    /// Get gimbal camera IP address
    GetIpAddressRequest(GetIpAddressRequest),
    /// IP address
    GetIpAddressResponse(GetIpAddressResponse),
    /// Set gimbal camera IP address
    SetIpAddressRequest(SetIpAddressRequest),
    /// Set IP address response
    SetIpAddressResponse(SetIpAddressResponse),
}
impl Message {
    pub fn cmd_id(&self) -> u8 {
        match self {
            Message::TcpHeartbeat(_) => 0x00,
            Message::FirmwareVersionRequest(_) => 0x01,
            Message::FirmwareVersionResponse(_) => 0x01,
            Message::HardwareIdRequest(_) => 0x02,
            Message::HardwareIdResponse(_) => 0x02,
            Message::AutoFocusRequest(_) => 0x04,
            Message::AutoFocusResponse(_) => 0x04,
            Message::ManualZoomRequest(_) => 0x05,
            Message::ManualZoomResponse(_) => 0x05,
            Message::ManualFocusRequest(_) => 0x06,
            Message::ManualFocusResponse(_) => 0x06,
            Message::GimbalRotationRequest(_) => 0x07,
            Message::GimbalRotationResponse(_) => 0x07,
            Message::CenterGimbalRequest(_) => 0x08,
            Message::CenterGimbalResponse(_) => 0x08,
            Message::CameraSystemInfoRequest(_) => 0x0A,
            Message::CameraSystemInfoResponse(_) => 0x0A,
            Message::FunctionFeedback(_) => 0x0B,
            Message::FunctionControl(_) => 0x0C,
            Message::GimbalAttitudeRequest(_) => 0x0D,
            Message::GimbalAttitudeResponse(_) => 0x0D,
            Message::SetGimbalAttitudeRequest(_) => 0x0E,
            Message::SetGimbalAttitudeResponse(_) => 0x0E,
            Message::AbsoluteZoomRequest(_) => 0x0F,
            Message::AbsoluteZoomResponse(_) => 0x0F,
            Message::VideoStitchingModeRequest(_) => 0x10,
            Message::VideoStitchingModeResponse(_) => 0x10,
            Message::SetVideoStitchingModeRequest(_) => 0x11,
            Message::SetVideoStitchingModeResponse(_) => 0x11,
            Message::GetTemperatureAtPointRequest(_) => 0x12,
            Message::GetTemperatureAtPointResponse(_) => 0x12,
            Message::LocalTemperatureMeasurementRequest(_) => 0x13,
            Message::LocalTemperatureMeasurementResponse(_) => 0x13,
            Message::GlobalTemperatureMeasurementRequest(_) => 0x14,
            Message::GlobalTemperatureMeasurementResponse(_) => 0x14,
            Message::LaserDistanceRequest(_) => 0x15,
            Message::LaserDistanceResponse(_) => 0x15,
            Message::MaxZoomRangeRequest(_) => 0x16,
            Message::MaxZoomRangeResponse(_) => 0x16,
            Message::LaserTargetLocationRequest(_) => 0x17,
            Message::LaserTargetLocationResponse(_) => 0x17,
            Message::CurrentZoomRequest(_) => 0x18,
            Message::CurrentZoomResponse(_) => 0x18,
            Message::GimbalModeRequest(_) => 0x19,
            Message::GimbalModeResponse(_) => 0x19,
            Message::PseudoColorRequest(_) => 0x1A,
            Message::PseudoColorResponse(_) => 0x1A,
            Message::SetPseudoColorRequest(_) => 0x1B,
            Message::SetPseudoColorResponse(_) => 0x1B,
            Message::EncodingParamsRequest(_) => 0x20,
            Message::EncodingParamsResponse(_) => 0x20,
            Message::SetEncodingParamsRequest(_) => 0x21,
            Message::SetEncodingParamsResponse(_) => 0x21,
            Message::SendAircraftAttitude(_) => 0x22,
            Message::SendRcChannelDataRequest(_) => 0x23,
            Message::RequestFlightControllerDataStreamRequest(_) => 0x24,
            Message::RequestFlightControllerDataStreamResponse(_) => 0x24,
            Message::RequestDataStreamRequest(_) => 0x25,
            Message::RequestDataStreamResponse(_) => 0x25,
            Message::MagneticEncoderAngleRequest(_) => 0x26,
            Message::MagneticEncoderAngleResponse(_) => 0x26,
            Message::GimbalControlModeRequest(_) => 0x27,
            Message::GimbalControlModeResponse(_) => 0x27,
            Message::WeakControlThresholdRequest(_) => 0x28,
            Message::WeakControlThresholdResponse(_) => 0x28,
            Message::SetWeakControlThresholdRequest(_) => 0x29,
            Message::SetWeakControlThresholdResponse(_) => 0x29,
            Message::MotorVoltageRequest(_) => 0x2A,
            Message::MotorVoltageResponse(_) => 0x2A,
            Message::SetUtcTimeRequest(_) => 0x30,
            Message::SetUtcTimeResponse(_) => 0x30,
            Message::GimbalSystemInfoRequest(_) => 0x31,
            Message::GimbalSystemInfoResponse(_) => 0x31,
            Message::SetLaserStateRequest(_) => 0x32,
            Message::SetLaserStateResponse(_) => 0x32,
            Message::ThermalOutputModeRequest(_) => 0x33,
            Message::ThermalOutputModeResponse(_) => 0x33,
            Message::SetThermalOutputModeRequest(_) => 0x34,
            Message::SetThermalOutputModeResponse(_) => 0x34,
            Message::GetSingleTemperatureFrameRequest(_) => 0x35,
            Message::GetSingleTemperatureFrameResponse(_) => 0x35,
            Message::ThermalGainModeRequest(_) => 0x37,
            Message::ThermalGainModeResponse(_) => 0x37,
            Message::SetThermalGainModeRequest(_) => 0x38,
            Message::SetThermalGainModeResponse(_) => 0x38,
            Message::ThermalEnvCorrectionParamsRequest(_) => 0x39,
            Message::ThermalEnvCorrectionParamsResponse(_) => 0x39,
            Message::SetThermalEnvCorrectionParamsRequest(_) => 0x3A,
            Message::SetThermalEnvCorrectionParamsResponse(_) => 0x3A,
            Message::ThermalEnvCorrectionSwitchRequest(_) => 0x3B,
            Message::ThermalEnvCorrectionSwitchResponse(_) => 0x3B,
            Message::SetThermalEnvCorrectionSwitchRequest(_) => 0x3C,
            Message::SetThermalEnvCorrectionSwitchResponse(_) => 0x3C,
            Message::SendGpsData(_) => 0x3E,
            Message::SystemTimeRequest(_) => 0x40,
            Message::SystemTimeResponse(_) => 0x40,
            Message::SingleAxisAttitudeRequest(_) => 0x41,
            Message::SingleAxisAttitudeResponse(_) => 0x41,
            Message::ThermalThresholdSwitchRequest(_) => 0x42,
            Message::ThermalThresholdSwitchResponse(_) => 0x42,
            Message::SetThermalThresholdSwitchRequest(_) => 0x43,
            Message::SetThermalThresholdSwitchResponse(_) => 0x43,
            Message::ThermalThresholdParamsRequest(_) => 0x44,
            Message::ThermalThresholdParamsResponse(_) => 0x44,
            Message::SetThermalThresholdParamsRequest(_) => 0x45,
            Message::SetThermalThresholdParamsResponse(_) => 0x45,
            Message::ThermalThresholdPrecisionRequest(_) => 0x46,
            Message::ThermalThresholdPrecisionResponse(_) => 0x46,
            Message::SetThermalThresholdPrecisionRequest(_) => 0x47,
            Message::SetThermalThresholdPrecisionResponse(_) => 0x47,
            Message::FormatSdCardRequest(_) => 0x48,
            Message::FormatSdCardResponse(_) => 0x48,
            Message::GetPictureNameTypeRequest(_) => 0x49,
            Message::GetPictureNameTypeResponse(_) => 0x49,
            Message::SetPictureNameTypeRequest(_) => 0x4A,
            Message::SetPictureNameTypeResponse(_) => 0x4A,
            Message::HdmiOsdStatusRequest(_) => 0x4B,
            Message::HdmiOsdStatusResponse(_) => 0x4B,
            Message::SetHdmiOsdStatusRequest(_) => 0x4C,
            Message::SetHdmiOsdStatusResponse(_) => 0x4C,
            Message::AiModeStatusRequest(_) => 0x4D,
            Message::AiModeStatusResponse(_) => 0x4D,
            Message::AiTrackingStreamStatusRequest(_) => 0x4E,
            Message::AiTrackingStreamStatusResponse(_) => 0x4E,
            Message::UpdateThermalShutterRequest(_) => 0x4F,
            Message::UpdateThermalShutterResponse(_) => 0x4F,
            Message::AiTrackingCoordinateStream(_) => 0x50,
            Message::SetAiTrackingStreamStatusRequest(_) => 0x51,
            Message::SetAiTrackingStreamStatusResponse(_) => 0x51,
            Message::WeakControlModeRequest(_) => 0x70,
            Message::WeakControlModeResponse(_) => 0x70,
            Message::SetWeakControlModeRequest(_) => 0x71,
            Message::SetWeakControlModeResponse(_) => 0x71,
            Message::SoftRebootRequest(_) => 0x80,
            Message::SoftRebootResponse(_) => 0x80,
            Message::GetIpAddressRequest(_) => 0x81,
            Message::GetIpAddressResponse(_) => 0x81,
            Message::SetIpAddressRequest(_) => 0x82,
            Message::SetIpAddressResponse(_) => 0x82,
        }
    }
    pub fn is_request(&self) -> bool {
        match self {
            Message::TcpHeartbeat(_) => true,
            Message::FirmwareVersionRequest(_) => true,
            Message::FirmwareVersionResponse(_) => false,
            Message::HardwareIdRequest(_) => true,
            Message::HardwareIdResponse(_) => false,
            Message::AutoFocusRequest(_) => true,
            Message::AutoFocusResponse(_) => false,
            Message::ManualZoomRequest(_) => true,
            Message::ManualZoomResponse(_) => false,
            Message::ManualFocusRequest(_) => true,
            Message::ManualFocusResponse(_) => false,
            Message::GimbalRotationRequest(_) => true,
            Message::GimbalRotationResponse(_) => false,
            Message::CenterGimbalRequest(_) => true,
            Message::CenterGimbalResponse(_) => false,
            Message::CameraSystemInfoRequest(_) => true,
            Message::CameraSystemInfoResponse(_) => false,
            Message::FunctionFeedback(_) => false,
            Message::FunctionControl(_) => true,
            Message::GimbalAttitudeRequest(_) => true,
            Message::GimbalAttitudeResponse(_) => false,
            Message::SetGimbalAttitudeRequest(_) => true,
            Message::SetGimbalAttitudeResponse(_) => false,
            Message::AbsoluteZoomRequest(_) => true,
            Message::AbsoluteZoomResponse(_) => false,
            Message::VideoStitchingModeRequest(_) => true,
            Message::VideoStitchingModeResponse(_) => false,
            Message::SetVideoStitchingModeRequest(_) => true,
            Message::SetVideoStitchingModeResponse(_) => false,
            Message::GetTemperatureAtPointRequest(_) => true,
            Message::GetTemperatureAtPointResponse(_) => false,
            Message::LocalTemperatureMeasurementRequest(_) => true,
            Message::LocalTemperatureMeasurementResponse(_) => false,
            Message::GlobalTemperatureMeasurementRequest(_) => true,
            Message::GlobalTemperatureMeasurementResponse(_) => false,
            Message::LaserDistanceRequest(_) => true,
            Message::LaserDistanceResponse(_) => false,
            Message::MaxZoomRangeRequest(_) => true,
            Message::MaxZoomRangeResponse(_) => false,
            Message::LaserTargetLocationRequest(_) => true,
            Message::LaserTargetLocationResponse(_) => false,
            Message::CurrentZoomRequest(_) => true,
            Message::CurrentZoomResponse(_) => false,
            Message::GimbalModeRequest(_) => true,
            Message::GimbalModeResponse(_) => false,
            Message::PseudoColorRequest(_) => true,
            Message::PseudoColorResponse(_) => false,
            Message::SetPseudoColorRequest(_) => true,
            Message::SetPseudoColorResponse(_) => false,
            Message::EncodingParamsRequest(_) => true,
            Message::EncodingParamsResponse(_) => false,
            Message::SetEncodingParamsRequest(_) => true,
            Message::SetEncodingParamsResponse(_) => false,
            Message::SendAircraftAttitude(_) => true,
            Message::SendRcChannelDataRequest(_) => true,
            Message::RequestFlightControllerDataStreamRequest(_) => true,
            Message::RequestFlightControllerDataStreamResponse(_) => false,
            Message::RequestDataStreamRequest(_) => true,
            Message::RequestDataStreamResponse(_) => false,
            Message::MagneticEncoderAngleRequest(_) => true,
            Message::MagneticEncoderAngleResponse(_) => false,
            Message::GimbalControlModeRequest(_) => true,
            Message::GimbalControlModeResponse(_) => false,
            Message::WeakControlThresholdRequest(_) => true,
            Message::WeakControlThresholdResponse(_) => false,
            Message::SetWeakControlThresholdRequest(_) => true,
            Message::SetWeakControlThresholdResponse(_) => false,
            Message::MotorVoltageRequest(_) => true,
            Message::MotorVoltageResponse(_) => false,
            Message::SetUtcTimeRequest(_) => true,
            Message::SetUtcTimeResponse(_) => false,
            Message::GimbalSystemInfoRequest(_) => true,
            Message::GimbalSystemInfoResponse(_) => false,
            Message::SetLaserStateRequest(_) => true,
            Message::SetLaserStateResponse(_) => false,
            Message::ThermalOutputModeRequest(_) => true,
            Message::ThermalOutputModeResponse(_) => false,
            Message::SetThermalOutputModeRequest(_) => true,
            Message::SetThermalOutputModeResponse(_) => false,
            Message::GetSingleTemperatureFrameRequest(_) => true,
            Message::GetSingleTemperatureFrameResponse(_) => false,
            Message::ThermalGainModeRequest(_) => true,
            Message::ThermalGainModeResponse(_) => false,
            Message::SetThermalGainModeRequest(_) => true,
            Message::SetThermalGainModeResponse(_) => false,
            Message::ThermalEnvCorrectionParamsRequest(_) => true,
            Message::ThermalEnvCorrectionParamsResponse(_) => false,
            Message::SetThermalEnvCorrectionParamsRequest(_) => true,
            Message::SetThermalEnvCorrectionParamsResponse(_) => false,
            Message::ThermalEnvCorrectionSwitchRequest(_) => true,
            Message::ThermalEnvCorrectionSwitchResponse(_) => false,
            Message::SetThermalEnvCorrectionSwitchRequest(_) => true,
            Message::SetThermalEnvCorrectionSwitchResponse(_) => false,
            Message::SendGpsData(_) => true,
            Message::SystemTimeRequest(_) => true,
            Message::SystemTimeResponse(_) => false,
            Message::SingleAxisAttitudeRequest(_) => true,
            Message::SingleAxisAttitudeResponse(_) => false,
            Message::ThermalThresholdSwitchRequest(_) => true,
            Message::ThermalThresholdSwitchResponse(_) => false,
            Message::SetThermalThresholdSwitchRequest(_) => true,
            Message::SetThermalThresholdSwitchResponse(_) => false,
            Message::ThermalThresholdParamsRequest(_) => true,
            Message::ThermalThresholdParamsResponse(_) => false,
            Message::SetThermalThresholdParamsRequest(_) => true,
            Message::SetThermalThresholdParamsResponse(_) => false,
            Message::ThermalThresholdPrecisionRequest(_) => true,
            Message::ThermalThresholdPrecisionResponse(_) => false,
            Message::SetThermalThresholdPrecisionRequest(_) => true,
            Message::SetThermalThresholdPrecisionResponse(_) => false,
            Message::FormatSdCardRequest(_) => true,
            Message::FormatSdCardResponse(_) => false,
            Message::GetPictureNameTypeRequest(_) => true,
            Message::GetPictureNameTypeResponse(_) => false,
            Message::SetPictureNameTypeRequest(_) => true,
            Message::SetPictureNameTypeResponse(_) => false,
            Message::HdmiOsdStatusRequest(_) => true,
            Message::HdmiOsdStatusResponse(_) => false,
            Message::SetHdmiOsdStatusRequest(_) => true,
            Message::SetHdmiOsdStatusResponse(_) => false,
            Message::AiModeStatusRequest(_) => true,
            Message::AiModeStatusResponse(_) => false,
            Message::AiTrackingStreamStatusRequest(_) => true,
            Message::AiTrackingStreamStatusResponse(_) => false,
            Message::UpdateThermalShutterRequest(_) => true,
            Message::UpdateThermalShutterResponse(_) => false,
            Message::AiTrackingCoordinateStream(_) => false,
            Message::SetAiTrackingStreamStatusRequest(_) => true,
            Message::SetAiTrackingStreamStatusResponse(_) => false,
            Message::WeakControlModeRequest(_) => true,
            Message::WeakControlModeResponse(_) => false,
            Message::SetWeakControlModeRequest(_) => true,
            Message::SetWeakControlModeResponse(_) => false,
            Message::SoftRebootRequest(_) => true,
            Message::SoftRebootResponse(_) => false,
            Message::GetIpAddressRequest(_) => true,
            Message::GetIpAddressResponse(_) => false,
            Message::SetIpAddressRequest(_) => true,
            Message::SetIpAddressResponse(_) => false,
        }
    }
    pub fn is_response(&self) -> bool {
        !self.is_request()
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        match self {
            Message::TcpHeartbeat(m) => m.encode(buf),
            Message::FirmwareVersionRequest(m) => m.encode(buf),
            Message::FirmwareVersionResponse(m) => m.encode(buf),
            Message::HardwareIdRequest(m) => m.encode(buf),
            Message::HardwareIdResponse(m) => m.encode(buf),
            Message::AutoFocusRequest(m) => m.encode(buf),
            Message::AutoFocusResponse(m) => m.encode(buf),
            Message::ManualZoomRequest(m) => m.encode(buf),
            Message::ManualZoomResponse(m) => m.encode(buf),
            Message::ManualFocusRequest(m) => m.encode(buf),
            Message::ManualFocusResponse(m) => m.encode(buf),
            Message::GimbalRotationRequest(m) => m.encode(buf),
            Message::GimbalRotationResponse(m) => m.encode(buf),
            Message::CenterGimbalRequest(m) => m.encode(buf),
            Message::CenterGimbalResponse(m) => m.encode(buf),
            Message::CameraSystemInfoRequest(m) => m.encode(buf),
            Message::CameraSystemInfoResponse(m) => m.encode(buf),
            Message::FunctionFeedback(m) => m.encode(buf),
            Message::FunctionControl(m) => m.encode(buf),
            Message::GimbalAttitudeRequest(m) => m.encode(buf),
            Message::GimbalAttitudeResponse(m) => m.encode(buf),
            Message::SetGimbalAttitudeRequest(m) => m.encode(buf),
            Message::SetGimbalAttitudeResponse(m) => m.encode(buf),
            Message::AbsoluteZoomRequest(m) => m.encode(buf),
            Message::AbsoluteZoomResponse(m) => m.encode(buf),
            Message::VideoStitchingModeRequest(m) => m.encode(buf),
            Message::VideoStitchingModeResponse(m) => m.encode(buf),
            Message::SetVideoStitchingModeRequest(m) => m.encode(buf),
            Message::SetVideoStitchingModeResponse(m) => m.encode(buf),
            Message::GetTemperatureAtPointRequest(m) => m.encode(buf),
            Message::GetTemperatureAtPointResponse(m) => m.encode(buf),
            Message::LocalTemperatureMeasurementRequest(m) => m.encode(buf),
            Message::LocalTemperatureMeasurementResponse(m) => m.encode(buf),
            Message::GlobalTemperatureMeasurementRequest(m) => m.encode(buf),
            Message::GlobalTemperatureMeasurementResponse(m) => m.encode(buf),
            Message::LaserDistanceRequest(m) => m.encode(buf),
            Message::LaserDistanceResponse(m) => m.encode(buf),
            Message::MaxZoomRangeRequest(m) => m.encode(buf),
            Message::MaxZoomRangeResponse(m) => m.encode(buf),
            Message::LaserTargetLocationRequest(m) => m.encode(buf),
            Message::LaserTargetLocationResponse(m) => m.encode(buf),
            Message::CurrentZoomRequest(m) => m.encode(buf),
            Message::CurrentZoomResponse(m) => m.encode(buf),
            Message::GimbalModeRequest(m) => m.encode(buf),
            Message::GimbalModeResponse(m) => m.encode(buf),
            Message::PseudoColorRequest(m) => m.encode(buf),
            Message::PseudoColorResponse(m) => m.encode(buf),
            Message::SetPseudoColorRequest(m) => m.encode(buf),
            Message::SetPseudoColorResponse(m) => m.encode(buf),
            Message::EncodingParamsRequest(m) => m.encode(buf),
            Message::EncodingParamsResponse(m) => m.encode(buf),
            Message::SetEncodingParamsRequest(m) => m.encode(buf),
            Message::SetEncodingParamsResponse(m) => m.encode(buf),
            Message::SendAircraftAttitude(m) => m.encode(buf),
            Message::SendRcChannelDataRequest(m) => m.encode(buf),
            Message::RequestFlightControllerDataStreamRequest(m) => m.encode(buf),
            Message::RequestFlightControllerDataStreamResponse(m) => m.encode(buf),
            Message::RequestDataStreamRequest(m) => m.encode(buf),
            Message::RequestDataStreamResponse(m) => m.encode(buf),
            Message::MagneticEncoderAngleRequest(m) => m.encode(buf),
            Message::MagneticEncoderAngleResponse(m) => m.encode(buf),
            Message::GimbalControlModeRequest(m) => m.encode(buf),
            Message::GimbalControlModeResponse(m) => m.encode(buf),
            Message::WeakControlThresholdRequest(m) => m.encode(buf),
            Message::WeakControlThresholdResponse(m) => m.encode(buf),
            Message::SetWeakControlThresholdRequest(m) => m.encode(buf),
            Message::SetWeakControlThresholdResponse(m) => m.encode(buf),
            Message::MotorVoltageRequest(m) => m.encode(buf),
            Message::MotorVoltageResponse(m) => m.encode(buf),
            Message::SetUtcTimeRequest(m) => m.encode(buf),
            Message::SetUtcTimeResponse(m) => m.encode(buf),
            Message::GimbalSystemInfoRequest(m) => m.encode(buf),
            Message::GimbalSystemInfoResponse(m) => m.encode(buf),
            Message::SetLaserStateRequest(m) => m.encode(buf),
            Message::SetLaserStateResponse(m) => m.encode(buf),
            Message::ThermalOutputModeRequest(m) => m.encode(buf),
            Message::ThermalOutputModeResponse(m) => m.encode(buf),
            Message::SetThermalOutputModeRequest(m) => m.encode(buf),
            Message::SetThermalOutputModeResponse(m) => m.encode(buf),
            Message::GetSingleTemperatureFrameRequest(m) => m.encode(buf),
            Message::GetSingleTemperatureFrameResponse(m) => m.encode(buf),
            Message::ThermalGainModeRequest(m) => m.encode(buf),
            Message::ThermalGainModeResponse(m) => m.encode(buf),
            Message::SetThermalGainModeRequest(m) => m.encode(buf),
            Message::SetThermalGainModeResponse(m) => m.encode(buf),
            Message::ThermalEnvCorrectionParamsRequest(m) => m.encode(buf),
            Message::ThermalEnvCorrectionParamsResponse(m) => m.encode(buf),
            Message::SetThermalEnvCorrectionParamsRequest(m) => m.encode(buf),
            Message::SetThermalEnvCorrectionParamsResponse(m) => m.encode(buf),
            Message::ThermalEnvCorrectionSwitchRequest(m) => m.encode(buf),
            Message::ThermalEnvCorrectionSwitchResponse(m) => m.encode(buf),
            Message::SetThermalEnvCorrectionSwitchRequest(m) => m.encode(buf),
            Message::SetThermalEnvCorrectionSwitchResponse(m) => m.encode(buf),
            Message::SendGpsData(m) => m.encode(buf),
            Message::SystemTimeRequest(m) => m.encode(buf),
            Message::SystemTimeResponse(m) => m.encode(buf),
            Message::SingleAxisAttitudeRequest(m) => m.encode(buf),
            Message::SingleAxisAttitudeResponse(m) => m.encode(buf),
            Message::ThermalThresholdSwitchRequest(m) => m.encode(buf),
            Message::ThermalThresholdSwitchResponse(m) => m.encode(buf),
            Message::SetThermalThresholdSwitchRequest(m) => m.encode(buf),
            Message::SetThermalThresholdSwitchResponse(m) => m.encode(buf),
            Message::ThermalThresholdParamsRequest(m) => m.encode(buf),
            Message::ThermalThresholdParamsResponse(m) => m.encode(buf),
            Message::SetThermalThresholdParamsRequest(m) => m.encode(buf),
            Message::SetThermalThresholdParamsResponse(m) => m.encode(buf),
            Message::ThermalThresholdPrecisionRequest(m) => m.encode(buf),
            Message::ThermalThresholdPrecisionResponse(m) => m.encode(buf),
            Message::SetThermalThresholdPrecisionRequest(m) => m.encode(buf),
            Message::SetThermalThresholdPrecisionResponse(m) => m.encode(buf),
            Message::FormatSdCardRequest(m) => m.encode(buf),
            Message::FormatSdCardResponse(m) => m.encode(buf),
            Message::GetPictureNameTypeRequest(m) => m.encode(buf),
            Message::GetPictureNameTypeResponse(m) => m.encode(buf),
            Message::SetPictureNameTypeRequest(m) => m.encode(buf),
            Message::SetPictureNameTypeResponse(m) => m.encode(buf),
            Message::HdmiOsdStatusRequest(m) => m.encode(buf),
            Message::HdmiOsdStatusResponse(m) => m.encode(buf),
            Message::SetHdmiOsdStatusRequest(m) => m.encode(buf),
            Message::SetHdmiOsdStatusResponse(m) => m.encode(buf),
            Message::AiModeStatusRequest(m) => m.encode(buf),
            Message::AiModeStatusResponse(m) => m.encode(buf),
            Message::AiTrackingStreamStatusRequest(m) => m.encode(buf),
            Message::AiTrackingStreamStatusResponse(m) => m.encode(buf),
            Message::UpdateThermalShutterRequest(m) => m.encode(buf),
            Message::UpdateThermalShutterResponse(m) => m.encode(buf),
            Message::AiTrackingCoordinateStream(m) => m.encode(buf),
            Message::SetAiTrackingStreamStatusRequest(m) => m.encode(buf),
            Message::SetAiTrackingStreamStatusResponse(m) => m.encode(buf),
            Message::WeakControlModeRequest(m) => m.encode(buf),
            Message::WeakControlModeResponse(m) => m.encode(buf),
            Message::SetWeakControlModeRequest(m) => m.encode(buf),
            Message::SetWeakControlModeResponse(m) => m.encode(buf),
            Message::SoftRebootRequest(m) => m.encode(buf),
            Message::SoftRebootResponse(m) => m.encode(buf),
            Message::GetIpAddressRequest(m) => m.encode(buf),
            Message::GetIpAddressResponse(m) => m.encode(buf),
            Message::SetIpAddressRequest(m) => m.encode(buf),
            Message::SetIpAddressResponse(m) => m.encode(buf),
        }
    }
    pub fn decode(frame: &Frame) -> Result<Self, DecodeError> {
        let data = frame.data_slice();
        match frame.cmd {
            0x00 => Ok(Message::TcpHeartbeat(TcpHeartbeat::decode(data)?)),
            0x01 => {
                if frame.ctrl.is_ack {
                    Ok(Message::FirmwareVersionResponse(
                        FirmwareVersionResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::FirmwareVersionRequest(
                        FirmwareVersionRequest::decode(data)?,
                    ))
                }
            }
            0x02 => {
                if frame.ctrl.is_ack {
                    Ok(Message::HardwareIdResponse(HardwareIdResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::HardwareIdRequest(HardwareIdRequest::decode(data)?))
                }
            }
            0x04 => {
                if frame.ctrl.is_ack {
                    Ok(Message::AutoFocusResponse(AutoFocusResponse::decode(data)?))
                } else {
                    Ok(Message::AutoFocusRequest(AutoFocusRequest::decode(data)?))
                }
            }
            0x05 => {
                if frame.ctrl.is_ack {
                    Ok(Message::ManualZoomResponse(ManualZoomResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::ManualZoomRequest(ManualZoomRequest::decode(data)?))
                }
            }
            0x06 => {
                if frame.ctrl.is_ack {
                    Ok(Message::ManualFocusResponse(ManualFocusResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::ManualFocusRequest(ManualFocusRequest::decode(
                        data,
                    )?))
                }
            }
            0x07 => {
                if frame.ctrl.is_ack {
                    Ok(Message::GimbalRotationResponse(
                        GimbalRotationResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::GimbalRotationRequest(
                        GimbalRotationRequest::decode(data)?,
                    ))
                }
            }
            0x08 => {
                if frame.ctrl.is_ack {
                    Ok(Message::CenterGimbalResponse(CenterGimbalResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::CenterGimbalRequest(CenterGimbalRequest::decode(
                        data,
                    )?))
                }
            }
            0x0A => {
                if frame.ctrl.is_ack {
                    Ok(Message::CameraSystemInfoResponse(
                        CameraSystemInfoResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::CameraSystemInfoRequest(
                        CameraSystemInfoRequest::decode(data)?,
                    ))
                }
            }
            0x0B => Ok(Message::FunctionFeedback(FunctionFeedback::decode(data)?)),
            0x0C => Ok(Message::FunctionControl(FunctionControl::decode(data)?)),
            0x0D => {
                if frame.ctrl.is_ack {
                    Ok(Message::GimbalAttitudeResponse(
                        GimbalAttitudeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::GimbalAttitudeRequest(
                        GimbalAttitudeRequest::decode(data)?,
                    ))
                }
            }
            0x0E => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetGimbalAttitudeResponse(
                        SetGimbalAttitudeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetGimbalAttitudeRequest(
                        SetGimbalAttitudeRequest::decode(data)?,
                    ))
                }
            }
            0x0F => {
                if frame.ctrl.is_ack {
                    Ok(Message::AbsoluteZoomResponse(AbsoluteZoomResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::AbsoluteZoomRequest(AbsoluteZoomRequest::decode(
                        data,
                    )?))
                }
            }
            0x10 => {
                if frame.ctrl.is_ack {
                    Ok(Message::VideoStitchingModeResponse(
                        VideoStitchingModeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::VideoStitchingModeRequest(
                        VideoStitchingModeRequest::decode(data)?,
                    ))
                }
            }
            0x11 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetVideoStitchingModeResponse(
                        SetVideoStitchingModeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetVideoStitchingModeRequest(
                        SetVideoStitchingModeRequest::decode(data)?,
                    ))
                }
            }
            0x12 => {
                if frame.ctrl.is_ack {
                    Ok(Message::GetTemperatureAtPointResponse(
                        GetTemperatureAtPointResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::GetTemperatureAtPointRequest(
                        GetTemperatureAtPointRequest::decode(data)?,
                    ))
                }
            }
            0x13 => {
                if frame.ctrl.is_ack {
                    Ok(Message::LocalTemperatureMeasurementResponse(
                        LocalTemperatureMeasurementResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::LocalTemperatureMeasurementRequest(
                        LocalTemperatureMeasurementRequest::decode(data)?,
                    ))
                }
            }
            0x14 => {
                if frame.ctrl.is_ack {
                    Ok(Message::GlobalTemperatureMeasurementResponse(
                        GlobalTemperatureMeasurementResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::GlobalTemperatureMeasurementRequest(
                        GlobalTemperatureMeasurementRequest::decode(data)?,
                    ))
                }
            }
            0x15 => {
                if frame.ctrl.is_ack {
                    Ok(Message::LaserDistanceResponse(
                        LaserDistanceResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::LaserDistanceRequest(LaserDistanceRequest::decode(
                        data,
                    )?))
                }
            }
            0x16 => {
                if frame.ctrl.is_ack {
                    Ok(Message::MaxZoomRangeResponse(MaxZoomRangeResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::MaxZoomRangeRequest(MaxZoomRangeRequest::decode(
                        data,
                    )?))
                }
            }
            0x17 => {
                if frame.ctrl.is_ack {
                    Ok(Message::LaserTargetLocationResponse(
                        LaserTargetLocationResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::LaserTargetLocationRequest(
                        LaserTargetLocationRequest::decode(data)?,
                    ))
                }
            }
            0x18 => {
                if frame.ctrl.is_ack {
                    Ok(Message::CurrentZoomResponse(CurrentZoomResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::CurrentZoomRequest(CurrentZoomRequest::decode(
                        data,
                    )?))
                }
            }
            0x19 => {
                if frame.ctrl.is_ack {
                    Ok(Message::GimbalModeResponse(GimbalModeResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::GimbalModeRequest(GimbalModeRequest::decode(data)?))
                }
            }
            0x1A => {
                if frame.ctrl.is_ack {
                    Ok(Message::PseudoColorResponse(PseudoColorResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::PseudoColorRequest(PseudoColorRequest::decode(
                        data,
                    )?))
                }
            }
            0x1B => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetPseudoColorResponse(
                        SetPseudoColorResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetPseudoColorRequest(
                        SetPseudoColorRequest::decode(data)?,
                    ))
                }
            }
            0x20 => {
                if frame.ctrl.is_ack {
                    Ok(Message::EncodingParamsResponse(
                        EncodingParamsResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::EncodingParamsRequest(
                        EncodingParamsRequest::decode(data)?,
                    ))
                }
            }
            0x21 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetEncodingParamsResponse(
                        SetEncodingParamsResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetEncodingParamsRequest(
                        SetEncodingParamsRequest::decode(data)?,
                    ))
                }
            }
            0x22 => Ok(Message::SendAircraftAttitude(SendAircraftAttitude::decode(
                data,
            )?)),
            0x23 => Ok(Message::SendRcChannelDataRequest(
                SendRcChannelDataRequest::decode(data)?,
            )),
            0x24 => {
                if frame.ctrl.is_ack {
                    Ok(Message::RequestFlightControllerDataStreamResponse(
                        RequestFlightControllerDataStreamResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::RequestFlightControllerDataStreamRequest(
                        RequestFlightControllerDataStreamRequest::decode(data)?,
                    ))
                }
            }
            0x25 => {
                if frame.ctrl.is_ack {
                    Ok(Message::RequestDataStreamResponse(
                        RequestDataStreamResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::RequestDataStreamRequest(
                        RequestDataStreamRequest::decode(data)?,
                    ))
                }
            }
            0x26 => {
                if frame.ctrl.is_ack {
                    Ok(Message::MagneticEncoderAngleResponse(
                        MagneticEncoderAngleResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::MagneticEncoderAngleRequest(
                        MagneticEncoderAngleRequest::decode(data)?,
                    ))
                }
            }
            0x27 => {
                if frame.ctrl.is_ack {
                    Ok(Message::GimbalControlModeResponse(
                        GimbalControlModeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::GimbalControlModeRequest(
                        GimbalControlModeRequest::decode(data)?,
                    ))
                }
            }
            0x28 => {
                if frame.ctrl.is_ack {
                    Ok(Message::WeakControlThresholdResponse(
                        WeakControlThresholdResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::WeakControlThresholdRequest(
                        WeakControlThresholdRequest::decode(data)?,
                    ))
                }
            }
            0x29 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetWeakControlThresholdResponse(
                        SetWeakControlThresholdResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetWeakControlThresholdRequest(
                        SetWeakControlThresholdRequest::decode(data)?,
                    ))
                }
            }
            0x2A => {
                if frame.ctrl.is_ack {
                    Ok(Message::MotorVoltageResponse(MotorVoltageResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::MotorVoltageRequest(MotorVoltageRequest::decode(
                        data,
                    )?))
                }
            }
            0x30 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetUtcTimeResponse(SetUtcTimeResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::SetUtcTimeRequest(SetUtcTimeRequest::decode(data)?))
                }
            }
            0x31 => {
                if frame.ctrl.is_ack {
                    Ok(Message::GimbalSystemInfoResponse(
                        GimbalSystemInfoResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::GimbalSystemInfoRequest(
                        GimbalSystemInfoRequest::decode(data)?,
                    ))
                }
            }
            0x32 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetLaserStateResponse(
                        SetLaserStateResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetLaserStateRequest(SetLaserStateRequest::decode(
                        data,
                    )?))
                }
            }
            0x33 => {
                if frame.ctrl.is_ack {
                    Ok(Message::ThermalOutputModeResponse(
                        ThermalOutputModeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::ThermalOutputModeRequest(
                        ThermalOutputModeRequest::decode(data)?,
                    ))
                }
            }
            0x34 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetThermalOutputModeResponse(
                        SetThermalOutputModeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetThermalOutputModeRequest(
                        SetThermalOutputModeRequest::decode(data)?,
                    ))
                }
            }
            0x35 => {
                if frame.ctrl.is_ack {
                    Ok(Message::GetSingleTemperatureFrameResponse(
                        GetSingleTemperatureFrameResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::GetSingleTemperatureFrameRequest(
                        GetSingleTemperatureFrameRequest::decode(data)?,
                    ))
                }
            }
            0x37 => {
                if frame.ctrl.is_ack {
                    Ok(Message::ThermalGainModeResponse(
                        ThermalGainModeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::ThermalGainModeRequest(
                        ThermalGainModeRequest::decode(data)?,
                    ))
                }
            }
            0x38 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetThermalGainModeResponse(
                        SetThermalGainModeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetThermalGainModeRequest(
                        SetThermalGainModeRequest::decode(data)?,
                    ))
                }
            }
            0x39 => {
                if frame.ctrl.is_ack {
                    Ok(Message::ThermalEnvCorrectionParamsResponse(
                        ThermalEnvCorrectionParamsResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::ThermalEnvCorrectionParamsRequest(
                        ThermalEnvCorrectionParamsRequest::decode(data)?,
                    ))
                }
            }
            0x3A => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetThermalEnvCorrectionParamsResponse(
                        SetThermalEnvCorrectionParamsResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetThermalEnvCorrectionParamsRequest(
                        SetThermalEnvCorrectionParamsRequest::decode(data)?,
                    ))
                }
            }
            0x3B => {
                if frame.ctrl.is_ack {
                    Ok(Message::ThermalEnvCorrectionSwitchResponse(
                        ThermalEnvCorrectionSwitchResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::ThermalEnvCorrectionSwitchRequest(
                        ThermalEnvCorrectionSwitchRequest::decode(data)?,
                    ))
                }
            }
            0x3C => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetThermalEnvCorrectionSwitchResponse(
                        SetThermalEnvCorrectionSwitchResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetThermalEnvCorrectionSwitchRequest(
                        SetThermalEnvCorrectionSwitchRequest::decode(data)?,
                    ))
                }
            }
            0x3E => Ok(Message::SendGpsData(SendGpsData::decode(data)?)),
            0x40 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SystemTimeResponse(SystemTimeResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::SystemTimeRequest(SystemTimeRequest::decode(data)?))
                }
            }
            0x41 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SingleAxisAttitudeResponse(
                        SingleAxisAttitudeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SingleAxisAttitudeRequest(
                        SingleAxisAttitudeRequest::decode(data)?,
                    ))
                }
            }
            0x42 => {
                if frame.ctrl.is_ack {
                    Ok(Message::ThermalThresholdSwitchResponse(
                        ThermalThresholdSwitchResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::ThermalThresholdSwitchRequest(
                        ThermalThresholdSwitchRequest::decode(data)?,
                    ))
                }
            }
            0x43 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetThermalThresholdSwitchResponse(
                        SetThermalThresholdSwitchResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetThermalThresholdSwitchRequest(
                        SetThermalThresholdSwitchRequest::decode(data)?,
                    ))
                }
            }
            0x44 => {
                if frame.ctrl.is_ack {
                    Ok(Message::ThermalThresholdParamsResponse(
                        ThermalThresholdParamsResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::ThermalThresholdParamsRequest(
                        ThermalThresholdParamsRequest::decode(data)?,
                    ))
                }
            }
            0x45 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetThermalThresholdParamsResponse(
                        SetThermalThresholdParamsResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetThermalThresholdParamsRequest(
                        SetThermalThresholdParamsRequest::decode(data)?,
                    ))
                }
            }
            0x46 => {
                if frame.ctrl.is_ack {
                    Ok(Message::ThermalThresholdPrecisionResponse(
                        ThermalThresholdPrecisionResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::ThermalThresholdPrecisionRequest(
                        ThermalThresholdPrecisionRequest::decode(data)?,
                    ))
                }
            }
            0x47 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetThermalThresholdPrecisionResponse(
                        SetThermalThresholdPrecisionResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetThermalThresholdPrecisionRequest(
                        SetThermalThresholdPrecisionRequest::decode(data)?,
                    ))
                }
            }
            0x48 => {
                if frame.ctrl.is_ack {
                    Ok(Message::FormatSdCardResponse(FormatSdCardResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::FormatSdCardRequest(FormatSdCardRequest::decode(
                        data,
                    )?))
                }
            }
            0x49 => {
                if frame.ctrl.is_ack {
                    Ok(Message::GetPictureNameTypeResponse(
                        GetPictureNameTypeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::GetPictureNameTypeRequest(
                        GetPictureNameTypeRequest::decode(data)?,
                    ))
                }
            }
            0x4A => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetPictureNameTypeResponse(
                        SetPictureNameTypeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetPictureNameTypeRequest(
                        SetPictureNameTypeRequest::decode(data)?,
                    ))
                }
            }
            0x4B => {
                if frame.ctrl.is_ack {
                    Ok(Message::HdmiOsdStatusResponse(
                        HdmiOsdStatusResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::HdmiOsdStatusRequest(HdmiOsdStatusRequest::decode(
                        data,
                    )?))
                }
            }
            0x4C => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetHdmiOsdStatusResponse(
                        SetHdmiOsdStatusResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetHdmiOsdStatusRequest(
                        SetHdmiOsdStatusRequest::decode(data)?,
                    ))
                }
            }
            0x4D => {
                if frame.ctrl.is_ack {
                    Ok(Message::AiModeStatusResponse(AiModeStatusResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::AiModeStatusRequest(AiModeStatusRequest::decode(
                        data,
                    )?))
                }
            }
            0x4E => {
                if frame.ctrl.is_ack {
                    Ok(Message::AiTrackingStreamStatusResponse(
                        AiTrackingStreamStatusResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::AiTrackingStreamStatusRequest(
                        AiTrackingStreamStatusRequest::decode(data)?,
                    ))
                }
            }
            0x4F => {
                if frame.ctrl.is_ack {
                    Ok(Message::UpdateThermalShutterResponse(
                        UpdateThermalShutterResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::UpdateThermalShutterRequest(
                        UpdateThermalShutterRequest::decode(data)?,
                    ))
                }
            }
            0x50 => Ok(Message::AiTrackingCoordinateStream(
                AiTrackingCoordinateStream::decode(data)?,
            )),
            0x51 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetAiTrackingStreamStatusResponse(
                        SetAiTrackingStreamStatusResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetAiTrackingStreamStatusRequest(
                        SetAiTrackingStreamStatusRequest::decode(data)?,
                    ))
                }
            }
            0x70 => {
                if frame.ctrl.is_ack {
                    Ok(Message::WeakControlModeResponse(
                        WeakControlModeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::WeakControlModeRequest(
                        WeakControlModeRequest::decode(data)?,
                    ))
                }
            }
            0x71 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetWeakControlModeResponse(
                        SetWeakControlModeResponse::decode(data)?,
                    ))
                } else {
                    Ok(Message::SetWeakControlModeRequest(
                        SetWeakControlModeRequest::decode(data)?,
                    ))
                }
            }
            0x80 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SoftRebootResponse(SoftRebootResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::SoftRebootRequest(SoftRebootRequest::decode(data)?))
                }
            }
            0x81 => {
                if frame.ctrl.is_ack {
                    Ok(Message::GetIpAddressResponse(GetIpAddressResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::GetIpAddressRequest(GetIpAddressRequest::decode(
                        data,
                    )?))
                }
            }
            0x82 => {
                if frame.ctrl.is_ack {
                    Ok(Message::SetIpAddressResponse(SetIpAddressResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Message::SetIpAddressRequest(SetIpAddressRequest::decode(
                        data,
                    )?))
                }
            }
            _ => Err(DecodeError::UnknownCmdId),
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert Frame to bytes (serialize)
pub fn frame_to_bytes(frame: &Frame, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let data_len = frame.data_len as usize;
    let total = 10 + data_len;
    if buf.len() < total {
        return Err(EncodeError::BufferTooSmall);
    }
    buf[0..2].copy_from_slice(&STX.to_le_bytes());
    buf[2] = frame.ctrl.to_u8();
    buf[3..5].copy_from_slice(&frame.data_len.to_le_bytes());
    buf[5..7].copy_from_slice(&frame.seq.to_le_bytes());
    buf[7] = frame.cmd;
    buf[8..8 + data_len].copy_from_slice(&frame.data[..data_len]);
    let crc = crc16_calc(&buf[..8 + data_len]);
    buf[8 + data_len..8 + data_len + 2].copy_from_slice(&crc.to_le_bytes());
    Ok(total)
}

/// Convert bytes to Frame (deserialize) - for complete packets
pub fn bytes_to_frame(data: &[u8]) -> Result<Frame, DecodeError> {
    if data.len() < 10 {
        return Err(DecodeError::FrameIncomplete);
    }
    let stx = u16::from_le_bytes([data[0], data[1]]);
    if stx != STX {
        return Err(DecodeError::InvalidStx);
    }
    let data_len = u16::from_le_bytes([data[3], data[4]]) as usize;
    let total = 10 + data_len;
    if data.len() < total {
        return Err(DecodeError::FrameIncomplete);
    }
    let crc_recv = u16::from_le_bytes([data[8 + data_len], data[8 + data_len + 1]]);
    let crc_calc = crc16_calc(&data[..8 + data_len]);
    if crc_recv != crc_calc {
        return Err(DecodeError::CrcMismatch);
    }
    let mut frame = Frame::default();
    frame.ctrl = CtrlByte::from_u8(data[2]);
    frame.seq = u16::from_le_bytes([data[5], data[6]]);
    frame.cmd = data[7];
    frame.data_len = data_len as u16;
    frame.data[..data_len].copy_from_slice(&data[8..8 + data_len]);
    Ok(frame)
}

/// Convert Message to bytes (full frame serialization)
pub fn message_to_bytes(msg: &Message, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let mut msg_buf = [0u8; MAX_MESSAGE_SIZE];
    let msg_len = msg.encode(&mut msg_buf)?;
    let mut frame = Frame::new(msg.cmd_id(), msg.is_response());
    frame.data[..msg_len].copy_from_slice(&msg_buf[..msg_len]);
    frame.data_len = msg_len as u16;
    frame_to_bytes(&frame, buf)
}

/// Convert bytes to Message (full frame deserialization)
pub fn bytes_to_message(data: &[u8]) -> Result<Message, DecodeError> {
    let frame = bytes_to_frame(data)?;
    Message::decode(&frame)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_frame_parser() {
        let mut parser = FrameParser::new();
        let frame = Frame::new(0x01, false);
        let mut buf = [0u8; MAX_FRAME_SIZE];
        let len = frame_to_bytes(&frame, &mut buf).unwrap();
        let mut parsed_flag = false;
        for i in 0..len {
            if let Ok(Some(parsed)) = parser.feed(buf[i]) {
                parsed_flag = true;
                assert_eq!(parsed.cmd, frame.cmd);
                break;
            }
        }
        assert!(parsed_flag);
    }
    #[test]
    fn test_helpers() {
        let frame = Frame::new(0x01, false);
        let mut buf = [0u8; MAX_FRAME_SIZE];
        let len = frame_to_bytes(&frame, &mut buf).unwrap();
        let decoded = bytes_to_frame(&buf[..len]).unwrap();
        assert_eq!(frame.cmd, decoded.cmd);
    }
}
