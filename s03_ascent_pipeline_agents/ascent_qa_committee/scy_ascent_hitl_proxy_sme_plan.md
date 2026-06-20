# 🛠️ SCY-AG16-HITL-PROXY-SME — PLAN D'IMPLÉMENTATION (PLAN)
**ID** : S03_ASCENT_AG16_HITL_PROXY_SME_PLAN · **Décision** : D-OPT-036  
**Spécification de référence** : `scy_ascent_hitl_proxy_sme_specs.md` (38KB — 6 modules détaillés)

## Flux
```
[Formation ingérée + Knowledge Cards NEURON-CHAINS]
   │
   ▼
[MODULE 0 — Domain Fingerprinting : 6 classes épistémologiques + classification auto]
   │
   ▼
[MODULE 1 — Dynamic Persona Bootstrapping : signaux persona + profil expert + scepticisme calibré]
   │
   ▼
[MODULE 2 — Multi-Framework Pedagogical Audit : SOLO + Cognitive Load + Constructive Alignment + Bloom Verbs]
   │
   ▼
[MODULE 3 — Red Team Scientific Audit : 4 questions adversariales + sophismes + Steel-Manning]
   │
   ▼
[MODULE 4 — Domain-Specific Validation : Classe A (Math) / C (Biomédical) / D (Rust statique+Borrow+Clippy+unsafe)]
   │
   ▼
[Matrice de priorisation Module 4 + contribution PQS (S_expert)]
   │
   ├── PQS < 88 ──► Rapport remédiation → APEX-AGENT (réécriture)
   │
   ▼ (PQS ≥ 88)
[Signature électronique Parcours B + traçabilité scy_qa_agent_reviews]
```
## Dépendances : Mastra TS + Rust (analyse statique, Borrow sim, Clippy lints), LlmRouter+BudgetGuard, Zod, Rig/RRAG.
## Fichiers : `backend_ts/src/ascent/agents/ag16_hitl_proxy_sme.ts`, `backend_rs/src/ascent/hitl/domain_classifier.rs`, `persona_bootstrap.rs`, `pedagogical_audit.rs`, `red_team_audit.rs`, `rust_validation.rs`.
