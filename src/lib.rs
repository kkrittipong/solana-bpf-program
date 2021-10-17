use crate::error::EscrowError;
use solana_program::program_error::ProgramError;

pub mod entrypoint;
pub mod instruction;
pub mod error;

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}