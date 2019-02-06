PATH=$PATH:/home/tedsta/code/amdgpu/rust/build/x86_64-unknown-linux-gnu/llvm/bin/
mkdir -p build/
clang -x assembler -target amdgcn-amd-amdhsa -mcpu=hawaii -c -o build/test.o target/amdgcn-amd-amdhsa/release/deps/test2-*.s
clang -target amdgcn-amd-amdhsa build/test.o -o build/test.co
