#!/bin/bash

openapi-generator generate -i dtz-core-api.yaml -g rust -o dtz-core --package-name dtz-core -t rust-template
rm -r dtz-core/git_push.sh dtz-core/.gitignore dtz-core/.travis.yml dtz-core/.openapi-generator dtz-core/.openapi-generator-ignore

openapi-generator generate -i dtz-identity-api.yaml -g rust -o dtz-identity --package-name dtz-identity -t rust-template
rm -r dtz-identity/git_push.sh dtz-identity/.gitignore dtz-identity/.travis.yml dtz-identity/.openapi-generator dtz-identity/.openapi-generator-ignore