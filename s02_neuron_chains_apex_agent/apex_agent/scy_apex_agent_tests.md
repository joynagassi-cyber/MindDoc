# 🧪 SCY-APEX-AGENT — TESTS
**ID** : S02_NEURON_APEX_AGENT_TESTS

- **TC1 (ReAct)** : Demande → type/ton/sources/budget/model déterminés → génération → scoring → décision.
- **TC2 (Section/section)** : Document multi-sections généré et scoré section par section (NC-002).
- **TC3 (Parallèle)** : Chaînes exécutées en parallèle via JoinSet + CancellationToken (D-OPT-059).
- **TC4 (SAGA)** : Échec d'une branche → annulation immédiate de toutes les autres.
- **TC5 (Seuils)** : score ≥85 → export ; 70-84 → réécriture ciblée ; 45-69 → complète ; <45 → alerte.
- **TC6 (Ancrage RAG)** : Aucune génération sans chunks RAG (couche 1 anti-hallucination).
- **TC7 (Budget)** : Budget tokens respecté (T04/T17 BudgetGuard).
- **TC8 (Traçabilité)** : Décisions journalisées dans `scy_agent_decisions`.
