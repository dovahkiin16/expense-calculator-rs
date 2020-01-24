#!/bin/bash

REPO="docker.pkg.github.com/dovahkiin16"
TAG=$(git describe)

echo "TAG $TAG"

# login docker
echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USER" --password-stdin docker.pkg.github.com

# build
docker build -t $REPO/expense_calculator:latest \
  -t $REPO/expense-calculator:"$TAG" .

# push docker image
docker push $REPO/expense_calculator:latest
docker push $REPO/expense_calculator:"$TAG"
