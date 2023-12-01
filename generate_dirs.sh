#!/usr/bin/env bash

for day in {1..25}
do
    cargo init "day$day-part1"
    cargo init "day$day-part2"
done
