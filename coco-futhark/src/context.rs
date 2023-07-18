use crate::{
    backend::{types, Backend},
    Config,
};

pub struct Context<B: Backend> {
    config: Config<B>,
    pub(crate) inner: *mut types::futhark_context,
}

// TODO: figure out if this is okay
unsafe impl<B: Backend> Send for Context<B> {}
unsafe impl<B: Backend> Sync for Context<B> {}

impl<B: Backend> Context<B> {
    pub fn new(config: Config<B>) -> Self {
        let inner = unsafe { B::futhark_context_new(config.inner) };
        assert!(!inner.is_null());
        Context { config, inner }
    }

    pub fn config(&self) -> &Config<B> {
        &self.config
    }

    pub fn sync(&self) -> bool {
        unsafe { B::futhark_context_sync(self.inner) == 0 }
    }
}

impl<B: Backend> Default for Context<B> {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl<B: Backend> Drop for Context<B> {
    fn drop(&mut self) {
        unsafe {
            B::futhark_context_free(self.inner);
        }
    }
}
