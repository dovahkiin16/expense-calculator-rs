#!/bin/bash

# login docker
echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USER" --password-stdin $DOCKER_HOST

# build
docker build -t $DOCKER_REPO/expense_calculator:latest \
  -t $DOCKER_REPO/expense-calculator:"$DOCKER_TAG" .
