// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rhema_ffi_chirho
//!
//! C FFI bindings for non-Rust consumers.
//!
//! Provides `extern "C"` functions for initializing the library, listing modules,
//! searching, and parsing queries. Uses an opaque handle pattern and thread-local
//! last-error for error reporting.

use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use rhema_integration_chirho::RhemaLibraryChirho;
use rhema_query_chirho::QueryParserChirho;
use rhema_query_chirho::planner_chirho::QueryPlannerChirho;

/// Opaque handle wrapping the Rhema library instance.
pub struct RhemaHandleChirho {
    lib_chirho: RhemaLibraryChirho,
}

thread_local! {
    static LAST_ERROR_CHIRHO: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Set the last error message.
fn set_last_error_chirho(msg_chirho: &str) {
    LAST_ERROR_CHIRHO.with(|e_chirho| {
        *e_chirho.borrow_mut() = Some(msg_chirho.to_string());
    });
}

/// Initialize the Rhema library and return an opaque handle.
///
/// Returns null on failure — call `rhema_last_error_chirho` for details.
///
/// # Safety
///
/// The returned pointer must be freed with `rhema_free_chirho`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhema_init_chirho() -> *mut RhemaHandleChirho {
    match RhemaLibraryChirho::init_chirho() {
        Ok(lib_chirho) => Box::into_raw(Box::new(RhemaHandleChirho { lib_chirho })),
        Err(e_chirho) => {
            set_last_error_chirho(&e_chirho.to_string());
            ptr::null_mut()
        }
    }
}

/// Free a Rhema handle.
///
/// # Safety
///
/// `handle_chirho` must be a valid pointer from `rhema_init_chirho` or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhema_free_chirho(handle_chirho: *mut RhemaHandleChirho) {
    if !handle_chirho.is_null() {
        drop(unsafe { Box::from_raw(handle_chirho) });
    }
}

/// List available modules as a JSON string.
///
/// # Safety
///
/// `handle_chirho` must be a valid pointer. Caller must free the returned string
/// with `rhema_free_string_chirho`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhema_modules_chirho(
    handle_chirho: *const RhemaHandleChirho,
) -> *mut c_char {
    if handle_chirho.is_null() {
        set_last_error_chirho("Null handle");
        return ptr::null_mut();
    }
    let handle_chirho = unsafe { &*handle_chirho };
    let modules_chirho = handle_chirho.lib_chirho.list_all_modules_chirho();
    let names_chirho: Vec<String> = modules_chirho
        .iter()
        .map(|m_chirho| m_chirho.name_chirho.clone())
        .collect();
    match serde_json::to_string(&names_chirho) {
        Ok(json_chirho) => string_to_c_chirho(&json_chirho),
        Err(e_chirho) => {
            set_last_error_chirho(&e_chirho.to_string());
            ptr::null_mut()
        }
    }
}

/// Parse a query and return the IR as JSON.
///
/// # Safety
///
/// `query_chirho` must be a valid null-terminated C string.
/// Caller must free the returned string with `rhema_free_string_chirho`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhema_parse_query_chirho(
    query_chirho: *const c_char,
) -> *mut c_char {
    if query_chirho.is_null() {
        set_last_error_chirho("Null query");
        return ptr::null_mut();
    }
    let c_str_chirho = unsafe { CStr::from_ptr(query_chirho) };
    let query_str_chirho = match c_str_chirho.to_str() {
        Ok(s_chirho) => s_chirho,
        Err(e_chirho) => {
            set_last_error_chirho(&e_chirho.to_string());
            return ptr::null_mut();
        }
    };

    match QueryParserChirho::parse_chirho(query_str_chirho) {
        Ok(parsed_chirho) => {
            let plan_chirho = QueryPlannerChirho::plan_chirho(&parsed_chirho);
            let explain_chirho = QueryPlannerChirho::explain_chirho(&plan_chirho);
            match serde_json::to_string(&serde_json::json!({
                "root_chirho": format!("{:?}", parsed_chirho.root_chirho),
                "plan_chirho": explain_chirho,
                "cost_chirho": plan_chirho.estimated_cost_chirho,
            })) {
                Ok(json_chirho) => string_to_c_chirho(&json_chirho),
                Err(e_chirho) => {
                    set_last_error_chirho(&e_chirho.to_string());
                    ptr::null_mut()
                }
            }
        }
        Err(e_chirho) => {
            set_last_error_chirho(&e_chirho.to_string());
            ptr::null_mut()
        }
    }
}

