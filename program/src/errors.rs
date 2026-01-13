use codama::CodamaErrors;
use pinocchio::error::ProgramError;
use thiserror::Error;

/// Errors that may be returned by the Escrow Program.
#[derive(Clone, Debug, Eq, PartialEq, Error, CodamaErrors)]
pub enum EscrowProgramError {
    /// (1) Escrow ID invalid or does not respect rules
    #[error("Escrow ID invalid or does not respect rules")]
    InvalidEscrowId,

    /// (2) Admin invalid or does not match escrow admin
    #[error("Admin invalid or does not match escrow admin")]
    InvalidAdmin,

    /// (3) Event authority PDA is invalid
    #[error("Event authority PDA is invalid")]
    InvalidEventAuthority,

    /// (4) Timelock has not expired yet
    #[error("Timelock has not expired yet")]
    TimelockNotExpired,

    /// (5) External hook rejected the operation
    #[error("External hook rejected the operation")]
    HookRejected,

    /// (6) Withdrawer does not match receipt depositor
    #[error("Withdrawer does not match receipt depositor")]
    InvalidWithdrawer,

    /// (7) Receipt escrow does not match escrow
    #[error("Receipt escrow does not match escrow")]
    InvalidReceiptEscrow,

    /// (8) Hook program mismatch
    #[error("Hook program mismatch")]
    HookProgramMismatch,

    /// (9) Mint is not allowed for this escrow
    #[error("Mint is not allowed for this escrow")]
    MintNotAllowed,

    /// (10) Mint has PermanentDelegate extension which is not allowed
    #[error("Mint has PermanentDelegate extension which is not allowed")]
    PermanentDelegateNotAllowed,

    /// (11) Mint has NonTransferable extension which is not allowed
    #[error("Mint has NonTransferable extension which is not allowed")]
    NonTransferableNotAllowed,

    /// (12) Mint has Pausable extension which is not allowed
    #[error("Mint has Pausable extension which is not allowed")]
    PausableNotAllowed,

    /// (13) Token extension already blocked
    #[error("Token extension already blocked")]
    TokenExtensionAlreadyBlocked,
}

impl From<EscrowProgramError> for ProgramError {
    fn from(e: EscrowProgramError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let error: ProgramError = EscrowProgramError::InvalidEscrowId.into();
        assert_eq!(error, ProgramError::Custom(0));

        let error: ProgramError = EscrowProgramError::InvalidAdmin.into();
        assert_eq!(error, ProgramError::Custom(1));

        let error: ProgramError = EscrowProgramError::InvalidEventAuthority.into();
        assert_eq!(error, ProgramError::Custom(2));

        let error: ProgramError = EscrowProgramError::TimelockNotExpired.into();
        assert_eq!(error, ProgramError::Custom(3));

        let error: ProgramError = EscrowProgramError::HookRejected.into();
        assert_eq!(error, ProgramError::Custom(4));

        let error: ProgramError = EscrowProgramError::InvalidWithdrawer.into();
        assert_eq!(error, ProgramError::Custom(5));
    }
}
