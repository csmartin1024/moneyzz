#!/bin/bash

docker kill money-db
docker rm money-db
docker build -t postgreshy .
docker run -d -p 7878:5432 --name money-db postgreshy 