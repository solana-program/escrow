use codama::CodamaPda;

/// PDA definition for the event authority.
/// This has no account data — it's only used for CPI event emission signing.
#[derive(CodamaPda)]
#[codama(seed(type = string(utf8), value = "event_authority"))]
pub struct EventAuthority;
