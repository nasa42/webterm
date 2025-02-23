#!/bin/bash

set -e

VERSION=$(grep '^version = ' Cargo.toml | head -n1 | cut -d'"' -f2)

if [ -z "$VERSION" ]; then
  echo "Error: Could not extract version from Cargo.toml"
  exit 1
fi

echo "Current version is $VERSION"

jq ".version = \"$VERSION\"" frontend/package.json > frontend/package.json.tmp && \
  mv frontend/package.json.tmp frontend/package.json

LATEST_TXT="frontend/latest.txt"

if [ ! -s "$LATEST_TXT" ]; then
  echo "Initialising $LATEST_TXT"
  echo "$VERSION" > "$LATEST_TXT"
elif ! grep -Fxq "$VERSION" "$LATEST_TXT"; then
  echo "Updating $LATEST_TXT"
  sed -i "1i$VERSION" "$LATEST_TXT"
  sed -i '3,$d' "$LATEST_TXT"
else
  echo "latest.txt is already up to date"
fi


echo "cat latest.txt"
cat $LATEST_TXT

echo "Done."
