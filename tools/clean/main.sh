#!/bin/bash

echo "Cleaning main"
sh tools/clean/arch.sh
cd example/ && cargo clean
