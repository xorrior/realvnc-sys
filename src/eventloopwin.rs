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
pub type HANDLE = *mut ::std::os::raw::c_void;
#[doc = " Callback receiving notifications for a Windows HANDLE-based event loop."]
#[doc = ""]
#[doc = " @see vnc_EventLoopWin_setCallback()"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_EventLoopWin_Callback {
    #[doc = " Notification that a Windows event is being added or removed.  This callback"]
    #[doc = " is required."]
    #[doc = ""]
    #[doc = " The implementation should call vnc_EventLoopWin_handleEvent() whenever a"]
    #[doc = " HANDLE owned by the SDK is signalled.  This callback does not get called"]
    #[doc = " for events that were created by the SDK prior to the callback being set;"]
    #[doc = " the implementation should use vnc_EventLoopWin_getEvents() to query those"]
    #[doc = " when vnc_EventLoopWin_setCallback() is called."]
    #[doc = ""]
    #[doc = " @param event The HANDLE of the event being added or removed."]
    #[doc = " @param add Set to #vnc_true to signify that the SDK wishes to be"]
    #[doc = "     notified when the given event occurs.  Set to #vnc_false when the"]
    #[doc = "     SDK is no longer interested in the given event; it must be removed"]
    #[doc = "     immediately from the set of monitored HANDLEs."]
    pub eventUpdated: ::std::option::Option<
        unsafe extern "C" fn(userData: *mut ::std::os::raw::c_void, event: HANDLE, add: vnc_bool_t),
    >,
    #[doc = " Notification that the timer expiry period has been updated.  When the"]
    #[doc = " specified expiry time has passed, the implementation should call"]
    #[doc = " vnc_EventLoopWin_handleEvent()."]
    #[doc = ""]
    #[doc = " This callback is optional, and is only needed if integrating with a"]
    #[doc = " third-party event loop rather than calling vnc_EventLoopWin_handleEvent()"]
    #[doc = " directly to a create a custom blocking event loop."]
    #[doc = ""]
    #[doc = " @param expiryMs The maximum time that should be allowed to pass before the"]
    #[doc = "     next call to vnc_EventLoopWin_handleEvent(), in milliseconds"]
    pub timerUpdated: ::std::option::Option<
        unsafe extern "C" fn(
            userData: *mut ::std::os::raw::c_void,
            expiryMs: ::std::os::raw::c_int,
        ),
    >,
}
#[test]
fn bindgen_test_layout_vnc_EventLoopWin_Callback() {
    assert_eq!(
        ::std::mem::size_of::<vnc_EventLoopWin_Callback>(),
        16usize,
        concat!("Size of: ", stringify!(vnc_EventLoopWin_Callback))
    );
    assert_eq!(
        ::std::mem::align_of::<vnc_EventLoopWin_Callback>(),
        8usize,
        concat!("Alignment of ", stringify!(vnc_EventLoopWin_Callback))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<vnc_EventLoopWin_Callback>())).eventUpdated as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vnc_EventLoopWin_Callback),
            "::",
            stringify!(eventUpdated)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<vnc_EventLoopWin_Callback>())).timerUpdated as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vnc_EventLoopWin_Callback),
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
    #[doc = "     The vnc_EventLoopWin_Callback::eventUpdated callback is NULL"]
    pub fn vnc_EventLoopWin_setCallback(
        callback: *const vnc_EventLoopWin_Callback,
        userData: *mut ::std::os::raw::c_void,
    ) -> vnc_status_t;
}
extern "C" {
    #[doc = " Gets the array of events that the SDK currently wishes to be notified of."]
    #[doc = ""]
    #[doc = " @param events An output array which will be filled with the list of events"]
    #[doc = "     the SDK is using.  The array pointed to by @p events must be"]
    #[doc = "     @c MAXIMUM_WAIT_OBJECTS in size."]
    #[doc = ""]
    #[doc = " @return The number of events set in the output array"]
    pub fn vnc_EventLoopWin_getEvents(events: *mut HANDLE) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Handles the given event (if any) and process expired timers.  This should be"]
    #[doc = " called whenever a timer expires or an event occurs.  If a timer expires and"]
    #[doc = " no event has been signalled, pass NULL as the handle to process timer events"]
    #[doc = " only."]
    #[doc = ""]
    #[doc = " @param event The event which has been signalled."]
    #[doc = ""]
    #[doc = " @return The number of milliseconds until the next timer expires"]
    pub fn vnc_EventLoopWin_handleEvent(event: HANDLE) -> ::std::os::raw::c_int;
}
