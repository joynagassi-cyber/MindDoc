# 🔌 SCY-INTEGRATIONS-SERVICES — SPÉCIFICATION (SPEC)
**ID** : S02_INTEGRATIONS_SPEC · **Phase** : Tier 1 MVP, Tier 2 Post-MVP · **Réf** : PRD §7.10

## 1. Purpose
Module **Intégrations — 11 Services** externes pour import/export bidirectionnel (notes, flashcards, bibliothèques, surlignages).

## 2. Les 11 Services

### Tier 1 — MVP
| Service | Stack | Fonction |
|---------|-------|----------|
| **Notion** | notion-client 0.9, OAuth | Import/export bidirectionnel |
| **Obsidian** | notify 6.x + pulldown-cmark 0.13 | Sync vault, wikilinks → graph |
| **Anki** | zip 2.1 + rusqlite 0.31 | Import SM-2 + export FSRS .apkg |
| **Readwise** | reqwest HTTP | Sync différentiel, highlights → flashcards |
| **Zotero** | OAuth 2.0 | Bibliothèque académique, annotations PDF |

### Tier 2 — Post-MVP
| Service | Stack | Fonction |
|---------|-------|----------|
| **Logseq** | Stack Obsidian réutilisée 70% | Format outline |
| **Evernote** | quick-xml 0.36 | Parser .enex, ENML → texte |
| **Roam Research** | serde_json | Blocs récursifs, `[[Page]]` |

### Formats Universels
- **PDF** : pdf_oxide + pdfplumber + lopdf (3 niveaux)
- **CSV** : csv 1.x (flashcards Anki/Quizlet)
- **EPUB** : epub 2.x (chapitres + TOC + Dublin Core)

## 3. Requirements (RFC 2119)
- **Tier 1** : Notion/Obsidian/Anki/Readwise/Zotero import/export opérationnels MVP.
- **Anki** : Import SM-2→FSRS (`stability = ease_factor × interval / 2.5`) + export FSRS .apkg.
- **Formats** : PDF/CSV/EPUB universels supportés.

## 4. Tests
- TC1 : Notion import/export. | TC2 : Obsidian sync vault + wikilinks. | TC3 : Anki import SM-2→FSRS + export. | TC4 : Readwise highlights → flashcards. | TC5 : Zotero annotations PDF.
