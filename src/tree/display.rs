use crate::Node;

#[cfg(not(feature = "wasm"))]
use std::fmt::{Display, Error, Formatter};

#[cfg(feature = "wasm")]
use sp_std::fmt::{Display, Error, Formatter};

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.operator.fmt(f)?;
        for child in self.children() {
            write!(f, " {}", child)?;
        }
        Ok(())
    }
}
