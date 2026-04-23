# Frontend Dockerfile for Philand Next.js App

FROM node:18-alpine AS base

# deps
FROM base AS deps
RUN apk add --no-cache libc6-compat
WORKDIR /app
COPY package.json package-lock.json* ./
# KHÔNG dùng --only=production ở stage build deps, vì build cần devDependencies (next, typescript,...)
RUN npm ci

# builder
FROM base AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# --- build-time public envs (baked into bundle) ---
ARG NEXT_PUBLIC_API_URL
ARG NEXT_PUBLIC_APP_VERSION
ARG NEXT_PUBLIC_APP_NAME
ARG NEXT_PUBLIC_BUILD_DATE
ARG NEXT_PUBLIC_GOOGLE_CLIENT_ID

ENV NEXT_TELEMETRY_DISABLED=1
ENV NODE_ENV=production

ENV NEXT_PUBLIC_API_URL=${NEXT_PUBLIC_API_URL}
ENV NEXT_PUBLIC_APP_VERSION=${NEXT_PUBLIC_APP_VERSION}
ENV NEXT_PUBLIC_APP_NAME=${NEXT_PUBLIC_APP_NAME}
ENV NEXT_PUBLIC_BUILD_DATE=${NEXT_PUBLIC_BUILD_DATE}
ENV NEXT_PUBLIC_GOOGLE_CLIENT_ID=${NEXT_PUBLIC_GOOGLE_CLIENT_ID}
# -----------------------------------------------

RUN npm run build

# runner
FROM node:18-alpine AS runner
WORKDIR /app

ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1

RUN addgroup --system --gid 1001 nodejs \
 && adduser  --system --uid 1001 nextjs

COPY --from=builder /app/public ./public
RUN mkdir .next && chown nextjs:nodejs .next

COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs
EXPOSE 3000
ENV PORT=3000
ENV HOSTNAME=0.0.0.0

# (optional) healthcheck: nếu bạn không có /api/health thì đổi sang /
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget -qO- http://127.0.0.1:3000/ >/dev/null || exit 1

CMD ["node", "server.js"]