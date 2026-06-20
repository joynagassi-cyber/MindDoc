# 📊 SCY-CREATOR-DASHBOARD — SPÉCIFICATION (SPEC)
**ID** : S12_B2B_CREATOR_DASHBOARD_SPEC · **Phase** : V1+ · **Réf** : PRD §2.6, §7.7quater

> **📌 RÉFÉRENCE CROISÉE** : La stratégie d'expansion B2B complète est dans **`scy_b2b_expansion_strategy.md`** (14KB). Ce fichier en est le résumé SDD.

## 1. Purpose
Le **Creator Dashboard / Console Manager** permet aux DRH, Directeurs Techniques et créateurs de contenu formateur de suivre la progression du SMI en temps réel de leurs équipes (module analytics SurveyJS Dashboard à $0 de licence). Inclut le suivi de cohorte, la détection des goulots cognitifs (Agent-13), et l'enregistrement de micro-clarifications en un clic.

## 2. Requirements (RFC 2119)
- **GIVEN** Un compte B2B (entreprise/créateur).
- **THEN** le système SHALL fournir un dashboard temps réel du SMI des équipes (SurveyJS Dashboard, $0 licence).
- **AND** le système SHALL détecter les goulots cognitifs de cohorte (Agent-13 Cognitive-Validator → alerte 80% blocage).
- **AND** le créateur SHALL pouvoir enregistrer une micro-clarification (audio/vidéo 1 min) ré-indexée instantanément dans Zilliz (Creator-to-Student Synaptic Loop, D-OPT-017).
- **AND** l'isolation multi-tenant SHALL être assurée par RLS PostgreSQL (D-OPT-029 k-anonymat ≥ 10 sur les profils d'élèves).

## 3. Tests
- TC1 : Dashboard SMI temps réel (SurveyJS $0). | TC2 : Goulot cognitif cohorte détecté (Agent-13). | TC3 : Micro-clarification créateur → Zilliz. | TC4 : RLS + k-anonymat ≥ 10.
