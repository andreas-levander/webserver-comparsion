FROM oven/bun:alpine

COPY package.json ./
COPY bun.lockb ./
COPY src ./

RUN bun install --production

# run the app
USER bun
EXPOSE 3000
ENV NODE_ENV=production

ENTRYPOINT [ "bun", "run", "index.ts" ]
