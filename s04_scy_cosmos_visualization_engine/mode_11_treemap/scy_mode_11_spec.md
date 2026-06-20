# 🟢 SCY-MODE-11 — TREEMAP CONCEPTUEL (SPEC)
**ID** : S04_COSMOS_MODE_11_SPEC · **Mode** : M11 — L'Allocation de Connaissance · **Moteur** : `@antv/g2` v5 (D-RENDER-006) · **UX** : D-UX-MODES-003

## 1. Purpose
Le **MODE 11** compare la masse documentaire et la maîtrise des sous-domaines via des **rectangles imbriqués** (Squarify). Surface ∝ nombre de cartes APEX, couleur ∝ Stabilité FSRS (Rouge→Vert).

## 2. Requirements (RFC 2119)
### Rendu treemap
- **GIVEN** Un JSON d'arborescence multiniveau Graphology.
- **WHEN** M11 activé.
- **THEN** le système SHALL le rendre via G2 v5 (partition Squarify WASM, surface ∝ volume cartes).
- **AND** couleur rectangle ∝ Stabilité FSRS globale du sous-domaine (Rouge→Vert).
### Clic drill-down & labels
- **WHEN** double-clic sur un rectangle.
- **THEN** le système SHALL l'agrandir pour occuper le viewport, révélant ses sous-rectangles.
- **AND** labels clamping : label affiché uniquement si largeur/hauteur ≥ police min 10px, sinon icône `?`.
### Résilience & FSRS
- **GIVEN** Resize fenêtre.
- **THEN** recalcul Squarify debounce 150ms.
- FSRS : compare le temps passé vs stabilité FSRS (identifier points de friction).

## 3. Boundaries
- 🚫 Couleurs hors tokens `design.md`. 🚫 G2 dans bundle initial.

## 4. Tests
- **TC1** : Rectangles rendus (surface ∝ volume, couleur ∝ FSRS).
- **TC2** : Double-clic → drill-down révèle sous-rectangles.
- **TC3** : Labels clamping (police min 10px, sinon `?`).
- **TC4** : Resize → recalcul debounce 150ms.
- **TC5** : Palette `design.md`.
