# 📊 CHECKLIST BUDGET SAAS POUR STARTUPS — SCY FORGE

Cette checklist de niveau corporate est conçue pour structurer, budgétiser et piloter méthodiquement le lancement de la startup **SCY Forge**. Elle intègre nos décisions stratégiques réelles de **juin 2026**, notamment notre architecture de déploiement sur **Northflank**, notre structure d'abonnement à 4 tiers et l'ingénierie augmentée par agents (*Agentic SDLC*).

---

## 🧭 TABLE DES MATIÈRES
1. [Estimer les Coûts Initiaux de Démarrage (CapEx)](#1-capex)
2. [Prévoir les Revenus & Tarifs d'Abonnement](#2-revenus)
3. [Détailler les Dépenses d'Exploitation (OpEx)](#3-opex)
4. [Calculer & Analyser le Cash Flow (Burn Rate & Runway)](#4-cashflow)
5. [Planifier les Imprévus (BudgetGuard)](#5-imprevus)
6. [Processus de Révision Budgétaire](#6-revision)
7. [Stratégie de Financement](#7-financement)
8. [Fiscalité & Comptabilité (Dépôts d'État & IRS)](#8-fiscalite)
9. [Conformité & Sécurité (Compliance)](#9-compliance)
10. [Ressources Humaines & Recrutement](#10-rh)
11. [Croissance Produit Post-MVP](#11-croissance)
12. [Suivi des Métriques de Croissance (SaaS KPIs)](#12-metrics)
13. [Service Client & Succès Client](#13-support)
14. [Partenariats, Ventes & Distribution](#14-sales)

---

## 1. Estimer les Coûts Initiaux de Démarrage (CapEx) <a name="1-capex"></a>

Cette section chiffre les dépenses indispensables à engager avant de pouvoir encaisser votre premier dollar de Chiffre d'Affaires.

### ☐ 1.1 Tâches Juridiques & Administratives (Création US C-Corp via Doola)
*   **[COMPLÉTÉ] plan Doola Tax & Compliance** : **1 999,00 $ / an**  
    *Inclut la formation, l'obtention de l'EIN (IRS), l'agent enregistré, l'adresse virtuelle aux US acceptée par Mercury, et les déclarations fiscales de fin d'année (Forms 1120 & 5472).*
*   **[COMPLÉTÉ] Frais de formation d'État Delaware** : **90,00 $** (Paiement unique).
*   **[À FAIRE] Dépôt de marque USPTO (Double Classe)** : **700,00 $**  
    * *Classe 42 (Web/SaaS)* : 350 $  
    * *Classe 9 (App mobile & Desktop installées localement)* : 350 $  
    *Note : Recommandé de déposer les deux classes dès le départ pour éviter de recommencer la procédure.*
*   **[COMPLÉTÉ] CGU & Politique de Confidentialité** : **0,00 $**  
    *Génération gratuite via les outils conformes Termly ou iubenda avec branding initial.*
*   **[REPORTÉ] Pacte d'actionnaires (Shareholders Agreement)** : **0,00 $**  
    *Inutile au stade de solo-founder. À prévoir uniquement lors de l'intégration d'associés ou d'investisseurs (budget estimé : 550 $ à 1 070 $).*
*   **[COMPLÉTÉ] Ouverture de compte bancaire professionnel** : **0,00 $** (Ouverture gratuite d'un compte Mercury avec bonus de cashback de +150 $ intégré au dashboard Doola).

### ☐ 1.2 Tâches de Développement Initial du Produit (MVP via Agentic SDLC)
Pour estimer le coût réel du code source de SCY Forge développé par des agents autonomes (*Spec-Driven Development*), nous traduisons l'effort en volume de tokens LLM consommés (avec un taux moyen de 85% de Prompt Caching) :
*   **Intégration UI/UX & CSS** (5 000 LOC sur Claude Sonnet 4.6) : **4,46 $** (1.25M de tokens).
*   **Développement Backend** (15 000 LOC sur Claude Opus 4.8 & Sonnet 4.6) : **16,04 $** (3.75M de tokens).
*   **Développement Frontend** (18 000 LOC sur Claude Sonnet 4.6 & DeepSeek Pro) : **14,77 $** (4.50M de tokens).
*   **Tests E2E & Debugging** (5 000 LOC sur DeepSeek Pro) : **5,00 $** (1.50M de tokens).
*   *Note d'ingénierie : L'architecture de Monolithe Unifié alliée au Prompt Caching de juin 2026 permet de générer 43 000 LOC de qualité industrielle pour un coût d'API LLM dérisoire de **40,27 $**.*

### ☐ 1.3 Infrastructure & Outils Initiaux de Lancement
*   **Achat du Nom de Domaine** (`scyforge.io`) : **35,00 $** (puis 66,00 $ / an).
*   **Email professionnel (Google Workspace - 1 utilisateur)** : **110,00 $ / an** (*contact@scyforge.io*, requis pour valider Stripe & Mercury).
*   **Email transactionnel automatisé (Resend)** : **0,00 $** (Plan gratuit jusqu'à 3 000 emails/mois).
*   **Gestion de projet** : **0,00 $** (Plan gratuit de Linear et Notion en solo-founder).
*   **Environnement de développement (IDEs & Outils)** : **0,00 $** (Utilisation d'outils open-source).

### 🏷️ TOTAL CAPEX ESTIMÉ : 2 974,27 $

---

## 2. Prévoir les Revenus & Tarifs d'Abonnement <a name="2-revenus"></a>

SCY Forge applique une structure de tarification récurrente (SaaS) divisée en 4 offres claires de service :

### ☐ 2.1 Configuration de la Grille d'Abonnement
*   **🟢 Free Tier (0 $ / mois)** :  
    *Usage* : Utilise exclusivement **DeepSeek V4 Pro/Flash** (0% de modèles fermés). Limité à 3 sources ingérées/mois, 15 requêtes BRAIN/jour et 1 parcours ASCENT simplifié (max 5 nœuds).  
    *Mémorisation* : Accès complet à APEX (FSRS 5.0) et COSMOS (limites physiques d'affichage à 10K nœuds).
*   **🟡 Lite Tier (10 $ / mois)** :  
    *Usage* : Ouvre l'accès à **Claude Sonnet 4.6** (bridé). Pas de Claude Opus. Limité à 10 sources/mois, 50 requêtes BRAIN/jour, 2 parcours actifs ASCENT (max 8 nœuds).
*   **🟠 Pro Tier (20 $ / mois)** :  
    *Usage* : Débloque l'utilisation sélective de **Claude Opus 4.8** (exclusivement dédié au DAG, à l'expert AGENT-16 et au Comité QA), de **Claude Sonnet 4.6** pour l'écriture et les révisions d'erreurs, et de **DeepSeek Pro** pour le chat et CHRONICLE.  
    *Limites* : 50 sources/mois, chat BRAIN illimité, 5 parcours (max 15 nœuds), 3 sessions ARENA/mois avec certification signée et assistant compagnon **CHRONICLE**.
*   **🔴 Ultra Tier (45 $ / mois)** :  
    *Usage* : Accès maximal sans restriction. **Claude Opus 4.8** est utilisé pour toutes les tâches de calcul et de rédaction. Parcours d'apprentissage et simulations d'ARENA illimités, avec hébergement de conteneurs prioritaires sur Northflank.

### ☐ 2.2 Définir les Métriques de Prévision (Objectif de Lancement à 1 000 utilisateurs)
*   **Taux de Churn cible** : < 5 % / mois (Soutenu par les notifications anti-démission de `DRIFT-GUARDIAN`).
*   **LTV (Lifetime Value) cible** : > 300,00 $ sur les tiers payants.
*   **CAC (Customer Acquisition Cost) cible** : < 15,00 $ par conversion payante.
*   **Hypothèse de conversion standard** : 5 % des visiteurs de la landing page s'inscrivent au Free Tier, et 10% des utilisateurs actifs se convertissent vers une offre payante (Lite ou Pro) sous 30 jours.

---

## 3. Détailler les Dépenses d'Exploitation (OpEx) <a name="3-opex"></a>

Cette section regroupe les dépenses récurrentes nécessaires à la livraison du service au jour le jour (lissage mensuel).

### ☐ 3.1 Coût des Marchandises Vendues (COGS - Infrastructure Always-on sur Northflank)
Notre architecture de Monolithe Unifié s'exécute de manière optimisée sur Northflank :
*   **`scy-gateway-app`** (Mastra TS Gateway, 0.5 vCPU, 1.0 Go RAM) : **12,00 $ / mois**.
*   **`scy-rust-core`** (Calculs physiques et IA ONNX, 1.0 vCPU, 1.5 Go RAM) : **21,00 $ / mois**.
*   **`scy-docling-sidecar`** (Pattern Bulkhead d'extraction PDF, 1.0 vCPU, 2.0 Go RAM) : **24,00 $ / mois**.
*   **PostgreSQL Addon** (Base de données relationnelle RLS multi-tenant, 1.0 Go RAM, 20 Go) : **20,00 $ / mois**.
*   **Zilliz Cloud Vector Search** (Index vectoriel serverless) : **10,00 $ / mois**.
*   **Stockage volume & Bande passante de sortie (Egress)** : **25,00 $ / mois**.
*   *Sous-total Infrastructure Cloud* : **112,00 $ / mois** (coût fixe stable à 100%).

### ☐ 3.2 Coût des APIs LLM à l'Usage (Worst-Case de Charge vs Réalité d'Usage)
Facture mensuelle d'API calculée pour **1 000 utilisateurs actifs** (avec 60% Free, 20% Lite, 15% Pro et 5% Ultra) :
*   **Worst-Case (100% de consommation des limites)** : **4 367,50 $ / mois**.
*   **Scénario Réel Constaté (30% de consommation moyenne)** : **1 310,25 $ / mois**.

### ☐ 3.3 Frais Généraux, Ventes & Marketing (S&M, G&A)
*   **Frais de traitement des abonnements (Stripe)** : **2,9% + 0,30 $** par transaction.  
    *(Sur un chiffre d'affaires mensuel de 7 250,00 $ pour 1 000 utilisateurs, les frais Stripe représentent environ **330,00 $ / mois**).*
*   **Outil de Marketing Automation & Analytics (PostHog)** : **0,00 $** (Plan gratuit jusqu'à 1 million d'événements par mois).
*   **Hébergement de la Landing Page (Vercel)** : **0,00 $** (Plan gratuit pour les fichiers statiques de présentation).
*   **Logiciel de Comptabilité** : **0,00 $** (Outil de comptabilité et facturation basique inclus dans le plan Doola Tax & Compliance).

---

## 4. Calculer & Analyser le Cash Flow (Burn Rate & Runway) <a name="4-cashflow"></a>

Le Cash Flow represents la différence nette entre vos entrées réelles de trésorerie (abonnements payés) et vos sorties (infrastructure, APIs, taxes).

### ☐ 4.1 Modélisation Budgétaire du MVP (Lancement avec 1 000 Utilisateurs)

```
[ Entrées Mensuelles (Chiffre d'Affaires Brut) ]  =  +7 250,00 $
  ├─ 200 Utilisateurs Lite à 10 $ = 2 000,00 $
  ├─ 150 Utilisateurs Pro à 20 $  = 3 000,00 $
  └─  50 Utilisateurs Ultra à 45 $ = 2 250,00 $

[ Sorties Mensuelles (OpEx - Scénario Réel Constaté à 30%) ] =  -1 752,25 $
  ├─ Infrastructure Northflank      =   -112,00 $
  ├─ Facturation APIs LLM Réelle    = -1 310,25 $
  └─ Frais de traitement Stripe     =   -330,00 $

[ BÉNÉFICE NET MENSUEL CONSTATÉ ]                  =  +5 497,75 $ / mois
[ BÉNÉFICE NET DANS LE PIRE SCÉNARIO (Worst-Case) ] =  +2 770,50 $ / mois
```

### ☐ 4.2 Métriques de Sécurité financière
*   **Burn Rate Mensuel Moyen** : **0,00 $** (La startup est immédiatement bénéficiaire dès l'atteinte de son premier jalon, ne nécessitant aucun apport de capital extérieur récurrent).
*   **Cash Runway Estimé** : **Infini** (Grâce à notre modèle économique optimisé, le chiffre d'affaires couvre à la fois l'infrastructure fixe et l'intégralité de la consommation à l'usage des APIs LLM).

---

## 5. Planifier les Imprévus (BudgetGuard) <a name="5-imprevus"></a>

Le principal risque financier d'un produit SaaS basé sur l'IA est le *Token Bleeding* (un utilisateur malveillant ou une boucle infinie d'agents consommant des milliers de dollars d'API en quelques secondes).

### ☐ 5.1 Sécurité Applicative du Budget
*   **[COMPLÉTÉ] Règle de design BudgetGuard (Tool T17)** : Le backend Rust intègre un garde-fou middleware qui intercepte chaque appel d'API LLM et enregistre la dépense en temps réel dans la table `scy_llm_spend_log`.
*   **[COMPLÉTÉ] Bloqueur de requêtes automatique** : Si l'utilisateur franchit 100% de la limite de tokens allouée à son tier d'abonnement au cours du mois courant, son accès est immédiatement basculé en mode "Lecture Seule", interdisant toute nouvelle génération ou appel LLM jusqu'au renouvellement de son forfait.
*   **[À FAIRE] Alertes d'Infrastructure** : Configurer des webhooks d'alertes automatiques (Slack ou Email) lorsque le budget global d'API de la startup franchit des paliers de 50%, 80% et 100% de la réserve mensuelle de prévoyance.

---

## 6. Processus de Révision Budgétaire <a name="6-revision"></a>

### ☐ 6.1 Planifier les Révisions
*   **Réunion budgétaire mensuelle (J+5 du mois suivant)** : Comparer la facture réelle émise par Northflank et nos consoles APIs (Anthropic, DeepSeek) avec les logs de la table `scy_llm_spend_log`.
*   **Ajustement de l'algorithme d'échantillonnage FSRS** : Si le coût d'interrogation de l'assistant de chat BRAIN dérape, appliquer une compression locale plus agressive des invites à l'aide de **LLMLingua-2 (ONNX)** avant l'envoi du contexte au LLM.

---

## 7. Stratégie de Financement <a name="7-financement"></a>

### ☐ 7.1 Choix de la Source de Financement
*   **Modèle retenu : 100% Bootstrapped / Indépendance financière.**  
    *Grâce au levier technologique de l'Agentic SDLC qui ramène le coût de développement initial à moins de 50 $, et au déploiement léger sur Northflank à 112 $/mois, aucun apport en capital externe (VC, business angel) n'est nécessaire au lancement.*
*   **Crédits Cloud (Optionnels)** : Soumettre le dossier de SCY Forge au programme de crédits pour startups de Northflank pour obtenir des remises sur le compute d'infrastructure lors de la phase de croissance.

---

## 8. Fiscalité & Comptabilité (Dépôts d'État & IRS) <a name="8-fiscalite"></a>

Notre Delaware C-Corp exige le strict respect du calendrier fiscal sous peine de pénalités majeures d'exclusion.

### ☐ 8.1 Les Échéances Fiscales du Delaware C-Corp (Incluses dans Doola Tax & Compliance)
*   **Rapport annuel Delaware (avant le 1er mars de chaque année)** : Dépôt obligatoire de **50,00 $** auprès de l'État du Delaware.
*   **Delaware Franchise Tax (avant le 1er mars de chaque année)** : Paiement de **400,00 $** (le minimum légal obligatoire).
    *   <critical>**ALERTE DE SÛRETÉ (Franchise Tax calculation) :** Toujours exiger de votre CPA Doola d'appliquer **la méthode d'évaluation "Assumed Par Value Capital"** lors du dépôt annuel. La méthode par défaut ("Authorized Shares") calcule la taxe sur le nombre brut d'actions de l'entreprise (10 millions), générant une facturation erronée de plus de **85 000,00 $ / an** au lieu des **400,00 $** légitimes.*</critical>
*   **Déclaration Fédérale IRS (avant le 15 avril de chaque année)** : Production et dépôt obligatoire du **Form 1120** (Imposition sur les sociétés) et du **Form 5472** (Obligatoire pour tout fondateur non-résident américain détenant plus de 25% du capital d'une C-Corp).
    *   <critical>**ALERTE DE SÛRETÉ (IRS Form 5472) :** L'omission ou l'erreur de dépôt du formulaire 5472 déclenche une amende immédiate de l'IRS de **25 000,00 $**, non négociable. Ce formulaire est entièrement rédigé et déposé par le CPA de Doola dans votre plan.*</critical>

---

## 9. Conformité & Sécurité (Compliance) <a name="9-compliance"></a>

### ☐ 9.1 RGPD & CCPA (Réglementations de Protection des Données)
*   **Isolation multi-tenant au niveau de la base de données** : Toutes les requêtes et écritures s'exécutent de manière étanche grâce aux règles **RLS (Row Level Security)** configurées sur les tables de PostgreSQL Northflank.
*   **Droit à l'anonymisation et à l'oubli** : L'anonymisation des données des cohortes étudiantes et l'exclusion des profils personnels d'apprentissage respectent les contraintes du RGPD (k-anonymat $\ge 10$).

### ☐ 9.2 Sécurité de l'Exécution Technique
*   **Isolation des outils tiers** : Les outils d'analyse et les scripts de scraping tiers s'exécutent de manière étanche au sein d'un bac à sable **WebAssembly (WASM)** sécurisé et isolé au runtime sur notre serveur Northflank, éliminant tout risque d'injection de code malveillant dans notre conteneur transactionnel principal.

---

## 10. Ressources Humaines & Recrutement <a name="10-rh"></a>

### ☐ 10.1 Phase Lancement (Solo-founder)
*   **Ressources Humaines** : **0,00 $**. Le projet s'exécute en solo-founder assisté par agents d'ingénierie et d'assurance qualité.
*   **Phase Croissance (Post-Revenue)** : Prévoir le recrutement de freelances ou d'un premier ingénieur frontend senior lorsque le chiffre d'affaires récurrent (MRR) franchit stablement le seuil des **15 000,00 $ / mois**.

---

## 11. Croissance Produit Post-MVP <a name="11-croissance"></a>

### ☐ 11.1 Évolutions Techniques Planifiées (Phase 2 & 3)
*   **Migration vectorielle sur Qdrant** : Planifiée uniquement si le temps de réponse p99 de recherche vectorielle de pgvector ou Zilliz dépasse **50 ms** sur un volume de plus de **5 millions de nœuds** sémantiques.
*   **Modèles d'Embeddings Multimodaux (ColPali)** : Intégration planifiée en Phase 3 uniquement pour les grands comptes d'entreprises (Enterprise Tier), nécessitant le déploiement de serveurs d'inférence GPU dédiés sur Northflank.

---

## 12. Suivi des Métriques de Croissance (SaaS KPIs) <a name="12-metrics"></a>

Le succès commercial et l'évaluation continue du produit s'appuient sur PostHog pour suivre nos indicateurs de performance clés :

*   **MRR (Monthly Recurring Revenue)** & **ARR (Annual Recurring Revenue)**.
*   **NRR (Net Revenue Retention)** : Objectif > 100% (croissance des revenus de la base d'élèves existante grâce aux upsells des parcours ASCENT supplémentaires).
*   **LTV / CAC Ratio** : Objectif minimum $\ge 3\times$ (modèle sain validé par notre structure de marges unitaires de plus de 80%).

---

## 13. Service Client & Succès Client <a name="13-support"></a>

### ☐ 13.1 Outils & Canaux de Support
*   **Base de connaissances socratique** : Intégrée nativement dans la documentation du produit (Notion/GitBook gratuit) pour permettre l'auto-résolution des pannes par les étudiants.
*   **Support de premier niveau** : Géré asynchronement par email (`contact@scyforge.io`) et centralisé gratuitement dans un outil de ticketing open-source.

---

## 14. Partenariats, Ventes & Distribution <a name="14-sales"></a>

### ☐ 14.1 Canaux de Distribution Ingrédients
*   **Lancement d'Acquisition Organique** : Soumission et promotion de SCY Forge sur la plateforme **Product Hunt** (0$ d'inscription).
*   **Canaux de créateurs (Consoles B2B)** : Mise à disposition de la console créateur permettant aux formateurs d'intégrer et de monétiser leurs propres cours et cohortes sur SCY Forge, générant un effet de réseau organique d'arrière-plan sans budget marketing direct.

---

*Document restructuré, finalisé et validé conformément au modèle financier et d'infrastructure de SCY Forge de juin 2026.*
