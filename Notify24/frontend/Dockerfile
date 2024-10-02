FROM node:alpine3.18 AS builder

WORKDIR /build

COPY --chown=node:node package*.json .

RUN npm ci

COPY --chown=node:node . .
RUN npm run build
RUN npm prune --production

FROM node:alpine AS production
RUN addgroup nodeapp && adduser -S -G nodeapp nodeapp
USER nodeapp

COPY --from=builder --chown=nodeapp:nodeapp /build/package.json /app/
COPY --from=builder --chown=nodeapp:nodeapp /build/node_modules /app/node_modules
COPY --from=builder --chown=nodeapp:nodeapp /build/build /app/build

EXPOSE 3000
CMD ["node","app/build", "--max-old-space-size=2048"]
