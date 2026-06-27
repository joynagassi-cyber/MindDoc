import type { Config } from 'tailwindcss';

// SCY Forge — config Tailwind (tokens design uniquement, CODE_STYLE §3.4)
// TODO : importer les design tokens depuis s04_scy_cosmos_visualization_engine/scy_design_tokens.md
const config: Config = {
  content: ['./index.html', './src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      // Design tokens à figer (palette stricte, ne pas modifier hors design.md)
      colors: {
        scy: {
          bg: '#0b0f17',
          surface: '#131a26',
          primary: '#6366f1',
          accent: '#22d3ee',
        },
      },
    },
  },
  plugins: [],
};

export default config;
