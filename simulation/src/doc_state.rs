use scraper::Html;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct DocState {
    data: Arc<RwLock<Html>>,
}

impl DocState {
    pub fn new(doc: Html) -> Self {
        Self {
            data: Arc::new(RwLock::new(doc)),
        }
    }

    /// Clones a `&'static` reference of the data.
    pub fn clone_reference(&self) -> &'static Arc<RwLock<Html>> {
        Box::leak(Box::new(Arc::clone(&self.data)))
    }

    /// Returns an immutable reference to the data. (read-only)
    pub fn read_value(&self) -> RwLockReadGuard<'_, Html> {
        let guard = self.data.read().unwrap();
        guard
    }

    /// Returns a mutable reference to the data. (write)
    pub fn write_value(&self) -> RwLockWriteGuard<'_, Html> {
        let guard = self.data.write().unwrap();
        guard
    }

    /// Manipulate the data in a closure.
    ///
    /// ```rust
    /// let state = DocState::new(...);
    /// state.manipulate(|doc| {
    ///     // do stuff with the document
    /// });
    /// ```
    pub fn manipulate<T>(&self, f: impl Fn(&mut Html) -> T) -> T {
        f(&mut self.write_value())
    }
}
