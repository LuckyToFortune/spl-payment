use anchor_lang::error_code;

#[error_code]
pub enum SplPaymentError {
    #[msg("SplPaymentError: Not allowed owner")]
    NotAllowedOwner,

    #[msg("SplPaymentError: Over max deposit amount")]
    MaxDepositAmount,

    #[msg("SplPaymentError: InvalidAmount")]
    InvalidAmount,

    #[msg("SplPaymentError: Should deposit than 0")]
    ZeroAmount,

    #[msg("SplPaymentError: The token mint address is not correct")]
    InvalidTokenAddress,
}
