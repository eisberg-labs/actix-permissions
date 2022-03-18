use std::future::{Future, Ready};

use crate::permission::Permission;

/// Permission builder
pub struct Builder<'r> {
    pub(crate) permissions: Vec<Box<dyn Permission<'r>>>,
}

impl<'r> Builder<'r> {
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
        P: Permission<'r> + 'static,
    {
        self.permissions.push(Box::new(permission));
        Self {
            permissions: self.permissions,
        }
    }
}

impl<'r> Default for Builder<'r> {
    fn default() -> Self {
        Self::new()
    }
}
