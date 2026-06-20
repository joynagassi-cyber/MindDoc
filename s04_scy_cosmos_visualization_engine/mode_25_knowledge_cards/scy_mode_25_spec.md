# 🔴 SCY-MODE-25 — KNOWLEDGE CARDS (SPEC)
**ID** : S04_COSMOS_MODE_25_SPEC · **Mode** : M25 — Le Dashboard Spatial Interactif 🔴 CRITIQUE · **Moteur** : `@xyflow/react` v12 (D-RENDER-001) · **UX** : D-UX-MODES-017

## 1. Purpose
Le **MODE 25** est le **mode d'apprentissage et d'action sémantique ultime** : cartes React riches éditables (jauge SMI, stabilité FSRS, mini-radar, actions rapides) connectées par des pipelines animés (particules lumineuses CSS `offset-path` ∝ similarité). L'apprenant manipule, édite et fait interagir ses concepts dans un tableau de bord spatial.

## 2. Requirements (RFC 2119)
### Rendu cartes riches
- **GIVEN** Les concepts et relations du sous-graphe d'étude actif.
- **WHEN** M25 activé.
- **THEN** le système SHALL rendre les cartes verticales riches (custom nodes React Flow) : jauge SMI, stabilité FSRS, mini-radar, boutons d'action rapide.
- **AND** les arêtes sont des pipelines SVG directionnels avec particules lumineuses (vitesse ∝ similarité sémantique).
- **AND** squelettes Shimmer localisés pendant le chargement (D-MODES-006) pour éliminer le clignotement.
- **AND** MiniMap de navigation GPS en bas-droite (D-UX-013).
### Clic / actions
- **WHEN** focus sur une carte.
- **THEN** le système SHALL permettre l'édition des notes, le lancement d'un Teach-Back, la révision de la flashcard, l'ouverture du document d'origine dans la Reader Suite.
### Résilience
- **GIVEN** Positionnement géométrique des cartes.
- **THEN** layout asynchrone `elkjs` en Web Worker (D-UX-014, 0$ serveur).

## 3. Boundaries
- 🚫 Couleurs hors tokens `design.md`. 🚫 Thread bloqué (elkjs Worker obligatoire).

## 4. Tests
- **TC1** : Cartes riches (jauge SMI, FSRS, mini-radar, actions) + pipelines animés particules.
- **TC2** : Squelettes Shimmer pendant le chargement (pas de clignotement).
- **TC3** : MiniMap GPS bas-droite.
- **TC4** : Focus carte → édition/Teach-Back/flashcard/Reader Suite.
- **TC5** : Layout elkjs Web Worker ; palette `design.md`.
