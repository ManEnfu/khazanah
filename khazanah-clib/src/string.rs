use std::ffi::{c_char, CString};

use crate::result::{self, KhzResult};

/// Frees string allocated by this library.
#[no_mangle]
pub unsafe extern "C" fn khz_string_free(string: *mut c_char) -> KhzResult {
    let _cstring = CString::from_raw(string);
    result::KHZ_OK
}
