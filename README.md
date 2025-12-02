# freshrss-image-cache-service-rs

[![Release](https://ghcr-badge.egpl.dev/s373r/freshrss-image-cache-service-rs/latest_tag?label=Release)](https://github.com/kamu-data/kamu-cli/releases/latest)

A simple service for caching images locally, specifically designed for
the [freshrss-image-cache-plugin](https://github.com/Victrid/freshrss-image-cache-plugin) extension. This can be
particularly useful in the case of time-limited links to images (e.g. in the case of [rsshub.app](https://rsshub.app/)).

This is an alternative implementation of the caching service, a drop-in replacement for
the [piccache.php](https://github.com/Victrid/freshrss-image-cache-plugin/blob/master/piccache.php.example).

## Quick Start

Install [freshrss-image-cache-plugin](https://github.com/Victrid/freshrss-image-cache-plugin) extension, by git cloning into extensions folder for your freshrss installation

```
git clone https://github.com/Victrid/freshrss-image-cache-plugin.git
```

Run either development or docker container of the cache server.

Enable "Image Cache" extension, under Settings -> Extensions

Open settings for extension of use the following URL format, can be different if you used an ip/port or reverse proxy, which is different from the php cache service:
```
Cache URL: 
https://freshrss-image-cache-rs.(example).com/?url=

Proactive Cache URL:
https://freshrss-image-cache-rs.(example).com

Access Token:
Token you created (if using docker compose, reference your docker-compose.yml).
```

To start the service locally for development, run just one command:

```shell
make start
```

## Docker Image

Here is an example of a Docker Compose configuration for quick deployment:

```yaml
# $ cat compose.yaml

services:
  cache_server:
    image: petertoe/freshrss-image-cache-service-rs:latest
    ports:
      - 3000:3000
    volumes:
      - ./images:/usr/src/app/images
    environment:
      - APP_PORT=3000
      - APP_ACCESS_TOKEN=TODO_REPLACE_ME_WITH_RANDOM_VALUE
      - APP_IMAGES_DIR=./images
      - APP_NO_ANSI_COLORS=1
```

⚠️ Make sure to replace `APP_ACCESS_TOKEN` with a unique value!

## Release procedure

```shell
make lint
make image
make image-push
```
