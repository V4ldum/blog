#!/usr/bin/env bash

HOME="/home/blog"

cd "$HOME/blog"
git pull

BUILD_VERSION=$(git rev-parse HEAD)
echo "$(date --utc +%FT%TZ): Releasing new finance version : $BUILD_VERSION"

echo "$(date --utc +%FT%TZ): Running build..."
docker build -t blog .

echo "$(date --utc +%FT%TZ): Running container..."
cd /root
OLD_CONTAINER=$(docker ps -aqf "name=blog")

docker container rm -f $OLD_CONTAINER
docker compose up -d --no-deps --no-recreate blog