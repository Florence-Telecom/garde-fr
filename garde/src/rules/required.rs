use crate::{Error, Result};

pub fn apply<T: Required>(v: &T, _: ()) -> Result {
    if !v.is_set() {
        return Err(Error::new("n'est pas défini"));
    }
    Ok(())
}

pub trait Required {
    fn is_set(&self) -> bool;
}

impl<T> Required for Option<T> {
    fn is_set(&self) -> bool {
        self.is_some()
    }
}
