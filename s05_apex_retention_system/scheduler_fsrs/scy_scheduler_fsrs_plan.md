# 🛠️ SCY-SCHEDULER-FSRS — PLAN D'IMPLÉMENTATION (PLAN)
**ID** : S05_APEX_SCHEDULER_FSRS_PLAN

## Flux
```
[Carte due (scy_apex_cards.next_review_at ≤ now)]
   │
   ▼
[Session APEX : carte présentée → utilisateur révèle (Space) → rating Again/Hard/Good/Easy]
   │
   ▼
[fsrs 0.6 (Rust pur, 0 LLM) : recalcul S, D, R=e^(-t/S), next_review_at]
   │
   ├── Again → state Relearning, +1 lapse, interval <1j
   ├── Hard  → interval ×0.5
   ├── Good  → interval ×2.5
   └── Easy  → interval ×4.0 + graduated
   │
   ▼
[Transition état (New→Learning→Review→Relearning→Review) + Undo (U/Ctrl+Z)]
   │
   ▼
[Event Sourcing : scy_apex_reviews (immuable, fsrs_state_before/after)]
   │
   ├── Calibration ≥1000 révisions → 17 params + par domaine (PAPER-005)
   ├── Self-Consistency Checker hebdo : 10 000 Monte Carlo → auto-calibration (D-OPT-028)
   └── Forecast 30j (Recharts bar chart) + retention cible 90%
```
## Dépendances : `fsrs` 0.6, property-based testing (proptest D-ARC-012). Tables : `scy_apex_cards`, `scy_apex_reviews`, `scy_apex_sessions`.
## Fichiers : `backend_rs/src/apex/scheduler/fsrs_engine.rs`, `calibrator.rs`, `forecast.rs`, `monte_carlo_checker.rs`.
