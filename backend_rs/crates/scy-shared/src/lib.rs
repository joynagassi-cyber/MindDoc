//! # scy-shared
//!
//! Types, erreurs, helpers et configuration partagés par tous les crates SCY Forge.
//!
//! Ce crate est la **fondation bottom-up** (Sprint 0) : tous les services transverses
//! (`scy-eventbus`, `scy-ingestion`, `scy-neuron-chains`...) en dépendent.
//!
//! Réf : `docs/PROJECT_STRUCTURE.md` §2 (scy-shared), `docs/CODE_STYLE.md`

pub mod config;
pub mod errors;
pub mod ids;
pub mod types;

pub use config::Config;
pub use errors::AppError;
pub use ids::new_id;
pub use types::{Card, Concept, Goal, Node, Source, User};
