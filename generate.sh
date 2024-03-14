#!/bin/bash

openapi-generator generate -i dtz-core-api.yaml -g rust -o dtz-core --package-name dtz-core -t rust-template
cd dtz-core && cargo build && cd ..
rm -r dtz-core/git_push.sh dtz-core/.gitignore dtz-core/.travis.yml dtz-core/.openapi-generator dtz-core/.openapi-generator-ignore

openapi-generator generate -i dtz-identity-api.yaml -g rust -o dtz-identity --package-name dtz-identity -t rust-template
cd dtz-identity && cargo build && cd ..
rm -r dtz-identity/git_push.sh dtz-identity/.gitignore dtz-identity/.travis.yml dtz-identity/.openapi-generator dtz-identity/.openapi-generator-ignore

openapi-generator generate -i dtz-observability-api.yaml -g rust -o dtz-observability --package-name dtz-observability -t rust-template
cd dtz-observability && cargo build && cd ..
rm -r dtz-observability/git_push.sh dtz-observability/.gitignore dtz-observability/.travis.yml dtz-observability/.openapi-generator dtz-observability/.openapi-generator-ignore

openapi-generator generate -i dtz-containers-api.yaml -g rust -o dtz-containers --package-name dtz-containers -t rust-template
cd dtz-containers && cargo build && cd ..
rm -r dtz-containers/git_push.sh dtz-containers/.gitignore dtz-containers/.travis.yml dtz-containers/.openapi-generator dtz-containers/.openapi-generator-ignore

openapi-generator generate -i dtz-rss2email-api.yaml -g rust -o dtz-rss2email --package-name dtz-rss2email -t rust-template
cd dtz-rss2email && cargo build && cd ..
rm -r dtz-rss2email/git_push.sh dtz-rss2email/.gitignore dtz-rss2email/.travis.yml dtz-rss2email/.openapi-generator dtz-rss2email/.openapi-generator-ignore

openapi-generator generate -i dtz-objectstore-api.yaml -g rust -o dtz-objectstore --package-name dtz-objectstore -t rust-template
cd dtz-objectstore && cargo build && cd ..
rm -r dtz-objectstore/git_push.sh dtz-objectstore/.gitignore dtz-objectstore/.travis.yml dtz-objectstore/.openapi-generator dtz-objectstore/.openapi-generator-ignore
