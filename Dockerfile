FROM dart:stable AS build
WORKDIR /work
COPY . .

# TailwindCSS
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-arm64
RUN chmod +x tailwindcss-linux-arm64
RUN mv tailwindcss-linux-arm64 tailwindcss

# Config
RUN dart --disable-analytics 
RUN dart pub get > /dev/null 2>&1

# Build
RUN dart run bin/main.dart > /dev/null


FROM nginx:alpine-slim
# Update nginx config
RUN sed -i '/^\s*#error_page\s*404/c\    error_page 404 /_404.html;' /etc/nginx/conf.d/default.conf

COPY --from=build /work/build /usr/share/nginx/html
