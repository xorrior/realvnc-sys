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
#[doc = " Callback notifying when messages are received."]
#[doc = ""]
#[doc = " @see vnc_MessagingManager_setCallback"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_MessagingManager_Callback {
    #[doc = " Notification that a message has been received."]
    #[doc = ""]
    #[doc = " The vnc_Databuffer object containing the data is owned by the SDK"]
    #[doc = " and is automatically destroyed when the callback completes. Take a copy"]
    #[doc = " of the data to interact with it. Do not call @ref vnc_DataBuffer_destroy."]
    #[doc = ""]
    #[doc = " @param messagingManager Belonging to either a Viewer or a Server."]
    #[doc = " @param sender The peer sending the message."]
    #[doc = " @param buffer The data received."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub messageReceived: ::std::option::Option<
        unsafe extern "C" fn(
            userData: *mut ::std::os::raw::c_void,
            messagingManager: *mut vnc_MessagingManager,
            sender: *mut vnc_Connection,
            buffer: *const vnc_DataBuffer,
        ),
    >,
}
#[test]
fn bindgen_test_layout_vnc_MessagingManager_Callback() {
    assert_eq!(
        ::std::mem::size_of::<vnc_MessagingManager_Callback>(),
        8usize,
        concat!("Size of: ", stringify!(vnc_MessagingManager_Callback))
    );
    assert_eq!(
        ::std::mem::align_of::<vnc_MessagingManager_Callback>(),
        8usize,
        concat!("Alignment of ", stringify!(vnc_MessagingManager_Callback))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<vnc_MessagingManager_Callback>())).messageReceived as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vnc_MessagingManager_Callback),
            "::",
            stringify!(messageReceived)
        )
    );
}
extern "C" {
    #[doc = " Registers a callback notifying when messages are received."]
    #[doc = ""]
    #[doc = " You should call this function before your app attempts"]
    #[doc = " either to connect or to listen for connections."]
    #[doc = ""]
    #[doc = " @param messagingManager Belonging to either a Viewer or a Server."]
    #[doc = " @param callback The callback."]
    #[doc = ""]
    #[doc = " @return #vnc_success or #vnc_failure, in which case call"]
    #[doc = "     vnc_getLastError() to get the error code."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub fn vnc_MessagingManager_setCallback(
        messagingManager: *mut vnc_MessagingManager,
        callback: *const vnc_MessagingManager_Callback,
        userData: *mut ::std::os::raw::c_void,
    ) -> vnc_status_t;
}
extern "C" {
    #[doc = " Sends a message."]
    #[doc = ""]
    #[doc = " @param messagingManager Belonging to either a Viewer or a Server."]
    #[doc = " @param buffer The data to send."]
    #[doc = " @param bufferLength The number of bytes. Maximum 1MB."]
    #[doc = " @param connection For a Server, the Viewer to send the message to. To send"]
    #[doc = " to all Viewers, iterate over connections. For a Viewer, specify NULL."]
    #[doc = ""]
    #[doc = " @return #vnc_success or #vnc_failure, in which case call"]
    #[doc = "     vnc_getLastError() to get the error code."]
    #[doc = ""]
    #[doc = " @retval PeerNotSupported"]
    #[doc = "     The peer does not support receiving messages (perhaps no add-on"]
    #[doc = "     code has been applied)."]
    #[doc = " @retval BufferFull"]
    #[doc = "     Too much data is being sent."]
    #[doc = ""]
    #[doc = " @since 1.5"]
    pub fn vnc_MessagingManager_sendMessage(
        messagingManager: *mut vnc_MessagingManager,
        buffer: *const ::std::os::raw::c_void,
        bufferLength: ::std::os::raw::c_int,
        connection: *mut vnc_Connection,
    ) -> vnc_status_t;
}