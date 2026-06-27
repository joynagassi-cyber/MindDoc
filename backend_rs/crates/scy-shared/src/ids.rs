//! Helpers UUID v7 (IDs triables chronologiquement, PRD SQL conventions).
//!
//! UUID v7 = timestamp Unix millisecondes + random → IDs naturellement ordonnés
//! dans le temps, parfaits pour les index et le sharding.

use uuid::Uuid;

/// Génère un nouvel UUID v7.
#[must_use]
pub fn new_id() -> Uuid {
    Uuid::now_v7()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_v7_est_unique_et_possede_une_version() {
        let a = new_id();
        let b = new_id();
        assert_ne!(a, b, "deux UUID v7 doivent différer");
        assert!(a.get_version().is_some(), "l'UUID possède une version");
    }
}
