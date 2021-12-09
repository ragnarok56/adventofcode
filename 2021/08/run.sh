#!/bin/bash
docker build -t aoc_20211208:latest .
test $? -eq 0 && docker run --rm -v $(pwd):/aoc aoc_20211208:latest signals $@