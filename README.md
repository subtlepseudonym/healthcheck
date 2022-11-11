# Healthcheck

[![docker_size](https://img.shields.io/docker/image-size/subtlepseudonym/healthcheck)](https://hub.docker.com/r/subtlepseudonym/healthcheck)

This project is intended as a lightweight binary that can be included in a
docker image for health checks. It totals around 279Kb with glibc and 379Kb when
statically linked.

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

```bash
# best performance, large binary, dynamically linked
cargo build

# smallest binary, dynamically linked
cargo build --release

# statically linked
cargo build --release --target x86_64-unknown-linux-musl
```
