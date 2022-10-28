use crate::{error::AppError, AppState};
use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, State, FromRef},
    http::{header::AUTHORIZATION, request::Parts, HeaderValue},
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use tracing::error;
use sha2::Sha384;
use time::{Duration, OffsetDateTime};

const SCHEME_PREFIX: &str = "Bearer ";
const DEFAULT_SESSION_LENGTH: Duration = Duration::weeks(2);

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AuthUser {
    pub user_id: String,
}

#[derive(Debug)]
pub struct MaybeAuthUser(pub Option<AuthUser>);

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthUserClaims {
    user_id: String,
    /// Standard JWT `exp` claim.
    exp: i64,
}

impl AuthUser {
    pub fn to_jwt(&self, ctx: &AppState) -> String {
        let hmac = Hmac::<Sha384>::new_from_slice(ctx.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        AuthUserClaims {
            user_id: self.user_id.clone(),
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
        }
        .sign_with_key(&hmac)
        .expect("HMAC signing should be infallible")
    }

    fn from_authorization(ctx: &AppState, auth_header: &HeaderValue) -> Result<Self, AppError> {
        let auth_header = auth_header.to_str().map_err(|e| {
            error!("Couldn't encode auth header as string, {e}");
            AppError::Unathorized
        })?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            error!("Missing Bearer prefix on authorization header");
            return Err(AppError::Unathorized);
        }

        let token = &auth_header[SCHEME_PREFIX.len()..];

        let jwt = jwt::Token::<jwt::Header, AuthUserClaims, _>::parse_unverified(token)
            .map_err(|_e| AppError::Unathorized)?;

        let hmac = Hmac::<Sha384>::new_from_slice(ctx.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        let jwt = jwt
            .verify_with_key(&hmac)
            .map_err(|e| {
                error!("Error on verifying tokey, {e}");
                AppError::Unathorized 
            })?;

        let (_header, claims) = jwt.into();

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            error!("Outdated token");
            return Err(AppError::Unathorized);
        }

        Ok(Self {
            user_id: claims.user_id,
        })
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync, AppState: FromRef<S>
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let State(state): State<AppState> = State::from_request_parts(parts, state)
            .await
            .map_err(|e| { 
                error!("error on getting request parts {e}");
                AppError::Unathorized 
            })?;

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or_else(|| {
                error!("Missing authorization header");
                AppError::Unathorized 
            })?;

        Self::from_authorization(&state, auth_header)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for MaybeAuthUser
where
    S: Send + Sync, AppState: FromRef<S>
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let State(state): State<AppState> = State::from_request_parts(parts, state)
            .await
            .map_err(|e| { 
                error!("error on getting request parts {e}");
                AppError::Unathorized 
            })?;

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or_else(|| return Self(None));

        match auth_header {
            Ok(header) => {
                let auth = AuthUser::from_authorization(&state, header).ok();
                Ok(Self(auth))
            },
            Err(e) => {
                Ok(e)
            }
        }
    }
}
