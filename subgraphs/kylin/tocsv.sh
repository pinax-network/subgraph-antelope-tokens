#!/bin/bash

for ENTITY in $(substreams-sink-graph-load list-entities ../../schema.graphql); do
    substreams-sink-graph-load tocsv tmp/substreams-entities tmp/substreams-csv $ENTITY 3000000 --bundle-size 1000000 --graphql-schema ../../schema.graphql
done
