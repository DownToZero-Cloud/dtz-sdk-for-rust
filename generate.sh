#!/bin/bash

rm -r dtz-billing/src
openapi-generator generate -i dtz-billing-api.yaml -g rust -o dtz-billing --package-name dtz-billing -t rust-template --type-mappings 'string+FeedId=dtz_identifier::FeedId' --type-mappings 'string+ContextId=dtz_identifier::ContextId' --type-mappings 'string+IdentityId=dtz_identifier::IdentityId' --type-mappings 'string+RoleId=dtz_identifier::RoleId' --type-mappings 'string+TaskId=dtz_identifier::TaskId' --type-mappings 'string+JobId=dtz_identifier::JobId' --type-mappings 'string+ServiceId=dtz_identifier::ServiceId' --type-mappings 'string+ExecutionId=dtz_identifier::ExecutionId'
cd dtz-billing && cargo build && cd ..
rm -r dtz-billing/git_push.sh dtz-billing/.gitignore dtz-billing/.travis.yml dtz-billing/.openapi-generator dtz-billing/.openapi-generator-ignore

rm -r dtz-containerregistry/src
openapi-generator generate -i dtz-containerregistry-api.yaml -g rust -o dtz-containerregistry --package-name dtz-containerregistry -t rust-template --type-mappings 'string+FeedId=dtz_identifier::FeedId' --type-mappings 'string+ContextId=dtz_identifier::ContextId' --type-mappings 'string+IdentityId=dtz_identifier::IdentityId' --type-mappings 'string+RoleId=dtz_identifier::RoleId' --type-mappings 'string+TaskId=dtz_identifier::TaskId' --type-mappings 'string+JobId=dtz_identifier::JobId' --type-mappings 'string+ServiceId=dtz_identifier::ServiceId' --type-mappings 'string+ExecutionId=dtz_identifier::ExecutionId'
cd dtz-containerregistry && cargo build && cd ..
rm -r dtz-containerregistry/git_push.sh dtz-containerregistry/.gitignore dtz-containerregistry/.travis.yml dtz-containerregistry/.openapi-generator dtz-containerregistry/.openapi-generator-ignore

rm -r dtz-core/src
openapi-generator generate -i dtz-core-api.yaml -g rust -o dtz-core --package-name dtz-core -t rust-template --type-mappings 'string+FeedId=dtz_identifier::FeedId' --type-mappings 'string+ContextId=dtz_identifier::ContextId' --type-mappings 'string+IdentityId=dtz_identifier::IdentityId' --type-mappings 'string+RoleId=dtz_identifier::RoleId' --type-mappings 'string+TaskId=dtz_identifier::TaskId' --type-mappings 'string+JobId=dtz_identifier::JobId' --type-mappings 'string+ServiceId=dtz_identifier::ServiceId' --type-mappings 'string+ExecutionId=dtz_identifier::ExecutionId'
cd dtz-core && cargo build && cd ..
rm -r dtz-core/git_push.sh dtz-core/.gitignore dtz-core/.travis.yml dtz-core/.openapi-generator dtz-core/.openapi-generator-ignore

rm -r dtz-identity/src
openapi-generator generate -i dtz-identity-api.yaml -g rust -o dtz-identity --package-name dtz-identity -t rust-template --type-mappings 'string+FeedId=dtz_identifier::FeedId' --type-mappings 'string+ContextId=dtz_identifier::ContextId' --type-mappings 'string+IdentityId=dtz_identifier::IdentityId' --type-mappings 'string+RoleId=dtz_identifier::RoleId' --type-mappings 'string+TaskId=dtz_identifier::TaskId' --type-mappings 'string+JobId=dtz_identifier::JobId' --type-mappings 'string+ServiceId=dtz_identifier::ServiceId' --type-mappings 'string+ExecutionId=dtz_identifier::ExecutionId'
cd dtz-identity && cargo build && cd ..
rm -r dtz-identity/git_push.sh dtz-identity/.gitignore dtz-identity/.travis.yml dtz-identity/.openapi-generator dtz-identity/.openapi-generator-ignore

