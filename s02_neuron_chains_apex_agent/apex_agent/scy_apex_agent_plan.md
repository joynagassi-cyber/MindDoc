# 🛠️ SCY-APEX-AGENT — PLAN (PLAN)
**ID** : S02_NEURON_APEX_AGENT_PLAN · **Décisions** : D-OPT-057/058/059, NC-001 à NC-006

## Flux ReAct
```
[INPUT : "Générer le contenu du nœud X"]
   │
   ▼
[REASON : T01 DocType → T02 Tone → T03 Outline → T04 Budget → T05 Model → T06 RAG top-15]
   │
   ▼
[ACT : T09 PromptCompressor (-60%) → génération section/section → T10 SectionScorer → T11 FactChecker]
   │
   ▼
[OBSERVE : T12 ConfidenceCalc]
   ├── score ≥ 85 → T15 ExportFormatter (export direct)
   ├── 70-84 → réécriture ciblée sections < 70 (1 cycle)
   ├── 45-69 → réécriture complète (1 cycle max)
   └── < 45 → alerte utilisateur + rapport détaillé
```
## Orchestration parallèle (D-OPT-059) :
```
[JoinSet Tokio + CancellationToken]
   ├── chaîne fiches concepts ──┐
   ├── chaîne examen ───────────┼── join_next() → si Err → cancel_token.cancel() (SAGA)
   └── chaîne exercices ────────┘
```
## Dépendances : Rig (CompletionModel/Tool), RRAG, LlmRouter+BudgetGuard, 18 tools. Tables : `scy_documents`, `scy_confidence_reports`, `scy_agent_decisions`.
## Fichiers : `backend_rs/src/neurochain/apex_agent/react_loop.rs`, `orchestration/custom_orchestrator.rs`.
