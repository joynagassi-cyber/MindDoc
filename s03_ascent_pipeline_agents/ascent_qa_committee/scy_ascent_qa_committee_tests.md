# 🧪 SCY-ASCENT-QA-COMMITTEE — TESTS
**ID** : S03_ASCENT_QA_COMMITTEE_TESTS · **Décision** : D-OPT-032

- **TC1** : Le pipeline exécute les 6 audits (chacun : sous-score + critical_findings + remediation_prompt).
- **TC2** : PQS calculé = 0.2·design + 0.3·expert + 0.3·align + 0.2·cognitive.
- **TC3** : PQS ≥ 88 → signature électronique + déblocage étude active (Parcours B).
- **TC4** : PQS < 88 → rejet mécanique Harmonist + rapport remédiation APEX-AGENT.
- **TC5** : Constructive Alignment : question d'examen hors cours → aligned=false + remediation_required.
- **TC6** : Couverture syllabus : point obligatoire manquant → recherche AGENT-02 ordonnée.
- **TC7** : Cognitive Load : règle « 1 idée = 1 bloc » + ELI5 pour l'abstrait vérifiées.
- **TC8** : Traçabilité : décisions consignées dans `scy_course_qa_audits`, `scy_qa_agent_reviews`, `scy_constructive_alignment_checks`.
- **TC9** : Aucun score arbitraire (tout justifié et tracé).
