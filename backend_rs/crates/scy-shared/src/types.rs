//! Domain types partagés SCY Forge.
//!
//! Conventions SQL (PRD) : préfixe `scy_`, UUID v7, timestamps INTEGER (Unix).

use serde::{Deserialize, Serialize};

/// Utilisateur (multi-tenant, RLS par `user_id`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub display_name: Option<String>,
    pub created_at: i64,
}

/// Objectif d'apprentissage déclaré par l'utilisateur (entrée de l'agent AG-01).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub created_at: i64,
}

/// Source de matière (sortie des 13 cores d'ingestion).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: uuid::Uuid,
    pub goal_id: uuid::Uuid,
    /// Origine : youtube, web, podcast, academic, drive, notion, evernote...
    pub core: String,
    pub url: Option<String>,
    pub raw_content: String,
    pub created_at: i64,
}

/// Nœud du Knowledge Graph / DAG ASCENT (concept structuré).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: uuid::Uuid,
    pub goal_id: uuid::Uuid,
    pub title: String,
    pub summary: String,
    /// Confiance 0.0–1.0 (AI Confidence System, double validation).
    pub confidence: f32,
    pub created_at: i64,
}

/// Concept atomique (extrait par NEURON-CHAINS chaîne BETA).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub id: uuid::Uuid,
    pub node_id: uuid::Uuid,
    pub label: String,
    pub definition: String,
    pub created_at: i64,
}

/// Carte de révision FSRS (types B01–B10).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: uuid::Uuid,
    pub concept_id: uuid::Uuid,
    /// Type B01–B10.
    pub card_type: String,
    pub front: String,
    pub back: String,
    pub created_at: i64,
}
