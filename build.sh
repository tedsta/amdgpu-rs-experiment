XARGO_RUST_SRC=/home/tedsta/code/amdgpu/rust/src
xargo rustc -v --release --target amdgcn-amd-amdhsa -- --emit=asm
