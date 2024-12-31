extern crate alloc;
use crate::error::Result;
use alloc::vec::Vec;

pub trait Read<T> {
    fn read(raw: &mut Vec<u8>) -> Result<T>;
}
