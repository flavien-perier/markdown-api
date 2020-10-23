![license](https://badgen.net/github/license/flavien-perier/markdown-api)
[![docker pulls](https://badgen.net/docker/pulls/flavienperier/markdown-api)](https://hub.docker.com/r/flavienperier/markdown-api)
[![ci status](https://badgen.net/github/checks/flavien-perier/markdown-api)](https://github.com/flavien-perier/markdown-api)

# Markdown API

API to expose a list of Markdown files with paging.

You can use the [Swagger UI](https://swagger.io/tools/swagger-ui/) tool to view the API documentation available at `https://raw.githubusercontent.com/flavien-perier/markdown-api/master/swagger.yaml`.

To use this application, simply add Markdown files to the `documents` folder and add a header to them.

Here is an example of file :

```md
---
title: Test
type: ARTICLE
description: Test article
author: Flavien PERIER <perier@flavien.io>
date: 2020-09-24 13:06
---

The content of the article
```

The type of the article can be: `ARTICLE`, `BLOG` or `DOCUMENTATION`. The date is used to sort the articles.

## Ports

- 8080: HTTP

## Volumes

- /opt/app/documents

## Docker-compose example

```yaml
markdown-api:
  image: flavienperier/markdown-api
  container_name: markdown-api
  restart: always
  volumes:
    - ./documents:/opt/app/documents
  ports:
    - 8080:8080
```
