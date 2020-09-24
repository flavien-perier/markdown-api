FROM node:lts-alpine as builder

LABEL maintainer="Flavien PERIER <perier@flavien.io>"
LABEL version="1.0"
LABEL description="Markdown API"

WORKDIR /opt/app
COPY . .

RUN rm -Rf node_modules && \
    chmod -R 750 /opt/app && \
    chown -R root:root /opt/app && \
    (npm install) && \
    npm run build && \
    rm -Rf node_modules documents && \
    npm install --production

FROM node:lts-alpine

ARG DOCKER_UID=500
ARG DOCKER_GID=500

ENV NODE_ENV=production

ENV NODE_ID=
ENV LOG=debug

WORKDIR /opt/app

COPY --from=builder /opt/app .

RUN addgroup -g $DOCKER_GID app && \
    adduser -G app -D -H -h /opt/app -u $DOCKER_UID app && \
    chown -R app:app /opt/app

VOLUME /opt/app/documents

USER app
EXPOSE 8080

CMD npm start
