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
jupyter:
  image: flavienperier/markdown-api
  container_name: jupyter
  restart: always
  volumes:
    - ./documents:/opt/app/documents
  ports:
    - 8080:8080
  environment:
    JUPYTER_PASSWORD: password
```
