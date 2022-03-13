use std::os::raw::c_void;

pub enum vnc_EventLoopType {
    vnc_EventLoopType_Default = 0,
    vnc_EventLoopType_Win,
    vnc_EventLoopType_Fd,
    vnc_EventLoopType_Cf,
    vnc_EventLoopType_Android,
}

pub const VNC_SDK_MAJOR_VERSION: u32 = 1;
pub const VNC_SDK_PATCH_VERSION: u32 = 1;
pub const VNC_SDK_BUILD_NUMBER: u32 = 46075;
