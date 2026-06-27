import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// SCY Forge — frontend React (Vite + React 18)
// Serveur dev : http://localhost:5173
export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    // Proxy API vers le backend Rust (Axum, port 3000) — Sprint 0+
    proxy: {
      '/api': 'http://localhost:3000',
    },
  },
  build: {
    target: 'es2020',
    sourcemap: true,
  },
});
