# Build the static site with blogen and serve it with nginx.
FROM rust:1 AS builder
RUN cargo install blogen --locked
WORKDIR /site
COPY . .
RUN blogen build

FROM nginx:alpine
COPY --from=builder /site/public /usr/share/nginx/html
EXPOSE 80
