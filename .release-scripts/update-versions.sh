#!/bin/bash
set -e

VERSION=$1

if [ -z "$VERSION" ]; then
  echo "Usage: $0 <version>"
  exit 1
fi

echo "Updating versions to: $VERSION"

# Update Cargo workspace versions
echo "Updating Cargo.toml versions..."
cargo set-version "$VERSION"
cargo update -w

# Check if there are any non-chart changes in this release
# by checking git log since last tag
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
if [ -n "$LAST_TAG" ]; then
  # Check if there are changes outside of charts/ directory
  NON_CHART_CHANGES=$(git log "$LAST_TAG..HEAD" --oneline --no-merges -- . ':(exclude)charts/' ':(exclude).github/workflows/' ':(exclude).releaserc.json' ':(exclude).release-scripts/' | wc -l)
else
  NON_CHART_CHANGES=1
fi

# Update Helm chart
echo "Updating Helm chart versions..."
CHART_FILE="charts/sgbf/Chart.yaml"

# Get current chart version
CURRENT_CHART_VERSION=$(grep '^version:' "$CHART_FILE" | awk '{print $2}')

if [ "$NON_CHART_CHANGES" -gt 0 ]; then
  # App code changed - update both appVersion and chart version
  echo "Detected application code changes - updating appVersion to $VERSION"
  sed -i "s/^appVersion: .*/appVersion: \"$VERSION\"/" "$CHART_FILE"

  # Bump chart version to match (same as app version for simplicity)
  sed -i "s/^version: .*/version: $VERSION/" "$CHART_FILE"
else
  # Only chart changes - bump chart version only
  echo "Only Helm chart changes detected - keeping appVersion unchanged"

  # Parse version and increment patch
  IFS='.' read -r -a version_parts <<< "$CURRENT_CHART_VERSION"
  major="${version_parts[0]}"
  minor="${version_parts[1]}"
  patch="${version_parts[2]}"

  # Increment patch version
  new_patch=$((patch + 1))
  NEW_CHART_VERSION="$major.$minor.$new_patch"

  sed -i "s/^version: .*/version: $NEW_CHART_VERSION/" "$CHART_FILE"
  echo "Bumped chart version to $NEW_CHART_VERSION"
fi

echo "Version update complete!"
