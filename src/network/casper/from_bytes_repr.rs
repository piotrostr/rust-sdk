use crate::network::{
    from_bytes_repr::{FromBytesRepr, Sanitized},
    specific::U256,
};

impl FromBytesRepr<Vec<u8>> for U256 {
    fn from_bytes_repr(bytes: Vec<u8>) -> Self {
        bytes.sanitized().as_slice().into()
    }
}