/// Free a string returned by other FFI functions.
///
/// # Safety
///
/// `s_chirho` must be a pointer from a `rhema_*` function or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhema_free_string_chirho(s_chirho: *mut c_char) {
    if !s_chirho.is_null() {
        drop(unsafe { CString::from_raw(s_chirho) });
    }
}

/// Get the last error message, or null if no error occurred.
///
/// # Safety
///
/// Caller must free the returned string with `rhema_free_string_chirho`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhema_last_error_chirho() -> *mut c_char {
    LAST_ERROR_CHIRHO.with(|e_chirho| match e_chirho.borrow().as_ref() {
        Some(msg_chirho) => string_to_c_chirho(msg_chirho),
        None => ptr::null_mut(),
    })
}

/// Convert a Rust string to a C string allocated on the heap.
fn string_to_c_chirho(s_chirho: &str) -> *mut c_char {
    match CString::new(s_chirho) {
        Ok(c_str_chirho) => c_str_chirho.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Parse a query string and return whether it's valid.
/// This is a safe (non-FFI) function for native testing.
pub fn validate_query_chirho(query_chirho: &str) -> bool {
    QueryParserChirho::parse_chirho(query_chirho).is_ok()
}

/// Get a list of all supported query prefixes.
pub fn supported_prefixes_chirho() -> Vec<&'static str> {
    vec![
        "strong:", "lemma:", "semantic:", "~", "concept:", "expand:",
        "xref:", "rel:", "prop:", "morph:", "pos:", "tense:", "voice:",
        "mood:", "case:", "number:", "gender:", "person:", "domain:",
        "sense:", "syntax:", "clause:",
    ]
}

/// Parse and explain a query, returning a formatted string.
pub fn parse_and_explain_chirho(query_chirho: &str) -> Result<String, String> {
    let parsed_chirho = QueryParserChirho::parse_chirho(query_chirho)
        .map_err(|e_chirho| e_chirho.to_string())?;
    let plan_chirho = QueryPlannerChirho::plan_chirho(&parsed_chirho);
    Ok(QueryPlannerChirho::explain_chirho(&plan_chirho))
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_validate_query_valid_chirho() {
        assert!(validate_query_chirho("love"));
        assert!(validate_query_chirho("love AND world"));
        assert!(validate_query_chirho("strong:G26"));
    }

    #[test]
    fn test_validate_query_invalid_chirho() {
        assert!(!validate_query_chirho(""));
        assert!(!validate_query_chirho("   "));
    }

    #[test]
    fn test_supported_prefixes_chirho() {
        let prefixes_chirho = supported_prefixes_chirho();
        assert!(prefixes_chirho.contains(&"strong:"));
        assert!(prefixes_chirho.contains(&"domain:"));
        assert!(prefixes_chirho.contains(&"syntax:"));
        assert!(prefixes_chirho.len() >= 20);
    }

    #[test]
    fn test_parse_and_explain_chirho() {
        let result_chirho = parse_and_explain_chirho("love AND world").unwrap();
        assert!(result_chirho.contains("Backend: cpu"));
        assert!(result_chirho.contains("Estimated cost"));
    }

    #[test]
    fn test_parse_and_explain_error_chirho() {
        let result_chirho = parse_and_explain_chirho("");
        assert!(result_chirho.is_err());
    }

    #[test]
    fn test_set_and_get_last_error_chirho() {
        set_last_error_chirho("test error");
        LAST_ERROR_CHIRHO.with(|e_chirho| {
            assert_eq!(e_chirho.borrow().as_deref(), Some("test error"));
        });
    }

    #[test]
    fn test_string_to_c_chirho() {
        let c_ptr_chirho = string_to_c_chirho("hello");
        assert!(!c_ptr_chirho.is_null());
        // SAFETY: We just allocated this
        unsafe {
            let c_str_chirho = CStr::from_ptr(c_ptr_chirho);
            assert_eq!(c_str_chirho.to_str().unwrap(), "hello");
            drop(CString::from_raw(c_ptr_chirho));
        }
    }

    #[test]
    fn test_string_to_c_null_byte_chirho() {
        // Strings with embedded null bytes should return null
        let c_ptr_chirho = string_to_c_chirho("hello\0world");
        assert!(c_ptr_chirho.is_null());
    }

    #[test]
    fn test_free_null_string_chirho() {
        // Freeing null should not crash
        unsafe {
            rhema_free_string_chirho(ptr::null_mut());
        }
    }

    #[test]
    fn test_free_null_handle_chirho() {
        // Freeing null handle should not crash
        unsafe {
            rhema_free_chirho(ptr::null_mut());
        }
    }
}
