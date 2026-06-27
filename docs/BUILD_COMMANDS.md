# SCY FORGE — BUILD COMMANDS
## Toutes les commandes pour build, tester, linter et lancer le projet

---

## 1. Backend Rust (`backend_rs/`)

```bash
# Compiler
cargo build                          # debug
cargo build --release                # production (optimized)

# Tests
cargo test                           # tous les tests
cargo test --package scy-apex-fsrs   # tests d'un crate spécifique
cargo test -- --nocapture            # avec output visible
cargo test property_tests            # tests property-based uniquement

# Lints
cargo clippy                         # lints Rust
cargo clippy -- -D warnings          # strict (warnings = erreurs)
cargo fmt                            # formatter
cargo fmt -- --check                 # vérifier sans modifier

# Audit sécurité
cargo audit                          # vulnérabilités dépendances

# Documentation
cargo doc --open                     # générer la doc
```

### Tests property-based (CRITIQUES pour FSRS/CPM)
```bash
cargo test --package scy-apex-fsrs fsrs_never_negative
cargo test --package scy-apex-fsrs circuit_breaker_state_machine
cargo test --package scy-cosmos-kg engram_vitality_no_nan
```

---

## 2. Backend TypeScript (`backend_ts/`)

```bash
# Installation
cd backend_ts
pnpm install                          # installe les dépendances

# Développement
pnpm dev                          # Mastra dev server (hot reload)

# Build
pnpm build                        # compile TypeScript → dist/

# Tests
pnpm test                             # tous les tests
pnpm test -- --grep "goal"            # tests spécifiques
pnpm test:e2e                     # tests end-to-end

# Lints
pnpm lint                         # ESLint
pnpm lint:fix                     # auto-fix
pnpm typecheck                    # tsc --noEmit (vérification types)
```

---

## 3. Frontend React (`frontend_react/`)

```bash
# Installation
cd frontend_react
pnpm install                          # installe les dépendances

# Développement
pnpm dev                          # Vite dev server (localhost:5173)

# Build
pnpm build                        # production build → dist/
pnpm preview                      # prévisualiser le build

# Tests
pnpm test                             # tous les tests (Vitest)
pnpm test -- --ui                     # interface visuelle
pnpm test:e2e                     # Playwright E2E

# Lints
pnpm lint                         # ESLint
pnpm lint:fix                     # auto-fix
pnpm typecheck                    # tsc --noEmit

# Bundle analyzer
pnpm build -- --analyze           # visualiser le bundle
```

---

## 4. Docker (Sidecars + Full Stack)

```bash
# Lancer TOUT le stack (dev)
cd docker
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up

# Sidecars seuls (pour dev local)
docker-compose up searxng perplexica

# Build images
docker-compose build

# Logs
docker-compose logs -f backend_rs
docker-compose logs -f perplexica
```

---

## 5. Base de Données

```bash
# Migrations (Rust sqlx)
cd backend_rs
sqlx migrate run --database-url $DATABASE_URL

# Migration spécifique
sqlx migrate run --source migrations/001_init.sql

# Sprint 0 DB init (depuis le PRD)
psql $DATABASE_URL -f ../minddoc/s00_prd/scy_sprint_0_db_init.sql

# RLS vérification
psql $DATABASE_URL -c "SELECT * FROM pg_policies WHERE schemaname = 'public';"
```

---

## 6. CI/CD (Northflank + Vercel)

```bash
# Backend deployment (Northflank auto-detect Cargo.toml)
git push origin main                → Northflank build automatique

# Frontend deployment (Vercel)
cd frontend_react && vercel --prod

# Preview deployment
vercel                              → preview URL automatique
```

---

## 7. Commandes de Diagnostic

```bash
# Santé du système
curl http://localhost:3000/health/live    # liveness
curl http://localhost:3000/health/ready   # readiness
curl http://localhost:3000/health/deep    # deep (DB + LLM + cache)

# Vérifier les coûts LLM
curl http://localhost:3000/api/budget/status

# EventBus status
curl http://localhost:3000/api/eventbus/stats

# COSMOS rendering benchmark
curl http://localhost:3000/api/cosmos/benchmark?nodes=5000
```

---

## 8. Tests de Charge / Performance

```bash
# COSMOS 60 FPS benchmark (1000 nœuds)
cd frontend_react && pnpm test:perf:cosmos

# FSRS 1800 révisions benchmark ($0 LLM)
cd backend_rs && cargo test --release --package scy-apex-fsrs -- benchmark

# RAG latency (p95 < 2s)
cd backend_rs && cargo test --package scy-brain-rag -- latency_test
```
