use std::ffi::CString;

#[inline]
fn to_cstr(message: &str) -> CString {
    let fixed = message.replace('\0', "(null)");
    CString::new(fixed).unwrap()
}
