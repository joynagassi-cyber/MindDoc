# 🟢 SCY-MODE-16 — HEATMAP MATRICIELLE (SPEC)
**ID** : S04_COSMOS_MODE_16_SPEC · **Mode** : M16 — La Matrice de Corrélation · **Moteur** : `nivo` Heatmap (D-RENDER-007) · **UX** : D-UX-MODES-008

## 1. Purpose
Le **MODE 16** identifie d'un coup d'œil les **redondances sémantiques** ou les fusions de concepts à réaliser via une matrice de similarité cosinus (intensité blanc→violet foncé).

## 2. Requirements (RFC 2119)
### Rendu heatmap
- **GIVEN** Une matrice symétrique d'affinité sémantique (cosinus).
- **WHEN** M16 activé.
- **THEN** le système SHALL la rendre via `@nivo/heatmap` (intensité blanc→violet ∝ similarité).
- **AND** un `⚠️` s'affiche si similarité > 0.95 (redondance/fusion).
### Clic
- **WHEN** clic sur une cellule.
- **THEN** le système SHALL ouvrir un volet proposant de fusionner les 2 concepts redondants ou d'établir un lien croisé.
### Résilience & confidentialité
- **GIVEN** Heatmap partagée en communauté (Marketplace).
- **THEN** filtre k-anonymity masque les cellules si < 100 profils contributeurs (D-SEC-003).

## 3. Boundaries
- 🚫 Couleurs hors tokens `design.md`. 🚫 nivo dans bundle initial.

## 4. Tests
- **TC1** : Heatmap rendue (intensité blanc→violet ∝ similarité).
- **TC2** : Similarité > 0.95 → `⚠️` de redondance.
- **TC3** : Clic cellule → volet fusion/lien.
- **TC4** : Marketplace → k-anonymity si < 100 profils.
- **TC5** : nivo lazy ; palette `design.md`.
