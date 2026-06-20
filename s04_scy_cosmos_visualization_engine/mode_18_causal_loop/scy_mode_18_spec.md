# 🟢 SCY-MODE-18 — CAUSAL LOOP DIAGRAM (SPEC)
**ID** : S04_COSMOS_MODE_18_SPEC · **Mode** : M18 — La Vue Dynamique · **Moteur** : `@antv/g6` v5 (D-RENDER-001) · **UX** : D-UX-MODES-010

## 1. Purpose
Le **MODE 18** modélise et **simule l'influence et les rétroactions** d'un système dynamique via des boucles causales polarisées (+/-). Remplace l'apprentissage par cœur par la modélisation de boucles.

## 2. Requirements (RFC 2119)
### Rendu causal
- **GIVEN** Un graphe Graphology orienté de relations causales polarisées.
- **WHEN** M18 activé.
- **THEN** le système SHALL le rendre via G6 v5 (nœuds variables, arêtes courbes : polarité `+` verte / `-` rouge).
- **AND** badges centraux : `R` (boucle renforcement, cercle vert rotatif) / `B` (équilibre, balance rouge).
### Simulation
- **WHEN** clic sur une variable + slider d'augmentation.
- **THEN** le système SHALL propager l'impulsion à 60 FPS le long des boucles.
### Résilience
- **GIVEN** Schéma avant simulation.
- **THEN** validation petgraph : aucune contradiction logique avant propagation.

## 3. Boundaries
- 🚫 Couleurs hors tokens `design.md`. ⚠️ Validation petgraph avant simulation.

## 4. Tests
- **TC1** : Nœuds variables + arêtes + (vert) / - (rouge) + badges R/B.
- **TC2** : Slider variable → propagation 60 FPS.
- **TC3** : Contradiction logique → bloquée avant simulation.
- **TC4** : Palette `design.md`.
