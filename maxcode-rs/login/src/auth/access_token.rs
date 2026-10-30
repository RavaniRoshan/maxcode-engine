const PERSONAL_ACCESS_TOKEN_PREFIX: &str = "at-";

pub(super) enum MaxcodeAccessToken<'a> {
    PersonalAccessToken(&'a str),
    AgentIdentityJwt(&'a str),
}

pub(super) fn classify_maxcode_access_token(access_token: &str) -> MaxcodeAccessToken<'_> {
    if access_token.starts_with(PERSONAL_ACCESS_TOKEN_PREFIX) {
        MaxcodeAccessToken::PersonalAccessToken(access_token)
    } else {
        MaxcodeAccessToken::AgentIdentityJwt(access_token)
    }
}

#[cfg(test)]
#[path = "access_token_tests.rs"]
mod tests;
