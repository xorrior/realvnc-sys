/* automatically generated by rust-bindgen 0.59.2 */

pub const vnc_true: u32 = 1;
pub const vnc_false: u32 = 0;
pub const vnc_success: u32 = 1;
pub const vnc_failure: u32 = 0;
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
#[doc = " Callback receiving display changes."]
#[doc = ""]
#[doc = " @see vnc_DisplayManager_setCallback"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_DisplayManager_Callback {
    #[doc = " Notification that a display has been added or removed, or the resolution"]
    #[doc = " of an existing display has changed."]
    #[doc = ""]
    #[doc = " @param displayManager A displayManager object containing a new list"]
    #[doc = " of all the displays."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub displaysChanged: ::std::option::Option<
        unsafe extern "C" fn(
            userData: *mut ::std::os::raw::c_void,
            displayManager: *mut vnc_DisplayManager,
        ),
    >,
}
#[test]
fn bindgen_test_layout_vnc_DisplayManager_Callback() {
    assert_eq!(
        ::std::mem::size_of::<vnc_DisplayManager_Callback>(),
        8usize,
        concat!("Size of: ", stringify!(vnc_DisplayManager_Callback))
    );
    assert_eq!(
        ::std::mem::align_of::<vnc_DisplayManager_Callback>(),
        8usize,
        concat!("Alignment of ", stringify!(vnc_DisplayManager_Callback))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<vnc_DisplayManager_Callback>())).displaysChanged as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vnc_DisplayManager_Callback),
            "::",
            stringify!(displaysChanged)
        )
    );
}

