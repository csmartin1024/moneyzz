#!/bin/bash

docker build -t postgreshy .
docker run -d -p 7878:5432 postgreshy