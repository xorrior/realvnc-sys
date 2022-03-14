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
#[doc = " Callback providing custom storage of data used by the SDK."]
#[doc = ""]
#[doc = " @see vnc_DataStore_createCustomStore()."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vnc_DataStore_Callback {
    #[doc = " Requests storing data in your custom data store.  This callback is"]
    #[doc = " required for a custom data store."]
    #[doc = ""]
    #[doc = " In your implementation, associate the data contained in @p value with the"]
    #[doc = " given key.  If a value with the same key exists already, overwrite it.  If"]
    #[doc = " @p value is NULL, remove any existing value stored for the key.  Note the"]
    #[doc = " @ref vnc_DataBuffer is destroyed when the callback function returns, so"]
    #[doc = " copy data out of the buffer rather than storing the buffer itself."]
    #[doc = ""]
    #[doc = " @param key A NULL-terminated string."]
    #[doc = " @param value A @ref vnc_DataBuffer to store, or NULL to remove an existing"]
    #[doc = "     value with the same key from your custom data store."]
    pub put: ::std::option::Option<
        unsafe extern "C" fn(
            userData: *mut ::std::os::raw::c_void,
            key: *const ::std::os::raw::c_char,
            value: *const vnc_DataBuffer,
        ),
    >,
    #[doc = " Requests data stored in your custom data store.  This callback is required"]
    #[doc = " for a custom data store."]
    #[doc = ""]
    #[doc = " In your implementation, you should retrieve the value associated with"]
    #[doc = " @p key and return it as a @ref vnc_DataBuffer, or else NULL if the key"]
    #[doc = " could not be found."]
    #[doc = ""]
    #[doc = " @param key A NULL-terminated string."]
    #[doc = ""]
    #[doc = " @return A @ref vnc_DataBuffer."]
    pub get: ::std::option::Option<
        unsafe extern "C" fn(
            userData: *mut ::std::os::raw::c_void,
            key: *const ::std::os::raw::c_char,
        ) -> *mut vnc_DataBuffer,
    >,
}
#[test]
fn bindgen_test_layout_vnc_DataStore_Callback() {
    assert_eq!(
        ::std::mem::size_of::<vnc_DataStore_Callback>(),
        16usize,
        concat!("Size of: ", stringify!(vnc_DataStore_Callback))
    );
    assert_eq!(
        ::std::mem::align_of::<vnc_DataStore_Callback>(),
        8usize,
        concat!("Alignment of ", stringify!(vnc_DataStore_Callback))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vnc_DataStore_Callback>())).put as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vnc_DataStore_Callback),
            "::",
            stringify!(put)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vnc_DataStore_Callback>())).get as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vnc_DataStore_Callback),
            "::",
            stringify!(get)
        )
    );
}

extern "C" {
    #[doc = " Destroys the current data store."]
    pub fn vnc_DataStore_destroyStore();
    #[doc = " Creates a web browser data store, in DOM @c localStorage."]
    #[doc = ""]
    #[doc = " The SDK manages storing and retrieving data for you.  Note this type of"]
    #[doc = " store is only available on the HTML5 platform."]
    #[doc = ""]
    #[doc = " @param prefix A prefix prepending to the @c localStorage key names, so that"]
    #[doc = "     the SDK's keys do not conflict with other libraries."]
    pub fn vnc_DataStore_createBrowserStore(prefix: *const ::std::os::raw::c_char);
    #[doc = " Creates a registry data store."]
    #[doc = ""]
    #[doc = " The SDK manages storing and retrieving data for you.  Note this type of"]
    #[doc = " store is only available on Windows."]
    #[doc = ""]
    #[doc = " Note that the data store contains sensitive information such as the RSA"]
    #[doc = " private key, and therefore it must not be readable by any potential attacker"]
    #[doc = " on the system.  For this reason it is essential that the registry key have"]
    #[doc = " appropriate permissions which do not allow any access to other users on the"]
    #[doc = " system."]
    #[doc = ""]
    #[doc = " @param registryPath A suitable path to and name for a registry key, for"]
    #[doc = "     example @c HKEY_LOCAL_USER\\\\SOFTWARE\\\\Company\\\\Application\\\\MyKey."]
    #[doc = ""]
    #[doc = " @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()"]
    #[doc = "     to get the error code."]
    #[doc = ""]
    #[doc = " @retval InvalidArgument"]
    #[doc = "     @p registryPath is invalid."]
    #[doc = " @retval DataStoreError"]
    #[doc = "     The registry key could not be opened."]
    pub fn vnc_DataStore_createRegistryStore(
        registryPath: *const ::std::os::raw::c_char,
    ) -> vnc_status_t;
    #[doc = " Creates a file data store."]
    #[doc = ""]
    #[doc = " The SDK manages storing and retrieving data for you.  Note this type of"]
    #[doc = " store is not available on the HTML5 platform."]
    #[doc = ""]
    #[doc = " Note that the data store contains sensitive information such as the RSA"]
    #[doc = " private key, and therefore it must not be readable by any potential attacker"]
    #[doc = " on the system.  For this reason it is essential that the parent"]
    #[doc = " directory/folder of the given path has been created with appropriate"]
    #[doc = " permissions which do not allow any access to other users on the system."]
    #[doc = ""]
    #[doc = " @param path A full path to and file name for the data store.  You can specify"]
    #[doc = "     any file extension; a binary file will be created."]
    #[doc = ""]
    #[doc = " @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()"]
    #[doc = "     to get the error code."]
    #[doc = ""]
    #[doc = " @retval InvalidArgument"]
    #[doc = "     @p path is invalid."]
    #[doc = " @retval DataStoreError"]
    #[doc = "     The file could not be opened or created."]
    pub fn vnc_DataStore_createFileStore(path: *const ::std::os::raw::c_char) -> vnc_status_t;
    #[doc = " Creates a custom data store.  You are responsible for storing and retrieving"]
    #[doc = " data in the correct format when notified by the SDK."]
    #[doc = ""]
    #[doc = " @param callback A @ref vnc_DataStore_Callback structure."]
    #[doc = " @param userData Custom data to pass through to callback function"]
    #[doc = "     implementations."]
    #[doc = ""]
    #[doc = " @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()"]
    #[doc = "     to get the error code."]
    #[doc = ""]
    #[doc = " @retval InvalidArgument"]
    #[doc = "      One or more mandatory callbacks in @p callback is @c NULL."]
    pub fn vnc_DataStore_createCustomStore(
        callback: *const vnc_DataStore_Callback,
        userData: *mut ::std::os::raw::c_void,
    ) -> vnc_status_t;
}