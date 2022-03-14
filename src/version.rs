/* automatically generated by rust-bindgen 0.59.2 */

pub const vnc_true: u32 = 1;
pub const vnc_false: u32 = 0;
pub const vnc_success: u32 = 1;
pub const vnc_failure: u32 = 0;
pub const VNC_SDK_MAJOR_VERSION: u32 = 1;
pub const VNC_SDK_MINOR_VERSION: u32 = 9;
pub const VNC_SDK_PATCH_VERSION: u32 = 1;
pub const VNC_SDK_BUILD_NUMBER: u32 = 46075;
#[doc = " @name Common primitive types */"]
pub type vnc_int8_t = ::std::os::raw::c_schar;
pub type vnc_int16_t = ::std::os::raw::c_short;
pub type vnc_int32_t = ::std::os::raw::c_int;
pub type vnc_int64_t = ::std::os::raw::c_longlong;
pub type vnc_uint8_t = ::std::os::raw::c_uchar;
pub type vnc_uint16_t = ::std::os::raw::c_ushort;
pub type vnc_uint31_t = ::std::os::raw::c_uint;
pub type vnc_uint32_t = ::std::os::raw::c_uint;
pub type vnc_uint64_t = ::std::os::raw::c_ulonglong;
#[doc = " @name Boolean type"]
#[doc = " @details Note that #vnc_true and #vnc_false are guaranteed to be defined as"]
#[doc = " 1 and 0 respectively in all future versions of the SDK.  There is no need to"]
#[doc = " explicitly reference these constants, unless you prefer that coding style."]
pub type vnc_bool_t = ::std::os::raw::c_int;
#[doc = " @name Status type"]
#[doc = " @details Note that #vnc_success and #vnc_failure are guaranteed to be defined"]
#[doc = " as 1 and 0 respectively in all future versions of the SDK.  There is no need"]
#[doc = " to explicitly reference these constants, unless you prefer that coding style;"]
#[doc = " either of these is acceptable:"]
#[doc = ""]
#[doc = " @code"]
#[doc = " if (!vnc_Logger_createFileLogger(\"logfile.txt\"))"]
#[doc = "     handleError();"]
#[doc = " @endcode"]
#[doc = ""]
#[doc = " or:"]
#[doc = " @code"]
#[doc = " if (vnc_Logger_createFileLogger(\"logfile.txt\") != vnc_success)"]
#[doc = "     handleError();"]
#[doc = " @endcode"]
#[doc = ""]
pub type vnc_status_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_Connection {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_ConnectionHandler {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_AnnotationManager {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_MessagingManager {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_DisplayManager {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_DataBuffer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_Server {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_Viewer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_PixelFormat {
    _unused: [u8; 0],
}
pub type vnc_assert_int8_t = [::std::os::raw::c_char; 1usize];
pub type vnc_assert_int16_t = [::std::os::raw::c_char; 1usize];
pub type vnc_assert_int32_t = [::std::os::raw::c_char; 1usize];
pub type vnc_assert_int64_t = [::std::os::raw::c_char; 1usize];
pub type vnc_assert_uint8_t = [::std::os::raw::c_char; 1usize];
pub type vnc_assert_uint16_t = [::std::os::raw::c_char; 1usize];
pub type vnc_assert_uint31_t = [::std::os::raw::c_char; 1usize];
pub type vnc_assert_uint32_t = [::std::os::raw::c_char; 1usize];
pub type vnc_assert_uint64_t = [::std::os::raw::c_char; 1usize];
pub const vnc_DummyEnum_vnc_DummyEnum1: vnc_DummyEnum = 0;
pub const vnc_DummyEnum_vnc_DummyEnum2: vnc_DummyEnum = 1;
pub type vnc_DummyEnum = ::std::os::raw::c_uint;
pub type vnc_assert_DummyEnum = [::std::os::raw::c_char; 1usize];
pub type VncAssertionHandler =
    ::std::option::Option<unsafe extern "C" fn(message: *const ::std::os::raw::c_char)>;
extern "C" {
    #[doc = " Returns the runtime major version number of the SDK."]
    pub fn vnc_getMajorVersion() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the runtime minor version number of the SDK."]
    pub fn vnc_getMinorVersion() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the runtime patch version number of the SDK."]
    pub fn vnc_getPatchVersion() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the runtime build number of the SDK."]
    pub fn vnc_getBuildNumber() -> ::std::os::raw::c_int;
}