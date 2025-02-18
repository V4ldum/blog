# Build
FROM dart:stable AS builder

WORKDIR /work
COPY . .

RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-arm64
RUN chmod +x tailwindcss-linux-arm64
RUN mv tailwindcss-linux-arm64 tailwindcss

RUN dart --disable-analytics
RUN dart pub get --offline
RUN dart run bin/main.dart

# Run
FROM nginx:alpine AS runner
RUN sed -i '/^\s*#error_page\s*404/c\    error_page 404 /_404.html;' /etc/nginx/conf.d/default.conf
COPY --from=builder /work/build /usr/share/nginx/html