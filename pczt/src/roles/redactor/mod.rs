use crate::{common::Global, Pczt};

pub mod orchard;
pub mod sapling;
pub mod transparent;

pub struct Redactor {
    pczt: Pczt,
}

impl Redactor {
    /// Instantiates the Redactor role with the given PCZT.
    pub fn new(pczt: Pczt) -> Self {
        Self { pczt }
    }

    /// Redacts the global transaction details with the given closure.
    pub fn redact_global_with<F>(mut self, f: F) -> Self
    where
        F: FnOnce(GlobalRedactor<'_>),
    {
        f(GlobalRedactor(&mut self.pczt.global));
        self
    }

    /// Finishes the Redactor role, returning the redacted PCZT.
    pub fn finish(self) -> Pczt {
        self.pczt
    }
}

/// An Redactor for the global transaction details.
pub struct GlobalRedactor<'a>(&'a mut Global);

impl<'a> GlobalRedactor<'a> {
    /// Redacts the proprietary value at the given key.
    pub fn redact_proprietary(&mut self, key: &str) {
        self.0.proprietary.remove(key);
    }

    /// Removes all proprietary values.
    pub fn clear_proprietary(&mut self) {
        self.0.proprietary.clear();
    }
}