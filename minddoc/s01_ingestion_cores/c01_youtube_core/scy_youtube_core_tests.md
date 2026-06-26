# 🧪 SCY-YOUTUBE-CORE — CONTRAT DE VALIDATION & TESTS (TESTS)
**ID Spécification** : S01_INGESTION_YOUTUBE_TESTS  
**Statut** : 🟢 SUITE DE TESTS DE SÛRETÉ PRÊTE POUR INTÉGRATION  

---

## 1. Scénarios de Validation Unitaires

### 🧪 Test 1.1 : Validation d'URL et de Métadonnées
* **Pré-conditions** : Le serveur dispose de la connectivité réseau et de `yt-dlp` installé.
* **Input** : `https://www.youtube.com/watch?v=dQw4w9WgXcQ`
* **Règle d'Exécution** : Appeler `extractYoutubeMetadata()`.
* **Post-conditions (Attendu)** :
  - Le titre extrait doit être une chaîne non vide.
  - Le format de la date de publication doit être au format ISO standard.
  - Aucune exception n'est jetée.

### 🧪 Test 1.2 : Résilience face à l'Absence de Sous-Titres (Whisper Activation)
* **Pré-conditions** : Soumission d'une vidéo YouTube connue pour ne posséder aucun sous-titre officiel ni auto-généré.
* **Input** : `https://www.youtube.com/watch?v=[ID_SANS_SOUS_TITRES]`
* **Règle d'Exécution** : Appeler le wrapper de traitement principal du Core d'Ingestion.
* **Post-conditions (Attendu)** :
  - Le système intercepte l'erreur d'extraction de transcription YouTube.
  - Le système bascule automatiquement sur la tâche de téléchargement de l'audio MP3.
  - Le sidecar Whisper-tiny s'active et génère une transcription correcte avec horodatages.
  - Le statut final de l'arête sémantique dans COSMOS-MINDGRAPH passe à `completed`.

### 🧪 Test 1.3 : Dé-duplication de Playlist (Zero-Bleeding Cost)
* **Pré-conditions** : Une vidéo de la playlist (ID: `vid-abc`) est déjà indexée dans `mfg_shared_content_cache`.
* **Input** : Une Playlist contenant 3 vidéos, dont `vid-abc`.
* **Règle d'Exécution** : Appeler le parseur de Playlist.
* **Post-conditions (Attendu)** :
  - Le système crée des tâches de générations uniquement pour les 2 nouvelles vidéos.
  - La vidéo `vid-abc` est sautée de manière transparente, évitant de re-calculer les embeddings ou de dépenser des jetons d'APIs.
