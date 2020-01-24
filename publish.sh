#!/bin/bash
docker image ls
docker push $DOCKER_REPO/expense_calculator:latest
docker image ls
docker push $DOCKER_REPO/expense_calculator:"$DOCKER_TAG"