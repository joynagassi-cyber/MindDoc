# 🧪 SCY-SCHEDULER-FSRS — TESTS
**ID** : S05_APEX_SCHEDULER_FSRS_TESTS

- **TC1** : Rating → S/D/next_review_at mis à jour ; R = e^(-t/S).
- **TC2** : Property-based (proptest) → aucune intervalle négative, stabilité > 0 (10 000 cas).
- **TC3** : Transitions d'état New→Learning→Review→Relearning→Review respectées.
- **TC4** : Effets Again (reset+lapse) / Hard (×0.5) / Good (×2.5) / Easy (×4.0+graduated) ; Undo (U/Ctrl+Z).
- **TC5** : ≥1000 révisions → 17 paramètres ajustés + différenciés par domaine.
- **TC6** : Forecast 30j calculé (cartes dues/jour) ; retention cible 90% (configurable 85-95%).
- **TC7** : 10 000 Monte Carlo hebdo → auto-calibration des constantes (D-OPT-028).
- **TC8** : `scy_apex_reviews` immuable (Event Sourcing RGPD, non modifiable).
- **TC9** : Aucun appel LLM dans le scheduler (Rust pur, $0).
