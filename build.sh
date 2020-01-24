#!/bin/bash

# login docker
echo "$DOCKER_PASSWORD" | docker login docker.pkg.github.com --username "$DOCKER_USER" --password-stdin

# build
docker build -t $DOCKER_REPO/expense_calculator:latest \
  -t $DOCKER_REPO/expense-calculator:"$DOCKER_TAG" .

docker image ls
