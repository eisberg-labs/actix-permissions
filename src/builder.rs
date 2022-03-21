use std::future::{Future, Ready};
use std::marker::PhantomData;

use crate::permission::Permission;

/// Permission builder
pub struct Builder<'l> {
    pub(crate) permissions: Vec<Box<dyn Permission + 'l>>,
}

impl<'l> Builder<'l> {
    pub fn new() -> Self {
        Self {
            permissions: vec![],
        }
    }

    /// Appends item to permissions
    ///
    /// # Arguments
    /// * `permission` - permission
    pub fn and<P>(mut self, permission: P) -> Self
    where
        P: Permission + 'l,
    {
        self.permissions.push(Box::new(permission));
        Self {
            permissions: self.permissions,
        }
    }
}

impl<'l> Default for Builder<'l> {
    fn default() -> Self {
        Self::new()
    }
}
