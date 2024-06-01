FROM node:hydrogen-alpine as builder

WORKDIR /opt/app
COPY . .

RUN npm install && \
    npm run build && \
    rm -Rf node_modules && \
    npm install --production

FROM node:hydrogen-alpine

LABEL org.opencontainers.image.title="Markdown API" \
      org.opencontainers.image.description="Markdown API" \
      org.opencontainers.image.version="1.1.0" \
      org.opencontainers.image.vendor="flavien.io" \
      org.opencontainers.image.maintainer="Flavien PERIER <perier@flavien.io>" \
      org.opencontainers.image.url="https://github.com/flavien-perier/markdown-api" \
      org.opencontainers.image.source="https://github.com/flavien-perier/markdown-api" \
      org.opencontainers.image.licenses="MIT"
      
ARG DOCKER_UID="1100" \
    DOCKER_GID="1100"

ENV NODE_ENV="production" \
    NODE_ID="" \
    LOG="debug"

WORKDIR /opt/app

RUN addgroup -g $DOCKER_GID app && \
    adduser -G app -D -H -h /opt/app -u $DOCKER_UID app && \
    chown -R app:app /opt/app

USER app

VOLUME /opt/app/documents

EXPOSE 8080

COPY --from=builder --chown=app:app /opt/app/LICENSE ./LICENSE
COPY --from=builder --chown=app:app /opt/app/package.json ./package.json
COPY --from=builder --chown=app:app /opt/app/node_modules ./node_modules
COPY --from=builder --chown=app:app /opt/app/swagger.yaml ./swagger.yaml
COPY --from=builder --chown=app:app /opt/app/dist ./dist
COPY --from=builder --chown=app:app /opt/app/configuration.yaml ./configuration.yaml

CMD npm start
