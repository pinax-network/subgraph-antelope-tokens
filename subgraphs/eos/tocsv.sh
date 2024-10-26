#!/bin/bash

for ENTITY in $(substreams-sink-graph-load list-entities ../../schema.graphql); do
    substreams-sink-graph-load tocsv tmp/substreams-entities tmp/substreams-csv $ENTITY 398000000 --bundle-size=10000 --graphql-schema ../../schema.graphql
done
