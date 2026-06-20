# 🟢 SCY-MODE-09 — CONCEPT MAP (SPEC)
**ID** : S04_COSMOS_MODE_09_SPEC · **Mode** : M9 — La Cartographie Sémantique · **Moteur** : `@antv/g6` v5 (D-RENDER-001) · **UX** : D-UX-MODES-001

## 1. Purpose
Le **MODE 9** cartographie les **propositions logiques de connaissances** : graphe non-arborescent (multi-parents) avec **arêtes étiquetées** par verbe sémantique (« utilise », « contredit », « est requis pour »). Mode par excellence de l'assimilation conceptuelle profonde.

## 2. Requirements (RFC 2119)
### Rendu graphe sémantique
- **GIVEN** Un graphe Graphology non-arborescent avec relations étiquetées.
- **WHEN** M9 activé.
- **THEN** le système SHALL le rendre via G6 v5 Canvas (ForceAtlas2 asynchrone D-PERF-003).
- **AND** nœuds cercles légers (couleur ∝ domaine, taille ∝ PageRank), arêtes directionnelles avec rectangle texte semi-transparent centré affichant la relation.
- **AND** badge indiquant si la liaison est validée user ou auto-générée IA.
### Clic nœud & arête
- **WHEN** clic arête.
- **THEN** le système SHALL mettre en valeur les 2 concepts connectés, assombrir le reste (opacity 0.15) et afficher un panneau d'explication de validité du lien.
- **AND** raccourci `E` ouvre l'éditeur du verbe de relation.
### Résilience
- **GIVEN** Densité trop élevée (clutter).
- **THEN** le plugin Fisheye (D-UX-001) s'active au déplacement souris pour agrandir localement.
### FSRS
- Formuler explicitement des propositions → modèle mental structuré.

## 3. Boundaries
- 🚫 Couleurs hors tokens `design.md`. ⚠️ Validation petgraph.

## 4. Tests
- **TC1** : Graphe rendu (arêtes étiquetées rectangle semi-transparent, badge validé/auto).
- **TC2** : Clic arête → mise en valeur + assombrissement + panneau validité.
- **TC3** : `E` → éditeur verbe relation.
- **TC4** : Clutter → Fisheye activé.
- **TC5** : Palette `design.md`.
