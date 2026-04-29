use anyhow::{Result, anyhow};
use std::collections::HashMap;

use sbd_gen_schema::Target;

pub struct Resources<'a> {
    // Key names a resource, value is informational "claimed by".
    claims: HashMap<&'a str, String>,
}

impl<'a> Resources<'a> {
    // (not using `_target` yet, keeping for future evolution)
    pub fn new(_target: &Target) -> Self {
        Resources {
            claims: HashMap::new(),
        }
    }

    /// Claim a resource.
    ///
    /// This function is used to mark a resource, represented as `&'a str`. `by` is informational.
    pub fn claim<T: AsRef<str>>(&mut self, resource: &'a str, by: T) -> Result<()> {
        let by = by.as_ref();

        if let Some(other) = self.claims.get(resource) {
            Err(anyhow!(
                "`{by}` wants to claim `{resource}` but that is already used by `{other}`"
            ))
        } else {
            self.claims.insert(resource, by.to_string());
            Ok(())
        }
    }
}
