# 🧪 SCY-ENGRAM-DECAY-VITALITY — CONTRAT DE VALIDATION & TESTS (TESTS)
**ID Spécification** : S11_ENGRAM_DECAY_VITALITY_TESTS  
**Statut** : 🟢 SUITE DE TESTS DE SÛRETÉ PRÊTE POUR INTÉGRATION  

---

## 1. Scénarios de Validation Unitaires

### 🧪 Test 1.1 : Sûreté Arithmétique de la Formule de Vitalité
* **Pré-conditions** : Accès au module de calcul Rust `engram_decay.rs`.
* **Inputs testés** :
  - $t = 0$ (interaction immédiate)
  - $t = 60$ (point d'inflexion sigmoïdal)
  - $t = 10000$ (intervalle de temps extrême simulé)
* **Règle d'Exécution** : Calculer $V_n(t)$ avec des coefficients $w_r = 0.4, w_c = 0.3, w_m = 0.3$ réglés à 100.
* **Post-conditions (Attendu)** :
  - Le score pour $t=0$ doit valoir $100.0$.
  - Le score pour $t=60$ applique une réduction d'environ $10.0$ (milieu de la courbe).
  - Le score pour $t=10000$ doit être de $0.0$ et ne jamais lever de débordement arithmétique (`NaN` ou crash).

### 🧪 Test 1.2 : Transaction de Transfert vers le Cold Engram Vault
* **Pré-conditions** : Un nœud sémantique d'un utilisateur possède un score de vitalité qui tombe à `18.5`.
* **Input** : Appel du service de synchronisation d'état.
* **Règle d'Exécution** : Exécuter la procédure de mise en dormance.
* **Post-conditions (Attendu)** :
  - Le système crée avec succès un nouvel enregistrement dans la table `scy_engram_vault`.
  - La table `scy_synaptic_vitality` ne contient plus ce nœud d'apprentissage actif pour l'utilisateur.
  - La requête d'interrogation de l'assistant de chat BRAIN ne peut plus récupérer ce nœud dans son contexte RAG par défaut.

### 🧪 Test 1.3 : Évaluation Sémantique et Seuil de Similarité de Résurrection
* **Pré-conditions** : Un nœud dormant est stocké avec les mots-clés `"mécanisme, attention, transformer"`.
* **Inputs de test** :
  - Essai 1 : `"Je ne sais plus du tout"` (similarité attendue < 0.20)
  - Essai 2 : `"C'est le mécanisme d'attention dans l'architecture Transformer"` (similarité attendue $\ge 0.75$)
* **Règle d'Exécution** : Soumettre les essais à la route `/api/neuroscience/forge/attempt`.
* **Post-conditions (Attendu)** :
  - Pour l'Essai 1, le système retourne un échec de résurrection, maintient la dormance froide et incrémente `attempts_count`.
  - Pour l'Essai 2, le système valide la résurrection, restaure le nœud actif avec une vitalité de `50.0` et nettoie `scy_engram_vault`.
