use std::fmt;

#[derive(Clone, PartialEq)]
pub struct MemoryAddress(pub usize);

impl fmt::Debug for MemoryAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            write!(f, "NULL")
        } else {
            write!(f, "0x{:x?}", self.0)
        }
    }
}
