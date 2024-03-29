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
#[doc = " Monitor using @c readfds with @c select() (the second argument), or"]
#[doc = "  @c POLLIN with @c poll()."]
pub const vnc_EventLoopFd_Event_vnc_EventLoopFd_Read: vnc_EventLoopFd_Event = 1;
#[doc = " Monitor using @c writefds with @c select() (the third argument), or"]
#[doc = "  @c POLLOUT with @c poll()."]
pub const vnc_EventLoopFd_Event_vnc_EventLoopFd_Write: vnc_EventLoopFd_Event = 2;
#[doc = " Monitor using @c exceptfds with @c select() (the fourth argument), or"]
#[doc = "  @c POLLPRI with @c poll()."]
pub const vnc_EventLoopFd_Event_vnc_EventLoopFd_Except: vnc_EventLoopFd_Event = 4;
#[doc = " Enumeration of file descriptor events for event selection."]
pub type vnc_EventLoopFd_Event = ::std::os::raw::c_uint;
#[doc = " Callback receiving notifications for a file-descriptor-based event loop."]
#[doc = ""]
#[doc = " @see vnc_EventLoopFd_setCallback()"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_EventLoopFd_Callback {
    #[doc = " Notification that a file descriptor's event mask has changed.  This"]
    #[doc = " callback is required."]
    #[doc = ""]
    #[doc = " @param fd The file descriptor whose event mask has changed."]
    #[doc = " @param eventMask Specifies the events we are interested in for this"]
    #[doc = "     descriptor.  When one of the specified events occurs, the"]
    #[doc = "     implementation should call vnc_EventLoopFd_markEvents() to mark which"]
    #[doc = "     event(s) occurred, then call vnc_EventLoopFd_handleEvents().  An"]
    #[doc = "     @p eventMask of 0 indicates the SDK is no longer using this file"]
    #[doc = "     descriptor; it should be removed immediately from the monitored set of"]
    #[doc = "     descriptors."]
    pub eventUpdated: ::std::option::Option<
        unsafe extern "C" fn(
            userData: *mut ::std::os::raw::c_void,
            fd: ::std::os::raw::c_int,
            eventMask: ::std::os::raw::c_int,
        ),
    >,
    #[doc = " Notification that the timer expiry period has been updated."]
    #[doc = " When the specified expiry time has passed, the implementation should call"]
    #[doc = " vnc_EventLoopFd_handleEvents()."]
    #[doc = ""]
    #[doc = " This callback is optional, and is only needed if integrating with a"]
    #[doc = " third-party event loop rather than calling vnc_EventLoopFd_handleEvents()"]
    #[doc = " directly to a create a custom blocking event loop."]
    #[doc = ""]
    #[doc = " @param expiryMs The maximum time that should be allowed to pass before the"]
    #[doc = "     next call to vnc_EventLoopFd_handleEvents(), in milliseconds"]
    pub timerUpdated: ::std::option::Option<
        unsafe extern "C" fn(
            userData: *mut ::std::os::raw::c_void,
            expiryMs: ::std::os::raw::c_int,
        ),
    >,
}
#[test]
fn bindgen_test_layout_vnc_EventLoopFd_Callback() {
    assert_eq!(
        ::std::mem::size_of::<vnc_EventLoopFd_Callback>(),
        16usize,
        concat!("Size of: ", stringify!(vnc_EventLoopFd_Callback))
    );
    assert_eq!(
        ::std::mem::align_of::<vnc_EventLoopFd_Callback>(),
        8usize,
        concat!("Alignment of ", stringify!(vnc_EventLoopFd_Callback))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<vnc_EventLoopFd_Callback>())).eventUpdated as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vnc_EventLoopFd_Callback),
            "::",
            stringify!(eventUpdated)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<vnc_EventLoopFd_Callback>())).timerUpdated as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vnc_EventLoopFd_Callback),
            "::",
            stringify!(timerUpdated)
        )
    );
}
extern "C" {
    #[doc = " Sets the event loop callback."]
    #[doc = ""]
    #[doc = " @param callback The new event loop callback."]
    #[doc = ""]
    #[doc = " @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()"]
    #[doc = "     to get the error code."]
    #[doc = ""]
    #[doc = " @retval InvalidArgument"]
    #[doc = "     The vnc_EventLoopFd_Callback::eventUpdated callback is NULL"]
    pub fn vnc_EventLoopFd_setCallback(
        callback: *const vnc_EventLoopFd_Callback,
        userData: *mut ::std::os::raw::c_void,
    ) -> vnc_status_t;
}
extern "C" {
    #[doc = " Marks event(s) that occurred on the specified file descriptor."]
    #[doc = ""]
    #[doc = " @param fd The file descriptor."]
    #[doc = " @param events A mask of events to be marked for handling."]
    #[doc = ""]
    #[doc = " @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()"]
    #[doc = "     to get the error code."]
    #[doc = ""]
    #[doc = " @retval InvalidArgument"]
    #[doc = "     @p fd is not recognized by the SDK"]
    pub fn vnc_EventLoopFd_markEvents(
        fd: ::std::os::raw::c_int,
        events: ::std::os::raw::c_int,
    ) -> vnc_status_t;
}
extern "C" {
    #[doc = " Handles events on the file descriptors and process expired timers."]
    #[doc = ""]
    #[doc = " @return The number of milliseconds until the next timer expires"]
    pub fn vnc_EventLoopFd_handleEvents() -> ::std::os::raw::c_int;
}
