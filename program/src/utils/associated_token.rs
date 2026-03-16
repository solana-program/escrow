use codama::CodamaPda;

/// Associated Token Account PDA definition for external program reference.
/// This generates an `additionalPrograms` entry in the IDL with the ATA program's
/// PDA seeds, allowing instruction accounts to reference it via `pda("associatedToken", ...)`.
#[derive(CodamaPda)]
#[codama(program(name = "associatedToken", address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"))]
#[codama(seed(name = "owner", type = public_key))]
#[codama(seed(name = "tokenProgram", type = public_key))]
#[codama(seed(name = "mint", type = public_key))]
pub struct AssociatedToken;
