// Auto-generated from SIYI_Gimbal_Camera_External_SDK_Protocol
// Protocol: TTL
// Camera: A2mini
#![no_std]
#![allow(dead_code, clippy::derivable_impls, unused, non_snake_case)]
use core::convert::TryInto;
pub const STX: u16 = 0x6655;
pub const STX_LITTLE: bool = true;
pub const MAX_MESSAGE_SIZE: usize = 512;
pub const MAX_FRAME_SIZE: usize = 522;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeError {
    BufferTooSmall,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub const fn crc16_calc(data: &[u8], crc_init: u16) -> u16 {
    let mut crc = crc_init;
    let mut i = 0;
    while i < data.len() {
        crc = (crc << 8) ^ CRC16_TAB[((crc >> 8) as u8 ^ data[i]) as usize];
        i += 1;
    }
    crc
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CtrlByte {
    pub need_ack: bool,
    pub is_ack: bool,
}
impl CtrlByte {
    pub const fn from_u8(val: u8) -> Self {
        Self {
            need_ack: (val & 1) != 0,
            is_ack: (val & 2) != 0,
        }
    }
    pub const fn to_u8(&self) -> u8 {
        (if self.need_ack { 1 } else { 0 }) | (if self.is_ack { 2 } else { 0 })
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
    pub const fn is_request(&self) -> bool {
        !self.is_ack
    }
    pub const fn is_response(&self) -> bool {
        self.is_ack
    }
}
impl Default for CtrlByte {
    fn default() -> Self {
        Self::request()
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BooleanStatus {
    Failed = 0,
    Success = 1,
}
impl BooleanStatus {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BooleanOnOff {
    Off = 0,
    On = 1,
}
impl BooleanOnOff {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GimbalMode {
    Lock = 0,
    Follow = 1,
    FPV = 2,
}
impl GimbalMode {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GimbalMountingDir {
    Reserved = 0,
    Normal = 1,
    Inverted = 2,
}
impl GimbalMountingDir {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VideoOutput {
    HDMI = 0,
    CVBS = 1,
}
impl VideoOutput {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecordingStatus {
    NotRecording = 0,
    Recording = 1,
    NoCard = 2,
    DataLoss = 3,
}
impl RecordingStatus {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FunctionType {
    TakePhoto = 0,
    HDRToggle = 1,
    StartRecording = 2,
    LockMode = 3,
    FollowMode = 4,
    FPVMode = 5,
    EnableHDMI = 6,
    EnableCVBS = 7,
    DisableVideo = 8,
    TiltDownward = 9,
    ZoomLinkage = 10,
}
impl FunctionType {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CenterPosition {
    CenterOnly = 1,
    CenterDownward = 2,
    Center = 3,
    Downward = 4,
}
impl CenterPosition {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VideoEncType {
    H264 = 1,
    H265 = 2,
}
impl VideoEncType {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StreamType {
    Recording = 0,
    Main = 1,
    Sub = 2,
}
impl StreamType {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TempMeasurementFlag {
    Disable = 0,
    Once = 1,
    Continuous = 2,
}
impl TempMeasurementFlag {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DataStreamType {
    Attitude = 1,
    LaserRange = 2,
    MagneticEncoder = 3,
    MotorVoltage = 4,
}
impl DataStreamType {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ThermalOutputMode {
    Fps30 = 0,
    Fps25WithTemp = 1,
}
impl ThermalOutputMode {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ThermalGainMode {
    LowGain = 0,
    HighGain = 1,
}
impl ThermalGainMode {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AITrackingStatus {
    NormalTracking = 0,
    IntermittentLoss = 1,
    Lost = 2,
    UserCanceled = 3,
    NormalTrackingAny = 4,
}
impl AITrackingStatus {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AITargetType {
    Human = 0,
    Car = 1,
    Bus = 2,
    Truck = 3,
    AnyObject = 255,
}
impl AITargetType {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ControlMode {
    AttitudeMode = 0,
    WeakMode = 1,
    MiddleMode = 2,
    FPVMode = 3,
    MotorClose = 4,
}
impl ControlMode {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FileType {
    Picture = 0,
    TempRawFile = 1,
    RecordVideo = 2,
}
impl FileType {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FileNameType {
    Reserve = 0,
    Index = 1,
    TimeStamp = 2,
}
impl FileNameType {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ThermalThresholdPrecision {
    MaxAccurate = 1,
    MidAccurate = 2,
    MinAccurate = 3,
}
impl ThermalThresholdPrecision {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SingleAxisControl {
    YawControl = 0,
    PitchControl = 1,
}
impl SingleAxisControl {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub ctrl: CtrlByte,
    pub seq: u16,
    pub cmd: u8,
    pub data: [u8; MAX_MESSAGE_SIZE],
    pub data_len: u16,
}
impl Frame {
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let total_len = 10 + self.data_len as usize;
        if buf.len() < total_len {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[0..2].copy_from_slice(&if STX_LITTLE {
            STX.to_le_bytes()
        } else {
            STX.to_be_bytes()
        });
        buf[2] = self.ctrl.to_u8();
        buf[3..5].copy_from_slice(&self.data_len.to_le_bytes());
        buf[5..7].copy_from_slice(&self.seq.to_le_bytes());
        buf[7] = self.cmd;
        buf[8..8 + self.data_len as usize].copy_from_slice(&self.data[..self.data_len as usize]);
        let crc = crc16_calc(&buf[0..8 + self.data_len as usize], 0);
        buf[8 + self.data_len as usize..total_len].copy_from_slice(&crc.to_le_bytes());
        Ok(total_len)
    }
    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        if buf.len() < 10 {
            return Err(DecodeError::FrameTooShort);
        }
        let stx = if STX_LITTLE {
            u16::from_le_bytes(buf[0..2].try_into().unwrap())
        } else {
            u16::from_be_bytes(buf[0..2].try_into().unwrap())
        };
        if stx != STX {
            return Err(DecodeError::InvalidStx);
        }
        let data_len = u16::from_le_bytes(buf[3..5].try_into().unwrap()) as usize;
        let expected_len = 10 + data_len;
        if buf.len() < expected_len {
            return Err(DecodeError::FrameIncomplete);
        }
        let crc_recv = u16::from_le_bytes(buf[expected_len - 2..expected_len].try_into().unwrap());
        if crc_recv != crc16_calc(&buf[..expected_len - 2], 0) {
            return Err(DecodeError::CrcMismatch);
        }
        let mut data = [0u8; MAX_MESSAGE_SIZE];
        data[..data_len].copy_from_slice(&buf[8..8 + data_len]);
        Ok(Self {
            ctrl: CtrlByte::from_u8(buf[2]),
            seq: u16::from_le_bytes(buf[5..7].try_into().unwrap()),
            cmd: buf[7],
            data,
            data_len: data_len as u16,
        })
    }
}

// ============================================================================
// Message Structures
// ============================================================================
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FirmwareVersionRequest {}
impl FirmwareVersionRequest {
    pub const CMD_ID: u8 = 0x01;
    pub const IS_REQUEST: bool = true;
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        Ok(Self {})
    }
}
impl Default for FirmwareVersionRequest {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FirmwareVersionResponse {
    pub camera_firmware_ver: u32,
    pub gimbal_firmware_ver: u32,
}
impl FirmwareVersionResponse {
    pub const CMD_ID: u8 = 0x01;
    pub const IS_REQUEST: bool = false;
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
        Ok(Self {
            camera_firmware_ver,
            gimbal_firmware_ver,
        })
    }
}
impl Default for FirmwareVersionResponse {
    fn default() -> Self {
        Self {
            camera_firmware_ver: 0,
            gimbal_firmware_ver: 0,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GimbalRotationRequest {
    pub yaw: i8,
    pub pitch: i8,
}
impl GimbalRotationRequest {
    pub const CMD_ID: u8 = 0x07;
    pub const IS_REQUEST: bool = true;
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
        Self { yaw: 0, pitch: 0 }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraSystemInfoRequest {}
impl CameraSystemInfoRequest {
    pub const CMD_ID: u8 = 0x0A;
    pub const IS_REQUEST: bool = true;
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mut idx = 0;
        Ok(idx)
    }
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut idx = 0;
        Ok(Self {})
    }
}
impl Default for CameraSystemInfoRequest {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraSystemInfoResponse {
    pub reserved1: u8,
    pub hdr_status: BooleanOnOff,
    pub reserved2: u8,
    pub gimbal_motion_mode: GimbalMode,
    pub gimbal_mounting_dir: GimbalMountingDir,
}
impl CameraSystemInfoResponse {
    pub const CMD_ID: u8 = 0x0A;
    pub const IS_REQUEST: bool = false;
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
        buf[idx] = self.gimbal_motion_mode as u8;
        idx += 1;
        if idx >= buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        buf[idx] = self.gimbal_mounting_dir as u8;
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
        let gimbal_motion_mode =
            GimbalMode::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        if idx >= data.len() {
            return Err(DecodeError::NotEnoughBytes);
        }
        let gimbal_mounting_dir =
            GimbalMountingDir::from_u8(data[idx]).ok_or(DecodeError::InvalidEnumValue)?;
        idx += 1;
        Ok(Self {
            reserved1,
            hdr_status,
            reserved2,
            gimbal_motion_mode,
            gimbal_mounting_dir,
        })
    }
}
impl Default for CameraSystemInfoResponse {
    fn default() -> Self {
        Self {
            reserved1: 0,
            hdr_status: BooleanOnOff::default(),
            reserved2: 0,
            gimbal_motion_mode: GimbalMode::default(),
            gimbal_mounting_dir: GimbalMountingDir::default(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FunctionFeedback {
    pub info_type: FeedbackInfoType,
}
impl FunctionFeedback {
    pub const CMD_ID: u8 = 0x0B;
    pub const IS_REQUEST: bool = false;
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
        Self {
            info_type: FeedbackInfoType::default(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FunctionControl {
    pub func_type: FunctionType,
}
impl FunctionControl {
    pub const CMD_ID: u8 = 0x0C;
    pub const IS_REQUEST: bool = true;
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
        Self {
            func_type: FunctionType::default(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SendRcChannelDataRequest {
    pub chan1_raw: u16,
    pub chan2_raw: u16,
    pub chan3_raw: u16,
    pub chan4_raw: u16,
    pub chan5_raw: u16,
    pub chan6_raw: u16,
    pub chan7_raw: u16,
    pub chan8_raw: u16,
    pub chan9_raw: u16,
    pub chan10_raw: u16,
    pub chan11_raw: u16,
    pub chan12_raw: u16,
    pub chan13_raw: u16,
    pub chan14_raw: u16,
    pub chan15_raw: u16,
    pub chan16_raw: u16,
    pub chan17_raw: u16,
    pub chan18_raw: u16,
    pub chancount: u8,
    pub rssi: u8,
}
impl SendRcChannelDataRequest {
    pub const CMD_ID: u8 = 0x23;
    pub const IS_REQUEST: bool = true;
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
        Self {
            chan1_raw: 0,
            chan2_raw: 0,
            chan3_raw: 0,
            chan4_raw: 0,
            chan5_raw: 0,
            chan6_raw: 0,
            chan7_raw: 0,
            chan8_raw: 0,
            chan9_raw: 0,
            chan10_raw: 0,
            chan11_raw: 0,
            chan12_raw: 0,
            chan13_raw: 0,
            chan14_raw: 0,
            chan15_raw: 0,
            chan16_raw: 0,
            chan17_raw: 0,
            chan18_raw: 0,
            chancount: 0,
            rssi: 0,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RequestFlightControllerDataStreamRequest {
    pub data_type: DataStreamType,
    pub data_freq: DataFrequency,
}
impl RequestFlightControllerDataStreamRequest {
    pub const CMD_ID: u8 = 0x24;
    pub const IS_REQUEST: bool = true;
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
        Self {
            data_type: DataStreamType::default(),
            data_freq: DataFrequency::default(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RequestFlightControllerDataStreamResponse {
    pub data_type: DataStreamType,
}
impl RequestFlightControllerDataStreamResponse {
    pub const CMD_ID: u8 = 0x24;
    pub const IS_REQUEST: bool = false;
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
        Self {
            data_type: DataStreamType::default(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FormatSdCardRequest {
    pub format_sta: BooleanStatus,
}
impl FormatSdCardRequest {
    pub const CMD_ID: u8 = 0x48;
    pub const IS_REQUEST: bool = true;
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
        Self {
            format_sta: BooleanStatus::default(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FormatSdCardResponse {
    pub format_sta: BooleanStatus,
}
impl FormatSdCardResponse {
    pub const CMD_ID: u8 = 0x48;
    pub const IS_REQUEST: bool = false;
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
        Self {
            format_sta: BooleanStatus::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Message {
    FirmwareVersionRequest(FirmwareVersionRequest),
    FirmwareVersionResponse(FirmwareVersionResponse),
    GimbalRotationRequest(GimbalRotationRequest),
    CameraSystemInfoRequest(CameraSystemInfoRequest),
    CameraSystemInfoResponse(CameraSystemInfoResponse),
    FunctionFeedback(FunctionFeedback),
    FunctionControl(FunctionControl),
    SendRcChannelDataRequest(SendRcChannelDataRequest),
    RequestFlightControllerDataStreamRequest(RequestFlightControllerDataStreamRequest),
    RequestFlightControllerDataStreamResponse(RequestFlightControllerDataStreamResponse),
    FormatSdCardRequest(FormatSdCardRequest),
    FormatSdCardResponse(FormatSdCardResponse),
}
impl Message {
    pub const fn cmd_id(&self) -> u8 {
        match self {
            Self::FirmwareVersionRequest(_) => FirmwareVersionRequest::CMD_ID,
            Self::FirmwareVersionResponse(_) => FirmwareVersionResponse::CMD_ID,
            Self::GimbalRotationRequest(_) => GimbalRotationRequest::CMD_ID,
            Self::CameraSystemInfoRequest(_) => CameraSystemInfoRequest::CMD_ID,
            Self::CameraSystemInfoResponse(_) => CameraSystemInfoResponse::CMD_ID,
            Self::FunctionFeedback(_) => FunctionFeedback::CMD_ID,
            Self::FunctionControl(_) => FunctionControl::CMD_ID,
            Self::SendRcChannelDataRequest(_) => SendRcChannelDataRequest::CMD_ID,
            Self::RequestFlightControllerDataStreamRequest(_) => {
                RequestFlightControllerDataStreamRequest::CMD_ID
            }
            Self::RequestFlightControllerDataStreamResponse(_) => {
                RequestFlightControllerDataStreamResponse::CMD_ID
            }
            Self::FormatSdCardRequest(_) => FormatSdCardRequest::CMD_ID,
            Self::FormatSdCardResponse(_) => FormatSdCardResponse::CMD_ID,
        }
    }
    pub const fn is_request(&self) -> bool {
        match self {
            Self::FirmwareVersionRequest(_) => FirmwareVersionRequest::IS_REQUEST,
            Self::FirmwareVersionResponse(_) => FirmwareVersionResponse::IS_REQUEST,
            Self::GimbalRotationRequest(_) => GimbalRotationRequest::IS_REQUEST,
            Self::CameraSystemInfoRequest(_) => CameraSystemInfoRequest::IS_REQUEST,
            Self::CameraSystemInfoResponse(_) => CameraSystemInfoResponse::IS_REQUEST,
            Self::FunctionFeedback(_) => FunctionFeedback::IS_REQUEST,
            Self::FunctionControl(_) => FunctionControl::IS_REQUEST,
            Self::SendRcChannelDataRequest(_) => SendRcChannelDataRequest::IS_REQUEST,
            Self::RequestFlightControllerDataStreamRequest(_) => {
                RequestFlightControllerDataStreamRequest::IS_REQUEST
            }
            Self::RequestFlightControllerDataStreamResponse(_) => {
                RequestFlightControllerDataStreamResponse::IS_REQUEST
            }
            Self::FormatSdCardRequest(_) => FormatSdCardRequest::IS_REQUEST,
            Self::FormatSdCardResponse(_) => FormatSdCardResponse::IS_REQUEST,
        }
    }
    pub const fn is_response(&self) -> bool {
        !self.is_request()
    }
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        match self {
            Self::FirmwareVersionRequest(m) => m.encode(buf),
            Self::FirmwareVersionResponse(m) => m.encode(buf),
            Self::GimbalRotationRequest(m) => m.encode(buf),
            Self::CameraSystemInfoRequest(m) => m.encode(buf),
            Self::CameraSystemInfoResponse(m) => m.encode(buf),
            Self::FunctionFeedback(m) => m.encode(buf),
            Self::FunctionControl(m) => m.encode(buf),
            Self::SendRcChannelDataRequest(m) => m.encode(buf),
            Self::RequestFlightControllerDataStreamRequest(m) => m.encode(buf),
            Self::RequestFlightControllerDataStreamResponse(m) => m.encode(buf),
            Self::FormatSdCardRequest(m) => m.encode(buf),
            Self::FormatSdCardResponse(m) => m.encode(buf),
        }
    }
    pub fn from_frame(frame: &Frame) -> Result<Self, DecodeError> {
        let data = &frame.data[..frame.data_len as usize];
        match frame.cmd {
            0x01 => {
                if frame.ctrl.is_response() {
                    Ok(Self::FirmwareVersionResponse(
                        FirmwareVersionResponse::decode(data)?,
                    ))
                } else {
                    Ok(Self::FirmwareVersionRequest(
                        FirmwareVersionRequest::decode(data)?,
                    ))
                }
            }
            0x07 => Ok(Self::GimbalRotationRequest(GimbalRotationRequest::decode(
                data,
            )?)),
            0x0A => {
                if frame.ctrl.is_response() {
                    Ok(Self::CameraSystemInfoResponse(
                        CameraSystemInfoResponse::decode(data)?,
                    ))
                } else {
                    Ok(Self::CameraSystemInfoRequest(
                        CameraSystemInfoRequest::decode(data)?,
                    ))
                }
            }
            0x0B => Ok(Self::FunctionFeedback(FunctionFeedback::decode(data)?)),
            0x0C => Ok(Self::FunctionControl(FunctionControl::decode(data)?)),
            0x23 => Ok(Self::SendRcChannelDataRequest(
                SendRcChannelDataRequest::decode(data)?,
            )),
            0x24 => {
                if frame.ctrl.is_response() {
                    Ok(Self::RequestFlightControllerDataStreamResponse(
                        RequestFlightControllerDataStreamResponse::decode(data)?,
                    ))
                } else {
                    Ok(Self::RequestFlightControllerDataStreamRequest(
                        RequestFlightControllerDataStreamRequest::decode(data)?,
                    ))
                }
            }
            0x48 => {
                if frame.ctrl.is_response() {
                    Ok(Self::FormatSdCardResponse(FormatSdCardResponse::decode(
                        data,
                    )?))
                } else {
                    Ok(Self::FormatSdCardRequest(FormatSdCardRequest::decode(
                        data,
                    )?))
                }
            }
            _ => Err(DecodeError::UnknownCmdId),
        }
    }
}
pub fn encode_message(msg: &Message, frame_buf: &mut [u8]) -> Result<usize, EncodeError> {
    let mut data_buf = [0u8; MAX_MESSAGE_SIZE];
    let data_len = msg.encode(&mut data_buf)?;
    let mut frame = Frame {
        ctrl: if msg.is_response() {
            CtrlByte::response()
        } else {
            CtrlByte::request()
        },
        seq: 0,
        cmd: msg.cmd_id(),
        data: [0u8; MAX_MESSAGE_SIZE],
        data_len: data_len as u16,
    };
    frame.data[..data_len].copy_from_slice(&data_buf[..data_len]);
    frame.encode(frame_buf)
}
pub fn decode_message(buf: &[u8]) -> Result<Message, DecodeError> {
    let frame = Frame::decode(buf)?;
    Message::from_frame(&frame)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_message_roundtrip() {
        let mut msg = FirmwareVersionRequest::default();
        let wrapped_msg = Message::FirmwareVersionRequest(msg);
        let mut frame_buf = [0u8; MAX_FRAME_SIZE];
        let len = encode_message(&wrapped_msg, &mut frame_buf).unwrap();
        let decoded_msg = decode_message(&frame_buf[..len]).unwrap();
        assert_eq!(wrapped_msg, decoded_msg);
    }
}

// ============================================================================
// Module Documentation
// ============================================================================

/// # SIYI Protocol - Generated Module
///
/// This module contains message definitions for the **A2mini** camera
/// using the **TTL** protocol.
///
/// ## Features
///
/// - **No heap allocation**: All operations use stack-allocated buffers
/// - **No lifetimes**: All data is owned in fixed-size arrays
/// - **CRC16 validation**: Automatic frame integrity checking
/// - **Type-safe enums**: Protocol enumerations with validation
/// - **No_std compatible**: Works in bare-metal environments
///
/// ## Quick Start
///
/// ### Encoding a Message
///
/// ```rust
/// use siyi_protocol::a2mini_ttl::*;
///
/// // Create a request
/// let request = FirmwareVersionRequest::default();
///
/// // Encode to frame buffer
/// let mut frame_buf = [0u8; MAX_FRAME_SIZE];
/// let msg = Message::FirmwareVersionRequest(request);
/// let frame_len = encode_message(&msg, &mut frame_buf).unwrap();
///
/// // Send frame_buf[..frame_len] over your transport layer
/// ```
///
/// ### Decoding a Message
///
/// ```rust
/// use siyi_protocol::a2mini_ttl::*;
///
/// // Receive data from your transport layer
/// let received_data: &[u8] = /* ... */;
///
/// // Decode the frame
/// match decode_message(received_data) {
///     Ok(Message::FirmwareVersionResponse(resp)) => {
///         println!("Camera FW: {}", resp.camera_firmware_ver);
///     }
///     Ok(msg) => println!("Other message: {:?}", msg),
///     Err(e) => eprintln!("Decode error: {:?}", e),
/// }
/// ```
///
/// ## Protocol Frame Format
///
/// ```text
/// +--------+------+---------+-----+--------+------+---------+
/// | STX    | CTRL | DATALEN | SEQ | CMD_ID | DATA | CRC16   |
/// | 2 bytes| 1    | 2       | 2   | 1      | N    | 2 bytes |
/// +--------+------+---------+-----+--------+------+---------+
/// ```
///
/// - **STX**: Start marker (0x6655, little-endian)
/// - **CTRL**: Control byte (bit 0: need_ack, bit 1: is_ack)
/// - **DATALEN**: Data payload length (little-endian)
/// - **SEQ**: Sequence number (little-endian)
/// - **CMD_ID**: Command identifier
/// - **DATA**: Message payload
/// - **CRC16**: CRC16-CCITT checksum (little-endian)
///
/// ## Available Messages
///
/// ### System Information
///
/// - [`FirmwareVersionRequest`] (0x01): Request firmware version
/// - [`FirmwareVersionResponse`] (0x01): Firmware version response
///
/// ### Gimbal Control
///
/// - [`GimbalRotationRequest`] (0x07): Control gimbal rotation
///
/// ### Camera Functions
///
/// - [`CameraSystemInfoRequest`] (0x0A): Request camera system info
/// - [`CameraSystemInfoResponse`] (0x0A): Camera system info
/// - [`FunctionFeedback`] (0x0B): Function feedback (sent by camera)
/// - [`FunctionControl`] (0x0C): Photo/video/mode control
///
/// ### Data Streams
///
/// - [`SendRcChannelDataRequest`] (0x23): Send RC channel data to gimbal
/// - [`RequestFlightControllerDataStreamRequest`] (0x24): Request flight controller to send data stream
/// - [`RequestFlightControllerDataStreamResponse`] (0x24): Flight controller data stream response
///
/// ### Configuration
///
/// - [`FormatSdCardRequest`] (0x48): Format SD card
/// - [`FormatSdCardResponse`] (0x48): Format SD card response
///
/// ## Constants
///
/// - [`STX`]: Protocol start marker (0x6655)
/// - [`MAX_MESSAGE_SIZE`]: Maximum message payload size (512 bytes)
/// - [`MAX_FRAME_SIZE`]: Maximum complete frame size (522 bytes)
///
/// ## Error Types
///
/// - [`EncodeError`]: Errors that can occur during message encoding
///   - `BufferTooSmall`: Output buffer is too small for the message
///
/// - [`DecodeError`]: Errors that can occur during message decoding
///   - `FrameTooShort`: Frame is shorter than minimum size
///   - `InvalidStx`: Start marker does not match expected value
///   - `FrameIncomplete`: Frame is incomplete based on length field
///   - `CrcMismatch`: CRC check failed
///   - `NotEnoughBytes`: Not enough bytes to decode field
///   - `InvalidEnumValue`: Enum value is not valid
///   - `ConversionError`: Type conversion failed
///   - `UnknownCmdId`: Unknown command ID
///
/// ## Memory Requirements
///
/// - Message encoding buffer: 512 bytes (stack)
/// - Frame encoding buffer: 522 bytes (stack)
/// - Per-message overhead: Varies by message type
///
/// All buffers are stack-allocated. No heap allocation is required.
///
/// ## Protocol-Specific Notes
///
/// ### TTL (Serial) Protocol
///
/// - Baud rate: 115200
/// - Data bits: 8, Stop bits: 1, Parity: None
/// - Full duplex communication
/// - Implement frame synchronization for byte-by-byte reception
/// - Some messages may not be supported (check camera documentation)
///
/// ## Camera-Specific Notes
///
/// ### A2mini
///
/// - Entry-level gimbal camera
/// - Basic gimbal control
/// - Simple zoom control
/// - Network only (TCP/UDP)
///
/// ## Data Encoding Notes
///
/// ### Angles
///
/// Angles are encoded as integers multiplied by 10:
///
/// ```rust
/// // Encoding: 45.5 degrees
/// let angle_deg = 45.5;
/// let angle_protocol = (angle_deg * 10.0) as i16;  // 455
///
/// // Decoding:
/// let received_value = 455i16;
/// let angle_deg = received_value as f32 / 10.0;  // 45.5
/// ```
///
/// ### Temperatures
///
/// Temperatures are encoded as integers multiplied by 100:
///
/// ```rust
/// // Encoding: 25.37°C
/// let temp_celsius = 25.37;
/// let temp_protocol = (temp_celsius * 100.0) as u16;  // 2537
///
/// // Decoding:
/// let received_value = 2537u16;
/// let temp_celsius = received_value as f32 / 100.0;  // 25.37
/// ```
///
/// ### Distances
///
/// Laser distances are measured in decimeters (dm):
///
/// ```rust
/// // Encoding: 150 meters
/// let distance_m = 150.0;
/// let distance_dm = (distance_m * 10.0) as u16;  // 1500
///
/// // Decoding:
/// let received_value = 1500u16;
/// let distance_m = received_value as f32 / 10.0;  // 150.0
/// ```
///
/// Minimum valid distance: 5.0 meters (50 dm)
///
/// ## Examples
///
/// ### Getting Gimbal Attitude
///
/// ```rust
/// use siyi_protocol::a2mini_ttl::*;
///
/// let request = GimbalAttitudeRequest::default();
/// let msg = Message::GimbalAttitudeRequest(request);
///
/// let mut frame_buf = [0u8; MAX_FRAME_SIZE];
/// let len = encode_message(&msg, &mut frame_buf).unwrap();
///
/// // Send frame_buf[..len] and receive response
/// // let response_data: &[u8] = receive_from_camera();
///
/// // Decode response
/// // match decode_message(response_data) {
/// //     Ok(Message::GimbalAttitudeResponse(resp)) => {
/// //         let yaw = resp.yaw as f32 / 10.0;
/// //         let pitch = resp.pitch as f32 / 10.0;
/// //         println!("Yaw: {:.1}°, Pitch: {:.1}°", yaw, pitch);
/// //     }
/// //     _ => {}
/// // }
/// ```
///
/// ### Setting Gimbal Position
///
/// ```rust
/// use siyi_protocol::a2mini_ttl::*;
///
/// // Set to 45° yaw, -30° pitch
/// let yaw = (45.0 * 10.0) as i16;
/// let pitch = (-30.0 * 10.0) as i16;
///
/// let mut request = SetGimbalAttitudeRequest::default();
/// request.yaw = yaw;
/// request.pitch = pitch;
///
/// let msg = Message::SetGimbalAttitudeRequest(request);
/// let mut frame_buf = [0u8; MAX_FRAME_SIZE];
/// let len = encode_message(&msg, &mut frame_buf).unwrap();
/// ```
///
/// ## See Also
///
/// - [SIYI SDK Documentation](https://shop.siyi.biz/)
/// - [Protocol Specification](https://github.com/AhmedBoin/siyi-protocol/blob/main/PROTOCOL.md)
/// - [Examples](https://github.com/AhmedBoin/siyi-protocol/tree/main/examples)
///
#[allow(unused)]
const _DOCUMENTATION: () = ();
