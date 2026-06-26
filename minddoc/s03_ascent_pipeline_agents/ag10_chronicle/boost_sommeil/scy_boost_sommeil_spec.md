# 😴 SCY-BOOST-SOMMEIL (CHRONICLE) — SPÉCIFICATION (SPEC)
**ID** : S03_CHRONICLE_BOOST_SOMMEIL_SPEC · **Décision** : D-OPT-048 · **Réf** : PRD §15.9

## 1. Purpose
Le **Boost Sommeil** (D-OPT-048) : CHRONICLE (Agent-10) planifie automatiquement une **micro-révision de 2 minutes** des concepts complexes juste avant le coucher de l'utilisateur pour cibler ces synapses pour la **consolidation hippocampale nocturne**.

## 2. Requirements (RFC 2119)
- **GIVEN** L'heure de coucher détectée/déclarée de l'utilisateur (chronotype CHRONICLE).
- **WHEN** L'heure approche (ex : 30 min avant coucher).
- **THEN** le système SHALL proposer une micro-révision de 2 min des concepts complexes les moins stables.
- **AND** le système SHALL cibler la consolidation hippocampale nocturne (sommeil).

## 3. Tests
- TC1 : Micro-révision 2 min proposée avant coucher. | TC2 : Cible les concepts complexes les moins stables (FSRS).
