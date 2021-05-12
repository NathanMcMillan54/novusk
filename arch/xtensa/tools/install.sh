#!/bin/bash

git clone https://github.com/MabezDev/rust-xtensa
cd rust-xtensa/ && git checkout xtensa-v0.2.0
cp -r ../x.py x.py
./configure --experimental-targets=Xtensa
./x.py build --stage 2
