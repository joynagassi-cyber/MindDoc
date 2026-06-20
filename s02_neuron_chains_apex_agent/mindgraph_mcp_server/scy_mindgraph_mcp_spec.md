# 🔌 SCY-MINDGRAPH-MCP-SERVER — SPÉCIFICATION (SPEC)
**ID** : S02_MINDGRAPH_MCP_SPEC · **Décision** : D-OPT-043 · **Phase** : P1 · **Réf** : PRD §15.9

## 1. Purpose
Le **MindGraph MCP Server** (D-OPT-043) est un serveur **Model Context Protocol (MCP)** local co-localisé au monolithe. Il permet aux agents d'interroger COSMOS-MINDGRAPH via un outil unique `query-mindgraph` s'appuyant sur des **requêtes SQL récursives CTE** hyper-rapides, réduisant de **4.5× la consommation de tokens**.

## 2. Requirements (RFC 2119)
- **GIVEN** Un agent nécessitant le contexte du graphe COSMOS.
- **WHEN** Il interroge le MindGraph MCP Server.
- **THEN** le système SHALL exécuter une requête SQL récursive CTE sur COSMOS-MINDGRAPH.
- **AND** le système SHALL réduire la consommation de tokens de 4.5× (vs récupération brute).

## 3. Tests
- TC1 : `query-mindgraph` retourne le sous-graphe pertinent. | TC2 : Réduction tokens 4.5×. | TC3 : SQL CTE récursive exécutée (< 100ms).
