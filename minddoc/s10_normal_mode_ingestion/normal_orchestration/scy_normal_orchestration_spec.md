# 🔄 SCY-NORMAL-ORCHESTRATION — SPÉCIFICATION (SPEC)
**ID** : S10_NORMAL_MODE_ORCHESTRATION_SPEC · **Phase** : Phase 0-1 · **Réf** : PRD §7.13.10

> **📌 RÉFÉRENCE CROISÉE** : Les specs d'orchestration sont dans **`../scy_normal_mode_orchestration_specs.md`** (22KB). Ce fichier en est le résumé SDD.

## 1. Purpose
Le **Mode Normal d'ingestion** est le pipeline d'ingestion hors ASCENT : l'utilisateur ingère une source directement, et COSMOS, les cartes FSRS (APEX), et BRAIN s'activent **immédiatement à 0$ de temps d'attente** dès la fin de l'ingestion brute. Sûreté : filtre d'intégrité minimal (anti-NaN, anti-division par zéro `softening_epsilon`) en arrière-plan sans bloquer l'étudiant. **Zéro certificat délivré** (Parcours A).

## 2. Requirements (RFC 2119)
- **GIVEN** Une source ingérée en Mode Normal (hors ASCENT).
- **THEN** le système SHALL activer COSMOS + APEX (cartes FSRS) + BRAIN **immédiatement** après ingestion (0$ attente).
- **AND** le système SHALL appliquer un filtre d'intégrité minimal (anti-NaN, anti-÷0 `softening_epsilon`) en arrière-plan sans bloquer.
- **AND** le système SHALL NE PAS délivrer de certificat (Parcours A, préservation de l'élite des diplômes).

## 3. Tests
- TC1 : Source → COSMOS/APEX/BRAIN activés immédiatement (0$ attente). | TC2 : Filtre anti-NaN/anti-÷0 actif sans blocage. | TC3 : Zéro certificat délivré (Parcours A).
