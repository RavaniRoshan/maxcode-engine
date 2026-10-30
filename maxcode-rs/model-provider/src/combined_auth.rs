//! Composes primary provider credentials with independently managed gateway OAuth credentials.

use std::sync::Arc;

use crate::auth::ResolvedProviderAuth;
use maxcode_api::AuthError;
use maxcode_api::AuthHeadersFuture;
use maxcode_api::AuthProvider;
use maxcode_api::AuthProviderFuture;
use maxcode_api::SharedAuthProvider;
use maxcode_login::GatewayAuthError;
use maxcode_login::GatewayAuthManager;
use maxcode_model_provider_info::GatewayOAuthConfig;
use maxcode_model_provider_info::GatewayOAuthDelivery;
use maxcode_model_provider_info::ModelProviderInfo;
use maxcode_protocol::error::MaxcodeErr;
use maxcode_protocol::error::MaxcodeErrorDetails;
use maxcode_protocol::error::Result;
use http::HeaderMap;
use http::HeaderName;
use http::HeaderValue;

pub(crate) async fn compose_auth(
    provider: &ModelProviderInfo,
    manager: Option<&std::result::Result<Arc<GatewayAuthManager>, String>>,
    mut resolved: ResolvedProviderAuth,
) -> Result<ResolvedProviderAuth> {
    let Some(config) = provider.gateway_oauth.as_ref() else {
        return Ok(resolved);
    };
    provider.validate().map_err(MaxcodeErr::InvalidRequest)?;
    let manager = manager.ok_or_else(|| {
        MaxcodeErr::InvalidRequest("gateway_oauth requires auth runtime configuration".into())
    })?;
    let manager = manager
        .as_ref()
        .map_err(|error| MaxcodeErr::InvalidRequest(error.clone()))?;
    let token = manager.resolve_access_token().await.map_err(|error| {
        if let Some(login_error) = error
            .get_ref()
            .and_then(|source| source.downcast_ref::<GatewayAuthError>())
        {
            return match login_error {
                GatewayAuthError::LoginRequired => {
                    MaxcodeErrorDetails::GatewayLoginRequired(login_error.to_string()).into()
                }
                GatewayAuthError::LoginInProgress => {
                    MaxcodeErr::InvalidRequest(login_error.to_string())
                }
            };
        }
        // Issuer errors may echo arbitrary credentials from configured URLs. Keep the
        // diagnostic safe and bounded, preserving retries for transient failures.
        MaxcodeErr::Io(std::io::Error::new(
            error.kind(),
            "Gateway OAuth authentication failed; check the gateway configuration and credential store.",
        ))
    })?;
    let (name, value) = gateway_header(config, &token)?;
    if resolved
        .auth
        .to_auth_headers()
        .map_err(|error| maxcode_api::map_api_error(error.into()))?
        .contains_key(&name)
    {
        return Err(MaxcodeErr::InvalidRequest(
            "gateway OAuth conflicts with primary auth headers".into(),
        ));
    }
    resolved.auth = Arc::new(CombinedAuth {
        primary: resolved.auth,
        name,
        value,
    });
    Ok(resolved)
}

fn gateway_header(config: &GatewayOAuthConfig, token: &str) -> Result<(HeaderName, HeaderValue)> {
    if token.is_empty() {
        return Err(MaxcodeErr::InvalidRequest(
            "gateway OAuth returned an empty token".into(),
        ));
    }
    let (name, value) = match &config.delivery {
        GatewayOAuthDelivery::Header { name, scheme } => (
            HeaderName::from_bytes(name.as_bytes())
                .map_err(|_| MaxcodeErr::InvalidRequest("invalid gateway header".into()))?,
            format!("{scheme} {token}"),
        ),
        GatewayOAuthDelivery::Cookie { name } => {
            // RFC 6265 cookie-octet excludes separators, quotes, whitespace and non-ASCII.
            if !token.bytes().all(
                |byte| matches!(byte, 0x21 | 0x23..=0x2b | 0x2d..=0x3a | 0x3c..=0x5b | 0x5d..=0x7e),
            ) {
                return Err(MaxcodeErr::InvalidRequest(
                    "gateway OAuth token is not a valid cookie value".into(),
                ));
            }
            (http::header::COOKIE, format!("{name}={token}"))
        }
    };
    let mut value = HeaderValue::from_str(&value)
        .map_err(|_| MaxcodeErr::InvalidRequest("invalid gateway OAuth token header".into()))?;
    value.set_sensitive(true);
    Ok((name, value))
}

struct CombinedAuth {
    primary: SharedAuthProvider,
    name: HeaderName,
    value: HeaderValue,
}

impl AuthProvider for CombinedAuth {
    fn add_auth_headers(&self, headers: &mut HeaderMap) -> std::result::Result<(), AuthError> {
        self.primary.add_auth_headers(headers)?;
        headers
            .entry(self.name.clone())
            .or_insert(self.value.clone());
        Ok(())
    }

    fn resolve_auth_headers(&self) -> AuthHeadersFuture<'_> {
        Box::pin(async move {
            let mut headers = self.primary.resolve_auth_headers().await?;
            if headers.contains_key(&self.name) {
                return Err(AuthError::Build(
                    "gateway OAuth conflicts with primary auth headers".into(),
                ));
            }
            headers.insert(self.name.clone(), self.value.clone());
            Ok(headers)
        })
    }

    fn apply_auth(&self, request: maxcode_http_client::Request) -> AuthProviderFuture<'_> {
        Box::pin(async move {
            let mut request = self.primary.apply_auth(request).await?;
            if request.headers.contains_key(&self.name) {
                return Err(AuthError::Build(
                    "gateway OAuth conflicts with request headers".into(),
                ));
            }
            request
                .headers
                .insert(self.name.clone(), self.value.clone());
            Ok(request)
        })
    }
}
