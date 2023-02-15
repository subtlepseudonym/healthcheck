# Healthcheck

[![github](https://img.shields.io/github/v/tag/subtlepseudonym/pihole-exporter?logo=github&sort=semver)](https://github.com/subtlepseudonym/pihole-exporter/tags) [![docker_size](https://img.shields.io/docker/image-size/subtlepseudonym/healthcheck?logo=docker)](https://hub.docker.com/r/subtlepseudonym/healthcheck) [![docker_pulls](https://img.shields.io/docker/pulls/subtlepseudonym/pihole-exporter?label=pulls&logo=docker)](https://hub.docker.com/r/subtlepseudonym/pihole-exporter) [![kofi](https://img.shields.io/badge/ko--fi-Support%20me%20-hotpink?logo=kofi&logoColor=white)](https://ko-fi.com/subtlepseudonym)

This project is intended as a lightweight binary that can be included in a
docker image for health checks. The latest image size is included in the
badge above.

It expects two arguments, a host address and an HTTP endpoint.
```bash
healthcheck localhost:80 /ok
```

Healthcheck makes a GET request to the provided host and endpoint and exits
with code 0 if it receives a response code of 200 OK. On any other error, it
exits with code 1.

### Examples

nginx.conf
```nginx
http {
  server {
    listen 8080;

    location /ok {
      return 200;
    }
  }
}
```

Dockerfile
```dockerfile
FROM nginx:latest
COPY nginx.conf /etc/nginx/nginx.cof
COPY --from=subtlespeudonym/healthcheck:latest /healthcheck /healthcheck

EXPOSE 8080
HEALTHCHECK CMD healthcheck localhost:8080 /ok
```

### Building from source

To get the smallest, static binary and the same one included in the docker
image:
```bash
make release
```

There are, however, a few other options to building this project:
```bash
# best performance, large binary, dynamically linked
cargo build

# smallest binary, dynamically linked
cargo build --release

# statically linked
cargo build --release --target x86_64-unknown-linux-musl
```
