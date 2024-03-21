# API client generation

use `generate.sh` for client generation.

# terraform provider generation

```
~/go/bin/tfplugingen-openapi generate --config dtz-rss2email-tf-config.yaml --output dtz-rss2email-provider-code-spec.json dtz-rss2email-api.yaml
```

```
~/go/bin/tfplugingen-framework generate all \
    --input dtz-rss2email-provider-code-spec.json \
    --output internal/provider
```