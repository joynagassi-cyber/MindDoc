# 🎯 SCY-REVEAL-BY-RELEVANCE — SPÉCIFICATION (SPEC)
**ID** : S04_COSMOS_REVEAL_BY_RELEVANCE_SPEC · **Décision** : D-OPP-005 · **Phase** : P2 · **Réf** : arch v4.5 D-OPP-005

## 1. Purpose
Le **Reveal by Relevance** (D-OPP-005) filtre dynamiquement l'affichage COSMOS pour masquer le superflu et recentrer l'attention exclusive sur les **150 concepts les plus pertinents** par rapport au contexte de l'étudiant (requêtes récentes, cours actif). Toggle « 🎯 Vue Pertinente », recalcul max 1×/30s.

## 2. Requirements (RFC 2119)
- **GIVEN** Le graphe COSMOS complet + le contexte étudiant (cours actif, requêtes récentes).
- **WHEN** L'utilisateur active « 🎯 Vue Pertinente ».
- **THEN** le système SHALL masquer les concepts non pertinents.
- **AND** le système SHALL afficher uniquement les 150 concepts les plus pertinents (cosine + contexte).
- **AND** le recalcul SHALL être limité à 1×/30s (performance).

## 3. Tests
- TC1 : Toggle → 150 concepts pertinents affichés. | TC2 : Recalcul ≤ 1×/30s. | TC3 : Pertinence basée sur contexte (cours actif + requêtes).
