#!/usr/bin/env bash
set -x
set -eo pipefail

RUNNING_CONTAINER=$(docker ps --filter 'name=redis' --format '{{.ID}}')
if [[ -n $RUNNING_CONTAINER ]]; then
  echo >&2 "there is a redis container already running, kill it with"
  echo >&2 "    docker kill ${RUNNING_CONTAINER}"
  exit 1
fi

# Launch Redis container
docker run \
  -p "6380:6380" \
  -d \
  --name "zero2prod_docker_redis_$(date '+%s')" \
  redis:7 \
  redis-server --appendonly yes --port 6380

>&2 echo "Redis is ready to Golang!"
