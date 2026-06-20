# 🟢 SCY-MODE-21 — HIERARCHICAL EDGE BUNDLING (SPEC)
**ID** : S04_COSMOS_MODE_21_SPEC · **Mode** : M21 — Le Désencombrement · **Moteur** : `d3` v7 (D-RENDER-008) · **UX** : D-UX-MODES-013

## 1. Purpose
Le **MODE 21** désencombre visuellement les **graphes de dépendance denses** en canalissant les arêtes le long de leur ontologie commune. Indispensable pour ontologies complexes (médecine, aéronautique).

## 2. Requirements (RFC 2119)
### Rendu faisceaux
- **GIVEN** Un graphe de dépendances denses + hiérarchie sous-jacente.
- **WHEN** M21 activé.
- **THEN** le système SHALL disposer les nœuds circulairement en périphérie et tracer les faisceaux (Bézier tendues) via d3.
- **AND** tension réglable par slider (0 = lignes droites, 1 = canaux fusionnés).
### Survol
- **WHEN** survol d'un nœud.
- **THEN** le système SHALL révéler tout le faisceau reliant le concept à ses cibles (allumage progressif).
### Résilience
- **GIVEN** > 2 000 nœuds périphériques.
- **THEN** bascule sur regroupement de branches (LOD 1).

## 3. Boundaries
- 🚫 Couleurs hors tokens `design.md`. 🚫 d3 dans bundle initial.

## 4. Tests
- **TC1** : Nœuds circulaires + faisceaux Bézier + slider tension.
- **TC2** : Survol → révélation du faisceau (allumage progressif).
- **TC3** : > 2 000 nœuds → regroupement de branches (LOD 1).
- **TC4** : d3 lazy ; palette `design.md`.
