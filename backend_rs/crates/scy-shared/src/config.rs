//! Configuration globale SCY Forge (chargée depuis l'environnement).
//! Réf : `.env.example`, `docs/PROJECT_STRUCTURE.md` §6

use serde::Deserialize;

/// Configuration applicative (Phase 0-1, env-based).
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub sqlite_path: String,
    pub zilliz_uri: String,
    pub zilliz_token: String,
    pub deepseek_api_key: String,
    pub searxng_api_url: String,
    pub perplexica_url: String,
    pub nango_secret_key: String,
    pub nango_host: String,
    pub jwt_secret: String,
}

impl Config {
    /// Charge la config depuis les variables d'environnement.
    ///
    /// # Errors
    /// Retourne `AppError` si une variable requise manque.
    pub fn from_env() -> Result<Self, crate::AppError> {
        fn env_or_err(key: &str) -> Result<String, crate::AppError> {
            std::env::var(key).map_err(|_| {
                crate::AppError::Validation(format!("variable d'environnement manquante: {key}"))
            })
        }

        Ok(Self {
            database_url: env_or_err("DATABASE_URL")?,
            sqlite_path: std::env::var("SQLITE_PATH")
                .unwrap_or_else(|_| "./data/scy_forge.db".to_string()),
            zilliz_uri: env_or_err("ZILLIZ_URI")?,
            zilliz_token: env_or_err("ZILLIZ_TOKEN")?,
            deepseek_api_key: env_or_err("DEEPSEEK_API_KEY")?,
            searxng_api_url: std::env::var("SEARXNG_API_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            perplexica_url: std::env::var("PERPLEXICA_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
            nango_secret_key: env_or_err("NANGO_SECRET_KEY")?,
            nango_host: std::env::var("NANGO_HOST")
                .unwrap_or_else(|_| "http://localhost:3003".to_string()),
            jwt_secret: env_or_err("JWT_SECRET")?,
        })
    }
}
