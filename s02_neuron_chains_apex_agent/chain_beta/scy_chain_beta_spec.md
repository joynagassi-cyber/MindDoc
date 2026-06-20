# 🟢 SCY-CHAIN-BETA — STRUCTURATION (SPEC)
**ID** : S02_NEURON_CHAIN_BETA_SPEC · **Phase** : MVP

## 1. Purpose
La chaîne **BETA** structure les chunks en graphe de connaissances : classification en 10 domaines, extraction des relations typées (is_a, part_of, prerequisite_of, example_of, contradicts), et construction de la hiérarchie conceptuelle avec scoring PageRank.

## 2. Agents
* **BETA-1 Taxonomiste** : classification en 10 domaines (CS, Math, Physics, Bio, Business, Arts, Law, Medicine, History, Spiritual).
* **BETA-2 Extracteur Relations** : relations typées (is_a, part_of, prerequisite_of, example_of, contradicts).
* **BETA-3 Architecte Hiérarchie** : arbre concepts + scoring PageRank (graphology-metrics, $0).

## 3. Requirements (RFC 2119)
### Taxonomie & Relations
- **GIVEN** Les chunks/résumés d'ALPHA.
- **THEN** BETA-1 SHALL classifier en 10 domaines ; BETA-2 SHALL extraire les relations typées.
### Hiérarchie & PageRank
- **AND** BETA-3 SHALL construire l'arbre conceptuel + calculer PageRank (`graphology-metrics`, $0).
- **AND** le système SHALL peupler COSMOS Knowledge Graph (`scy_concepts`, `scy_concept_relations`).

## 4. Boundaries
🚫 Relations inventées sans source. ⚠️ Validation Zod + petgraph.

## 5. Tests
- **TC1** : Chunks → classification 10 domaines correcte.
- **TC2** : Relations typées extraites (is_a, prerequisite_of, etc.).
- **TC3** : PageRank calculé ; COSMOS KG peuplé.
