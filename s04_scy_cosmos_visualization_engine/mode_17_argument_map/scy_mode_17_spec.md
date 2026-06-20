# 🟢 SCY-MODE-17 — ARGUMENT MAP (SPEC)
**ID** : S04_COSMOS_MODE_17_SPEC · **Mode** : M17 — La Vue Dialectique · **Moteur** : `@xyflow/react` v12 (D-RENDER-001/004) · **UX** : D-UX-MODES-009

## 1. Purpose
Le **MODE 17** structure visuellement les **débats, argumentations juridiques et raisonnements logiques** : thèse centrale (bleu), supports (vert, flèche → thèse), réfutations (rouge, flèche diamant). Indispensable pour droit, philosophie, analyse de crises.

## 2. Requirements (RFC 2119)
### Rendu argumentaire
- **GIVEN** Un graphe de propositions logiques structuré.
- **WHEN** M17 activé.
- **THEN** le système SHALL le rendre via React Flow (thèse bleu, supports vert `supports`, réfutations rouge `refutes`, flèches d'implications).
- **AND** badge source documentaire de l'argument.
### Interactions
- **WHEN** double-clic sur le texte d'un argument.
- **THEN** le système SHALL permettre l'édition en direct.
- **AND** bouton `+` flottant → créer sous-argument/contre-argument.
### Résilience
- **GIVEN** Positionnement des arguments.
- **THEN** layout asynchrone `elkjs` en Web Worker (D-UX-014).

## 3. Boundaries
- 🚫 Couleurs hors tokens `design.md`. 🚫 Max 1000 nœuds (D-RENDER-004).

## 4. Tests
- **TC1** : Thèse bleu + supports vert + réfutations rouge + flèches.
- **TC2** : Double-clic → édition texte argument.
- **TC3** : Bouton `+` → sous-argument/contre-argument.
- **TC4** : Layout elkjs Web Worker (thread non bloqué).
- **TC5** : Palette `design.md`.
