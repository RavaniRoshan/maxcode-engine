use super::*;

#[test]
fn classifies_personal_access_tokens_by_prefix() {
    assert!(matches!(
        classify_maxcode_access_token("at-example"),
        MaxcodeAccessToken::PersonalAccessToken("at-example")
    ));
    assert!(matches!(
        classify_maxcode_access_token("header.payload.signature"),
        MaxcodeAccessToken::AgentIdentityJwt("header.payload.signature")
    ));
}
