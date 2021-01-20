#!/bin/sh

## EXIT CODES:
##  - 1: GitHub Actions did not (successfully) complete

apk add \
  curl \
  jq \
  unzip

attempt=0
workflow_id="null"

while [ $attempt -lt 3 ] && [ $workflow_id = "null" ]; do
  attempt=$((attempt + 1))

  echo "Retrieving workflow runs for commit '$CI_COMMIT_SHA' on ref '$CI_COMMIT_REF_SLUG' from GitHub Actions (attempt $attempt of 3)..."

  if [ "$CI_COMMIT_TAG" ]; then
    workflow_runs=$(curl --user "Kage-Yami:$GITHUB_API_TOKEN" \
      "https://api.github.com/repos/Kage-Yami/dynamic-dns-client-for-cloudflare/actions/runs?branch=$CI_COMMIT_TAG&event=push")
  else
    workflow_runs=$(curl --user "Kage-Yami:$GITHUB_API_TOKEN" \
      "https://api.github.com/repos/Kage-Yami/dynamic-dns-client-for-cloudflare/actions/runs?branch=$CI_COMMIT_REF_SLUG&event=push")
  fi

  echo "... workflow runs retrieved!"

  # Returns null if there are no matching runs
  workflow_id=$(echo "$workflow_runs" | jq "[ .workflow_runs[] | \
    select(.head_sha == \"$CI_COMMIT_SHA\" and .status == \"completed\" and .conclusion == \"success\") | \
    .id ] | sort | reverse | .[0]")

  if [ "$workflow_id" = "null" ]; then
    echo "No completed and successful runs were found; waiting four minutes before trying again."
    sleep 4m
  fi
done

if [ "$workflow_id" = "null" ]; then
  echo "No completed and successful runs were found; attempts exhausted! Failing..."
  exit 1
fi

if [ -z "$CI_COMMIT_TAG" ]; then
  echo "Completed and successful workflow runs found; not on a tagged pipeline, so exiting with success."
  exit 0
fi

################################################### TAGGED PIPELINES ###################################################

echo "Retrieving artifact information for workflow run..."

artifacts=$(curl --user "Kage-Yami:$GITHUB_API_TOKEN" \
  "https://api.github.com/repos/Kage-Yami/dynamic-dns-client-for-cloudflare/actions/runs/$workflow_id/artifacts")

echo "... artifact information retrieved!"

links=$(echo "$artifacts" | jq '[ .artifacts[] | { name: .name, url: .archive_download_url } ]')

for i in 0 1 2 3 4 5 6 7; do
  name=$(echo "$links" | jq --raw-output ".[$i].name")
  link=$(echo "$links" | jq --raw-output ".[$i].url")

  echo "Downloading artifact $((i + 1)): $name..."
  echo "Link: $link"

  mkdir "$name"
  curl --header "accept: application/vnd.github.v3+json" \
    --location --user "Kage-Yami:$GITHUB_API_TOKEN" "$link" --output "$name.zip"
  unzip "$name.zip" -d "$name/"

  echo "... download complete! Uploading artifact..."

  file="$name/ddns-for-cloudflare"

  if [ "$name" = "windows-x86_64" ] || [ "$name" = "windows-i686" ]; then
    file="$file.exe"
  fi

  curl --header "JOB-TOKEN: $CI_JOB_TOKEN" --upload-file "$file" \
    "$CI_API_V4_URL/projects/$CI_PROJECT_ID/packages/generic/$name/$CI_COMMIT_TAG/"

  echo "... upload complete!"
done
