FROM node:lts-alpine as builder

WORKDIR /opt/app
COPY . .

RUN npm install && \
    npm run build && \
    rm -Rf node_modules && \
    npm install --production

FROM node:lts-alpine

LABEL maintainer="Flavien PERIER <perier@flavien.io>" \
      version="1.1.0" \
      description="Markdown API"

ARG DOCKER_UID="500" \
    DOCKER_GID="500"

ENV NODE_ENV="production" \
    NODE_ID="" \
    LOG="debug"

WORKDIR /opt/app

RUN addgroup -g $DOCKER_GID app && \
    adduser -G app -D -H -h /opt/app -u $DOCKER_UID app

COPY --from=builder --chown=app:app /opt/app/dist ./dist
COPY --from=builder --chown=app:app /opt/app/swagger.yaml ./swagger.yaml
COPY --from=builder --chown=app:app /opt/app/configuration.yaml ./configuration.yaml
COPY --from=builder --chown=app:app /opt/app/package.json ./package.json
COPY --from=builder --chown=app:app /opt/app/node_modules ./node_modules
COPY --from=builder --chown=app:app /opt/app/LICENSE ./LICENSE

USER app

VOLUME /opt/app/documents

EXPOSE 8080

CMD npm start
