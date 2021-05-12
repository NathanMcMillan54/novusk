#!/bin/bash

echo "Installing Xtensa LLVM tools..."
git clone https://github.com/MabezDev/llvm-project.git
cd llvm-project/llvm && mkdir build/
cd build/
cmake .. -DLLVM_TARGETS_TO_BUILD="X86" -DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD="Xtensa" -DCMAKE_BUILD_TYPE=Release -G "Ninja"
cmake --build .
echo "Installing Xtensa Rust..."
git clone https://github.com/MabezDev/rust-xtensa.git
cd rust-xtensa/ && git checkout xtensa-v0.2.0
cp -r ../x.py x.py
./configure --llvm-root=../llvm-project/llvm/build/
./x.py build