rm -r dtz-observability/src
openapi-generator generate -i dtz-observability-api.yaml -g rust -o dtz-observability --package-name dtz-observability -t rust-template --type-mappings 'string+FeedId=dtz_identifier::FeedId' --type-mappings 'string+ContextId=dtz_identifier::ContextId' --type-mappings 'string+IdentityId=dtz_identifier::IdentityId' --type-mappings 'string+RoleId=dtz_identifier::RoleId' --type-mappings 'string+TaskId=dtz_identifier::TaskId' --type-mappings 'string+JobId=dtz_identifier::JobId' --type-mappings 'string+ServiceId=dtz_identifier::ServiceId' --type-mappings 'string+ExecutionId=dtz_identifier::ExecutionId'
cd dtz-observability && cargo build && cd ..
rm -r dtz-observability/git_push.sh dtz-observability/.gitignore dtz-observability/.travis.yml dtz-observability/.openapi-generator dtz-observability/.openapi-generator-ignore

rm -r dtz-containers/src
openapi-generator generate -i dtz-containers-api.yaml -g rust -o dtz-containers --package-name dtz-containers -t rust-template --type-mappings 'string+FeedId=dtz_identifier::FeedId' --type-mappings 'string+ContextId=dtz_identifier::ContextId' --type-mappings 'string+IdentityId=dtz_identifier::IdentityId' --type-mappings 'string+RoleId=dtz_identifier::RoleId' --type-mappings 'string+TaskId=dtz_identifier::TaskId' --type-mappings 'string+JobId=dtz_identifier::JobId' --type-mappings 'string+ServiceId=dtz_identifier::ServiceId' --type-mappings 'string+ExecutionId=dtz_identifier::ExecutionId'
cd dtz-containers && cargo build && cd ..
rm -r dtz-containers/git_push.sh dtz-containers/.gitignore dtz-containers/.travis.yml dtz-containers/.openapi-generator dtz-containers/.openapi-generator-ignore

rm -r dtz-rss2email/src
openapi-generator generate -i dtz-rss2email-api.yaml -g rust -o dtz-rss2email --package-name dtz-rss2email -t rust-template --type-mappings 'string+FeedId=dtz_identifier::FeedId' --type-mappings 'string+ContextId=dtz_identifier::ContextId' --type-mappings 'string+IdentityId=dtz_identifier::IdentityId' --type-mappings 'string+RoleId=dtz_identifier::RoleId' --type-mappings 'string+TaskId=dtz_identifier::TaskId' --type-mappings 'string+JobId=dtz_identifier::JobId' --type-mappings 'string+ServiceId=dtz_identifier::ServiceId' --type-mappings 'string+ExecutionId=dtz_identifier::ExecutionId'
cd dtz-rss2email && cargo build && cd ..
rm -r dtz-rss2email/git_push.sh dtz-rss2email/.gitignore dtz-rss2email/.travis.yml dtz-rss2email/.openapi-generator dtz-rss2email/.openapi-generator-ignore

rm -r dtz-objectstore/src
openapi-generator generate -i dtz-objectstore-api.yaml -g rust -o dtz-objectstore --package-name dtz-objectstore -t rust-template --type-mappings 'string+FeedId=dtz_identifier::FeedId' --type-mappings 'string+ContextId=dtz_identifier::ContextId' --type-mappings 'string+IdentityId=dtz_identifier::IdentityId' --type-mappings 'string+RoleId=dtz_identifier::RoleId' --type-mappings 'string+TaskId=dtz_identifier::TaskId' --type-mappings 'string+JobId=dtz_identifier::JobId' --type-mappings 'string+ServiceId=dtz_identifier::ServiceId' --type-mappings 'string+ExecutionId=dtz_identifier::ExecutionId'
cd dtz-objectstore && cargo build && cd ..
rm -r dtz-objectstore/git_push.sh dtz-objectstore/.gitignore dtz-objectstore/.travis.yml dtz-objectstore/.openapi-generator dtz-objectstore/.openapi-generator-ignore
