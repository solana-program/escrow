use codama::CodamaAccount;

/// Event authority PDA — no account data, only used for CPI event emission signing.
#[derive(CodamaAccount)]
#[codama(seed(type = string(utf8), value = "event_authority"))]
pub struct EventAuthority;
