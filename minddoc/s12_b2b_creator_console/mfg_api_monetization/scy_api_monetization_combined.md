# 🛠️ SCY-API-MONETIZATION — PLAN / TÂCHES / TESTS
**ID** : S12_B2B_API_MONETIZATION_PLAN / TASKS / TESTS
## Flux : [partenaire B2B + clé API] → endpoints SCY Forge (ingestion/génération/COSMOS/APEX) → auth API + rate limiting/quotas par tier → facturation à l'usage (tokens/appels/ingestion) → BudgetGuard par clé API.
## Référence : `scy_api_monetization_specs.md`.
## Dépendances : BudgetGuard, rate limiting, facturation usage-based. 
## Fichiers : `backend_ts/src/b2b/api/api_gateway.ts`, `usage_billing.ts`, `api_rate_limiter.ts`.
## Tâches : AM.1 Endpoints API + auth clé API (25min) | AM.2 Facturation à l'usage + quotas tier (25min) | AM.3 BudgetGuard par clé API (20min).
## Tests : TC1 endpoints auth | TC2 facturation usage | TC3 rate limiting+quotas | TC4 BudgetGuard/clé.
