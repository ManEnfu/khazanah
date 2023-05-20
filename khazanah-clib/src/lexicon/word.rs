use khazanah_core::{
    xml::{ReadXml, WriteXml},
    Word,
};

use libc::c_void;
use uuid::Uuid;

use std::{
    ffi::{c_char, CStr, CString},
    ptr, slice,
};

/// cbindgen:ignore
pub type KhzWord = Word;

use crate::result::{self, KhzResult};

/// Creates a new word.
/// The caller takes ownership of the word and is responsible to free it.
#[no_mangle]
pub unsafe extern "C" fn khz_word_new(word: *mut *mut KhzWord) -> KhzResult {
    if word.is_null() {
        return result::KHZ_ERR_NULL;
    }

    *word = Box::into_raw(Box::new(KhzWord::new()));

    result::KHZ_OK
}

/// Creates a new word with a specific id.
/// The caller takes ownership of the word and is responsible to free it.
#[no_mangle]
pub unsafe extern "C" fn khz_word_new_with_id(word: *mut *mut KhzWord, id: *const u8) -> KhzResult {
    if word.is_null() || id.is_null() {
        return result::KHZ_ERR_NULL;
    }

    let id = match Uuid::from_slice_le(slice::from_raw_parts(id, 16)) {
        Err(_) => return result::KHZ_ERR_OTHER,
        Ok(id) => id,
    };

    *word = Box::into_raw(Box::new(KhzWord::new_with_id(id)));

    result::KHZ_OK
}

/// Loads a word from a XML file.
/// The caller takes ownership of the word and is responsible to free it.
#[no_mangle]
pub unsafe extern "C" fn khz_word_load_xml_file(
    word: *mut *mut KhzWord,
    path: *const c_char,
) -> KhzResult {
    if word.is_null() || path.is_null() {
        return result::KHZ_ERR_NULL;
    }

    *word = ptr::null_mut();

    let path_str = match CStr::from_ptr(path).to_str() {
        Err(_) => {
            return result::KHZ_ERR_CSTRING;
        }
        Ok(p) => p,
    };

    match KhzWord::load_xml_file(path_str) {
        Err(_) => result::KHZ_ERR_XML,
        Ok(w) => {
            *word = Box::into_raw(Box::new(w));
            result::KHZ_OK
        }
    }
}

/// Loads a word from a XML string.
/// The caller takes ownership of the word and is responsible to free it.
#[no_mangle]
pub unsafe extern "C" fn khz_word_load_xml_string(
    word: *mut *mut KhzWord,
    s: *const c_char,
) -> KhzResult {
    if word.is_null() || s.is_null() {
        return result::KHZ_ERR_NULL;
    }

    *word = ptr::null_mut();

    let s_str = match CStr::from_ptr(s).to_str() {
        Err(_) => {
            return result::KHZ_ERR_CSTRING;
        }
        Ok(s) => s,
    };

    match KhzWord::load_xml_str(s_str) {
        Err(_) => result::KHZ_ERR_XML,
        Ok(w) => {
            *word = Box::into_raw(Box::new(w));
            result::KHZ_OK
        }
    }
}

/// Saves the word to a XML file.
#[no_mangle]
pub unsafe extern "C" fn khz_word_save_xml_file(
    word: *const KhzWord,
    path: *const c_char,
) -> KhzResult {
    if path.is_null() || word.is_null() {
        return result::KHZ_ERR_NULL;
    }

    let path_str = match CStr::from_ptr(path).to_str() {
        Err(_) => {
            return result::KHZ_ERR_CSTRING;
        }
        Ok(p) => p,
    };

    match word.as_ref().unwrap().save_xml_file(path_str) {
        Err(_) => result::KHZ_ERR_XML,
        Ok(_) => result::KHZ_OK,
    }
}

/// Saves the word to a XML string.
/// The caller takes ownership of the returned string `dst_s` and is responsible to free it.
#[no_mangle]
pub unsafe extern "C" fn khz_word_save_xml_string(
    word: *const KhzWord,
    dst_s: *mut *mut c_char,
) -> KhzResult {
    if word.is_null() || dst_s.is_null() {
        return result::KHZ_ERR_NULL;
    }

    *dst_s = ptr::null_mut();

    let s = match word.as_ref().unwrap().save_xml_string() {
        Err(_) => return result::KHZ_ERR_XML,
        Ok(s) => s,
    };

    match CString::new(s) {
        Err(_) => result::KHZ_ERR_CSTRING,
        Ok(cs) => {
            *dst_s = cs.into_raw();
            result::KHZ_OK
        }
    }
}

/// Gets the id of the word and saves it into `dst`.
#[no_mangle]
pub unsafe extern "C" fn khz_word_get_id(word: *const KhzWord, dst: *mut u8) -> KhzResult {
    if word.is_null() || dst.is_null() {
        return result::KHZ_ERR_NULL;
    }

    let w = &*word;
    let dst_slice = w.id().unwrap_or_default().to_bytes_le();
    libc::memcpy(dst as *mut c_void, dst_slice.as_ptr() as *const c_void, 16);

    result::KHZ_OK
}

