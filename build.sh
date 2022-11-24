#!/bin/bash

# update module docs 
./scripts/doc.sh json/README.md json/src/lib.rs
./scripts/doc.sh json/README.md wsd/src/json.rs

cargo build

