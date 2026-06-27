/**
 * SCY Forge — point d'entrée backend TS (Mastra).
 * Sprint 0 : stub minimal de démarrage.
 * L'orchestration des 18 agents ASCENT arrive au Sprint 5 (IMPLEMENTATION_ORDER).
 *
 * Réf : docs/PROJECT_STRUCTURE.md §3, docs/CODE_STYLE.md §2
 */

const PORT = Number(process.env.PORT ?? 3000);

async function main(): Promise<void> {
  // TODO Sprint 5 : bootstrap Mastra + 18 agents ASCENT + intégration Nango MCP.
  // TODO Sprint 0 : serveur HTTP /health (via framework dédié ou proxy Axum).
  console.info(`[SCY Forge] backend_ts prêt sur le port ${PORT} (stub Sprint 0)`);
}

main().catch((err: unknown) => {
  console.error("[SCY Forge] erreur fatale:", err);
  process.exit(1);
});
