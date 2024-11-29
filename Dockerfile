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
COPY --from=builder /work/build /usr/share/nginx/html