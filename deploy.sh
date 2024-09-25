#!/bin/bash

set -e

if [ -z "$1" ] || [ "$1" != "alpha" ]; then
    echo "Error: ENV must be 'alpha'. Usage: ./deploy.sh alpha"
    exit 1
fi
ENV=$1

git fetch
git checkout alpha
git pull origin alpha --ff-only

VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

docker build -t peersity-api:$VERSION .

set +e
docker network create --driver bridge database-postgres_network 2>/dev/null || true
docker network create --driver bridge peersity-alpha_network 2>/dev/null || true
docker stop peersity-api-alpha || true
docker rm peersity-api-alpha || true
set -e

docker run -d -p 1504:8080 \
    --name peersity-api-alpha \
    --network peersity-alpha_network \
    --network database-postgres_network \
    peersity-api:$VERSION

echo "Deployed peersity-api:$VERSION in environment $ENV"