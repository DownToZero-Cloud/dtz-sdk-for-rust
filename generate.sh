#!/bin/bash

openapi-generator generate -i dtz-core-api.yaml -g rust -o dtz-core --package-name dtz-core -t rust-template
rm -r dtz-core/git_push.sh dtz-core/.gitignore dtz-core/.travis.yml dtz-core/.openapi-generator