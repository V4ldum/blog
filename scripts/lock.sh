#!/usr/bin/env bash

LOCK_FILE="/tmp/blog.lock"
HOME="/root/blog"
flock -n $LOCK_FILE -c "bash $HOME/blog/scripts/deploy_if_changed.sh" >> "$HOME/logs/blog-deploy.log" 2>&1