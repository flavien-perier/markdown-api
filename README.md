# Markdown API

API to expose a list of Markdown files with paging.

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
