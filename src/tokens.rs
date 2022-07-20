use serde::Deserialize;
use std::time::Instant;

#[derive(Debug, Clone)]
pub(super) struct BearerToken {
    access_token: String,
    time_alive: Instant,
}

impl BearerToken {
    const TOKEN_MAX_AGE: u64 = 7200;

    pub(super) fn new(access_token: String) -> Self {
        Self {
            access_token,
            time_alive: Instant::now(),
        }
    }

    pub(super) fn close_to_expiring(&self) -> bool {
        (self.time_alive.elapsed().as_secs() + Self::TOKEN_MAX_AGE / 10) >= Self::TOKEN_MAX_AGE
    }

    pub(super) fn access_token(&self) -> String {
        self.access_token.clone()
    }
}

impl From<TokenResponse> for BearerToken {
    fn from(tr: TokenResponse) -> Self {
        Self::new(tr.access_token)
    }
}

#[derive(Debug, Clone, Deserialize)]
enum TokenType {
    Bearer,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct TokenResponse {
    // NOTE: There's really no point to using refresh token currently. Perhaps in the
    //       future if we implement an authorization grant workflow.
    // refresh_token: Option<String>,
    access_token: String,
    // expires_in: u16,
    // token_type: TokenType,
}
