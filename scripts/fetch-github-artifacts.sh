#!/usr/bin/sh

## EXIT CODES:
##  - 1: GitHub Actions did not (successfully) complete

apk add \
  curl \
  jq

retries=0
workflow_id="null"

while [ $retries -lt 3 ] && [ $workflow_id = "null" ]; do
  retries=$((retries + 1))

  workflow_runs=$(curl --user "Kage-Yami:$GITHUB_API_TOKEN" \
    "https://api.github.com/repos/Kage-Yami/dynamic-dns-client-for-cloudflare/actions/runs?branch=develop&event=push")

  workflow_id=$(echo "$workflow_runs" | jq ".workflow_runs[] | \
    select(.head_sha == \"$CI_COMMIT_SHA\" and .status == \"completed\" and .conclusion == \"success\") \
    [id] | sort | reverse | .[0]")

  # GitHub Actions might not have finished, so wait a bit before trying again
  sleep 4m
done

# Exit with failure if GitHub Actions did not complete successfully
if [ "$workflow_id" = "null" ]; then
  exit 1
fi

# Exit early with success if we're not on a tag pipeline
if [ "$CI_COMMIT_TAG" ]; then
  exit 0
fi

################################################### TAGGED PIPELINES ###################################################

artifacts=$(curl --user "Kage-Yami:$GITHUB_API_TOKEN" \
  "https://api.github.com/repos/Kage-Yami/dynamic-dns-client-for-cloudflare/actions/runs/$workflow_id/artifacts")

links=$(echo "$artifacts" | jq '[ .artifacts[] | { name: .name, url: .archive_download_url } ]')

for i in 0 1 2 3 4 5; do
  name=$(echo "$links" | jq ".[$i].name")
  link=$(echo "$links" | jq ".[$i].url")

  mkdir "$name"
  curl --user "Kage-Yami:$GITHUB_API_TOKEN" "$link" --output "$name.zip"
  tar xvf "$name.zip" -C "$name/"

  curl --header "JOB-TOKEN: $CI_JOB_TOKEN" --upload-file "$name"/* \
    "$CI_API_V4_URL/projects/$CI_PROJECT_ID/packages/generic/$name/$CI_COMMIT_TAG/"
done
