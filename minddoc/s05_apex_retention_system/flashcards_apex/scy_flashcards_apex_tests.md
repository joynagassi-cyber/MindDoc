# 🧪 SCY-FLASHCARDS-APEX — TESTS
**ID** : S05_APEX_FLASHCARDS_TESTS

- **TC1** : 12 cartes/nœud générées (B02/B03/B04/B05), validées Zod, persistées dans `scy_apex_cards` (state=New).
- **TC2** : MCQ B03 avec distracteurs plausibles (erreurs communes réelles, pas aléatoires).
- **TC3** : Carte >8 lapses → tag #leech + alternatives (B06/B01/C01/IMPRINT Cran5) ; >5 leeches → alerte DRIFT-GUARDIAN.
- **TC4** : Gestion suspend/bury/flag/edit/note fonctionnelle.
- **TC5** : TTS `R` sur B01-B05 (mode mains-libres) ; deep link Reader Suite à la position exacte.
- **TC6** : B07 Teaching évalué par BRAIN → contribution dimension Mirror du SMI.
- **TC7** : Aucune carte inventée sans source (traçabilité EPSILON).
