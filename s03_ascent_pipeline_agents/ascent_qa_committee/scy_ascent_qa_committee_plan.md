# 🛠️ SCY-ASCENT-QA-COMMITTEE — PLAN D'IMPLÉMENTATION (PLAN)
**ID** : S03_ASCENT_QA_COMMITTEE_PLAN · **Décision** : D-OPT-032  
**Spécification de référence** : `scy_ascent_qa_pedagogical_pipeline_specs.md` (SPEC-SCY-ASCENT-QA-PIPELINE-V1.0)

## Flux
```
[Matière brute ingérée + parsée (Docling)] → Étape 1 : Construire syllabus + cours (Rust)
   │
   ▼
[Étape 2 : SAGA de portail ASCENT-QA (6 audits asynchrones)]
   ├── QA-01 Curriculum Designer (progression DAG)
   ├── QA-02 SME (exactitude technique)
   ├── QA-03 Alignment Orchestrator (couverture syllabus → AGENT-02 si manquant)
   ├── QA-04 Cognitive Load Guarantor (densité, 1 idée=1 bloc, ELI5)
   ├── QA-05 Content Validator (clarté, markdown, LaTeX, calcul PQS)
   └── QA-06 Academic Certifier (Constructive Alignment cours↔examen SurveyJS)
   │
   ▼
[Calcul PQS = 0.2·design + 0.3·expert + 0.3·align + 0.2·cognitive]
   │
   ├── PQS < 88 ──► Rejet Harmonist → APEX-AGENT (réécriture ciblée)
   │
   ▼ (PQS ≥ 88)
[Signature électronique + déblocage étude active (Parcours B)]
   │
   ▼
[Traçabilité : scy_course_qa_audits + scy_qa_agent_reviews + scy_constructive_alignment_checks]
```
## Dépendances : Mastra TS (SAGA), Rust, LlmRouter+BudgetGuard, SurveyJS, Zod (ConstructiveAlignmentSchema), Harmonist gate.
## Fichiers : `backend_ts/src/ascent/qa/qa_committee_orchestrator.ts`, `qa_01_curriculum.ts` → `qa_06_certifier.ts`, `scy_ascent_qa_alignment.ts`.
