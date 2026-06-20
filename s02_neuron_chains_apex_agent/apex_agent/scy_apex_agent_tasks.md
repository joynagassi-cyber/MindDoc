# 📋 SCY-APEX-AGENT — TÂCHES (TASKS)
**ID** : S02_NEURON_APEX_AGENT_TASKS

### Tâche AX.1 : Coder la boucle ReAct (Reason→Act→Observe) (30 min)
* **Fichier** : `backend_rs/src/neurochain/apex_agent/react_loop.rs`
* **Description** : Boucle ReAct appelant T01-T06 (reason), génération+T10+T11 (act), T12 (observe/décision).
* **Critère** : Demande → raisonnement → génération → scoring → décision export/réécriture/alerte.

### Tâche AX.2 : Coder l'orchestrateur parallèle JoinSet + CancellationToken (25 min)
* **Fichier** : `backend_rs/src/neurochain/orchestration/custom_orchestrator.rs`
* **Description** : Fan-out des chaînes via JoinSet, CancellationToken, SAGA fallback (échec → annulation globale, D-OPT-059).
* **Critère** : Chaînes parallèles ; échec d'une branche → annulation immédiate des autres.

### Tâche AX.3 : Intégrer Rig + RRAG (20 min)
* **Fichier** : `backend_rs/src/neurochain/apex_agent/react_loop.rs`
* **Description** : Instanciation APEX-AGENT via Rig CompletionModel + Tool composables + RRAG récupération hybride (D-OPT-057/058).
* **Critère** : Typage strict compilation ; RAG hybride opérationnel.

### Tâche AX.4 : Coder les seuils de décision + traçabilité (20 min)
* **Fichier** : `backend_rs/src/neurochain/apex_agent/react_loop.rs`
* **Description** : Seuils (≥85 export / 70-84 ciblé / 45-69 complet / <45 alerte) + journalisation `scy_agent_decisions`.
* **Critère** : Décisions correctes ; traçabilité complète.
