FROM rust:alpine as builder

WORKDIR /opt/app
COPY . .

RUN apk add --no-cache musl-dev && \
    cargo build --release

FROM alpine:latest

LABEL org.opencontainers.image.title="Markdown API" \
      org.opencontainers.image.description="Markdown API" \
      org.opencontainers.image.version="1.0.0" \
      org.opencontainers.image.vendor="flavien.io" \
      org.opencontainers.image.maintainer="Flavien PERIER <perier@flavien.io>" \
      org.opencontainers.image.url="https://github.com/flavien-perier/markdown-api" \
      org.opencontainers.image.source="https://github.com/flavien-perier/markdown-api" \
      org.opencontainers.image.licenses="MIT"
      
ARG DOCKER_UID="1000" \
    DOCKER_GID="1000"

WORKDIR /opt/app

RUN addgroup -g $DOCKER_GID app && \
    adduser -G app -D -H -s /bin/sh -u $DOCKER_UID app && \
    chown -R app:app /opt/app

ENV DOMAIN="http://localhost:8080"

USER app

VOLUME /opt/app/documents

EXPOSE 8080

COPY --from=builder --chown=app:app /opt/app/target/release/markdown_api ./markdown_api

CMD ./markdown_api
