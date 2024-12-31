#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::error::Error;

pub trait Chain<E: Error> {
    fn parse(data: &Vec<u8>) -> Result<String, E>;
}
