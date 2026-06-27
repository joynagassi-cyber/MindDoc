# backend_ts — TypeScript (Mastra orchestration)
FROM node:20-slim
WORKDIR /app
COPY package.json pnpm-lock.yaml* ./
RUN corepack enable && pnpm install --frozen-lockfile || pnpm install
COPY . .
RUN pnpm build
EXPOSE 3000
CMD ["node", "dist/index.js"]
