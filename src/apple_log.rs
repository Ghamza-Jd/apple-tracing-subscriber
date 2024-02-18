use crate::sys::ats_get_default_log;
use crate::sys::os_log_t;
use crate::sys::os_release;
use std::ffi::c_void;

pub struct AppleLog {
    inner: os_log_t,
}

unsafe impl Send for AppleLog {}
unsafe impl Sync for AppleLog {}

impl Drop for AppleLog {
    fn drop(&mut self) {
        unsafe {
            if self.inner != ats_get_default_log() {
                os_release(self.inner as *mut c_void);
            }
        }
    }
}
