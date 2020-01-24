#!/bin/bash

docker push $DOCKER_REPO/expense_calculator:latest
docker push $DOCKER_REPO/expense_calculator:"$DOCKER_TAG"