use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::Deserialize;

use super::Claims;
use crate::core::rest::AppError;
use crate::errors::ErrJwtTokenCreation;
use crate::errors::ErrUnauthorized;

fn default_expiration_secs() -> i64 {
    3600
}

/// JWT settings used by the authentication middleware.
///
/// Values from the environment take precedence over the TOML configuration:
/// `JWT_SECRET` and `JWT_EXPIRATION_SECS`. In production, configure the secret
/// through `JWT_SECRET` rather than storing it in the configuration file.
#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// HMAC secret used to sign and validate tokens.
    pub secret: String,

    /// Lifetime of newly issued tokens, in seconds.
    pub expiration_secs: i64,
}

#[derive(Deserialize)]
struct JwtConfigFile {
    secret: Option<String>,
    expiration_secs: Option<i64>,
}

impl<'de> Deserialize<'de> for JwtConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let file = JwtConfigFile::deserialize(deserializer)?;
        let secret = std::env::var("JWT_SECRET")
            .ok()
            .or(file.secret)
            .ok_or_else(|| serde::de::Error::custom("jwt.secret or JWT_SECRET must be set"))?;
        let expiration_secs = std::env::var("JWT_EXPIRATION_SECS")
            .ok()
            .map(|value| {
                value.parse::<i64>().map_err(|err| {
                    serde::de::Error::custom(format!(
                        "JWT_EXPIRATION_SECS must be an integer: {err}"
                    ))
                })
            })
            .transpose()?
            .or(file.expiration_secs)
            .unwrap_or_else(default_expiration_secs);

        validate_secret::<D::Error>(secret).map(|secret| Self {
            secret,
            expiration_secs,
        })
    }
}

fn validate_secret<E>(secret: String) -> Result<String, E>
where
    E: serde::de::Error,
{
    if secret.trim().is_empty() {
        return Err(E::custom("jwt.secret and JWT_SECRET must not be empty"));
    }
    Ok(secret)
}

impl JwtConfig {
    pub fn new(secret: impl Into<String>, expiration_secs: i64) -> Self {
        Self {
            secret: secret.into(),
            expiration_secs,
        }
    }

    /// Creates a signed JWT for the authenticated user.
    pub fn generate_token(&self, user_id: i64) -> Result<String, AppError> {
        let now = chrono::Utc::now().timestamp();
        let claims = Claims {
            user_id,
            exp: now.saturating_add(self.expiration_secs),
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|err| ErrJwtTokenCreation.with_cause(err, "generate jwt token"))
    }

    /// Verifies a JWT signature, algorithm and expiry, returning its claims.
    pub fn decode_token(&self, token: &str) -> Result<Claims, AppError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        decode::<Claims>(token, &DecodingKey::from_secret(self.secret.as_bytes()), &validation)
            .map(|data| data.claims)
            .map_err(|err| ErrUnauthorized.with_cause(err, "decode jwt token"))
    }
}

#[cfg(test)]
mod tests {
    use super::JwtConfig;

    #[test]
    fn token_round_trip() {
        let config = JwtConfig::new("test-secret", 60);
        let token = config.generate_token(42).unwrap();
        let claims = config.decode_token(&token).unwrap();
        assert_eq!(claims.user_id, 42);
    }

    #[test]
    fn token_signed_with_another_secret_is_rejected() {
        let token = JwtConfig::new("test-secret", 60)
            .generate_token(42)
            .unwrap();
        assert!(
            JwtConfig::new("another-secret", 60)
                .decode_token(&token)
                .is_err()
        );
    }

    #[test]
    fn secret_is_required_when_deserializing_config() {
        let missing = toml::from_str::<JwtConfig>("expiration_secs = 60");
        assert!(missing.is_err());

        let empty = toml::from_str::<JwtConfig>("secret = \"  \"\nexpiration_secs = 60");
        assert!(empty.is_err());
    }

    #[test]
    fn expiration_defaults_when_omitted() {
        let config = toml::from_str::<JwtConfig>("secret = \"test-secret\"").unwrap();
        assert_eq!(config.expiration_secs, 3600);
    }
}
