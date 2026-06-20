# 🪞 SCY-MIROIR-COGNITIF — SPÉCIFICATION (SPEC)
**ID** : S05_MIROIR_COGNITIF_SPEC · **Phase** : V2 · **Réf** : PRD §7.5.4

## 1. Purpose
Le **Miroir Cognitif** pose une question métacognitive adaptée au SMI **après chaque session APEX** (1 question/session). C'est la dimension « Mirror » du SMI (calibration Dunning-Kruger).

## 2. Les 3 Modes selon SMI

| Mode | SMI | Question | Scoring |
|------|-----|----------|---------|
| **Mode 1** | < 40% | « En une phrase, qu'avez-vous principalement retenu aujourd'hui ? » | Longueur + mots-clés (1-5) |
| **Mode 2** | 40-70% | « Expliquez [concept] comme si vous l'enseigniez à un enfant de 10 ans » | BRAIN évalue cohérence/complétude/clarté (1-5) |
| **Mode 3** | > 70% | « Où pourriez-vous concrètement utiliser [concept] dans votre travail demain ? » | Spécificité + faisabilité |

## 3. Features communes
- Scoring confiance auto-déclaré (1-5)
- Détection sur-confiance Dunning-Kruger (confiance >> performance → alerte)
- Stockage historique (`scy_apex_mirror_responses`)
- Dimension Metacognition SMI calculée (100 - |confiance - performance| × 2)

## 4. Tests
- TC1 : SMI<40 → Mode 1 (restitution). | TC2 : SMI 40-70 → Mode 2 (BRAIN évalue). | TC3 : SMI>70 → Mode 3 (application). | TC4 : Dunning-Kruger détecté si confiance >> performance.
