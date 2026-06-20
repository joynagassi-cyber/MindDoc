# 📋 SCY-SCHEDULER-FSRS — TÂCHES (TASKS)
**ID** : S05_APEX_SCHEDULER_FSRS_TASKS

### Tâche SF.1 : Coder le moteur FSRS + recalcul S/D/R (25 min)
* **Fichier** : `backend_rs/src/apex/scheduler/fsrs_engine.rs`
* **Description** : Intégration `fsrs` 0.6, recalcul S/D/`R=e^(-t/S)`/next_review_at après rating.
* **Critère** : Recalcul correct ; R formule officielle.

### Tâche SF.2 : Coder la machine à états + feedback 4 niveaux (25 min)
* **Fichier** : `backend_rs/src/apex/scheduler/fsrs_engine.rs`
* **Description** : Transitions New→Learning→Review→Relearning, effets Again/Hard/Good/Easy, Undo (U/Ctrl+Z).
* **Critère** : Transitions et effets corrects ; Undo fonctionnel.

### Tâche SF.3 : Coder les tests property-based (20 min)
* **Fichier** : `backend_rs/src/apex/scheduler/fsrs_engine.rs`
* **Description** : proptest (D-ARC-012) — aucune intervalle négative, stabilité > 0.
* **Critère** : Invariant respecté sur 10 000 cas aléatoires.

### Tâche SF.4 : Coder la calibration + par domaine (25 min)
* **Fichier** : `backend_rs/src/apex/scheduler/calibrator.rs`
* **Description** : Ajustement 17 paramètres après ≥1000 révisions + différenciation par domaine (PAPER-005).
* **Critère** : Paramètres ajustés personnellement et par domaine.

### Tâche SF.5 : Coder Forecast + Monte Carlo Self-Consistency (25 min)
* **Fichiers** : `forecast.rs`, `monte_carlo_checker.rs`
* **Description** : Forecast 30j (bar chart) + retention cible 90% + 10 000 Monte Carlo hebdo (D-OPT-028).
* **Critère** : Forecast 30j correct ; Monte Carlo auto-calibre les constantes.
