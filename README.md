# Healthcheck

This project is intended as a lightweight binary that can be included in a
docker image for health checks.

For example:

nginx.conf
```nginx
http {
	server {
		listen 8080;

		location / {
			return 200;
		}
	}
}
```

Dockerfile
```docker
FROM nginx:latest
COPY nginx.conf /etc/nginx/nginx.cof
COPY --from=subtlespeudonym/healthcheck:latest /healthcheck /healthcheck

EXPOSE 8080
HEALTHCHECK CMD healthcheck localhost:8080 /ok
```
