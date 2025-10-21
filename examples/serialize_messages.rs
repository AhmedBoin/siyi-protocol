use siyi_protocol::zt30_tcp::*;

fn main() -> Result<(), EncodeError> {
    let messages = [
        Message::ManualZoomRequest(ManualZoomRequest { zoom: 1 }),
        Message::ManualZoomRequest(ManualZoomRequest { zoom: -1 }),
        Message::AbsoluteZoomRequest(AbsoluteZoomRequest {
            zoom_int: 4,
            zoom_float: 5,
        }),
        Message::ManualFocusRequest(ManualFocusRequest { focus: 1 }),
        Message::ManualFocusRequest(ManualFocusRequest { focus: -1 }),
        Message::FunctionControl(FunctionControl {
            func_type: FunctionType::TakePhoto,
        }),
        Message::FunctionControl(FunctionControl {
            func_type: FunctionType::StartRecording,
        }),
        Message::FunctionControl(FunctionControl {
            func_type: FunctionType::LockMode,
        }),
        Message::FunctionControl(FunctionControl {
            func_type: FunctionType::FollowMode,
        }),
        Message::FunctionControl(FunctionControl {
            func_type: FunctionType::FPVMode,
        }),
        Message::GimbalRotationRequest(GimbalRotationRequest {
            yaw: 100,
            pitch: 100,
        }),
        Message::AutoFocusRequest(AutoFocusRequest {
            auto_focus: 1,
            touch_x: 300,
            touch_y: 100,
        }),
        Message::CenterGimbalRequest(CenterGimbalRequest {
            center_pos: CenterPosition::CenterOnly,
        }),
        Message::HardwareIdRequest(HardwareIdRequest {}),
        Message::FirmwareVersionRequest(FirmwareVersionRequest {}),
        Message::CameraSystemInfoRequest(CameraSystemInfoRequest {}),
        Message::CurrentZoomRequest(CurrentZoomRequest {}),
        Message::GimbalModeRequest(GimbalModeRequest {}),
        Message::PseudoColorRequest(PseudoColorRequest {}),
        Message::SetPseudoColorRequest(SetPseudoColorRequest {
            pseudo_color: PseudoColor::Ironbow,
        }),
        Message::SetGimbalAttitudeRequest(SetGimbalAttitudeRequest {
            yaw: 0,
            pitch: -900,
        }),
        Message::MaxZoomRangeRequest(MaxZoomRangeRequest {}),
        Message::GetSingleTemperatureFrameRequest(GetSingleTemperatureFrameRequest {}),
        Message::GlobalTemperatureMeasurementRequest(GlobalTemperatureMeasurementRequest {
            get_temp_flag: TempMeasurementFlag::Once,
        }),
        Message::SendGpsData(SendGpsData {
            time_boot_ms: 0,
            lat: 222768300,
            lon: 1141761200,
            alt: 0,
            alt_ellipsoid: 0,
            vn: 0,
            ve: 0,
            vd: 0,
        }),
    ];

    let mut buf = [0; MAX_FRAME_SIZE];
    for msg in messages {
        let n = encode_message(&msg, &mut buf)?;
        println!("{:?}", &buf[..n]);
    }

    Ok(())
}
