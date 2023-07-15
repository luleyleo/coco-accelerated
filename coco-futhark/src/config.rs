use crate::sys;

pub struct Config {
    pub(crate) inner: *mut sys::futhark_context_config,
}

impl Config {
    pub fn new() -> Self {
        let inner = unsafe { sys::futhark_context_config_new() };
        assert!(!inner.is_null());
        Config { inner }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe {
            sys::futhark_context_config_free(self.inner);
        }
    }
}
