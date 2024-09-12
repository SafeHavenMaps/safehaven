use std::collections::HashMap;

use axum::response::{IntoResponse, Response};
use axum::RequestExt;
use axum::{
    async_trait,
    extract::{FromRequestParts, Request, State},
    http::request::Parts,
    middleware::Next,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::family::Family;
use crate::{
    api::{AppError, AppState, DbConn},
    models::access_token::{AccessToken, Permissions},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapUserTokenClaims {
    pub perms: Permissions,
    pub fam_priv_idx: HashMap<Uuid, Vec<String>>,
    pub exp: usize,
    pub iat: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for MapUserTokenClaims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // Extract the claims from the request extensions
        let claims = parts
            .extensions
            .get::<MapUserTokenClaims>()
            .ok_or(AppError::Unauthorized)?;

        Ok(claims.clone())
    }
}

pub async fn viewer_authentication_middleware(
    State(app_state): State<AppState>,
    DbConn(mut conn): DbConn,
    mut request: Request,
    next: Next,
) -> Response {
    // We try to get the token from the bearer. If it fails, we check the header SH-Plain-AccessToken
    // we then try to get the access token from the database.
    // If we can't find it, we 401 the request, otherwise we add the claims to the request extensions
    // and we set a new header with the new token named SH-Renew-Token.

    // Extract the token from the authorization header
    let bearer = match request
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
    {
        Ok(bearer) => bearer,
        Err(_) => return AppError::TokenValidation.into_response(),
    };

    let token_data = jsonwebtoken::decode::<MapUserTokenClaims>(
        bearer.token(),
        &jsonwebtoken::DecodingKey::from_secret(app_state.config.token_secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    );

    let (token_data, new_jwt) = match token_data {
        // If the token is valid, we return the claims directly
        Ok(token_data) => (token_data.claims, None),

        // If the token is invalid, we try to get the plain access token from the X-SH-Plain-AccessToken header
        Err(_) => {
            // Get string from custom header X-SH-Plain-AccessToken
            let plain_access_token_header = request
                .headers()
                .get("X-SH-Plain-AccessToken")
                .ok_or(AppError::TokenValidation);

            let plain_access_token = match plain_access_token_header {
                Ok(plain_access_token) => match plain_access_token.to_str() {
                    Ok(plain_access_token) => plain_access_token,
                    Err(_) => return AppError::TokenValidation.into_response(),
                },
                Err(app_error) => return app_error.into_response(),
            };

            // Get the access token from the database
            let access_token =
                match AccessToken::get(plain_access_token.to_string(), &mut conn).await {
                    Ok(access_token) => access_token,
                    Err(app_error) => return app_error.into_response(),
                };

            let perms = access_token.permissions.0;

            let families = match Family::list_restricted(&perms.families_policy, &mut conn).await {
                Ok(families) => families,
                Err(app_error) => return app_error.into_response(),
            };

            let fam_priv_idx: HashMap<Uuid, Vec<String>> =
                Family::get_privately_indexed_fields_for_families(&families);

            // Create a new token
            let new_claims = MapUserTokenClaims {
                iat: Utc::now().timestamp() as usize,
                exp: (Utc::now() + TimeDelta::try_hours(1).expect("valid duration")).timestamp()
                    as usize,
                fam_priv_idx,
                perms: perms.clone(),
            };
            let new_token = app_state.generate_token(new_claims.clone());

            (new_claims, Some(new_token))
        }
    };

    request.extensions_mut().insert(token_data);

    let mut response = next.run(request).await;

    // attach new token to the response headers if it exists
    if let Some(new_jwt) = new_jwt {
        response.headers_mut().insert(
            "X-SH-Renew-Token",
            new_jwt.parse().expect("valid header value"),
        );
    }

    response
}
