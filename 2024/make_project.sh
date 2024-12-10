#! /bin/sh

# expects: day number, old_day number
day="$1"
old_day="$2"

# creates a new go project in directory day_$day
# get input from ~/Downloads and utils from day_$old_day
mkdir "day_$day"
cd "day_$day"

go mod init "advent_of_code/2024/day_$day"
touch README.md
touch main.go
touch example.input
mv ~/Downloads/input .
cp ../day_$old_day/utils.go .

git add .

echo "Project created in day_$day"
