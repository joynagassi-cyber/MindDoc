# ⚡ SCY-UX-SESSION-APEX — SPÉCIFICATION (SPEC)
**ID** : S05_UX_SESSION_APEX_SPEC · **Phase** : P0-P1 · **Réf** : PRD §7.5.12bis

## 1. Purpose
**UX Session APEX — Fonctionnalités Manquantes** : TTS étendu, timer optionnel, mode focus plein écran, mode nuit automatique.

## 2. Features

### Audio TTS étendu (Phase 1)
- TTS disponible pour B01-B05 (pas seulement B10)
- Touche `R` → lecture audio carte courante
- Usage : mode mains-libres (conduite, sport), accessibilité malvoyants
- Techno : OpenAI TTS API

### Timer Optionnel par Carte (Phase 1)
- Configurable : 5s / 10s / 15s / 30s / Désactivé
- Force rappel actif rapide, simule conditions examen
- Alerte sonore douce à expiration (pas de pénalité auto)

### Mode Focus Plein Écran (Phase 0)
- `F` → masque navigation, notifications silencées, fond épuré
- Barre progression minimale (% session, streak)
- `Escape` → quitter

### Mode Nuit Automatique (Phase 0)
- Basculement dark mode selon heure système (20h-7h défaut)
- Ou suivi `prefers-color-scheme` OS
- Transition douce 300ms

## 3. Tests
- TC1 : `R` → TTS sur B01-B05. | TC2 : Timer 15s + alerte douce. | TC3 : `F` → focus plein écran. | TC4 : Mode nuit auto 20h + transition 300ms.
