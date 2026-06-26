# 🛠️ SCY-NORMAL-ORCHESTRATION — PLAN / TÂCHES / TESTS
**ID** : S10_NORMAL_MODE_ORCHESTRATION_PLAN / TASKS / TESTS
## Flux : [source ingérée Mode Normal] → activation immédiate COSMOS + APEX (FSRS) + BRAIN (0$ attente) → filtre intégrité minimal arrière-plan (anti-NaN/anti-÷0 softening_epsilon) → Parcours A (zéro certificat).
## Référence : `scy_normal_mode_orchestration_specs.md`.
## Fichiers : `backend_rs/src/normal/orchestrator.rs`, `integrity_filter.rs`.
## Tâches : NO.1 Activation immédiate COSMOS/APEX/BRAIN post-ingestion (25min) | NO.2 Filtre intégrité minimal (anti-NaN/÷0) (20min).
## Tests : TC1 activation 0$ attente | TC2 filtre sans blocage | TC3 zéro certificat (Parcours A).
