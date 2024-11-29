# Build
FROM rust:alpine AS builder


RUN apk update -q && \
    apk add -q --no-cache libc-dev

WORKDIR /work
COPY . .

RUN cargo build --release 2>&1 | grep -Ev "^\s+Compiling"
RUN cargo run --release

# Run
FROM nginx:alpine AS runner
RUN sed -i '/^\s*#error_page\s*404/c\    error_page 404 /_404.html;' /etc/nginx/conf.d/default.conf
COPY --from=builder /work/build /usr/share/nginx/html