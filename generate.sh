#!/bin/bash

openapi-generator generate -i dtz-core-api.yaml -g rust -o dtz-core --package-name dtz-core -t rust-template
rm -r dtz-core/git_push.sh dtz-core/.gitignore dtz-core/.travis.yml dtz-core/.openapi-generator dtz-core/.openapi-generator-ignore

openapi-generator generate -i dtz-identity-api.yaml -g rust -o dtz-identity --package-name dtz-identity -t rust-template
rm -r dtz-identity/git_push.sh dtz-identity/.gitignore dtz-identity/.travis.yml dtz-identity/.openapi-generator dtz-identity/.openapi-generator-ignore

openapi-generator generate -i dtz-observability-api.yaml -g rust -o dtz-observability --package-name dtz-observability -t rust-template
rm -r dtz-observability/git_push.sh dtz-observability/.gitignore dtz-observability/.travis.yml dtz-observability/.openapi-generator dtz-observability/.openapi-generator-ignore

openapi-generator generate -i dtz-containers-api.yaml -g rust -o dtz-containers --package-name dtz-containers -t rust-template
rm -r dtz-containers/git_push.sh dtz-containers/.gitignore dtz-containers/.travis.yml dtz-containers/.openapi-generator dtz-containers/.openapi-generator-ignore

openapi-generator generate -i dtz-flows-api.yaml -g rust -o dtz-flows --package-name dtz-flows -t rust-template
rm -r dtz-flows/git_push.sh dtz-flows/.gitignore dtz-flows/.travis.yml dtz-flows/.openapi-generator dtz-flows/.openapi-generator-ignore