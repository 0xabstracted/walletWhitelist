use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Src Balance < LP Deposit Amount.")]
    NotEnoughBalance,
    #[msg("Can't decerease as the count is more than number of available spots.")]
    InvalidNumberofWL,
}
