#!/bin/bash

# Partly from https://github.com/RSWilli/advent-of-code/blob/master/2022/rust/scripts/setupday.sh

set -e

if [ -f "$HOME/.adventofcode.session" ]; then
    SESSION=$(cat "$HOME/.adventofcode.session")
else
    echo "No session file found. Please create a file called .adventofcode.session in your home directory and put your session cookie in it."
    exit 1
fi

DATE="${1:-$(date +%d | sed 's/^0*//')}"
PADDED_DATE="$(printf "%02d" "${DATE}")"

if [ -f "src/day${PADDED_DATE}.rs" ]; then
    echo "Day ${PADDED_DATE} has already been prepared."
    exit 1
fi

echo "Setting up day $DATE…"

input="$(curl -s "https://adventofcode.com/2025/day/${DATE}/input" -H "Cookie: session=$SESSION")"

if [ -z "$input" ]; then
    echo "No input found for day $DATE"
    exit 1
fi

if [[ $input == "404 Not Found"* ]]; then
    echo "Day $DATE 404."
    exit 1
fi

if [[ $input == "Please don't repeatedly request this endpoint before it unlocks"* ]]; then
    echo "Day $DATE is not unlocked yet."
    exit 1
fi

echo -n "$input" >"inputs/day${PADDED_DATE}.txt"

touch "inputs/day${PADDED_DATE}_test1.txt"

cp -r "src/template.rs" "src/day${PADDED_DATE}.rs"

sed -i "s|day00|day${PADDED_DATE}|" "src/day${PADDED_DATE}.rs"

echo "
[[bin]]
name = \"day${PADDED_DATE}\"
path = \"src/day${PADDED_DATE}.rs\"" >>Cargo.toml