/// Gets the romanization of the word.
/// The caller takes ownership of the returned string `dst` and is responsible to free it.
#[no_mangle]
pub unsafe extern "C" fn khz_word_get_romanization(
    word: *const KhzWord,
    dst: *mut *mut c_char,
) -> KhzResult {
    if word.is_null() || dst.is_null() {
        return result::KHZ_ERR_NULL;
    }

    let w = &*word;
    *dst = ptr::null_mut();

    let dst_cstring = match CString::new(w.romanization()) {
        Err(_) => return result::KHZ_ERR_CSTRING,
        Ok(cs) => cs,
    };

    *dst = dst_cstring.into_raw();

    result::KHZ_OK
}

/// Sets the romanization of the word.
#[no_mangle]
pub unsafe extern "C" fn khz_word_set_romanization(
    word: *mut KhzWord,
    value: *const c_char,
) -> KhzResult {
    if word.is_null() || value.is_null() {
        return result::KHZ_ERR_NULL;
    }

    let w = &mut *word;

    let value_string = match CStr::from_ptr(value).to_str() {
        Err(_) => return result::KHZ_ERR_CSTRING,
        Ok(s) => s.to_string(),
    };

    w.set_romanization(value_string);

    result::KHZ_OK
}

/// Gets the translation of the word.
/// The caller takes ownership of the returned string `dst` and is responsible to free it.
#[no_mangle]
pub unsafe extern "C" fn khz_word_get_translation(
    word: *const KhzWord,
    dst: *mut *mut c_char,
) -> KhzResult {
    if word.is_null() || dst.is_null() {
        return result::KHZ_ERR_NULL;
    }

    let w = &*word;
    *dst = ptr::null_mut();

    let dst_cstring = match CString::new(w.translation()) {
        Err(_) => return result::KHZ_ERR_CSTRING,
        Ok(cs) => cs,
    };

    *dst = dst_cstring.into_raw();

    result::KHZ_OK
}

/// Sets the translation of the word.
#[no_mangle]
pub unsafe extern "C" fn khz_word_set_translation(
    word: *mut KhzWord,
    value: *const c_char,
) -> KhzResult {
    if word.is_null() || value.is_null() {
        return result::KHZ_ERR_NULL;
    }

    let w = &mut *word;

    let value_string = match CStr::from_ptr(value).to_str() {
        Err(_) => return result::KHZ_ERR_CSTRING,
        Ok(s) => s.to_string(),
    };

    w.set_translation(value_string);

    result::KHZ_OK
}

/// Gets the IPA pronunciation of the word.
/// The caller takes ownership of the returned string `dst` and is responsible to free it.
#[no_mangle]
pub unsafe extern "C" fn khz_word_get_pronunciation(
    word: *const KhzWord,
    dst: *mut *mut c_char,
) -> KhzResult {
    if word.is_null() || dst.is_null() {
        return result::KHZ_ERR_NULL;
    }

    let w = &*word;
    *dst = ptr::null_mut();

    let dst_cstring = match CString::new(w.pronunciation()) {
        Err(_) => return result::KHZ_ERR_CSTRING,
        Ok(cs) => cs,
    };

    *dst = dst_cstring.into_raw();

    result::KHZ_OK
}

/// Sets the IPA pronunciation of the word.
#[no_mangle]
pub unsafe extern "C" fn khz_word_set_pronunciation(
    word: *mut KhzWord,
    value: *const c_char,
) -> KhzResult {
    if word.is_null() || value.is_null() {
        return result::KHZ_ERR_NULL;
    }

    let w = &mut *word;

    let value_string = match CStr::from_ptr(value).to_str() {
        Err(_) => return result::KHZ_ERR_CSTRING,
        Ok(s) => s.to_string(),
    };

    w.set_pronunciation(value_string);

    result::KHZ_OK
}

/// Gets the X-SAMPA pronunciation of the word.
/// The caller takes ownership of the returned string `dst` and is responsible to free it.
#[no_mangle]
pub unsafe extern "C" fn khz_word_get_xsampa_pronunciation(
    word: *const KhzWord,
    dst: *mut *mut c_char,
) -> KhzResult {
    if word.is_null() || dst.is_null() {
        return result::KHZ_ERR_NULL;
    }

    let w = &*word;
    *dst = ptr::null_mut();

    let dst_cstring = if let Some(xs) = w.xsampa_pronunciation() {
        match CString::new(xs) {
            Err(_) => return result::KHZ_ERR_CSTRING,
            Ok(cs) => cs,
        }
    } else {
        return result::KHZ_OK;
    };

    *dst = dst_cstring.into_raw();

    result::KHZ_OK
}

/// Sets the X-SAMPA pronunciation of the word.
/// The value will be converted to IPA pronunciation and used
/// to set the pronunciation of the word.
#[no_mangle]
pub unsafe extern "C" fn khz_word_set_xsampa_pronunciation(
    word: *mut KhzWord,
    value: *const c_char,
) -> KhzResult {
    if word.is_null() {
        return result::KHZ_ERR_NULL;
    }

    let w = &mut *word;

    let value_opt = if !value.is_null() {
        match CStr::from_ptr(value).to_str() {
            Err(_) => return result::KHZ_ERR_CSTRING,
            Ok(s) => Some(s.to_string()),
        }
    } else {
        None
    };

    w.set_xsampa_pronunciation(value_opt);

    result::KHZ_OK
}

/// Frees the word allocation.
#[no_mangle]
pub unsafe extern "C" fn khz_word_free(word: *mut KhzWord) -> KhzResult {
    let _b = Box::from_raw(word);
    result::KHZ_OK
}
