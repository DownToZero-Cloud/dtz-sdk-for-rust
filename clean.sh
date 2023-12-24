#!/bin/bash

cd dtz-core && cargo clean && cd ..
cd dtz-identity && cargo clean && cd ..
cd dtz-observability && cargo clean && cd ..
cd dtz-containers && cargo clean && cd ..
cd dtz-flows && cargo clean && cd ..
cd dtz-objectstore && cargo clean && cd ..
