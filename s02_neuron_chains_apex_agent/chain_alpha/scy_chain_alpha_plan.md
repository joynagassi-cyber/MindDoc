# 🛠️ SCY-CHAIN-ALPHA — PLAN (PLAN)
**ID** : S02_NEURON_CHAIN_ALPHA_PLAN
## Flux : [contenu brut] → ALPHA-1 extraction (DOM/OCR/transcription) → ALPHA-2 chunking sémantique (500-2000 tok, overlap 10%, tokio parallèle 10-50) → ALPHA-3 résumés L1/L2/L3 → persistance `scy_chunks` (embedding VECTOR 512).
## Dépendances : tokio (parallèle), Docling (OCR), pgvector (embeddings). Tables : `scy_chunks`.
## Fichiers : `backend_rs/src/neurochain/chains/alpha/extractor.rs`, `chunker.rs`, `summarizer.rs`.
