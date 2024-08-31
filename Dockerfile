# Build
FROM rustlang/rust:nightly-alpine AS builder

RUN apk update -q && \
    apk add -q --no-cache bash curl npm libc-dev binaryen

RUN npm install -g sass

#RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
RUN cargo install -q cargo-leptos
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release 2>&1 | grep -Ev "^\s+Compiling" 

# Run
FROM scratch AS runner

WORKDIR /app

COPY --from=builder /work/target/release/blog /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="[::]:3000"
ENV LEPTOS_SITE_ROOT="site"

EXPOSE 3000
CMD ["/app/blog"]
