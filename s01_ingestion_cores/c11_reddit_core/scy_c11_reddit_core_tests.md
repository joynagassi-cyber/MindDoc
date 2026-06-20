# 🧪 SCY-REDDIT-CORE — CONTRAT DE VALIDATION & TESTS (TESTS)
**ID Spécification** : S01_INGESTION_REDDIT_TESTS  
**Statut** : 🟢 SUITE DE TESTS DE SÛRETÉ PRÊTE POUR INTÉGRATION  

---

## 1. Scénarios de Validation Unitaires

### 🧪 Test 11.1 : Post + Arbre de Commentaires (Happy Path)
* **Pré-conditions** : Client Roux OAuth opérationnel.
* **Input** : URL d'un post Reddit public avec commentaires.
* **Règle d'Exécution** : Appeler `ingestRedditPost(url)`.
* **Post-conditions (Attendu)** :
  - Les métadonnées du post (titre, auteur, corps, score) sont extraites.
  - L'arbre de commentaires est structuré en Markdown imbriqué.
  - Le score DRACO est ≥ 80/100.

### 🧪 Test 11.2 : Hiérarchie Parent-Enfant
* **Pré-conditions** : Post avec commentaires imbriqués sur plusieurs niveaux.
* **Input** : URL du post.
* **Règle d'Exécution** : Appeler `reconstructCommentTree(post_id)`.
* **Post-conditions (Attendu)** :
  - La hiérarchie est respectée (indentation par profondeur).
  - Chaque commentaire inclut son auteur et son score.

### 🧪 Test 11.3 : Filtrage par Score & Marquage
* **Pré-conditions** : Post avec commentaires de scores variés et certains supprimés.
* **Input** : Post + seuil de score minimum (ex : 5).
* **Règle d'Exécution** : Appeler `reconstructCommentTree` avec filtrage.
* **Post-conditions (Attendu)** :
  - Les commentaires sous le seuil sont éliminés.
  - Les commentaires supprimés sont marqués `[deleted]` / `[removed]`.

### 🧪 Test 11.4 : Dé-duplication par Cache
* **Pré-conditions** : Un post (`post_id: abc123`) déjà indexé dans `mfg_shared_content_cache` sans changement.
* **Input** : Nouvelle demande d'ingestion du post `abc123`.
* **Règle d'Exécution** : Appeler `ingestRedditPost(url)`.
* **Post-conditions (Attendu)** :
  - Le post est sauté sans appel API.
  - Zéro coût réseau.

### 🧪 Test 11.5 : Post Introuvable
* **Pré-conditions** : Client Roux opérationnel.
* **Input** : URL d'un post supprimé ou inexistant.
* **Règle d'Exécution** : Appeler `ingestRedditPost(url)`.
* **Post-conditions (Attendu)** :
  - Le système renvoie le code `REDDIT_POST_NOT_FOUND`.
  - Aucune exception non gérée n'est levée.

### 🧪 Test 11.6 : Respect du Rate Limiting
* **Pré-conditions** : Client Roux configuré.
* **Input** : Série de requêtes approchant 60/min.
* **Règle d'Exécution** : Appeler `batchIngest(...)`.
* **Post-conditions (Attendu)** :
  - Aucune fenêtre d'1 minute ne dépasse 60 requêtes.
  - Le `User-Agent` est présent sur chaque appel.
