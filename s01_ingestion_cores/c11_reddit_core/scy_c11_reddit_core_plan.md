# 🛠️ SCY-REDDIT-CORE — PLAN D'IMPLÉMENTATION TECHNIQUE (PLAN)
**ID Spécification** : S01_INGESTION_REDDIT_PLAN  
**Statut** : 🟢 PLAN D'IMPLÉMENTATION IMMUABLE ET VALIDÉ  

---

## 1. Architecture du Flux de Données

```
 [URL Post Reddit / post_id / subreddit]
                 │
                 ▼
     [Résolution : parsing URL → subreddit + post_id]
                 │
                 ▼
     [Mastra TS Ingestion Worker]
                 │
                 ├──► [Vérification mfg_shared_content_cache (post_id)] ──► Hit + inchangé ──► (Fin)
                 │
                 └──► [Miss : crate Roux (OAuth secret + User-Agent)]
                             │
              ┌──────────────┴──────────────────────┐
              ▼                                     ▼
     [Subreddit.article(id)]               [Subreddit.comments(id)]
     (métadonnées du post)                 (arbre des commentaires)
     → titre, auteur, corps, score         + pagination
              │                                     │
              ▼                                     ▼
        [dom_smoothie                       [Reconstruction hiérarchique
         (corps → Markdown)]                 parent-enfant (réponses)]
              │                             + score + auteur par commentaire
              │                             + profondeur max configurable
              └──────────────┬──────────────────────┘
                             ▼
                [Filtrage optionnel par score min]
                [Marquage commentaires supprimés]
                [Score d'intégrité DRACO]
                             │
                             ▼
        [Écriture PostgreSQL mfg_project_sources]
        [Indexation vectorielle Zilliz]
        [Arête sémantique COSMOS-MINDGRAPH → status: completed]
```

---

## 2. Dépendances Techniques Strictes
* **API Reddit** via **crate Roux** :
  - OAuth : `client_id` + `client_secret` stockés dans le gestionnaire de secrets Northflank.
  - `Subreddit::new(name)` → `article(id)` / `comments(id)`.
  - Limite : 60 requêtes/min (OAuth).
* **Runtimes Rust** :
  - `roux` (client Reddit).
  - `serde_json` (parsing des Listings/Things Reddit).
  - `governor` (rate limiting préventif ≤ 60 req/min).
  - Circuit Breaker (ARC-001) sur `429` répétés.
* **Conversion** :
  - `dom_smoothie` (corps Reddit → Markdown propre).
* **Stockage** :
  - PostgreSQL (Northflank) : `mfg_project_sources`, `mfg_shared_content_cache`, `mfg_sync_queue`.

---

## 3. Fichiers et Tables Impactés
- **`backend_ts/src/normal_pipeline/cores/redditCore.ts`** : Orchestration Mastra TS (résolution URL, cache, enqueue SAGA).
- **`backend_rs/src/cores/reddit/roux_client.rs`** : Client Roux (OAuth, rate limiting, Circuit Breaker).
- **`backend_rs/src/cores/reddit/comment_tree.rs`** : Reconstruction hiérarchique de l'arbre de commentaires + pagination + filtrage.
- **`backend_rs/src/cores/reddit/identity.rs`** : Parsing d'URL Reddit → subreddit + post_id.
- **`mfg_project_sources`** : Stockage du Markdown hiérarchique (post + arbre).
- **`mfg_shared_content_cache`** : Dé-duplication par `post_id` (+ détection changement du nombre de commentaires).
- **`mfg_sync_queue`** : File d'attente asynchrone SAGA pour la reconstruction de threads massifs.