extern "C" {
    #[doc = " Returns whether the server is sharing its screens, whether it is being"]
    #[doc = " called from a server or a viewer."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = " @return #vnc_true if screens are shared, otherwise #vnc_false."]
    #[doc = ""]
    #[doc = " @since 1.8.0"]
    pub fn vnc_DisplayManager_isEnabled(displayManager: *mut vnc_DisplayManager) -> vnc_bool_t;
    #[doc = " Specifies whether screen sharing is enabled."]
    #[doc = " If this is the server then this sets whether it shares its screen with"]
    #[doc = " viewers."]
    #[doc = " If this is the viewer then this sets whether it requires to see the server's"]
    #[doc = " screen."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = " @param enable true if screen sharing enabled, false otherwise"]
    #[doc = " @return #vnc_success or #vnc_failure, in which case call"]
    #[doc = "     vnc_getLastError() to get the error code."]
    #[doc = ""]
    #[doc = " @since 1.8.0"]
    pub fn vnc_DisplayManager_enable(
        displayManager: *mut vnc_DisplayManager,
        enable: vnc_bool_t,
    ) -> vnc_status_t;
    #[doc = "  Returns whether the selectDisplay() call is permitted."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = " @return #vnc_true if selectDisplay is permitted, otherwise #vnc_false."]
    pub fn vnc_DisplayManager_isSelectDisplayAllowed(
        displayManager: *mut vnc_DisplayManager,
    ) -> vnc_bool_t;
    #[doc = " Gets the vertical origin of the display in pixels."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = " @param index The 0-based index of the required display."]
    #[doc = ""]
    #[doc = " @return The Y coordinate of the top left corner of the display"]
    #[doc = " relative to the operating system's virtual map of all the displays."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub fn vnc_DisplayManager_getOriginY(
        displayManager: *mut vnc_DisplayManager,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    #[doc = " Gets the horizontal origin of the display in pixels."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = " @param index The 0-based index of the required display."]
    #[doc = ""]
    #[doc = " @return The X coordinate of the top left corner of the display"]
    #[doc = " relative to the operating system's virtual map of all the displays."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub fn vnc_DisplayManager_getOriginX(
        displayManager: *mut vnc_DisplayManager,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    #[doc = " Gets the vertical resolution of the display in pixels."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = " @param index The 0-based index of the required display."]
    #[doc = ""]
    #[doc = " @return A height in pixels."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub fn vnc_DisplayManager_getResolutionY(
        displayManager: *mut vnc_DisplayManager,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    #[doc = " Gets the horizontal resolution of the display in pixels."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = " @param index The 0-based index of the required display."]
    #[doc = ""]
    #[doc = " @return A width in pixels."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub fn vnc_DisplayManager_getResolutionX(
        displayManager: *mut vnc_DisplayManager,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    #[doc = " Returns whether this is the primary (or main) display."]
    #[doc = ""]
    #[doc = " On Windows, the main display acts as the active desktop, showing"]
    #[doc = " the taskbar and sign in and lock screens. Most applications open"]
    #[doc = " on the main display by default."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = " @param index The 0-based index of the required display."]
    #[doc = ""]
    #[doc = " @return #vnc_true if this is the primary display, otherwise #vnc_false."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub fn vnc_DisplayManager_isPrimary(
        displayManager: *mut vnc_DisplayManager,
        index: ::std::os::raw::c_int,
    ) -> vnc_bool_t;
    #[doc = " Gets the name of the display (typically a human-readable string)."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = " @param index The 0-based index of the required display."]
    #[doc = ""]
    #[doc = " @return A friendly name."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub fn vnc_DisplayManager_getName(
        displayManager: *mut vnc_DisplayManager,
        index: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    #[doc = " Gets the ID of the display (typically a short string)."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = " @param index The 0-based index of the required display."]
    #[doc = ""]
    #[doc = " @return A unique identifier."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub fn vnc_DisplayManager_getId(
        displayManager: *mut vnc_DisplayManager,
        index: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    #[doc = " Gets the index of the currently selected display."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = ""]
    #[doc = " @return The 0-based index of the display selected to remote,"]
    #[doc = "     or -1 for all displays."]
    #[doc = ""]
    #[doc = " @since 1.8.0"]
    pub fn vnc_DisplayManager_getDisplayIndex(
        displayManager: *mut vnc_DisplayManager,
    ) -> ::std::os::raw::c_int;
    #[doc = " Chooses a particular display to remote to connected Viewer app users."]
    #[doc = ""]
    #[doc = " This replaces any previous selection, so either a single"]
    #[doc = " display is remoted or all are."]
    #[doc = ""]
    #[doc = " If the selected display is removed, connected Viewers will cease to"]
    #[doc = " receive framebuffer updates until the display is replaced or a new"]
    #[doc = " display is selected."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = " @param index The 0-based index of the display to remote,"]
    #[doc = " or -1 to remote all displays."]
    #[doc = ""]
    #[doc = " @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()"]
    #[doc = "     to get the error code."]
    #[doc = ""]
    #[doc = " @retval InvalidArgument"]
    #[doc = "     @p index is invalid."]
    #[doc = " @retval NotEnabled"]
    #[doc = "     Selecting the display is not permitted at present."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub fn vnc_DisplayManager_selectDisplay(
        displayManager: *mut vnc_DisplayManager,
        index: ::std::os::raw::c_int,
    ) -> vnc_status_t;
    #[doc = " Registers a callback notifying when displays are added or removed, or the"]
    #[doc = " resolution of an existing display changes."]
    #[doc = ""]
    #[doc = " @param displayManager The display manager."]
    #[doc = " @param callback The callback."]
    #[doc = " @param userData Optional custom data to pass to the callback."]
    #[doc = ""]
    #[doc = " @return #vnc_success or #vnc_failure, in which case call"]
    #[doc = "     vnc_getLastError() to get the error code."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub fn vnc_DisplayManager_setCallback(
        displayManager: *mut vnc_DisplayManager,
        callback: *const vnc_DisplayManager_Callback,
        userData: *mut ::std::os::raw::c_void,
    ) -> vnc_status_t;
    #[doc = " Returns the number of displays."]
    pub fn vnc_DisplayManager_getDisplayCount(
        displayManager: *mut vnc_DisplayManager,
    ) -> ::std::os::raw::c_int;
}
