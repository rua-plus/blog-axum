use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::utils::config::AppConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug)]
pub enum JwtError {
    InvalidToken,
    ExpiredToken,
    JsonWebTokenError(jsonwebtoken::errors::Error),
    ConfigError,
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtError::InvalidToken => write!(f, "Invalid token"),
            JwtError::ExpiredToken => write!(f, "Expired token"),
            JwtError::JsonWebTokenError(e) => write!(f, "JWT error: {}", e),
            JwtError::ConfigError => write!(f, "JWT config error"),
        }
    }
}

impl std::error::Error for JwtError {}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::ExpiredToken,
            _ => JwtError::JsonWebTokenError(err),
        }
    }
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
    expires_in: u64,
}

impl JwtService {
    pub fn from_config(config: &AppConfig) -> Result<Self, JwtError> {
        let secret = &config.jwt.secret;
        if secret.is_empty() {
            return Err(JwtError::ConfigError);
        }

        let expires_in = parse_expires_in(&config.jwt.expires_in)?;

        Ok(Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            validation: Validation::new(Algorithm::HS256),
            expires_in,
        })
    }

    pub fn generate_token(&self, user_id: &str) -> Result<String, JwtError> {
        let now = chrono::Utc::now().timestamp() as usize;
        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + self.expires_in as usize,
        };

        encode(&Header::default(), &claims, &self.encoding_key).map_err(JwtError::from)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtError> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map_err(JwtError::from)?;

        Ok(token_data.claims)
    }
}

fn parse_expires_in(expires_in: &str) -> Result<u64, JwtError> {
    let (num, unit) = expires_in.trim().split_at(
        expires_in
            .find(|c: char| !c.is_ascii_digit())
            .ok_or(JwtError::ConfigError)?,
    );

    let num = num.parse::<u64>().map_err(|_| JwtError::ConfigError)?;

    let seconds = match unit.trim().to_lowercase().as_str() {
        "s" => num,
        "m" => num * 60,
        "h" => num * 3600,
        "d" => num * 86400,
        "w" => num * 604800,
        _ => return Err(JwtError::ConfigError),
    };

    Ok(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::config::AppConfig;

    #[test]
    fn test_jwt_service_from_config() {
        let config = AppConfig {
            postgresql: Default::default(),
            jwt: crate::utils::config::JwtConfig {
                secret: "test-secret".to_string(),
                expires_in: "7d".to_string(),
            },
        };

        let jwt_service = JwtService::from_config(&config);
        assert!(jwt_service.is_ok());
    }

    #[test]
    fn test_generate_and_validate_token() {
        let config = AppConfig {
            postgresql: Default::default(),
            jwt: crate::utils::config::JwtConfig {
                secret: "test-secret".to_string(),
                expires_in: "1h".to_string(),
            },
        };

        let jwt_service = JwtService::from_config(&config).unwrap();
        let token = jwt_service.generate_token("user123").unwrap();
        let claims = jwt_service.validate_token(&token).unwrap();

        assert_eq!(claims.sub, "user123");
        assert!(claims.exp > chrono::Utc::now().timestamp() as usize);
    }

    #[test]
    fn test_parse_expires_in() {
        assert_eq!(parse_expires_in("30s").unwrap(), 30);
        assert_eq!(parse_expires_in("10m").unwrap(), 600);
        assert_eq!(parse_expires_in("2h").unwrap(), 7200);
        assert_eq!(parse_expires_in("7d").unwrap(), 604800);
        assert_eq!(parse_expires_in("1w").unwrap(), 604800);
    }
}
