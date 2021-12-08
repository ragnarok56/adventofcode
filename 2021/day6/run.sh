#!/bin/bash
docker run --rm -v $(pwd):/aoc node:17-alpine3.12 node /aoc/fish.js $1