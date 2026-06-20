# 📋 SCY-ASCENT-QA-COMMITTEE — TÂCHES (TASKS)
**ID** : S03_ASCENT_QA_COMMITTEE_TASKS · **Décision** : D-OPT-032

### Tâche QA.1 : Coder l'orchestrateur SAGA des 6 audits (25 min)
* **Fichier** : `backend_ts/src/ascent/qa/qa_committee_orchestrator.ts`
* **Description** : Orchestration asynchrone des 6 rôles d'audit après ingestion.
* **Critère** : Les 6 audits s'exécutent et produisent chacun un sous-score + findings + remediation.

### Tâche QA.2 : Coder QA-01 + QA-02 + QA-03 (30 min)
* **Fichiers** : `qa_01_curriculum.ts`, `qa_02_sme.ts`, `qa_03_alignment.ts`
* **Description** : Curriculum Designer (progression), SME (véracité), Alignment Orchestrator (couverture syllabus → AGENT-02 si manquant).
* **Critère** : Sous-scores design/expert/align produits ; lacune syllabus → recherche AGENT-02.

### Tâche QA.3 : Coder QA-04 + QA-05 (PQS) (30 min)
* **Fichiers** : `qa_04_cognitive_load.ts`, `qa_05_content_validator.ts`
* **Description** : Cognitive Load Guarantor (densité, ELI5) + Content Validator (markdown/LaTeX, calcul PQS).
* **Critère** : Sous-score cognitive + PQS global calculé.

### Tâche QA.4 : Coder QA-06 Academic Certifier + Constructive Alignment (30 min)
* **Fichiers** : `qa_06_certifier.ts`, `scy_ascent_qa_alignment.ts`
* **Description** : Audit cours↔examen SurveyJS (ConstructiveAlignmentSchema Zod), aligned/mismatch.
* **Critère** : Question hors cours → aligned=false + remediation précis.

### Tâche QA.5 : Coder la gate PQS Harmonist + signature + traçabilité (25 min)
* **Fichier** : `backend_ts/src/ascent/qa/qa_committee_orchestrator.ts`
* **Description** : Gate PQS (≥ 88 → signature+déblocage Parcours B ; < 88 → rejet + rapport APEX-AGENT) + écriture tables QA.
* **Critère** : PQS ≥ 88 → signature ; < 88 → remédiation ; traçabilité dans 3 tables.
