# 🧪 SCY-TIKTOK-CORE — CONTRAT DE VALIDATION & TESTS (TESTS)
**ID Spécification** : S01_INGESTION_TIKTOK_TESTS  
**Statut** : 🟢 SUITE DE TESTS DE SÛRETÉ PRÊTE POUR INTÉGRATION  

---

## 1. Scénarios de Validation Unitaires

### 🧪 Test 10.1 : Scraping & Métadonnées (Happy Path)
* **Pré-conditions** : Scrapling opérationnel, réseau accessible.
* **Input** : URL d'une vidéo TikTok publique valide avec parole.
* **Règle d'Exécution** : Appeler `ingestTiktok(url)`.
* **Post-conditions (Attendu)** :
  - Les métadonnées (auteur, description, hashtags) sont extraites.
  - Le Markdown combine transcription temporelle + description + hashtags.
  - Le score DRACO est ≥ 80/100.

### 🧪 Test 10.2 : Vidéo Musicale (Sans Parole)
* **Pré-conditions** : Whisper-tiny opérationnel.
* **Input** : URL d'une vidéo TikTok avec audio purement musical.
* **Règle d'Exécution** : Appeler `ingestTiktok(url)`.
* **Post-conditions (Attendu)** :
  - Aucune transcription incohérente n'est produite.
  - Le document est annoté `audio_non_parlé`.
  - Seules les métadonnées (description, hashtags, musique) sont ingérées.

### 🧪 Test 10.3 : Dé-duplication par Cache
* **Pré-conditions** : Une vidéo (`video_id: 701234`) déjà indexée dans `mfg_shared_content_cache`.
* **Input** : Nouvelle demande d'ingestion de la même vidéo.
* **Règle d'Exécution** : Appeler `ingestTiktok(url)`.
* **Post-conditions (Attendu)** :
  - La vidéo est sautée sans scraping ni transcription.
  - Zéro coût réseau et CPU.

### 🧪 Test 10.4 : Gestion du Blocage Anti-Bot
* **Pré-conditions** : Scrapling confronté à un blocage répété (CAPTCHA/rate limit simulé).
* **Input** : URL bloquée.
* **Règle d'Exécution** : Appeler `ingestTiktok(url)`.
* **Post-conditions (Attendu)** :
  - Le backoff exponentiel est appliqué sur les premières tentatives.
  - Après N échecs, le code `TIKTOK_BLOCKED` est renvoyé.
  - Aucune boucle infinie.

### 🧪 Test 10.5 : Vidéo Indisponible
* **Pré-conditions** : Scrapling opérationnel.
* **Input** : URL d'une vidéo supprimée ou privée.
* **Règle d'Exécution** : Appeler `ingestTiktok(url)`.
* **Post-conditions (Attendu)** :
  - Le système renvoie le code `TIKTOK_UNAVAILABLE`.
  - Aucune exception non gérée n'est levée.
