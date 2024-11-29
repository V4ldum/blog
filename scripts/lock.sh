#!/usr/bin/env bash

LOCK_FILE="/tmp/blog.lock"
HOME="/home/blog"
flock -n $LOCK_FILE "$HOME/blog/scripts/deploy_if_changed.sh" >> "$HOME/logs/blog-deploy.log" 2>&1