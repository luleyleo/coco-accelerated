use crate::{sys, Config};

pub struct Context {
    config: Config,
    pub(crate) inner: *mut sys::futhark_context,
}

// TODO: figure out if this is okay
unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn new(config: Config) -> Self {
        let inner = unsafe { sys::futhark_context_new(config.inner) };
        assert!(!inner.is_null());
        Context { config, inner }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn sync(&self) -> bool {
        unsafe { sys::futhark_context_sync(self.inner) == 0 }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            sys::futhark_context_free(self.inner);
        }
    }
}
