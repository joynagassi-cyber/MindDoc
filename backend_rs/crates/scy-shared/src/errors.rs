//! Erreurs applicatives unifiées (pattern thiserror + IntoResponse).
//! Réf : `docs/CODE_STYLE.md` §1.4

use thiserror::Error;

/// Erreur applicative globale SCY Forge.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Ressource introuvable: {0}")]
    NotFound(String),

    #[error("Validation échouée: {0}")]
    Validation(String),

    #[error("Erreur d'authentification: {0}")]
    Auth(String),

    #[error("Erreur base de données: {0}")]
    Database(String),

    #[error("Erreur LLM / provider externe: {0}")]
    Llm(String),

    #[error("Budget LLM dépassé (BudgetGuard)")]
    BudgetExceeded,

    #[error("Erreur d'intégration ({provider}): {message}")]
    Integration { provider: String, message: String },

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Erreur interne: {0}")]
    Internal(String),
}

/// Alias de Result pour tout le code SCY Forge.
pub type AppResult<T> = Result<T, AppError>;
