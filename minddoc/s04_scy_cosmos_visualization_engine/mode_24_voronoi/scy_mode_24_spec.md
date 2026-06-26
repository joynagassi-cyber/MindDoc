# 🟢 SCY-MODE-24 — VORONOI CONCEPT MAP (SPEC)
**ID** : S04_COSMOS_MODE_24_SPEC · **Mode** : M24 — La Vue Territoriale · **Moteur** : `d3` v7 + `d3-delaunay` (D-RENDER-008) · **UX** : D-UX-MODES-016

## 1. Purpose
Le **MODE 24** structure **territorialement** l'espace de connaissances (cellules polygonales ∝ importance du concept) pour une assimilation visuelle-spatiale. L'apprenant conquiert ses « territoires de compétences » (polygones rouges → verts).

## 2. Requirements (RFC 2119)
### Rendu Voronoi
- **GIVEN** Des coordonnées cartésiennes des concepts.
- **WHEN** M24 activé.
- **THEN** le système SHALL générer les cellules Voronoi via `d3-delaunay` (taille ∝ importance, couleur ∝ SMI Rouge→Vert).
### Clic
- **WHEN** clic sur un territoire.
- **THEN** le système SHALL déployer les sous-concepts internes du polygone de manière fluide.
### Résilience
- **GIVEN** Cellules de bordure.
- **THEN** coordonnées clampées aux dimensions du viewport (pas d'extension à l'infini).

## 3. Boundaries
- 🚫 Couleurs hors tokens `design.md`. 🚫 d3 dans bundle initial.

## 4. Tests
- **TC1** : Cellules Voronoi (∝ importance, couleur ∝ SMI).
- **TC2** : Clic territoire → sous-concepts internes déployés.
- **TC3** : Cellules bordure → clampées au viewport.
- **TC4** : d3 lazy ; palette `design.md`.
