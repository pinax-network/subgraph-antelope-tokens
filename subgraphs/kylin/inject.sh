#!/bin/bash

for ENTITY in $(substreams-sink-graph-load list-entities ../../schema.graphql); do
    substreams-sink-graph-load inject-csv QmXKuMJiBMNH66ujrc9YGpLLmtAtPoBMswEJ3tqsuWotkt ./tmp/substreams-csv $entity ../../schema.graphql 'postgresql://graph-node:let-me-in@localhost:5432/database' 2 3000000
done
