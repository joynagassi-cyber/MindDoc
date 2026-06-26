# 🧪 SCY-AG15-AXIOMATIZER — CONTRAT DE VALIDATION & TESTS (TESTS)
**ID Spécification** : S03_ASCENT_AG15_AXIOMATIZER_TESTS  
**Statut** : 🟢 SUITE DE TESTS DE SÛRETÉ PRÊTE POUR INTÉGRATION  

---

### 🧪 Test 15.1 : Distillation (Happy Path)
* **Pré-conditions** : ≥ 100 traces convergentes dans `scy_procedural_traces`.
* **Règle d'Exécution** : Lancer la tâche de distillation.
* **Post-conditions (Attendu)** : 1 axiome formulé, validé par `AxiomSchema`.

### 🧪 Test 15.2 : Seuil k Non Atteint
* **Pré-conditions** : k < seuil (ex : 30 traces).
* **Attendu** : Aucun axiome généré (attente accumulation).

### 🧪 Test 15.3 : Filtre PII
* **Input** : Axiome candidat contenant un identifiant personnel.
* **Attendu** : Identifiant strippé avant persistance ; si non strippable → rejet (GDPR).

### 🧪 Test 15.4 : k-Anonymat
* **Attendu** : Les profils contributeurs sont masqués (k ≥ 10).

### 🧪 Test 15.5 : Purge des Micro-Règles
* **Pré-conditions** : Axiome consolidé.
* **Attendu** : Les micro-règles/traces d'origine sont supprimées.

### 🧪 Test 15.6 : Partage Global Invisible
* **Attendu** : L'axiome est accessible à tous les utilisateurs sans action de leur part.

### 🧪 Test 15.7 : Non-Blocage UX (Async)
* **Attendu** : La distillation en arrière-plan ne bloque pas l'expérience utilisateur.

### 🧪 Test 15.8 : Aucune Invention
* **Attendu** : Sans trace convergente, aucun axiome n'est inventé.
