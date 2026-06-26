# 🔍 SCY-GAP-DETECTION-VISUEL — SPÉCIFICATION (SPEC)
**ID** : S04_COSMOS_GAP_DETECTION_SPEC · **Phase** : P0-P1 · **Réf** : PRD §7.4.3bis A, §7.4.7 A

## 1. Purpose
La **Gap Detection Visuel** identifie les **concepts prérequis manquants** dans la base de l'utilisateur et les affiche en **pointillés rouges** dans COSMOS. CTA « Combler cette lacune » → déclenche Agent-02 CONTENT-SCOUT. Feature killer différenciante, coût nul (petgraph Rust).

## 2. Requirements (RFC 2119)
- **GIVEN** Le graphe COSMOS (arêtes `prerequisite_of`).
- **WHEN** Le système traverse le graphe (petgraph, $0).
- **THEN** le système SHALL identifier les nœuds prérequis absents de la base utilisateur.
- **AND** le système SHALL les afficher en **pointillés rouges** dans COSMOS.
- **AND** le système SHALL offrir un CTA « Combler cette lacune » → Agent-02 (ingestion source + NEURON-CHAINS).
- **AND** le système SHALL intégrer les gaps dans la décision de remédiation de l'Agent-06.

## 3. Tests
- TC1 : Prérequis manquant → pointillés rouges COSMOS. | TC2 : CTA « Combler » → Agent-02 ingestion. | TC3 : Agent-06 intègre les gaps.
