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
#[doc = " Callback which receives RSA key details."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_RsaKey_Callback {
    #[doc = " Notification that the SDK's RSA key has been retrieved.  This callback is"]
    #[doc = " required for retrieving the RSA key details."]
    #[doc = ""]
    #[doc = " @param rsaPublic The RSA public part, encoded using the RFB data format"]
    #[doc = "     (N's bit-length as a four-byte big-endian number, followed by N and e"]
    #[doc = "     in unsigned big-endian bytes, with e padded to N's byte-length).  This"]
    #[doc = "     public part is suitable for hashing with SHA-256 and using for key"]
    #[doc = "     verification, if the 64-bit @p hexFingerprint does not meet security"]
    #[doc = "     requirements."]
    #[doc = " @param hexFingerprint The SHA-1 hash of the RSA public part, truncated to"]
    #[doc = "     64 bits and written as a hexadecimal string."]
    #[doc = " @param catchphraseFingerprint The SHA-1 hash of the RSA public part,"]
    #[doc = "     truncated to 64 bits and written using words from a dictionary (this is"]
    #[doc = "     a reversible encoding of the @p hexFingerprint string)."]
    pub detailsReady: ::std::option::Option<
        unsafe extern "C" fn(
            userData: *mut ::std::os::raw::c_void,
            rsaPublic: *const vnc_DataBuffer,
            hexFingerprint: *const ::std::os::raw::c_char,
            catchphraseFingerprint: *const ::std::os::raw::c_char,
        ),
    >,
}
#[test]
fn bindgen_test_layout_vnc_RsaKey_Callback() {
    assert_eq!(
        ::std::mem::size_of::<vnc_RsaKey_Callback>(),
        8usize,
        concat!("Size of: ", stringify!(vnc_RsaKey_Callback))
    );
    assert_eq!(
        ::std::mem::align_of::<vnc_RsaKey_Callback>(),
        8usize,
        concat!("Alignment of ", stringify!(vnc_RsaKey_Callback))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<vnc_RsaKey_Callback>())).detailsReady as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vnc_RsaKey_Callback),
            "::",
            stringify!(detailsReady)
        )
    );
}
extern "C" {
    #[doc = " Requests the details for the machine's identifying RSA public key.  The"]
    #[doc = " results are returned asynchronously via the vnc_RsaKey_Callback::detailsReady"]
    #[doc = " callback."]
    #[doc = ""]
    #[doc = " The key is read from and stored in the data store."]
    #[doc = ""]
    #[doc = " If a callback is already queued with identical @p userData, a second callback"]
    #[doc = " will not be queued (that is, the callback will not be invoked twice)."]
    #[doc = ""]
    #[doc = " @param callback The callback to be invoked when the key details are ready."]
    #[doc = " @param generateIfMissing This will normally be #vnc_true, so that a key is"]
    #[doc = "     generated on-demand.  If set to #vnc_false and a key does not yet exist,"]
    #[doc = "     a key will not be generated and vnc_RsaKey_getDetails() will return with"]
    #[doc = "     a @c NoRsaKeyError error."]
    #[doc = ""]
    #[doc = " @return #vnc_success is returned on success, and the callback will be"]
    #[doc = "     invoked later.  In the case of an error, #vnc_failure is returned, the"]
    #[doc = "     callback not be called, and vnc_getLastError() can be used to get the"]
    #[doc = "     error code."]
    #[doc = ""]
    #[doc = " @retval NoRsaKeyError"]
    #[doc = "     @p generateIfMissing was set to #vnc_false, and no key was found"]
    #[doc = ""]
    #[doc = " @retval RsaKeyError"]
    #[doc = "     The loaded key is invalid"]
    #[doc = ""]
    #[doc = " @retval DataStoreError"]
    #[doc = "     There was an error reading from the data store or there is no data store"]
    pub fn vnc_RsaKey_getDetails(
        callback: *const vnc_RsaKey_Callback,
        userData: *mut ::std::os::raw::c_void,
        generateIfMissing: vnc_bool_t,
    ) -> vnc_status_t;
}
extern "C" {
    #[doc = " Cancels any outstanding notifications for the given callback.  Has no effect"]
    #[doc = " if no callbacks with the given @p userData are outstanding."]
    #[doc = ""]
    #[doc = " @param userData The callback to be cancelled."]
    pub fn vnc_RsaKey_cancelDetails(userData: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[doc = " Removes any stored RSA key from the data store."]
    #[doc = ""]
    #[doc = " @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()"]
    #[doc = "     to get the error code."]
    #[doc = ""]
    #[doc = " @retval DataStoreError"]
    #[doc = "     There was an error writing to the data store, or there is no data store"]
    pub fn vnc_RsaKey_clear() -> vnc_status_t;
}
