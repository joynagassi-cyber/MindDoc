# ⚡ SCY-AG14-DET-SUGGESTER — SPÉCIFICATION (SPEC)
**ID** : S03_AG14_DET_SUGGESTER_SPEC · **Phase** : P0 (Epic 4.1) · **Réf** : PRD epics Story 4.1, sprint SQL `why_suggested`

## 1. Purpose
L'**AGENT-14 (Det-Suggester)** est l'**Agent Déterministe de Suggestions** de documents. À partir des métadonnées d'une source extraite (Docling), il renvoie en **moins de 5ms** exactement **3 codes de documents pertinents** (ex : G01, C01, W01) **sans aucun appel LLM externe**. Arbre de décision déterministe Rust pur ($0).

## 2. Tech Stack
* **Moteur** : Rust pur (arbre de décision déterministe, 0 LLM, < 5ms).
* **Entrées** : métadonnées Docling (codeblocks détectés, has_math, has_figures, domain, complexity, word_count).
* **Sortie** : 3 codes de documents (familles A-H) + `why_suggested` (explication).
* **Table** : `scy_project_deliverables.why_suggested` (sprint SQL ligne 121).

## 3. Requirements (RFC 2119)
- **GIVEN** Les métadonnées d'une source extraite par Docling.
- **WHEN** AGENT-14 est invoqué.
- **THEN** le système SHALL retourner en < 5ms exactement 3 codes de documents pertinents.
- **AND** le système SHALL NE PAS effectuer d'appel LLM externe ($0).
- **AND** le système SHALL fournir `why_suggested` (explication de la sélection).

## 4. Exemples de règles déterministes
- `has_code = true` + `domain = tech` → suggère G01 (Guide), C01 (Référence API), X03 (Défi pratique)
- `has_math = true` + `domain = academic` → suggère S01 (Synthèse), C03 (Glossaire), W02 (Critique)
- `word_count > 10000` → suggère S03 (Executive Brief), D01 (Mindmap), Y01 (Syllabus)

## 5. Tests
- TC1 : Métadonnées → 3 codes docs en < 5ms ($0 LLM). | TC2 : `why_suggested` fourni. | TC3 : Déterministe (même input → même output).
