#!/bin/bash

echo "Cleaning arch/x86_64/"
cd arch/x86_64 && make clean
cd ../../arch/aarch64 && make clean
