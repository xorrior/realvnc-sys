/* automatically generated by rust-bindgen 0.59.2 */

pub const vnc_true: u32 = 1;
pub const vnc_false: u32 = 0;
pub const vnc_success: u32 = 1;
pub const vnc_failure: u32 = 0;
pub const XK_BackSpace: u32 = 65288;
pub const XK_Tab: u32 = 65289;
pub const XK_Return: u32 = 65293;
pub const XK_Pause: u32 = 65299;
pub const XK_Scroll_Lock: u32 = 65300;
pub const XK_Sys_Req: u32 = 65301;
pub const XK_Escape: u32 = 65307;
pub const XK_Delete: u32 = 65535;
pub const XK_Home: u32 = 65360;
pub const XK_Left: u32 = 65361;
pub const XK_Up: u32 = 65362;
pub const XK_Right: u32 = 65363;
pub const XK_Down: u32 = 65364;
pub const XK_Page_Up: u32 = 65365;
pub const XK_Page_Down: u32 = 65366;
pub const XK_End: u32 = 65367;
pub const XK_Print: u32 = 65377;
pub const XK_Insert: u32 = 65379;
pub const XK_Menu: u32 = 65383;
pub const XK_Break: u32 = 65387;
pub const XK_KP_Enter: u32 = 65421;
pub const XK_KP_Home: u32 = 65429;
pub const XK_KP_Left: u32 = 65430;
pub const XK_KP_Up: u32 = 65431;
pub const XK_KP_Right: u32 = 65432;
pub const XK_KP_Down: u32 = 65433;
pub const XK_KP_Page_Up: u32 = 65434;
pub const XK_KP_Page_Down: u32 = 65435;
pub const XK_KP_End: u32 = 65436;
pub const XK_KP_Insert: u32 = 65438;
pub const XK_KP_Delete: u32 = 65439;
pub const XK_KP_Multiply: u32 = 65450;
pub const XK_KP_Add: u32 = 65451;
pub const XK_KP_Separator: u32 = 65452;
pub const XK_KP_Subtract: u32 = 65453;
pub const XK_KP_Decimal: u32 = 65454;
pub const XK_KP_Divide: u32 = 65455;
pub const XK_KP_0: u32 = 65456;
pub const XK_KP_1: u32 = 65457;
pub const XK_KP_2: u32 = 65458;
pub const XK_KP_3: u32 = 65459;
pub const XK_KP_4: u32 = 65460;
pub const XK_KP_5: u32 = 65461;
pub const XK_KP_6: u32 = 65462;
pub const XK_KP_7: u32 = 65463;
pub const XK_KP_8: u32 = 65464;
pub const XK_KP_9: u32 = 65465;
pub const XK_F1: u32 = 65470;
pub const XK_F2: u32 = 65471;
pub const XK_F3: u32 = 65472;
pub const XK_F4: u32 = 65473;
pub const XK_F5: u32 = 65474;
pub const XK_F6: u32 = 65475;
pub const XK_F7: u32 = 65476;
pub const XK_F8: u32 = 65477;
pub const XK_F9: u32 = 65478;
pub const XK_F10: u32 = 65479;
pub const XK_F11: u32 = 65480;
pub const XK_F12: u32 = 65481;
pub const XK_Shift_L: u32 = 65505;
pub const XK_Shift_R: u32 = 65506;
pub const XK_Control_L: u32 = 65507;
pub const XK_Control_R: u32 = 65508;
pub const XK_Alt_L: u32 = 65513;
pub const XK_Alt_R: u32 = 65514;
pub const XK_Super_L: u32 = 65515;
pub const XK_Super_R: u32 = 65516;
pub const XK_ISO_Level3_Shift: u32 = 65027;
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
    #[doc = " Converts a unicode character to a keysym, suitable for passing to"]
    #[doc = " vnc_Viewer_sendKeyDown()."]
    #[doc = ""]
    #[doc = " @param unicodeChar The unicode character."]
    #[doc = ""]
    #[doc = " @return The keysym corresponding to @p unicodeChar, or zero if not"]
    #[doc = "     recognized."]
    pub fn vnc_unicodeToKeysym(unicodeChar: vnc_uint31_t) -> vnc_uint31_t;
}
extern "C" {
    #[doc = " Converts a keysym to a unicode - suitable for converting a keysym"]
    #[doc = " received on the server in vnc_Server_InputEventsCallback::keyEventCallback."]
    #[doc = ""]
    #[doc = " @param keysym The keysym to convert."]
    #[doc = ""]
    #[doc = " @return The unicode character corresponding to the keysym, else zero."]
    pub fn vnc_keysymToUnicode(keysym: vnc_uint31_t) -> vnc_uint31_t;
}
