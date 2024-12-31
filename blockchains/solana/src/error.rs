extern crate alloc;
use alloc::string::String;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolanaError {
    #[error("Program `{0}` is not supported yet")]
    UnsupportedProgram(String),

    #[error("Meet invalid data when reading `{0}`")]
    InvalidData(String),

    #[error("Error occurred when parsing program instruction, reason: `{0}`")]
    ProgramError(String),

    #[error("Could not found account for `{0}`")]
    AccountNotFound(String),
}

// 使用 core 替代 std
pub type Result<T> = core::result::Result<T, SolanaError>;
